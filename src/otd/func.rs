use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

use serde_json::Value;

use super::context::{Context, CtxValue, VPPaths, VPath};
use super::exec::{IFunc, RunState};
use super::otd::Otd;

pub struct FuncManage;

impl IFunc for FuncManage {
    fn get(&self, name: &str) -> Option<fn(Context, &Otd, &dyn IFunc) -> RunState> {
        match name {
            // 输出
            "" | "echo" => Some(echo),
            "debug" => Some(debug),
            // 基本
            "go" => Some(go),
            "get" => Some(get),
            "for" => Some(for1),
            "break" => Some(break1),
            "continue" => Some(continue1),
            // 递归
            "recurs" => Some(recurs),
            // 条件
            "if" => Some(if1),
            "exist" => Some(exist),
            // 变量
            "push" => Some(push),
            "drop" => Some(drop1),
            "global" => Some(global),
            // 特殊
            "osa3type" => Some(osa3type),
            // 注释
            s if s.starts_with("#") => Some(noeffect),
            _ => None,
        }
    }
}

pub fn noeffect(_: Context, _: &Otd, _: &dyn IFunc) -> RunState {
    RunState::None
}

pub fn echo(_: Context, otd: &Otd, _: &dyn IFunc) -> RunState {
    print!("{}", otd.args[0].0.as_str());
    RunState::None
}

pub fn debug(ctx: Context, otd: &Otd, _: &dyn IFunc) -> RunState {
    if !otd.args.is_empty() {
        eprintln!("{:?}", ctx.get(&otd.args[0].0.replace(".", "/")));
    }

    RunState::None
}

pub fn go(ctx: Context, otd: &Otd, _: &dyn IFunc) -> RunState {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    assert!(
        otd.args.len() == 2,
        "error: there are {} parameters, but 2 are required, {:?}",
        otd.args.len(),
        otd
    );

    if let Some(val) = ctx.get(&otd.args[0].0.replace(".", "/")) {
        return RunState::Variable(otd.args[1].0.to_string(), val);
    }

    panic!("error: not found {}, {:?}", otd.args[0].0, otd);
}

pub fn get(ctx: Context, otd: &Otd, _: &dyn IFunc) -> RunState {
    // get_base(ctx, otd, funcmanage, false)
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

        return RunState::None;
    }

    if otd.args.len() > 1 {
        print!("{}", otd.args[1].0);
        if otd.is_line {
            print!("\n");
        }

        return RunState::None;
    }

    RunState::None
}

pub fn continue1(_: Context, _: &Otd, _: &dyn IFunc) -> RunState {
    RunState::Continue
}

pub fn break1(_: Context, _: &Otd, _: &dyn IFunc) -> RunState {
    RunState::Break
}

pub fn if1(_: Context, _: &Otd, _: &dyn IFunc) -> RunState {
    RunState::None
}

