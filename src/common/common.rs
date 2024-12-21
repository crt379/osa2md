use serde_json::Value;

pub fn try_ref_get<'a>(val: &'a Value, key: &str, ref_str: &str) -> Option<&'a Value> {
    if ref_str.chars().nth(0) == Some('#') {
        if let Some(val) = val.pointer(&ref_str[1..]) {
            return key.split('.').try_fold(val, |val, token| match val {
                Value::Object(map) => map.get(token),
                _ => None,
            });
        }
    }

    None
}

pub fn get_or_ref<'a>(val: &'a Value, ref_val: &'a Value, key: &str) -> Option<&'a Value> {
    if let Some(v) = key.split('.').try_fold(val, |val, token| match val {
        Value::Object(map) => map.get(token),
        _ => None,
    }) {
        return Some(v);
    }

    if let Some(val) = val.get("$ref") {
        return try_ref_get(ref_val, key, val.as_str().unwrap());
    }

    None
}
