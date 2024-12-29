#[derive(Debug)]
pub struct Otd {
    pub func: String,
    // arg, conds, nconds
    pub args: Vec<(String, Option<Vec<String>>, Option<Vec<String>>)>,
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
}

#[derive(Debug)]
enum OtdState {
    Undef(String, (usize, usize)),
    Func(Otd),
    Args(Otd),
    FuncNext(Otd),
    Cond(Otd),
    NCond(Otd),
    CondNext(Otd),
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
                otd.args.push((s, None, None));
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
                        otd.args.push((s, None, None));
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
                    otd.args.push((String::new(), None, None));
                    return Self::Args(otd);
                }

                if c == '{' {
                    let (_, c, n) = otd.args.last_mut().unwrap();
                    *c = Some(vec![String::new()]);
                    *n = Some(vec![String::new()]);
                    return Self::Cond(otd);
                }

                if c == ' ' {
                    if !otd.args.last().unwrap().0.is_empty() {
                        panic!("error: cannot add space in arg")
                    }
                    return Self::Args(otd);
                }

                if otd.args.is_empty() {
                    otd.args.push((String::new(), None, None));
                }

                let (arg, _, _) = otd.args.last_mut().unwrap();
                arg.push(c);

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
                        otd.args.push(('\n'.to_string(), None, None));
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
                    return Self::CondNext(otd).push(otds, ' ', row, col, c_is_end);
                }

                if c == '!' {
                    return Self::NCond(otd);
                }

                if c == ',' {
                    otd.args
                        .last_mut()
                        .unwrap()
                        .1
                        .as_mut()
                        .unwrap()
                        .push(String::new());
                    return Self::Cond(otd);
                }

                if c == ' ' {
                    if !otd.args.last().unwrap().1.as_ref().unwrap().is_empty() {
                        panic!("error: cannot add space in arg cond")
                    }
                    return Self::Cond(otd);
                }

                otd.args
                    .last_mut()
                    .unwrap()
                    .1
                    .as_mut()
                    .unwrap()
                    .last_mut()
                    .unwrap()
                    .push(c);

                Self::Cond(otd)
            }
            Self::NCond(mut otd) => {
                if c == '}' {
                    return Self::CondNext(otd).push(otds, ' ', row, col, c_is_end);
                }

                if c == ',' {
                    otd.args
                        .last_mut()
                        .unwrap()
                        .2
                        .as_mut()
                        .unwrap()
                        .push(String::new());
                    return Self::Cond(otd);
                }

                if c == ' ' {
                    if !otd.args.last().unwrap().2.as_ref().unwrap().is_empty() {
                        panic!("error: cannot add space in arg cond")
                    }
                    return Self::NCond(otd);
                }

                otd.args
                    .last_mut()
                    .unwrap()
                    .2
                    .as_mut()
                    .unwrap()
                    .last_mut()
                    .unwrap()
                    .push(c);
                Self::NCond(otd)
            }
            Self::CondNext(mut otd) => {
                let (_, c, n) = otd.args.last_mut().unwrap();
                if c.as_ref().unwrap().last().unwrap().is_empty() {
                    c.as_mut().unwrap().pop();
                }

                if c.as_ref().unwrap().is_empty() {
                    *c = None;
                }

                if n.as_ref().unwrap().last().unwrap().is_empty() {
                    n.as_mut().unwrap().pop();
                }

                if n.as_ref().unwrap().is_empty() {
                    *n = None;
                }

                return Self::Args(otd);
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
