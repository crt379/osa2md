use std::collections::HashMap;

use serde_json::Value;

use super::stack::Stack;

#[derive(Debug)]
pub struct Otd {
    func: String,
    args: Vec<String>,
    cond: Vec<Option<Vec<String>>>,
    ncond: Vec<Option<Vec<String>>>,
    spac: Option<Vec<Otd>>,
    debug_info: (usize, usize, usize, usize),
}

impl Otd {
    pub fn new() -> Self {
        Self {
            func: String::new(),
            args: Vec::new(),
            cond: Vec::new(),
            ncond: Vec::new(),
            spac: None,
            debug_info: (0, 0, 0, 0),
        }
    }

    pub fn parse(rows: &Vec<&str>) -> Vec<Self> {
        let mut otds = Vec::new();
        let mut otd_state = OtdState::new();
        for (ri, row) in rows.iter().enumerate() {
            if row.is_empty() {
                // 空行，需要添加换行符
                otd_state = otd_state.push(&mut otds, '\n', ri, 0);
                continue;
            }

            if otd_state.is_undef() && !otd_state.is_empty() && !rows[ri - 1].is_empty() {
                otd_state = otd_state.push(&mut otds, '\n', ri - 1, rows[ri - 1].len());
            }

            for (ci, c) in row.chars().enumerate() {
                otd_state = otd_state.push(&mut otds, c, ri, ci);
            }
        }

        if otd_state.is_undef() && !otd_state.is_empty() {
            otds.push(otd_state.undef2otd());
        }

        otds
    }

    pub fn run(&self, stack: &mut Stack) {
        match self.func.as_str() {
            "" => print!("{}", self.args[0].as_str()),
            "go" => self._go(stack),
            "get" => self._get(stack, false),
            "tryget" => self._get(stack, true),
            "for" => self._for(stack),
            _ => panic!("error: unknown func {}", self.func),
        }
    }

    #[inline(always)]
    fn _panic(&self, msg: impl std::fmt::Display) -> ! {
        panic!("{}, {:?}", msg, self);
    }

