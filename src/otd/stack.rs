use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug)]
pub enum StackValue {
    Ref(Value),
    Val(HashMap<String, Value>),
}

#[derive(Debug)]
pub struct Stack {
    stack: Vec<StackValue>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: vec![] }
    }

    pub fn push(&mut self, val: HashMap<String, Value>) {
        // println!("push: {:?}", val);
        self.stack.push(StackValue::Val(val));
    }

    pub fn push_ref(&mut self, val: Value) {
        self.stack.push(StackValue::Ref(val));
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.stack.pop()
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        if self.stack.is_empty() {
            return None;
        }

        // let mut limit = 2;
        // for sv in self.stack.iter().rev() {
        //     if limit == 0 {
        //         break;
        //     }
        //     limit -= 1;

        //     match sv {
        //         StackValue::Ref(v) => {
        //             if let Some(v) = v.get(key) {
        //                 return Some(v);
        //             }
        //         }
        //         StackValue::Val(v) => {
        //             if let Some(v) = v.get(key) {
        //                 return Some(v);
        //             }
        //         }
        //     }
        // }

        let sv = self.stack.last().unwrap();
        if key.find('.').is_some() {
            match sv {
                StackValue::Ref(v) => {
                    let v = key.split('.').try_fold(v, |target, token| match target {
                        Value::Object(map) => map.get(token),
                        _ => None,
                    });

                    if let Some(v) = v {
                        return Some(v);
                    }
                }
                StackValue::Val(v) => {
                    let mut p = key.split('.');
                    let v = v.get(p.next().unwrap()).unwrap();
                    let v = p.try_fold(v, |target, token| match target {
                        Value::Object(map) => map.get(token),
                        _ => None,
                    });

                    if let Some(v) = v {
                        return Some(v);
                    }
                }
            }
        } else {
            match sv {
                StackValue::Ref(v) => {
                    if let Some(v) = v.get(key) {
                        return Some(v);
                    }
                }
                StackValue::Val(v) => {
                    if let Some(v) = v.get(key) {
                        return Some(v);
                    }
                }
            }
        }

        None
    }

    pub fn ref_object_get(&self, key: &str) -> Option<&Value> {
        if let Some(v) = self.get("$ref") {
            let ref_str = v.as_str().unwrap();
            if ref_str.chars().nth(0) == Some('#') {
                if let StackValue::Ref(v) = &self.stack[0] {
                    return v.pointer(&ref_str[1..]).unwrap().get(key);
                }
            }
        }

        None
    }
}