use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    pub type Store;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "store"])]
    pub async fn load(path: &str) -> Store;

    #[wasm_bindgen(method)]
    pub async fn get(this: &Store, path: &str) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn set(this: &Store, path: &str, value: JsValue);
}
