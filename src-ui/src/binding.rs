use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Send a command to the Tauri backend.
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub(crate) async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[macro_export]
macro_rules! invoke {
    ($cmd:expr) => {{
        let result = crate::binding::invoke($cmd, wasm_bindgen::JsValue::NULL).await;
        serde_wasm_bindgen::from_value(result).unwrap()
    }};
    ($cmd:expr, $($args:tt)+) => {{
        let args = serde_json::json!($($args)+);
        let args = serde_wasm_bindgen::to_value(&args).expect("invalid arguments");
        let result = crate::binding::invoke($cmd, args).await;
        serde_wasm_bindgen::from_value(result).unwrap()
    }};
}
