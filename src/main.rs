use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
// use std::sync::LazyLock;

// static CMD_FUNC: LazyLock<HashSet<&'static str>> =
//     LazyLock::new(|| ["get", "for", "break"].into_iter().collect());

fn main() {
    let openapi = read_json("Northwind-V3.openapi3.json");

    let template = vec![
        "$go(paths, paths);",
        "$for(paths, path{/Orders}, m_o):",
        "# $get(path);\n",
        "",
        "$for(m_o, method{!parameters}, o):",
        "## $get(method);\n",
        "$;",
        "$;",
    ];

    let mut ecmds = Vec::new();
    ECmd::parse(&template, &mut ecmds, 0, 0);
    println!("{:?}", ecmds);

    let mut stack = Stack::new();
    stack.push_ref(openapi);
    for cmd in ecmds.iter() {
        cmd.run(&mut stack);
    }
}

fn read_json(filepath: &str) -> Value {
    let file_content = fs::read_to_string(filepath).expect("LogRocket: error reading file");
    serde_json::from_str(&file_content).expect("LogRocket: error serializing to JSON")
}

#[derive(Debug)]
enum StackValue {
    Ref(Value),
    Val(HashMap<String, Value>),
}

struct Stack {
    stack: Vec<StackValue>,
}

impl Stack {
    fn new() -> Self {
        Stack { stack: vec![] }
    }

    fn push(&mut self, val: HashMap<String, Value>) {
        self.stack.push(StackValue::Val(val));
    }

    fn push_ref(&mut self, val: Value) {
        self.stack.push(StackValue::Ref(val));
    }

    fn pop(&mut self) -> Option<StackValue> {
        self.stack.pop()
    }

    fn get(&self, key: &str) -> Option<&Value> {
        if self.stack.is_empty() {
            return None;
        }

        let sv = self.stack.last().unwrap();
        match sv {
            StackValue::Ref(v) => {
                if let Some(v) = v.get(key) {
                    return Some(v);
                }
            }
            StackValue::Val(v) => {
                if let Some(v) = v.get(key) {
                    return Some(v);
                }
            }
        }

        None
    }

    fn ref_object_get(&self, key: &str) -> Option<&Value> {
        if let Some(v) = self.get("$ref") {
            let ref_str = v.as_str().unwrap();
            if ref_str.chars().nth(0) == Some('#') {
                if let StackValue::Ref(v) = &self.stack[0] {
                    return v.pointer(&ref_str[1..]).unwrap().get(key);
                }
            }
        }

        None
    }
}

#[derive(Debug)]
enum ECmd {
    Static(String),
    Fun(Cmd),
}

impl ECmd {
    fn parse(
        rows: &Vec<&str>,
        ecmds: &mut Vec<Self>,
        mut start_ri: usize,
        mut start_ci: usize,
    ) -> (usize, usize) {
        let mut state = 0;
        let mut cmd = Cmd::new();
        let mut undef = String::new();
        let mut cond = Vec::new();
        let mut ncond = HashSet::new();
        'rowfor: for (ri, row) in rows.iter().enumerate() {
            if ri < start_ri {
                continue;
            }

            if row.is_empty() {
                // 空行，需要添加换行符
                undef.push('\n');
                continue;
            }

            if state == 0 && !undef.is_empty() && !rows[ri - 1].is_empty() {
                // undef 不为空，表示换行了，因为上面单独处理空行所以上一行为空行时不处理
                // 不和空行合拼处理是：空行要跳到下一行，这里不能跳到下一行
                undef.push('\n');
            }

            for (ci, c) in row.chars().enumerate() {
                if ri == start_ri && ci < start_ci {
                    continue;
                }

                // println!("parse char: {:?}", (ri, ci, c));

                match state {
                    0 => {
                        if c == '$' {
                            state = 1;
                            if !undef.is_empty() {
                                ecmds.push(ECmd::Static(undef));
                                undef = String::new();
                            }
                            continue;
                        }
                    }
                    1 => {
                        if c == '(' {
                            // func args start
                            state = 2;
                            cmd.func = undef;
                            undef = String::new();
                            continue;
                        }

                        if c == ';' {
                            // block end
                            return (ri, ci);
                        }
                    }
                    2 => {
                        if c == ')' {
                            // func args end
                            state = 3;
                            cmd.args.push(undef.trim().to_string());
                            undef.clear();
                            continue;
                        }

                        if c == '{' {
                            state = 5;
                            cmd.args.push(undef.trim().to_string());
                            undef.clear();
                            continue;
                        }

                        if c == ',' {
                            // func args separator
                            cmd.args.push(undef.trim().to_string());
                            undef.clear();
                            continue;
                        }
                    }
                    3 => {
                        if c == ':' {
                            // block start
                            state = 4;
                            undef.clear();
                            continue;
                        }

                        if c == ';' {
                            // cmd end
                            ecmds.push(ECmd::Fun(cmd));
                            cmd = Cmd::new();
                            state = 0;
                            continue;
                        }
                    }
                    4 => {
                        // block
                        state = 0;
                        let mut spac_ecmds = Vec::new();
                        (start_ri, start_ci) = ECmd::parse(rows, &mut spac_ecmds, ri, ci);
                        cmd.spac = Some(spac_ecmds);

                        ecmds.push(ECmd::Fun(cmd));
                        cmd = Cmd::new();

                        // 设置代码块后，开始处理的地方
                        if start_ci + 1 < rows[start_ri].len() {
                            start_ci += 1;
                        } else {
                            start_ci = 0;
                            start_ri += 1;
                        }

                        if start_ri >= rows.len() {
                            break 'rowfor;
                        }

                        if start_ri > ri {
                            continue 'rowfor;
                        }

                        continue;
                    }
                    5 => {
                        // parameter condition
                        if c == '}' {
                            state = 2;
                            cond.push(undef.trim().to_string());
                            undef = cmd.args.pop().unwrap();

                            for _ in 0..(cmd.args.len() - cmd.args_cond.len() - 1) {
                                cmd.args_cond.push((Vec::new(), None));
                            }
                            cmd.args_cond.push((cond, Some(ncond)));
                            cond = Vec::new();
                            ncond = HashSet::new();
                            continue;
                        }

                        if c == ',' {
                            cond.push(undef.trim().to_string());
                            continue;
                        }

                        if c == '!' {
                            state = 6;
                            continue;
                        }
                    }
                    6 => {
                        if c == '}' {
                            state = 2;
                            ncond.insert(undef.trim().to_string());
                            undef = cmd.args.pop().unwrap();

                            for _ in 0..(cmd.args.len() - cmd.args_cond.len() - 1) {
                                cmd.args_cond.push((Vec::new(), None));
                            }
                            cmd.args_cond.push((cond, Some(ncond)));
                            cond = Vec::new();
                            ncond = HashSet::new();
                            continue;
                        }

                        if c == ',' {
                            state = 5;
                            ncond.insert(undef.trim().to_string());
                            continue;
                        }
                    }
                    _ => {}
                }

                undef.push(c);
            }
        }