fn for_by_basics(ctx: Context, otd: &Otd, funcmanage: &dyn IFunc, vps: &Vec<VPath>) -> RunState {
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
                'itemt: for (i, _) in vals.iter().enumerate() {
                    let item = CtxValue::Basics(
                        VPPaths::new(vps).push_new_vec(VPath::Index(i)),
                        ctx.basics.clone(),
                    );

                    if !sub_conds.iter().any(|(cond, cval)| {
                        item.str_get_value(cond)
                            .is_some_and(|v| v.as_str().unwrap() == *cval)
                    }) {
                        continue;
                    }

                    let forctx = ctx.son();
                    forctx.insert(otd.args[1].0.to_string(), Rc::new(item.clone()));
                    for so in otd.spac.as_ref().unwrap() {
                        match funcmanage.get(&so.func).unwrap()(forctx.clone(), so, funcmanage) {
                            RunState::Variable(n, v) => {
                                if otd.args.len() >= 3 && otd.args[2].0 == n {
                                    ctx.insert(n, v);
                                } else {
                                    forctx.insert(n, v);
                                }
                            }
                            RunState::Break | RunState::Return => {
                                if otd.args.len() >= 3 {
                                    if let Some(sobjs) = ctx.locals_get(&otd.args[2].0) {
                                        return RunState::Variable(
                                            otd.args[2].0.to_string(),
                                            sobjs,
                                        );
                                    }
                                }
                                return RunState::Return;
                            }
                            RunState::Continue => continue 'itemt,
                            _ => {}
                        }
                    }
                }

                if otd.args.len() >= 3 {
                    if let Some(sobjs) = ctx.locals_get(&otd.args[2].0) {
                        return RunState::Variable(otd.args[2].0.to_string(), sobjs);
                    }
                }

                return RunState::None;
            }
            Value::Object(map) => {
                'ftem: for (k, _) in map.iter() {
                    if otd.args[1].1.as_ref().map_or(false, |v| !v.contains(k))
                        || otd.args[1].1.is_none()
                            && otd.args[1].2.as_ref().map_or(false, |v| v.contains(k))
                    {
                        continue;
                    }

                    let forctx = ctx.son();
                    forctx.insert(
                        otd.args[1].0.to_string(),
                        Rc::new(CtxValue::Locals(Rc::new(Value::from(k.as_str())))),
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
                        match funcmanage.get(&so.func).unwrap()(forctx.clone(), so, funcmanage) {
                            RunState::Variable(n, v) => {
                                if otd.args.len() >= 4 && otd.args[3].0 == n {
                                    ctx.insert(n, v);
                                } else {
                                    forctx.insert(n, v);
                                }
                            }
                            RunState::Break | RunState::Return => {
                                if otd.args.len() >= 4 {
                                    if let Some(sobjs) = ctx.locals_get(&otd.args[3].0) {
                                        return RunState::Variable(
                                            otd.args[3].0.to_string(),
                                            sobjs,
                                        );
                                    }
                                }
                                return RunState::Return;
                            }
                            RunState::Continue => continue 'ftem,
                            _ => {}
                        }
                    }
                }

                if otd.args.len() >= 4 {
                    if let Some(sobjs) = ctx.locals_get(&otd.args[3].0) {
                        return RunState::Variable(otd.args[3].0.to_string(), sobjs);
                    }
                }

                return RunState::None;
            }
            _ => return RunState::None,
        }
    }

    RunState::None
}

fn for_by_refobjs(
    ctx: Context,
    otd: &Otd,
    funcmanage: &dyn IFunc,
    map: &Rc<RwLock<HashMap<String, Vec<VPath>>>>,
) -> RunState {
    let binding = map.read().unwrap();
    let mut kvs: Vec<(&String, &Vec<VPath>)> = binding.iter().collect();
    kvs.sort_by_key(|x| x.0);

    'ftem: for (name, paths) in kvs {
        let forctx = ctx.son();
        forctx.insert(
            otd.args[1].0.to_string(),
            Rc::new(CtxValue::Locals(Rc::new(Value::from(name.as_str())))),
        );

        if otd.args.len() > 2 {
            forctx.insert(
                otd.args[2].0.to_string(),
                Rc::new(CtxValue::Basics(paths.clone(), ctx.basics.clone())),
            );
        }

        for so in otd.spac.as_ref().unwrap() {
            match funcmanage.get(&so.func).unwrap()(forctx.clone(), so, funcmanage) {
                RunState::Variable(n, v) => {
                    if otd.args.len() >= 4 && otd.args[3].0 == n {
                        ctx.insert(n, v);
                    } else {
                        forctx.insert(n, v);
                    }
                }
                RunState::Break | RunState::Return => {
                    if otd.args.len() >= 4 {
                        if let Some(sobjs) = ctx.locals_get(&otd.args[3].0) {
                            return RunState::Variable(otd.args[3].0.to_string(), sobjs);
                        }
                    }
                    return RunState::Return;
                }
                RunState::Continue => continue 'ftem,
                _ => {}
            }
        }
    }

    if otd.args.len() >= 4 {
        if let Some(sobjs) = ctx.locals_get(&otd.args[3].0) {
            return RunState::Variable(otd.args[3].0.to_string(), sobjs);
        }
    }

    RunState::None
}

