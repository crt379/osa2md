use std::rc::Rc;

use serde_json::Value;

use crate::common::common;

use super::context::{Context, CtxValue, VPPaths, VPath};
use super::exec::IFunc;
use super::otd::Otd;

pub struct FuncManage;

impl IFunc for FuncManage {
    fn get(
        &self,
        func_name: &str,
    ) -> Option<fn(Context, &Otd, &dyn IFunc) -> Option<(String, Rc<CtxValue>)>> {
        match func_name {
            "" => Some(echo),
            "go" => Some(go),
            "ret" => Some(ret),
            "get" => Some(get),
            "tryget" => Some(tryget),
            "for" => Some(for1),
            "tryfor" => Some(tryfor),
            "osa3type" => Some(osa3type),
            "recurs" => Some(recurs),
            _ => None,
        }
    }
}

pub fn echo(_: Context, otd: &Otd, _: &dyn IFunc) -> Option<(String, Rc<CtxValue>)> {
    print!("{}", otd.args[0].0.as_str());
    None
}

pub fn go(ctx: Context, otd: &Otd, _: &dyn IFunc) -> Option<(String, Rc<CtxValue>)> {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    assert!(
        otd.args.len() == 2,
        "error: there are {} parameters, but 2 are required, {:?}",
        otd.args.len(),
        otd
    );

    if let Some(val) = ctx.get(&otd.args[0].0.replace(".", "/")) {
        return Some((otd.args[1].0.to_string(), val));
    }

    panic!("error: not found {}, {:?}", otd.args[0].0, otd);
}

pub fn ret(mut ctx: Context, _: &Otd, _: &dyn IFunc) -> Option<(String, Rc<CtxValue>)> {
    ctx.clear();
    return None;
}

fn get_base(
    ctx: Context,
    otd: &Otd,
    _: &dyn IFunc,
    is_try: bool,
) -> Option<(String, Rc<CtxValue>)> {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    if let Some(val) = ctx.get(&otd.args[0].0.replace(".", "/")) {
        let v = val.as_ref().ref_value().unwrap();
        match v {
            Value::String(s) => print!("{}", s),
            _ => print!("{}", v.to_string()),
        }

        if otd.is_line {
            print!("\n");
        }

        return None;
    }

    if is_try {
        if otd.args.len() > 1 {
            print!("{}", otd.args[1].0);
            if otd.is_line {
                print!("\n");
            }
        }

        return None;
    }

    panic!("error: not found {}, {:?}", otd.args[0].0, otd);
}

pub fn get(ctx: Context, otd: &Otd, funcmanage: &dyn IFunc) -> Option<(String, Rc<CtxValue>)> {
    get_base(ctx, otd, funcmanage, false)
}

pub fn tryget(ctx: Context, otd: &Otd, funcmanage: &dyn IFunc) -> Option<(String, Rc<CtxValue>)> {
    get_base(ctx, otd, funcmanage, true)
}