    fn _go(&self, stack: &mut Stack) {
        if self.args.is_empty() {
            self._panic("error: args is empty");
        }

        if self.args.len() != 2 {
            self._panic(format!(
                "error: there are {} parameters, but 2 are required",
                self.args.len()
            ));
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

        self._panic(format!("error: not found {}", path));
    }

    fn _get(&self, stack: &mut Stack, is_try: bool) {
        if self.args.is_empty() {
            self._panic("error: args is empty");
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

        if is_try {
            return;
        }

        self._panic(format!("error: not found {}", name));
    }

    fn _for(&self, stack: &mut Stack) {
        if self.args.is_empty() {
            self._panic("error: args is empty");
        }

        if self.args.len() < 2 {
            self._panic("error: fewer than 2 parameters");
        }

        let name = &self.args[1];
        let source = &self.args[0];
        let val = match stack.get(source) {
            Some(val) => val,
            None => {
                if let Some(val) = stack.ref_object_get(source) {
                    val
                } else {
                    self._panic(format!("error: not found {}", name))
                }
            }
        };

        match val {
            Value::Array(_vec) => {}
            Value::Object(map) => {
                if self.cond.len() > 1 && self.cond[1].is_some() {
                    let map = map.clone();
                    for only in self.cond[1].as_ref().unwrap().iter() {
                        let mut new_stack =
                            HashMap::from([(name.to_string(), Value::from(only.to_string()))]);
                        if self.args.len() > 2 {
                            new_stack
                                .insert(self.args[2].to_string(), map.get(only).unwrap().clone());
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
                    if self.ncond.len() > 1
                        && self.ncond[1].is_some()
                        && self.ncond[1].as_ref().unwrap().contains(k)
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
}

enum OtdState {
    Undef(String),
    Func(Otd),
    Args(Otd),
    FuncNext(Otd),
    Cond(Otd),
    NCond(Otd),
    Block(Otd, Box<Self>),
    BlockEnd,
}

impl OtdState {
    fn new() -> Self {
        OtdState::Undef(String::new())
    }

    fn is_undef(&self) -> bool {
        matches!(*self, Self::Undef(_))
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Undef(s) => s.is_empty(),
            _ => false,
        }
    }

    fn undef2otd(self) -> Otd {
        match self {
            Self::Undef(s) => {
                let mut otd = Otd::new();
                otd.args.push(s);
                otd
            },
            _ => panic!("error: cannot convert to Otd"),
        }
    }

    fn push(self, otds: &mut Vec<Otd>, c: char, row: usize, col: usize) -> Self {
        match self {
            Self::Undef(mut s) => {
                if c == '$' {
                    if !s.is_empty() {
                        let mut otd = Otd::new();
                        otd.args.push(s);
                        otds.push(otd);
                    }

                    let mut otd = Otd::new();
                    (otd.debug_info.0, otd.debug_info.1) = (row, col);
                    return Self::Func(otd);
                }

                s.push(c);
                Self::Undef(s)
            }
            Self::Func(mut otd) => {
                if c == '(' {
                    return Self::Args(otd);
                }

                if c == ';' {
                    return Self::BlockEnd;
                }

                otd.func.push(c);
                Self::Func(otd)
            }
            Self::Args(mut otd) => {
                if c == ')' {
                    return Self::FuncNext(otd);
                }

                if c == ',' {
                    otd.args.push(String::new());
                    return Self::Args(otd);
                }

                if c == '{' {
                    return Self::Cond(otd);
                }

                if c == ' ' {
                    // todo 中间有空格
                    return Self::Args(otd);
                }

                if otd.args.is_empty() {
                    otd.args.push(c.to_string());
                } else {
                    otd.args.last_mut().unwrap().push(c);
                }

                Self::Args(otd)
            }
            Self::FuncNext(mut otd) => {
                if c == ':' {
                    otd.spac = Some(Vec::new());
                    return Self::Block(otd, Box::new(OtdState::new()));
                }

                if c == ';' {
                    (otd.debug_info.2, otd.debug_info.3) = (row, col);
                    otds.push(otd);
                    return Self::Undef(String::new());
                }

                Self::FuncNext(otd)
            }
            Self::Cond(mut otd) => {
                if c == '}' {
                    return Self::Args(otd);
                }

                if c == '!' {
                    return Self::NCond(otd);
                }

                if c == ',' {
                    otd.cond
                        .last_mut()
                        .unwrap()
                        .get_or_insert(vec![String::new()])
                        .push(String::new());
                    return Self::Cond(otd);
                }

                if c == ' ' {
                    // todo 中间有空格
                    return Self::Cond(otd);
                }

                for _ in 0..otd.args.len() - otd.cond.len() {
                    otd.cond.push(None);
                }

                otd.cond
                    .last_mut()
                    .unwrap()
                    .get_or_insert(vec![String::new()])
                    .last_mut()
                    .get_or_insert(&mut String::new())
                    .push(c);

                Self::Cond(otd)
            }
            Self::NCond(mut otd) => {
                if c == '}' {
                    return Self::Args(otd);
                }

                if c == ',' {
                    otd.ncond
                        .last_mut()
                        .unwrap()
                        .get_or_insert(Vec::new())
                        .push(String::new());
                    return Self::Cond(otd);
                }

                if c == ' ' {
                    // todo 中间有空格
                    return Self::NCond(otd);
                }

                for _ in 0..(otd.args.len() - otd.ncond.len()) {
                    otd.ncond.push(None);
                }

                otd.ncond
                    .last_mut()
                    .unwrap()
                    .get_or_insert(Vec::new())
                    .last_mut()
                    .get_or_insert(&mut String::new())
                    .push(c);

                Self::NCond(otd)
            }
            Self::Block(mut otd, sub_state) => {
                let sub_state = sub_state.push(otd.spac.as_mut().unwrap(), c, row, col);
                match sub_state {
                    OtdState::BlockEnd => {
                        (otd.debug_info.2, otd.debug_info.3) = (row, col);
                        otds.push(otd);
                        Self::Undef(String::new())
                    }
                    _ => Self::Block(otd, Box::new(sub_state)),
                }
            }
            Self::BlockEnd => {
                // 正常不会出现这种情况，以 Self::BlockEnd 返回 Block 应该扭转为 End
                panic!("error: the otd block does not turn around properly to end!");
            }
        }
    }
}
