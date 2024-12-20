use std::collections::HashMap;

use serde_json::Value;

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
    fn get(&self, func_name: &str) -> Option<fn(&Otd, &mut Stack, &dyn OtdFuncManageI)> {
        match func_name {
            "" => Some(echo),
            "go" => Some(go),
            "get" => Some(get),
            "tryget" => Some(tryget),
            "for" => Some(for1),
            _ => None,
        }
    }
}

pub fn echo(otd: &Otd, _stack: &mut Stack, _funcmanage: &dyn OtdFuncManageI) {
    print!("{}", otd.args[0].as_str());
}

pub fn go(otd: &Otd, stack: &mut Stack, _funcmanage: &dyn OtdFuncManageI) {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    assert!(
        otd.args.len() == 2,
        "error: there are {} parameters, but 2 are required, {:?}",
        otd.args.len(),
        otd
    );

    let path = &otd.args[0];
    let name = &otd.args[1];
    if let Some(val) = stack.get(path) {
        stack.push(HashMap::from([(name.to_string(), val.clone())]));
        return;
    }

    if let Some(val) = stack.ref_object_get(path) {
        stack.push(HashMap::from([(name.to_string(), val.clone())]));
        return;
    }

    panic!("error: not found {}, {:?}", path, otd);
}

fn _get(otd: &Otd, stack: &mut Stack, _funcmanage: &dyn OtdFuncManageI, is_try: bool) {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    let vp = |val: &Value| {
        match val {
            Value::String(s) => print!("{}", s),
            _ => print!("{}", val.to_string()),
        };

        if otd.is_line {
            print!("\n");
        }
    };

    let name = &otd.args[0];
    if let Some(val) = stack.get(name) {
        vp(val);
        return;
    }

    if let Some(val) = stack.ref_object_get(name) {
        vp(val);
        return;
    }

    if is_try {
        if otd.args.len() > 1 {
            print!("{}", otd.args[1]);
            if otd.is_line {
                print!("\n");
            }
        }
        return;
    }

    panic!("error: not found {}, {:?}", name, otd);
}

pub fn get(otd: &Otd, stack: &mut Stack, funcmanage: &dyn OtdFuncManageI) {
    _get(otd, stack, funcmanage, false);
}

pub fn tryget(otd: &Otd, stack: &mut Stack, funcmanage: &dyn OtdFuncManageI) {
    _get(otd, stack, funcmanage, true);
}

pub fn for1(otd: &Otd, stack: &mut Stack, funcmanage: &dyn OtdFuncManageI) {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    assert!(
        otd.args.len() >= 2,
        "error: fewer than 2 parameters, {:?}",
        otd
    );

    let name = &otd.args[1];
    let source = &otd.args[0];
    let val = match stack.get(source) {
        Some(val) => val,
        None => {
            if let Some(val) = stack.ref_object_get(source) {
                val
            } else {
                panic!("error: not found {}, {:?}", name, otd);
            }
        }
    };

    match val {
        Value::Array(_vec) => {}
        Value::Object(map) => {
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

                stack.push(new_stack);
                for cmd in otd.spac.as_ref().unwrap() {
                    cmd.run(stack, funcmanage);
                }
                stack.pop();
            }
        }
        _ => {}
    }
}