fn for_basics(
    ctx: Context,
    otd: &Otd,
    funcmanage: &dyn IFunc,
    vps: &Vec<VPath>,
    _basics: &Rc<Value>,
) {
    let mut sub_conds = Vec::<(&str, &str)>::new();
    if otd.args[1].1.is_some() {
        for cond in otd.args[1].1.as_ref().unwrap() {
            if let Some(c) = cond.split_once(':') {
                sub_conds.push(c);
            }
        }
    }

    if let Some(val) = ctx.get(&otd.args[0].0.replace(".", "/")) {
        match val.as_ref().ref_value().unwrap() {
            Value::Array(vals) => {
                for (i, _) in vals.iter().enumerate() {
                    let item = CtxValue::Basics(
                        VPPaths::new(vps).push_new_vec(VPath::Index(i)),
                        ctx.basics.clone(),
                    );

                    for (cond, cval) in sub_conds.iter() {
                        if item
                            .str_get_value(cond)
                            .is_some_and(|v| v.as_str().unwrap() != *cval)
                        {
                            continue;
                        }

                        let mut forctx = ctx.son();
                        for so in otd.spac.as_ref().unwrap() {
                            let mut ictx = forctx.son();
                            ictx.insert(otd.args[1].0.to_string(), Rc::new(item.clone()));
                            if let Some((n, v)) =
                                funcmanage.get(&so.func).unwrap()(ictx, so, funcmanage)
                            {
                                forctx.insert(n, v);
                            }
                        }
                    }
                }
            }
            Value::Object(map) => {
                for (k, _) in map.iter() {
                    if otd.args[1].1.as_ref().is_some_and(|v| !v.contains(k)) {
                        continue;
                    } else if otd.args[1].1.is_none()
                        && otd.args[1].2.as_ref().is_some_and(|v| v.contains(k))
                    {
                        continue;
                    }

                    let mut forctx = ctx.son();
                    forctx.insert(
                        otd.args[1].0.to_string(),
                        Rc::new(CtxValue::Locals(
                            VPPaths::new(vps).push_new_vec(VPath::Key(k.clone())),
                            Rc::new(Value::from(k.as_str())),
                        )),
                    );

                    if otd.args.len() > 2 {
                        forctx.insert(
                            otd.args[2].0.to_string(),
                            Rc::new(CtxValue::Basics(
                                VPPaths::new(vps).push_new_vec(VPath::Key(k.clone())),
                                ctx.basics.clone(),
                            )),
                        );
                    }

                    for so in otd.spac.as_ref().unwrap() {
                        if let Some((n, v)) =
                            funcmanage.get(&so.func).unwrap()(forctx.son(), so, funcmanage)
                        {
                            forctx.insert(n, v);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn for_base(
    ctx: Context,
    otd: &Otd,
    funcmanage: &dyn IFunc,
    is_try: bool,
) -> Option<(String, Rc<CtxValue>)> {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    assert!(
        otd.args.len() >= 2,
        "error: fewer than 2 parameters, {:?}",
        otd
    );

    let val = ctx.get(&otd.args[0].0.replace(".", "/"));
    if val.is_none() && is_try {
        return None;
    }

    match val
        .unwrap_or_else(|| panic!("error: not found {}, {:?}", otd.args[0].0, otd))
        .as_ref()
    {
        CtxValue::Locals(_, _) => panic!("error: value is locals"),
        CtxValue::Basics(path, val) => {
            for_basics(ctx, otd, funcmanage, path, val);
        }
    }

    None
}

pub fn for1(ctx: Context, otd: &Otd, funcmanage: &dyn IFunc) -> Option<(String, Rc<CtxValue>)> {
    for_base(ctx, otd, funcmanage, false)
}

pub fn tryfor(ctx: Context, otd: &Otd, funcmanage: &dyn IFunc) -> Option<(String, Rc<CtxValue>)> {
    for_base(ctx, otd, funcmanage, true)
}

pub fn osa3type(
    ctx: Context,
    otd: &Otd,
    _funcmanage: &dyn IFunc,
) -> Option<(String, Rc<CtxValue>)> {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    let val = ctx
        .get(&otd.args[0].0.replace(".", "/"))
        .unwrap_or_else(|| panic!("error: not found {}, {:?}", otd.args[0].0, otd));

    if let Some(tval) = val.str_get_value("type") {
        match tval {
            Value::String(t) => match t.as_str() {
                "array" => {
                    let items = val.clone().get(["items"].iter().cloned()).unwrap();
                    let mut typestr = items.str_get_value("type").unwrap().as_str().unwrap();
                    match typestr {
                        "object" => {
                            if let Some(title) = items.str_get_value("title") {
                                typestr = title.as_str().unwrap();
                            }
                            print!("[{}]", typestr);
                        }
                        _ => {
                            if let Some(ienum) = items.str_get_value("enum") {
                                if let Some(e) = ienum.as_array() {
                                    print!(
                                        "[enum[{}]]",
                                        e.iter()
                                            .filter_map(|i| i.as_str())
                                            .collect::<Vec<&str>>()
                                            .join(",")
                                    );
                                }
                            } else {
                                print!("[{}]", typestr);
                            }
                        }
                    }
                }
                _ => print!("{}", t),
            },
            _ => panic!(
                "error: {}.type, value type not implemented, {:?}",
                otd.args[0].0, otd
            ),
        }
    }

    if let Some(aval) = val.str_get("anyOf") {
        if let Some(v) = aval.ref_value() {
            match v {
                Value::Array(vec) => {
                    let mut of_types = Vec::with_capacity(vec.len());
                    vec.iter().enumerate().for_each(|(i, v)| match v {
                        Value::Object(_) => {
                            let nps = common::vec_clone_and_pushs(
                                aval.paths(),
                                [VPath::Index(i), VPath::Key("type".to_string())],
                            );
                            let t = VPPaths::new(&nps)
                                .value(ctx.basics.as_ref())
                                .unwrap_or_else(|| {
                                    panic!("error: {} anyOf not type, {:?}", val.path(), otd)
                                });
                            of_types.push(t.as_str().unwrap());
                        }
                        _ => panic!("error: {} anyOf not an object, {:?}", val.path(), otd),
                    });
                    print!("or[{}]", of_types.join(","));
                }
                _ => panic!("error: {}.anyOf not an array, {:?}", otd.args[0].0, otd),
            }
        }
    }

    if let Some(aval) = val.str_get("allOf") {
        if let Some(v) = aval.ref_value() {
            match v {
                Value::Array(vec) => {
                    let mut of_types = Vec::with_capacity(vec.len());
                    vec.iter().enumerate().for_each(|(i, v)| match v {
                        Value::Object(_) => {
                            let nps = common::vec_clone_and_pushs(
                                aval.paths(),
                                [VPath::Index(i), VPath::Key("type".to_string())],
                            );
                            let t = VPPaths::new(&nps)
                                .value(ctx.basics.as_ref())
                                .unwrap_or_else(|| {
                                    panic!("error: {} allOf, item not type, {:?}", val.path(), otd)
                                });
                            of_types.push(t.as_str().unwrap());
                        }
                        _ => panic!("error: {} allOf not an object, {:?}", val.path(), otd),
                    });
                    print!("all[{}]", of_types.join(","));
                }
                _ => panic!("error: {}.allOf not an array, {:?}", otd.args[0].0, otd),
            }
        }
    }

    if otd.is_line {
        print!("\n");
    }

    return None;
}

pub fn recurs(
    _ctx: Context,
    _otd: &Otd,
    _funcmanage: &dyn IFunc,
) -> Option<(String, Rc<CtxValue>)> {
    None
}
