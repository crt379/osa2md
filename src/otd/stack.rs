use std::collections::HashMap;

use serde_json::Value;

use crate::common::common;

#[derive(Debug)]
pub enum StackValue {
    Val(Value),
    HVal(HashMap<String, Value>),
}

#[derive(Debug)]
pub struct Stack {
    stack: Vec<StackValue>,
    context: HashMap<String, Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
            context: HashMap::new(),
        }
    }

    pub fn push_val(&mut self, val: Value) {
        self.stack.push(StackValue::Val(val));
    }

    pub fn push_hval(&mut self, val: HashMap<String, Value>) {
        self.stack.push(StackValue::HVal(val));
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.stack.pop()
    }

    pub fn set_context(&mut self, key: String, value: Value) {
        self.context.insert(key, value);
    }

    pub fn get_context(&self, key: &str) -> Option<&Value> {
        self.context.get(key)
    }

    pub fn remove_context(&mut self, key: &str) {
        self.context.remove(key);
    }

    pub fn first_value(&self) -> Option<&Value> {
        if self.stack.is_empty() {
            return None;
        }

        match &self.stack[0] {
            StackValue::Val(v) => Some(v),
            StackValue::HVal(_) => None,
        }
    }

    pub fn last(&self) -> Option<&StackValue> {
        self.stack.last()
    }

    pub fn _get(&self, key: &str) -> Option<&Value> {
        if self.stack.is_empty() {
            return None;
        }

        match self.last().unwrap() {
            StackValue::Val(value) => {
                if let Some(ret) = key
                    .split('.')
                    .try_fold(value, |target, token| match target {
                        Value::Object(map) => map.get(token),
                        _ => None,
                    })
                {
                    return Some(ret);
                }

                None
            }
            StackValue::HVal(value) => {
                let keys: Vec<&str> = key.split('.').collect();
                if let Some(value) = value.get(keys[0]) {
                    if keys.len() == 1 {
                        return Some(value);
                    }

                    if let Some(ret) =
                        keys.clone()
                            .iter()
                            .skip(1)
                            .try_fold(value, |value, token| match value {
                                Value::Object(map) => map.get(*token),
                                _ => None,
                            })
                    {
                        return Some(ret);
                    }
                }

                None
            }
        }
    }

    pub fn get_or_ref(&self, key: &str) -> Option<&Value> {
        if self.stack.is_empty() {
            return None;
        }

        match self.last().unwrap() {
            StackValue::Val(value) => {
                if let Some(ret) = key
                    .split('.')
                    .try_fold(value, |target, token| match target {
                        Value::Object(map) => map.get(token),
                        _ => None,
                    })
                {
                    return Some(ret);
                }

                if let Some(refval) = value.get("$ref") {
                    return common::try_ref_get(
                        self.first_value().unwrap(),
                        key,
                        refval.as_str().unwrap(),
                    );
                }

                None
            }
            StackValue::HVal(value) => {
                let keys: Vec<&str> = key.split('.').collect();
                if let Some(value) = value.get(keys[0]) {
                    if keys.len() == 1 {
                        return Some(value);
                    }

                    if let Some(ret) =
                        keys.clone()
                            .iter()
                            .skip(1)
                            .try_fold(value, |value, token| match value {
                                Value::Object(map) => map.get(*token),
                                _ => None,
                            })
                    {
                        return Some(ret);
                    }

                    if let Some(refval) = value.get("$ref") {
                        return common::try_ref_get(
                            self.first_value().unwrap(),
                            &keys[1..].join("."),
                            refval.as_str().unwrap(),
                        );
                    }
                }

                None
            }
        }
    }
}
