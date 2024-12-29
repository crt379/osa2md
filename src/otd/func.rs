use std::rc::Rc;

use serde_json::{Map, Value};

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

    let path = &otd.args[0].0;
    let name = &otd.args[1].0;
    if let Some(val) = ctx.get(path) {
        return Some((name.to_string(), val));
    }

    panic!("error: not found {}, {:?}", path, otd);
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

    let name = &otd.args[0].0;
    if let Some(val) = ctx.get(name) {
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

    panic!("error: not found {}, {:?}", name, otd);
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

    if let Some(val) = ctx.get(&otd.args[0].0) {
        match val.as_ref().ref_value().unwrap() {
            Value::Array(vals) => {
                for (i, _) in vals.iter().enumerate() {
                    let item = CtxValue::Basics(
                        VPPaths::new(vps).push_new_vec(VPath::Index(i)),
                        ctx.basics.clone(),
                    );

                    for (cond, cval) in sub_conds.iter() {
                        if !item
                            .str_get_value(cond)
                            .is_some_and(|v| v.as_str().unwrap() == *cval)
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
                    if otd.args[1].1.is_some() && !otd.args[1].1.as_ref().unwrap().contains(k) {
                        continue;
                    } else if otd.args[1].1.is_none()
                        && otd.args[1].2.is_some()
                        && otd.args[1].2.as_ref().unwrap().contains(k)
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

    let val = ctx.get(&otd.args[0].0);
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

fn array_item_type(val: &Map<String, Value>, key: &str, otd: &Otd) -> String {
    let item = val.get("items").unwrap_or_else(|| 
        // schema.items
        panic!(
            "error: {}, not found items, {:?}",
            key, otd
        ));

    if let Some(e) = item.get("enum") {
        let mut ret = String::from("enum[");
        let len = ret.len();

        e.as_array().unwrap().iter().for_each(|i| {
            ret.push_str(i.as_str().unwrap());
            ret.push(',');
        });

        if ret.len() == len {
            panic!("error: {}.enum, not items, {:?}", key, otd);
        }

        ret.pop();
        ret.push(']');
        return ret;
    }

    // not in [enum]
    let it = item.get("type").unwrap_or_else(|| 
        // schema.items.type
        panic!(
            "error: {}.items, not found type, {:?}",
            key, otd
        ));

    it.to_string()
}

fn anyof_item_type(val: &Vec<Value>, key: &str, otd: &Otd) -> String {
    if val.is_empty() {
        panic!("error: {}, not items, {:?}", key, otd);
    }

    let mut ret = String::from("or[");

    val.iter().for_each(|i| match i {
        Value::Object(val) => {
            if let Some(tval) = val.get("type") {
                match tval {
                    Value::String(t) => {
                        ret.push_str(t);
                        ret.push(',');
                        return;
                    }
                    _ => panic!(
                        "error: {} item: {}, type not implemented, {:?}",
                        key,
                        tval.to_string(),
                        otd
                    ),
                }
            }
        }
        _ => panic!("error: {} item not an object, {:?}", key, otd),
    });

    ret.pop();
    ret.push(']');
    return ret;
}

pub fn osa3type(
    ctx: Context,
    otd: &Otd,
    _funcmanage: &dyn IFunc,
) -> Option<(String, Rc<CtxValue>)> {
    assert!(!otd.args.is_empty(), "error: args is empty, {:?}", otd);

    let name = &otd.args[0].0;

    if let Some(val) = ctx.get(name) {
        match val.as_ref() {
            CtxValue::Locals(_, _) => panic!("error: Locals value not implemented"),
            CtxValue::Basics(_, value) => match value.as_ref() {
                Value::Object(val) => {
                    if let Some(tval) = val.get("type") {
                        match tval {
                            Value::String(t) => match t.as_str() {
                                "array" => print!("[{}]", array_item_type(val, name, otd)),
                                _ => print!("{}", t),
                            },
                            _ => panic!(
                                "error: {}.type: {}, value type not implemented, {:?}",
                                name,
                                tval.to_string(),
                                otd
                            ),
                        }
                    }

                    if let Some(aval) = val.get("anyOf") {
                        let path = format!("{}", name);
                        match aval {
                            Value::Array(vec) => print!("{}", anyof_item_type(vec, &path, otd)),
                            _ => panic!("error: {} anyOf not an array, {:?}", name, otd),
                        }
                    }
                }
                _ => panic!("error: {} not an object, {:?}", name, otd),
            },
        }

        if otd.is_line {
            print!("\n");
        }
    }

    panic!("error: not found {}, {:?}", name, otd);
}

pub fn recurs(
    _ctx: Context,
    _otd: &Otd,
    _funcmanage: &dyn IFunc,
) -> Option<(String, Rc<CtxValue>)> {
    None
}
