use serde_json::Value;

use super::stack::Stack;

pub trait OtdFuncManageI {
    fn get(
        &self,
        func_name: &str,
    ) -> Option<fn(&Otd, &mut Stack, &dyn OtdFuncManageI) -> Option<Value>>;
}

#[derive(Debug)]
pub struct Otd {
    pub func: String,
    pub args: Vec<String>,
    pub cond: Vec<Option<Vec<String>>>,
    pub ncond: Vec<Option<Vec<String>>>,
    pub remark: Option<String>,
    pub spac: Option<Vec<Otd>>,
    // start_row, start_col, end_row, end_col
    pub row_col: (usize, usize, usize, usize),
    // 是否是一行
    pub is_line: bool,
}

impl Otd {
    pub fn new() -> Self {
        Self {
            func: String::new(),
            args: Vec::new(),
            cond: Vec::new(),
            ncond: Vec::new(),
            remark: None,
            spac: None,
            row_col: (0, 0, 0, 0),
            is_line: false,
        }
    }

    pub fn parse(rows: &Vec<&str>) -> Vec<Self> {
        let mut otds = Vec::new();
        let mut otd_state = OtdState::new();
        for (ri, row) in rows.iter().enumerate() {
            if otd_state.is_undef() && !otd_state.is_empty() && !rows[ri - 1].is_empty() {
                otd_state = otd_state.push(&mut otds, '\n', ri - 1, rows[ri - 1].len(), false);
            }

            if row.is_empty() {
                // 空行，需要添加换行符
                otd_state = otd_state.push(&mut otds, '\n', ri, 0, true);
                continue;
            }

            for (ci, c) in row.chars().enumerate() {
                otd_state = otd_state.push(&mut otds, c, ri, ci, ci == (row.len() - 1));
            }
        }

        if otd_state.is_undef() && !otd_state.is_empty() {
            otds.push(otd_state.undef2otd());
        }

        otds
    }

    pub fn run(&self, stack: &mut Stack, funcmanager: &dyn OtdFuncManageI) -> Option<Value> {
        if let Some(func) = funcmanager.get(&self.func) {
            return func(self, stack, funcmanager);
        } else {
            panic!("error: unknown func {}", self.func);
        }
    }
}

#[derive(Debug)]
enum OtdState {
    Undef(String, (usize, usize)),
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
        OtdState::Undef(String::new(), (0, 0))
    }

    fn is_undef(&self) -> bool {
        match self {
            Self::Undef(_, _) => true,
            Self::Block(_, ss) => ss.is_undef(),
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Undef(s, _) => s.is_empty(),
            Self::Block(_, ss) => ss.is_empty(),
            _ => false,
        }
    }

    fn undef2otd(self) -> Otd {
        match self {
            Self::Undef(s, _) => {
                let mut otd = Otd::new();
                otd.args.push(s);
                otd
            }
            _ => panic!("error: cannot convert to otd"),
        }
    }

    fn push(self, otds: &mut Vec<Otd>, c: char, row: usize, col: usize, c_is_end: bool) -> Self {
        match self {
            Self::Undef(mut s, (ri, ci)) => {
                if c == '$' {
                    if !s.is_empty() {
                        let mut otd = Otd::new();
                        otd.args.push(s);
                        (otd.row_col.0, otd.row_col.1) = (ri, ci);
                        (otd.row_col.2, otd.row_col.3) = (row, col);

                        if col != 0 {
                            otd.row_col.3 = col - 1;
                        }

                        otds.push(otd);
                    }

                    let mut otd = Otd::new();
                    (otd.row_col.0, otd.row_col.1) = (row, col);
                    return Self::Func(otd);
                }

                s.push(c);

                if s.len() == 1 {
                    return Self::Undef(s, (row, col));
                }

                Self::Undef(s, (ri, ci))
            }
            Self::Func(mut otd) => {
                if c == '(' {
                    return Self::Args(otd);
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
                    if otd.func.is_empty() {
                        // $();
                        return Self::BlockEnd;
                    }

                    (otd.row_col.2, otd.row_col.3) = (row, col);

                    if c_is_end && otd.row_col.1 == 0 && otd.row_col.0 == otd.row_col.2 {
                        otd.is_line = true;
                    }

                    if c_is_end && otd.row_col.1 > 0 && otd.row_col.0 == otd.row_col.2 {
                        otds.push(otd);

                        otd = Otd::new();
                        otd.args.push('\n'.to_string());
                        (otd.row_col.0, otd.row_col.1) = (row, col + 1);
                        (otd.row_col.2, otd.row_col.3) = (row, col + 1);
                    }

                    otds.push(otd);
                    return Self::Undef(String::new(), (0, 0));
                }

                otd.remark.get_or_insert(String::new()).push(c);

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
                let sub_state = sub_state.push(otd.spac.as_mut().unwrap(), c, row, col, c_is_end);
                match sub_state {
                    OtdState::BlockEnd => {
                        (otd.row_col.2, otd.row_col.3) = (row, col);
                        otds.push(otd);
                        Self::Undef(String::new(), (0, 0))
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
