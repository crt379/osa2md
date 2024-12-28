use std::{collections::HashMap, rc::Rc, sync::RwLock, usize};

use serde_json::Value;

#[derive(Debug, Clone)]
pub enum BAPath {
    Key(String),
    Index(usize),
}

#[derive(Debug)]
pub enum CtxValue {
    Locals(String, Rc<Value>),
    Basics(String, Rc<Value>),
    BArray(String, Vec<BAPath>, Rc<Value>),
}

impl CtxValue {
    pub fn get<'a, I>(&'a self, mut keys: I) -> Option<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        let v = self.value();
        match self {
            Self::Locals(_, _) => {
                // 目前 Locals 只保存 string
                panic!("locals is string value, not object");
            }
            Self::Basics(path, value) => {
                let mut p = path.to_string();
                keys.try_fold(v, |v, k| match v {
                    Value::Object(map) => {
                        if let Some(v) = map.get(k) {
                            p.push('/');
                            p.push_str(&k.replace("/", "~1"));
                            Some(v)
                        } else {
                            if let Some(r) = map.get("$ref") {
                                if let Some(v) =
                                    value.get(&r.as_str().unwrap()[1..]).unwrap().get(k)
                                {
                                    p = format!("{}/{}", r.as_str().unwrap(), k.replace("/", "~1"));
                                    Some(v)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                    }
                    _ => None,
                })
                .map(|_| Self::Basics(p, value.clone()))
            }
            Self::BArray(path, paths, value) => {
                let mut tb = false;
                let mut p = path.to_string();
                let mut npaths = paths.clone();
                keys.try_fold(v, |v, k| match v {
                    Value::Object(map) => {
                        if let Some(v) = map.get(k) {
                            if !tb {
                                npaths.push(BAPath::Key(k.to_string()));
                            } else {
                                p = format!("{}/{}", p, k.replace("/", "~1"));
                            }
                            Some(v)
                        } else {
                            if let Some(r) = map.get("$ref") {
                                if let Some(v) =
                                    value.get(&r.as_str().unwrap()[1..]).unwrap().get(k)
                                {
                                    p = format!("{}/{}", r.as_str().unwrap(), k.replace("/", "~1"));
                                    tb = true;
                                    Some(v)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                    }
                    _ => None,
                })
                .map(|_| {
                    if !tb {
                        Self::BArray(p, npaths, value.clone())
                    } else {
                        Self::Basics(p, value.clone())
                    }
                })
            }
        }
    }

    pub fn value(&self) -> &Value {
        match self {
            Self::Locals(_path, value) => &value,
            Self::Basics(path, value) => {
                if path == "#" {
                    value.as_ref()
                } else {
                    value.as_ref().pointer(&path[1..]).unwrap()
                }
            }
            Self::BArray(path, paths, value) => paths
                .iter()
                .try_fold(
                    value.as_ref().pointer(&path[1..]).unwrap(),
                    |val, p| match p {
                        BAPath::Key(key) => match val.as_object().unwrap().get(key) {
                            Some(val) => Some(val),
                            None => match val.as_object().unwrap().get("$ref") {
                                Some(r) => value.get(&r.as_str().unwrap()[1..]).unwrap().get(key),
                                None => None,
                            },
                        },
                        BAPath::Index(index) => val.as_array().unwrap().get(*index),
                    },
                )
                .unwrap(),
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
        if key.starts_with('#') {
            return self
                .basics
                .pointer(&key[1..])
                .map(|_| Rc::new(CtxValue::Basics(key.to_string(), self.basics.clone())));
        }

        let ks = key
            .split(&['/', '.'][..])
            .map(|x| x.replace("~1", "/"))
            .collect::<Vec<_>>();

        if let Some(val) = self.locals.read().unwrap().get(&ks[0]) {
            if ks.len() > 1 {
                if let Some(v) = val.as_ref().get(ks[1..].iter().map(|s| s.as_str())) {
                    return Some(Rc::new(v));
                }
            } else {
                return Some(val.clone());
            }
        }

        if self.previou.is_some() {
            if let Some(val) = self.previou.as_ref().unwrap().read().unwrap().get(&ks[0]) {
                if ks.len() > 1 {
                    if let Some(v) = val.as_ref().get(ks[1..].iter().map(|s| s.as_str())) {
                        return Some(Rc::new(v));
                    }
                } else {
                    return Some(val.clone());
                }
            }
        } else {
            // 第一个ctx
            if let Some(val) = CtxValue::Basics("#".to_string(), self.basics.clone())
                .get(ks.iter().map(|s| s.as_str()))
            {
                return Some(Rc::new(val));
            }
        }

        None
    }
}
