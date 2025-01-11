use std::{collections::HashMap, fmt, rc::Rc, sync::RwLock, usize};

use serde_json::{Map, Value};

use crate::common::common;

use super::otd::Otd;

static EMPTY_VPATHS: Vec<VPath> = Vec::new();

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

    pub fn vec_from_str(s: &str) -> Vec<Self> {
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

// pub struct VPaths(Vec<VPath>);

// impl VPaths {
//     pub fn new(paths: Vec<VPath>) -> Self {
//         Self(paths)
//     }

//     pub fn value<'a>(&mut self, val: &'a Value) -> Option<&'a Value> {
//         let mut paths = Vec::new();
//         if let Some(ret) = self.0.iter().try_fold(val, |v, p| match p {
//             VPath::Key(k) => {
//                 if let Some(obj) = v.as_object() {
//                     if let Some(value) = obj.get(k) {
//                         paths.push(VPath::Key(k.clone()));
//                         return Some(value);
//                     }
//                     if let Some(ref_value) = obj.get("$ref").and_then(|r| r.as_str()) {
//                         if let Some(ref_obj) = val.pointer(&ref_value[1..]) {
//                             paths = VPath::vec_from_str(ref_value);
//                             paths.push(VPath::Key(k.clone()));
//                             return ref_obj.get(k);
//                         }
//                     }
//                 }
//                 None
//             }
//             VPath::Index(i) => v.as_array().and_then(|arr| {
//                 paths.push(VPath::Index(*i));
//                 arr.get(*i)
//             }),
//         }) {
//             self.0 = paths;
//             Some(ret)
//         } else {
//             None
//         }
//     }
// }

#[derive(Clone)]
pub enum CtxValue {
    Locals(Rc<Value>),
    Basics(Vec<VPath>, Rc<Value>),
    Arrays(Rc<RwLock<Vec<Rc<Value>>>>),
    RefObjs(Rc<RwLock<HashMap<String, Vec<VPath>>>>, Rc<Value>),
}

impl CtxValue {
    pub fn get<'a, I>(&'a self, mut keys: I) -> Option<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        match self {
            Self::Basics(paths, value) => {
                let mut npaths = paths.clone();
                keys.try_fold(self.ref_value().unwrap(), |v, k| match v {
                    Value::Object(map) => Self::handle_object_map(&mut npaths, map, k, value),
                    Value::Array(arr) => common::parse_index(k).and_then(|i| {
                        npaths.push(VPath::Index(i));
                        arr.get(i)
                    }),
                    _ => None,
                })
                .map(|_| Self::Basics(npaths, value.clone()))
            }
            Self::RefObjs(map, value) => {
                let first_key = keys.next().unwrap();
                if let Some(paths) = map.as_ref().read().unwrap().get(first_key) {
                    if let Some(val) = VPPaths::new(paths).value(value) {
                        let mut npaths = paths.clone();
                        return keys
                            .try_fold(val, |v, k| match v {
                                Value::Object(map) => {
                                    Self::handle_object_map(&mut npaths, map, k, value)
                                }
                                _ => None,
                            })
                            .map(|_| Self::Basics(npaths, value.clone()));
                    }
                }

                None
            }
            _ => panic!("Not implemented"),
        }
    }

    fn handle_object_map<'a>(
        paths: &mut Vec<VPath>,
        map: &'a Map<String, Value>,
        key: &str,
        value: &'a Value,
    ) -> Option<&'a Value> {
        if let Some(v) = map.get(key) {
            paths.push(VPath::Key(key.to_string()));
            Some(v)
        } else if let Some(r) = map.get("$ref").and_then(|r| r.as_str()) {
            *paths = VPath::vec_from_str(r);
            VPPaths::new(paths)
                .value(value)
                .and_then(|v| v.get(key))
                .map(|v| {
                    paths.push(VPath::Key(key.to_string()));
                    v
                })
        } else {
            None
        }
    }

    pub fn str_get(&self, key: &str) -> Option<Self> {
        self.get([key].iter().cloned())
    }

    // pub fn index_get(&self, index: usize) -> Option<Self> {
    //     self.get([index.to_string().as_str()].iter().cloned())
    // }

    pub fn str_get_value(&self, key: &str) -> Option<&Value> {
        let v = self.ref_value().unwrap();
        match v {
            Value::Object(map) => match map.get(key) {
                Some(v) => Some(v),
                None => {
                    if let Some(r) = map.get("$ref").and_then(|r| r.as_str()) {
                        VPPaths::new(&VPath::vec_from_str(r))
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

    pub fn get2<'a, I>(&'a self, keys: I) -> Option<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        if let Some(v) = self.get(keys) {
            if let Some(r) = v.get(["$ref"].iter().cloned()) {
                let p = VPath::vec_from_str(r.ref_value().unwrap().as_str().unwrap());
                match v {
                    CtxValue::Basics(_, value) => Some(CtxValue::Basics(p, value)),
                    _ => panic!("Not implemented"),
                }
            } else {
                Some(v)
            }
        } else {
            None
        }
    }

    pub fn str_get2(&self, key: &str) -> Option<Self> {
        self.get2([key].iter().cloned())
    }

    pub fn index_get2(&self, index: usize) -> Option<Self> {
        self.get2([index.to_string().as_str()].iter().cloned())
    }

    pub fn ref_value(&self) -> Option<&Value> {
        match self {
            Self::Locals(value) => Some(&value),
            Self::Basics(paths, value) => VPPaths::new(paths).value(value),
            _ => None,
        }
    }

    pub fn value(&self) -> &Value {
        match self {
            Self::Locals(value) => value,
            Self::Basics(_, value) => value,
            _ => panic!("Not implemented"),
        }
    }

    pub fn path(&self) -> String {
        match self {
            Self::Basics(paths, _) => paths
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join("/"),
            _ => "".to_string(),
        }
    }

    pub fn paths(&self) -> &Vec<VPath> {
        match self {
            Self::Basics(paths, _) => paths,
            _ => &EMPTY_VPATHS,
        }
    }

    pub fn refobj_insert(&self, key: &str, paths: Vec<VPath>) {
        match self {
            Self::RefObjs(refobjs, _) => {
                refobjs.write().unwrap().insert(key.to_string(), paths);
            }
            _ => panic!("not in refobjs"),
        }
    }

    pub fn arrays_push(&self, value: Rc<Value>) {
        match self {
            Self::Arrays(arrays) => {
                arrays.write().unwrap().push(value);
            }
            _ => panic!("not in arrays"),
        }
    }
}

impl fmt::Debug for CtxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Locals(arg0) => f.debug_tuple("Locals").field(arg0).finish(),
            Self::Basics(arg0, _) => f.debug_tuple("Basics").field(arg0).finish(),
            Self::RefObjs(arg0, _) => f.debug_tuple("RefObjs").field(arg0).finish(),
            Self::Arrays(arg0) => f.debug_tuple("Array").field(arg0).finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub basics: Rc<Value>,
    locals: Rc<RwLock<HashMap<String, Rc<CtxValue>>>>,
    global: Rc<RwLock<HashMap<String, Rc<CtxValue>>>>,
    previou: Option<Rc<RwLock<HashMap<String, Rc<CtxValue>>>>>,
    reuseotd: Rc<RwLock<HashMap<String, Rc<Otd>>>>,
}

impl Context {
    pub fn new(basics: Value) -> Self {
        Self {
            basics: Rc::new(basics),
            locals: Default::default(),
            global: Default::default(),
            previou: None,
            reuseotd: Default::default(),
        }
    }

    pub fn son(&self) -> Self {
        Self {
            basics: self.basics.clone(),
            locals: Default::default(),
            global: self.global.clone(),
            previou: {
                if !self.locals.read().unwrap().is_empty() {
                    Some(self.locals.clone())
                } else {
                    None
                }
            },
            reuseotd: self.reuseotd.clone(),
        }
    }

    pub fn insert(&self, key: String, value: Rc<CtxValue>) {
        self.locals.write().unwrap().insert(key, value);
    }

    // pub fn remove(&self, key: &str) {
    //     self.locals.write().unwrap().remove(key);
    // }

    // pub fn clear(&self) {
    //     self.locals.write().unwrap().clear();
    // }

    // pub fn previou_insert(&self, key: String, value: Rc<CtxValue>) {
    //     if let Some(previou) = self.previou.as_ref() {
    //         previou.write().unwrap().insert(key, value);
    //     }
    // }

    pub fn global_insert(&self, key: String, value: Rc<CtxValue>) {
        self.global.write().unwrap().insert(key, value);
    }

    pub fn global_remove(&self, key: &str) {
        self.global.write().unwrap().remove(key);
    }

    fn str2keys(key: &str) -> Vec<String> {
        key.split(&['/', '.'][..])
            .skip_while(|k| *k == "#" || *k == "/" || *k == "#/")
            .map(|k| k.replace("~1", "/"))
            .collect()
    }

    fn _locals_get(&self, ks: &Vec<String>) -> Option<Rc<CtxValue>> {
        if let Some(val) = self.locals.read().unwrap().get(ks[0].as_str()) {
            if ks.len() > 1 {
                if let Some(v) = val.as_ref().get(ks[1..].iter().map(|p| p.as_str())) {
                    return Some(Rc::new(v));
                }
            } else {
                return Some(val.clone());
            }
        }
        None
    }

    fn _previou_get(&self, ks: &Vec<String>) -> Option<Rc<CtxValue>> {
        if let Some(previou) = self.previou.as_ref() {
            if let Some(val) = previou.read().unwrap().get(ks[0].as_str()) {
                if ks.len() > 1 {
                    if let Some(v) = val.as_ref().get(ks[1..].iter().map(|p| p.as_str())) {
                        return Some(Rc::new(v));
                    }
                } else {
                    return Some(val.clone());
                }
            }
        }
        None
    }

    fn _globals_get(&self, ks: &Vec<String>) -> Option<Rc<CtxValue>> {
        if let Some(val) = self.global.read().unwrap().get(ks[0].as_str()) {
            if ks.len() > 1 {
                if let Some(v) = val.as_ref().get(ks[1..].iter().map(|p| p.as_str())) {
                    return Some(Rc::new(v));
                }
            } else {
                return Some(val.clone());
            }
        }
        None
    }

    pub fn locals_get(&self, key: &str) -> Option<Rc<CtxValue>> {
        self._locals_get(&Self::str2keys(key))
    }

    pub fn previou_get(&self, key: &str) -> Option<Rc<CtxValue>> {
        self._previou_get(&Self::str2keys(key))
    }

    // pub fn globals_get(&self, key: &str) -> Option<Rc<CtxValue>> {
    //     self._globals_get(&Self::str2keys(key))
    // }

    pub fn get(&self, key: &str) -> Option<Rc<CtxValue>> {
        let ks = Self::str2keys(key);

        if let Some(val) = self._locals_get(&ks) {
            return Some(val);
        }

        if let Some(val) = self._previou_get(&ks) {
            return Some(val);
        };

        if let Some(val) = self._globals_get(&ks) {
            return Some(val);
        }

        // 第一个ctx
        CtxValue::Basics(Vec::new(), self.basics.clone())
            .get(ks.iter().map(|p| p.as_str()))
            .map(|v| Rc::new(v))
    }

    pub fn reuseotd(&self, name: &str) -> Option<Rc<Otd>> {
        self.reuseotd.read().unwrap().get(name).cloned()
    }

    pub fn insert_reuseotd(&self, name: String, otd: Rc<Otd>) {
        self.reuseotd.write().unwrap().insert(name, otd);
    }
}