        if state != 0 {
            panic!(
                "parse error! finally not is ';' ! state: {}, cmd: {:?}",
                state, cmd
            );
        }

        if !undef.is_empty() {
            ecmds.push(ECmd::Static(undef));
        }

        (rows.len() - 1, rows[rows.len() - 1].len() - 1)
    }

    fn run(&self, stack: &mut Stack) {
        // println!("run: {:?}", self);
        match self {
            Self::Static(s) => print!("{}", s),
            Self::Fun(f) => {
                f.run(stack);
            }
        }
    }
}

#[derive(Debug)]
struct Cmd {
    func: String,
    args: Vec<String>,
    args_cond: Vec<(Vec<String>, Option<HashSet<String>>)>,
    spac: Option<Vec<ECmd>>,
}

impl Cmd {
    fn new() -> Self {
        Cmd {
            func: String::new(),
            args: Vec::new(),
            args_cond: Vec::new(),
            spac: None,
        }
    }

    fn run(&self, stack: &mut Stack) {
        match self.func.as_str() {
            "go" => {
                if self.args.is_empty() {
                    panic!("error: args is empty");
                }

                if self.args.len() != 2 {
                    panic!(
                        "error: there are {} parameters, but 2 are required",
                        self.args.len()
                    );
                }

                let path = &self.args[0];
                let name = &self.args[1];
                if let Some(val) = stack.get(path) {
                    stack.push(HashMap::from([(name.to_string(), val.clone())]));
                    return;
                }

                if let Some(val) = stack.ref_object_get(path) {
                    stack.push(HashMap::from([(name.to_string(), val.clone())]));
                    return;
                }

                panic!("error: not found {}", name);
            }
            "get" => {
                if self.args.is_empty() {
                    panic!("error: args is empty");
                }

                let name = &self.args[0];
                if let Some(val) = stack.get(name) {
                    print!("{}", val.to_string());
                    return;
                }

                if let Some(val) = stack.ref_object_get(name) {
                    print!("{}", val.to_string());
                    return;
                }

                panic!("error: not found {}", name);
            }
            "for" => {
                if self.args.is_empty() {
                    panic!("error: args is empty");
                }

                if self.args.len() < 2 {
                    panic!("error: fewer than 2 parameters");
                }

                let name = &self.args[1];
                let source = &self.args[0];
                let val = match stack.get(source) {
                    Some(val) => val,
                    None => {
                        if let Some(val) = stack.ref_object_get(source) {
                            val
                        } else {
                            panic!("error: not found {}", name);
                        }
                    }
                };

                match val {
                    Value::Array(_vec) => {}
                    Value::Object(map) => {
                        if !self.args_cond.is_empty() && !self.args_cond[0].0.is_empty() {
                            let map = map.clone();
                            for only in self.args_cond[0].0.iter() {
                                let mut new_stack = HashMap::from([(
                                    name.to_string(),
                                    Value::from(only.to_string()),
                                )]);
                                if self.args.len() > 2 {
                                    new_stack.insert(
                                        self.args[2].to_string(),
                                        map.get(only).unwrap().clone(),
                                    );
                                }

                                stack.push(new_stack);
                                for cmd in self.spac.as_ref().unwrap() {
                                    cmd.run(stack);
                                }
                                stack.pop();
                            }
                            return;
                        }

                        for (k, v) in map.clone().iter() {
                            if self.args_cond.len() > 1
                                && self.args_cond[1].1.is_some()
                                && self.args_cond[1].1.as_ref().unwrap().contains(k)
                            {
                                continue;
                            }

                            let mut new_stack =
                                HashMap::from([(name.to_string(), Value::from(k.as_str()))]);
                            if self.args.len() > 2 {
                                new_stack.insert(self.args[2].to_string(), v.clone());
                            }

                            stack.push(new_stack);
                            for cmd in self.spac.as_ref().unwrap() {
                                cmd.run(stack);
                            }
                            stack.pop();
                        }
                    }
                    _ => {}
                }
            }
            _ => {
                panic!("error: unknown func {}", self.func);
            }
        };
    }
}