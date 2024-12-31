use std::{collections::HashMap, rc::Rc, sync::RwLock, usize};

use serde_json::Value;

use crate::common::common;

#[derive(Debug, Clone)]
pub enum VPath {
    Key(String),
    Index(usize),
}

impl VPath {
    pub fn to_string(&self) -> String {
        match self {
            Self::Key(s) => s.clone(),
            Self::Index(i) => i.to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        if s.chars().next().unwrap().is_ascii_digit() {
            Self::Index(s.parse::<usize>().unwrap())
        } else {
            Self::Key(s.to_string())
        }
    }

    pub fn vec_from_string(s: &str) -> Vec<Self> {
        s.split(&['/'][..])
            .skip_while(|k| *k == "#" || *k == "/" || *k == "#/")
            .map(|k| Self::from_string(&k.replace("~1", "/")))
            .collect::<Vec<_>>()
    }
}

pub struct VPPaths<'a>(&'a Vec<VPath>);

impl<'a> VPPaths<'a> {
    pub fn new(paths: &'a Vec<VPath>) -> Self {
        Self(paths)
    }

    pub fn value<'b>(&self, val: &'b Value) -> Option<&'b Value> {
        self.0.iter().try_fold(val, |v, p| match p {
            VPath::Key(k) => {
                if let Some(obj) = v.as_object() {
                    if let Some(value) = obj.get(k) {
                        return Some(value);
                    }
                    if let Some(ref_value) = obj.get("$ref").and_then(|r| r.as_str()) {
                        if let Some(ref_obj) = val.pointer(&ref_value[1..]) {
                            return ref_obj.get(k);
                        }
                    }
                }
                None
            }
            VPath::Index(i) => v.as_array().and_then(|arr| arr.get(*i)),
        })
    }

    pub fn push_new_vec(&self, p: VPath) -> Vec<VPath> {
        common::vec_clone_and_push(self.0, p)
    }
}

#[derive(Debug, Clone)]
pub enum CtxValue {
    Locals(Vec<VPath>, Rc<Value>),
    Basics(Vec<VPath>, Rc<Value>),
}

impl CtxValue {
    pub fn get<'a, I>(&'a self, mut keys: I) -> Option<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        match self {
            Self::Locals(_, _) => {
                // 目前 Locals 只保存 string
                panic!("locals is string value, not object");
            }
            Self::Basics(paths, value) => {
                let mut npaths = paths.clone();
                keys.try_fold(self.ref_value().unwrap(), |v, k| match v {
                    Value::Object(map) => {
                        if let Some(v) = map.get(k) {
                            npaths.push(VPath::Key(k.to_string()));
                            Some(v)
                        } else {
                            if let Some(r) = map.get("$ref").and_then(|r| r.as_str()) {
                                npaths = VPath::vec_from_string(r);
                                if let Some(v) =
                                    VPPaths::new(&npaths).value(value).and_then(|v| v.get(k))
                                {
                                    npaths.push(VPath::Key(k.to_string()));
                                    Some(v)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                    }
                    Value::Array(arr) => common::parse_index(k).and_then(|i| {
                        npaths.push(VPath::Index(i));
                        arr.get(i)
                    }),
                    _ => None,
                })
                .map(|_| Self::Basics(npaths, value.clone()))
            }
        }
    }

    pub fn str_get(&self, key: &str) -> Option<Self> {
        self.get([key].iter().cloned())
    }

    pub fn index_get(&self, index: usize) -> Option<Self> {
        self.get([index.to_string().as_str()].iter().cloned())
    }

    pub fn str_get_value(&self, key: &str) -> Option<&Value> {
        let v = self.ref_value().unwrap();
        match v {
            Value::Object(map) => match map.get(key) {
                Some(v) => Some(v),
                None => {
                    if let Some(r) = map.get("$ref").and_then(|r| r.as_str()) {
                        VPPaths::new(&VPath::vec_from_string(r))
                            .value(self.value())
                            .unwrap()
                            .get(key)
                    } else {
                        None
                    }
                }
            },
            _ => None,
        }
    }

    pub fn ref_value(&self) -> Option<&Value> {
        match self {
            Self::Locals(_, value) => Some(&value),
            Self::Basics(paths, value) => VPPaths::new(paths).value(value),
        }
    }

    pub fn value(&self) -> &Value {
        match self {
            Self::Locals(_, value) => value,
            Self::Basics(_, value) => value,
        }
    }

    pub fn path(&self) -> String {
        match self {
            Self::Locals(paths, _) => paths
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join("/"),
            Self::Basics(paths, _) => paths
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join("/"),
        }
    }

    pub fn paths(&self) -> &Vec<VPath> {
        match self {
            Self::Locals(paths, _) => paths,
            Self::Basics(paths, _) => paths,
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pub basics: Rc<Value>,
    locals: Rc<RwLock<HashMap<String, Rc<CtxValue>>>>,
    // shared: Rc<RwLock<HashMap<String, Rc<CtxValue>>>>,
    previou: Option<Rc<RwLock<HashMap<String, Rc<CtxValue>>>>>,
}

impl Context {
    pub fn new(basics: Value) -> Self {
        Self {
            basics: Rc::new(basics),
            locals: Default::default(),
            // shared: Default::default(),
            previou: None,
        }
    }

    pub fn son(&self) -> Self {
        Self {
            basics: self.basics.clone(),
            locals: Default::default(),
            // shared: self.shared.clone(),
            previou: {
                if !self.locals.read().unwrap().is_empty() {
                    Some(self.locals.clone())
                } else {
                    None
                }
            },
        }
    }

    pub fn insert(&mut self, key: String, value: Rc<CtxValue>) {
        self.locals.write().unwrap().insert(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.locals.write().unwrap().remove(key);
    }

    pub fn clear(&mut self) {
        self.locals.write().unwrap().clear();
    }

    // pub fn ginsert(&self, key: String, value: Rc<CtxValue>) {
    //     self.shared.write().unwrap().insert(key, value);
    // }

    // pub fn gremove(&self, key: &str) {
    //     self.shared.write().unwrap().remove(key);
    // }

    pub fn get(&self, key: &str) -> Option<Rc<CtxValue>> {
        let ks = key
            .split(&['/', '.'][..])
            .skip_while(|k| *k == "#" || *k == "/" || *k == "#/")
            .map(|k| k.replace("~1", "/"))
            .collect::<Vec<_>>();

        let _get = |lock: &RwLock<HashMap<String, Rc<CtxValue>>>| {
            if let Some(val) = lock.read().unwrap().get(ks[0].as_str()) {
                if ks.len() > 1 {
                    if let Some(v) = val.as_ref().get(ks[1..].iter().map(|p| p.as_str())) {
                        return Some(Rc::new(v));
                    }
                } else {
                    return Some(val.clone());
                }
            }

            None
        };

        if let Some(val) = _get(self.locals.as_ref()) {
            return Some(val);
        }

        if self.previou.is_some() {
            return _get(&self.previou.as_ref().unwrap());
        } else {
            // 第一个ctx
            if let Some(val) =
                CtxValue::Basics(Vec::new(), self.basics.clone()).get(ks.iter().map(|p| p.as_str()))
            {
                return Some(Rc::new(val));
            }
        }

        None
    }
}