pub fn for1(ctx: Context, otd: &Otd, funcmanage: &dyn IFunc) -> RunState {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);
    assert!(
        otd.args.len() >= 2,
        "error: fewer than 2 parameters, {:?}",
        otd
    );

    let val = ctx.get(&otd.args[0].0.replace(".", "/"));
    if val.is_none() {
        return RunState::None;
    }

    match val
        .unwrap_or_else(|| panic!("error: not found {}, {:?}", otd.args[0].0, otd))
        .as_ref()
    {
        // CtxValue::Locals(_, _) => panic!("error: Not implemented"),
        CtxValue::Basics(path, _) => for_by_basics(ctx, otd, funcmanage, path),
        CtxValue::RefObjs(map, _) => for_by_refobjs(ctx, otd, funcmanage, map),
        // CtxValue::Array(_, _) => panic!("error: Not implemented"),
        _ => panic!("error: Not implemented"),
    }
}

pub fn recurs(ctx: Context, otd: &Otd, funcmanage: &dyn IFunc) -> RunState {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    assert!(
        otd.args.len() >= 3,
        "error: fewer than 3 parameters, {:?}",
        otd
    );

    let rc_otd = match ctx.reuseotd(&otd.args[0].0) {
        Some(this) => {
            if this.spac.is_some() && otd.spac.is_none() {
                // 递归参数可能不一样, 重新生成otd
                let mut o = otd.clone();
                o.spac = this.spac.clone();
                Rc::new(o)
            } else {
                Rc::new(otd.clone())
            }
        }
        None => {
            let o = Rc::new(otd.clone());
            ctx.insert_reuseotd(otd.args[0].0.clone(), o.clone());
            o
        }
    };

    let otd = rc_otd.as_ref();
    let source = &otd.args[1].0;
    let itemref = &otd.args[2].0;

    // source 为 none 不再递归调用
    if let Some(val) = ctx.get(&source.replace(".", "/")) {
        let ictx = ctx.son();
        ictx.insert(itemref.to_string(), val);

        for so in otd.spac.as_ref().unwrap() {
            funcmanage.get(&so.func).unwrap()(ictx.clone(), so, funcmanage);
        }
    }

    RunState::None
}

pub fn global(ctx: Context, otd: &Otd, _: &dyn IFunc) -> RunState {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);
    assert!(
        otd.args.len() >= 2,
        "error: fewer than 2 parameters, {:?}",
        otd
    );

    match otd.args[1].0.as_str() {
        "arrays" => {
            ctx.global_insert(
                otd.args[0].0.clone(),
                Rc::new(CtxValue::Arrays(Rc::default())),
            );
        }
        _ => {}
    }

    RunState::None
}

pub fn drop1(ctx: Context, otd: &Otd, _: &dyn IFunc) -> RunState {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    ctx.global_remove(otd.args[0].0.as_str());

    RunState::None
}

pub fn push(ctx: Context, otd: &Otd, _: &dyn IFunc) -> RunState {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);
    assert!(
        otd.args.len() >= 2,
        "error: fewer than 2 parameters, {:?}",
        otd
    );

    if let Some(arr) = ctx.get(&otd.args[0].0.replace(".", "/")) {
        if let Some(val) = ctx.get(&otd.args[1].0.replace(".", "/")) {
            arr.arrays_push(val.ref_value().unwrap().clone().into());
        }
    }

    RunState::None
}

pub fn exist(ctx: Context, otd: &Otd, funcmanage: &dyn IFunc) -> RunState {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);
    assert!(
        otd.args.len() >= 2,
        "error: fewer than 2 parameters, {:?}",
        otd
    );

    let Some(arr) = ctx.get(&otd.args[0].0.replace(".", "/")) else {
        return RunState::None;
    };
    let Some(val) = ctx.get(&otd.args[1].0.replace(".", "/")) else {
        return RunState::None;
    };

    match arr.as_ref() {
        CtxValue::Arrays(arr) => {
            let arr = arr.read().unwrap();
            if arr.is_empty() {
                return RunState::None;
            }

            if let Some(v) = val.ref_value().unwrap().as_str() {
                if arr.iter().any(|i| i.as_str().unwrap() == v) {
                    for so in otd.spac.as_ref().unwrap() {
                        let r = funcmanage.get(&so.func).unwrap()(ctx.clone(), so, funcmanage);
                        match r {
                            RunState::None => {}
                            _ => return r,
                        }
                    }
                }
            }
        }
        _ => return RunState::None,
    }

    RunState::None
}

