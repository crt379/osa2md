use std::collections::HashMap;

use serde_json::Value;

use crate::common::common;

use super::otd::{Otd, OtdFuncManageI};
use super::stack::Stack;

pub struct OtdFuncManage;

impl OtdFuncManage {
    pub fn new() -> Self {
        OtdFuncManage {}
    }
}

impl Default for OtdFuncManage {
    fn default() -> Self {
        Self::new()
    }
}

impl OtdFuncManageI for OtdFuncManage {
    fn get(
        &self,
        func_name: &str,
    ) -> Option<fn(&Otd, &mut Stack, &dyn OtdFuncManageI) -> Option<Value>> {
        match func_name {
            "" => Some(echo),
            "go" => Some(go),
            "ret" => Some(ret),
            "get" => Some(get),
            "tryget" => Some(tryget),
            "for" => Some(for1),
            "tryfor" => Some(tryfor),
            _ => None,
        }
    }
}

pub fn echo(otd: &Otd, _stack: &mut Stack, _funcmanage: &dyn OtdFuncManageI) -> Option<Value> {
    print!("{}", otd.args[0].as_str());
    None
}

pub fn go(otd: &Otd, stack: &mut Stack, _funcmanage: &dyn OtdFuncManageI) -> Option<Value> {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    assert!(
        otd.args.len() == 2,
        "error: there are {} parameters, but 2 are required, {:?}",
        otd.args.len(),
        otd
    );

    let path = &otd.args[0];
    let name = &otd.args[1];
    if let Some(val) = stack.get_or_ref(path) {
        stack.push_hval(HashMap::from([(name.to_string(), val.clone())]));
        return None;
    }

    panic!("error: not found {}, {:?}", path, otd);
}

pub fn ret(_otd: &Otd, stack: &mut Stack, _funcmanage: &dyn OtdFuncManageI) -> Option<Value> {
    stack.pop();
    return None;
}

fn _get(
    otd: &Otd,
    stack: &mut Stack,
    _funcmanage: &dyn OtdFuncManageI,
    is_try: bool,
) -> Option<Value> {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    let name = &otd.args[0];
    if let Some(val) = stack.get_or_ref(name) {
        match val {
            Value::String(s) => print!("{}", s),
            _ => print!("{}", val.to_string()),
        };

        if otd.is_line {
            print!("\n");
        }

        return None;
    }

    if is_try {
        if otd.args.len() > 1 {
            print!("{}", otd.args[1]);
            if otd.is_line {
                print!("\n");
            }
        }
        return None;
    }

    println!("stack: {:?}", stack.get_or_ref(name));

    panic!("error: not found {}, {:?}", name, otd);
}

pub fn get(otd: &Otd, stack: &mut Stack, funcmanage: &dyn OtdFuncManageI) -> Option<Value> {
    _get(otd, stack, funcmanage, false)
}

pub fn tryget(otd: &Otd, stack: &mut Stack, funcmanage: &dyn OtdFuncManageI) -> Option<Value> {
    _get(otd, stack, funcmanage, true)
}

fn _for(
    otd: &Otd,
    stack: &mut Stack,
    funcmanage: &dyn OtdFuncManageI,
    is_try: bool,
) -> Option<Value> {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    assert!(
        otd.args.len() >= 2,
        "error: fewer than 2 parameters, {:?}",
        otd
    );

    let name = &otd.args[1];
    let source = &otd.args[0];
    let val = stack.get_or_ref(source);

    if val.is_none() && is_try {
        return None;
    }

    match val.unwrap_or_else(|| panic!("error: not found {}, {:?}", source, otd)) {
        Value::Array(vals) => {
            let mut sub_conds = Vec::<(&str, &str)>::new();
            if otd.cond.len() > 1 && otd.cond[1].is_some() {
                for cond in otd.cond[1].as_ref().unwrap() {
                    if let Some(c) = cond.split_once(':') {
                        sub_conds.push(c);
                    }
                }
            }

            for v in vals.clone().iter() {
                for (cond, cond_val) in sub_conds.iter() {
                    if !common::get_or_ref(v, stack.first_value().unwrap(), cond)
                        .is_some_and(|v| v.as_str().unwrap() == *cond_val)
                    {
                        continue;
                    }

                    let new_stack = HashMap::from([(name.to_string(), v.clone())]);
                    stack.push_hval(new_stack);
                    for cmd in otd.spac.as_ref().unwrap() {
                        cmd.run(stack, funcmanage);
                    }
                    stack.pop();
                }
            }

            None
        }
        Value::Object(map) => {
            let mut sub_conds = Vec::<(&str, &str)>::new();
            if otd.cond.len() > 2 && otd.cond[2].is_some() {
                for cond in otd.cond[2].as_ref().unwrap() {
                    if let Some(c) = cond.split_once(':') {
                        sub_conds.push(c);
                    }
                }
            }

            for (k, v) in map.clone().iter() {
                if otd.cond.len() > 1
                    && otd.cond[1].is_some()
                    && !otd.cond[1].as_ref().unwrap().contains(k)
                {
                    continue;
                } else if !(otd.cond.len() > 1 && otd.cond[1].is_some())
                    && otd.ncond.len() > 1
                    && otd.ncond[1].is_some()
                    && otd.ncond[1].as_ref().unwrap().contains(k)
                {
                    continue;
                }

                let mut new_stack = HashMap::from([(name.to_string(), Value::from(k.as_str()))]);
                if otd.args.len() > 2 {
                    new_stack.insert(otd.args[2].to_string(), v.clone());
                }

                stack.push_hval(new_stack);
                for cmd in otd.spac.as_ref().unwrap() {
                    cmd.run(stack, funcmanage);
                }
                stack.pop();
            }

            None
        }
        _ => None,
    }
}

pub fn for1(otd: &Otd, stack: &mut Stack, funcmanage: &dyn OtdFuncManageI) -> Option<Value> {
    _for(otd, stack, funcmanage, false)
}

pub fn tryfor(otd: &Otd, stack: &mut Stack, funcmanage: &dyn OtdFuncManageI) -> Option<Value> {
    _for(otd, stack, funcmanage, true)
}
