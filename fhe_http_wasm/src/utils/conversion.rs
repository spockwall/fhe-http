use fhe_http_core::serde_json;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[allow(unused)]
pub(crate) fn hashmap_to_js_object(map: &HashMap<String, String>) -> js_sys::Object {
    let js_object = js_sys::Object::new();
    for (key, value) in map.iter() {
        js_sys::Reflect::set(
            &js_object,
            &JsValue::from_str(key),
            &JsValue::from_str(value),
        )
        .unwrap();
    }
    js_object
}

#[allow(unused)]
pub(crate) fn js_object_to_hashmap(js_object: &js_sys::Object) -> HashMap<String, String> {
    let keys = js_sys::Reflect::own_keys(&js_object).unwrap();
    let mut map = HashMap::new();
    for key in keys.iter() {
        let key = key.as_string().unwrap();
        let value = js_sys::Reflect::get(&js_object, &JsValue::from_str(&key))
            .unwrap()
            .as_string()
            .unwrap();
        map.insert(key, value);
    }
    map
}

#[allow(unused)]
pub(crate) fn hashmap_to_json_str(
    map: &HashMap<String, String>,
) -> Result<String, serde_json::Error> {
    serde_json::to_string(map)
}

#[allow(unused)]
pub(crate) fn js_object_to_json_str(
    js_object: &js_sys::Object,
) -> Result<String, serde_json::Error> {
    let map = js_object_to_hashmap(js_object);
    hashmap_to_json_str(&map)
}