fn ctx_insert_new_refobjs(ctx: &Context, k: &str, n: &str, p: Vec<VPath>) {
    let mut m = HashMap::<String, Vec<VPath>>::new();
    m.insert(n.to_string(), p);
    ctx.insert(
        k.to_string(),
        Rc::new(CtxValue::RefObjs(
            Rc::new(RwLock::new(m)),
            ctx.basics.clone(),
        )),
    );
}

fn insert_or_update_refobjs(ctx: &Context, key: &str, typestr: &str, paths: Vec<VPath>) {
    if let Some(sobjs) = ctx.locals_get(key) {
        sobjs.as_ref().refobj_insert(typestr, paths);
    } else if let Some(sobjs) = ctx.previou_get(key) {
        sobjs.as_ref().refobj_insert(typestr, paths);
    } else {
        ctx_insert_new_refobjs(ctx, key, typestr, paths);
    }
}

fn xxxof(ctx: &Context, otd: &Otd, val: &Rc<CtxValue>, of: &str) -> Option<String> {
    if let Some(of_val) = val.str_get(of) {
        if let Some(v) = of_val.ref_value() {
            match v {
                Value::Array(vec) => {
                    let mut types = Vec::with_capacity(vec.len());
                    vec.iter().enumerate().for_each(|(i, v)| match v {
                        Value::Object(_) => {
                            let item = of_val.index_get2(i).unwrap();
                            let item_type_str =
                                item.str_get_value("type").unwrap().as_str().unwrap();
                            match item_type_str {
                                "object" => {
                                    let typestr;
                                    if let Some(title) = item.str_get_value("title") {
                                        typestr = title.as_str().unwrap().to_string();
                                    } else {
                                        typestr = item_type_str.to_string();
                                    }

                                    if otd.args.len() >= 2 {
                                        insert_or_update_refobjs(
                                            ctx,
                                            &otd.args[1].0,
                                            &typestr,
                                            item.paths().clone(),
                                        );
                                    }

                                    types.push(typestr)
                                }
                                _ => types.push(item_type_str.to_string()),
                            }
                        }
                        _ => panic!("error: {}/{} not an object, {:?}", val.path(), of, otd),
                    });
                    return Some(types.join(","));
                }
                _ => panic!("error: {}.{} not an array, {:?}", otd.args[0].0, of, otd),
            }
        }
    }

    None
}

pub fn osa3type(ctx: Context, otd: &Otd, _funcmanage: &dyn IFunc) -> RunState {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    let val = ctx
        .get(&otd.args[0].0.replace(".", "/"))
        .unwrap_or_else(|| panic!("error: not found {}, {:?}", otd.args[0].0, otd));

    if let Some(type_val) = val.str_get("type") {
        let tval = type_val.ref_value().unwrap();
        match tval {
            Value::String(t) => match t.as_str() {
                "array" => {
                    let items = val.str_get2("items").unwrap();
                    let typeval = items.str_get("type").unwrap();
                    let mut typestr = typeval.ref_value().unwrap().as_str().unwrap();
                    match typestr {
                        "object" => {
                            if let Some(title) = items.str_get_value("title") {
                                typestr = title.as_str().unwrap();
                            }

                            if otd.args.len() >= 2 {
                                insert_or_update_refobjs(
                                    &ctx,
                                    &otd.args[1].0,
                                    typestr,
                                    items.paths().clone(),
                                );
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
    } else if let Some(anyof) = xxxof(&ctx, otd, &val, "anyOf") {
        print!("or[{}]", anyof);
    } else if let Some(allof) = xxxof(&ctx, otd, &val, "allOf") {
        print!("all[{}]", allof);
    }

    if otd.is_line {
        print!("\n");
    }

    if otd.args.len() >= 2 {
        if let Some(sobjs) = ctx.locals_get(&otd.args[1].0) {
            return RunState::Variable(otd.args[1].0.clone(), sobjs);
        }
    }

    return RunState::None;
}
