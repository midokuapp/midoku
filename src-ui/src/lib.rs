mod app;
mod gallery;
mod header;
mod navigation;
mod view;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Send a command to the Tauri backend with optional JSON serializable arguments.
///
/// # Examples
///
/// ```rust,no_run
/// // With arguments:
/// let name = "Alice".to_string();
/// let result = invoke!("greet", {"name": name});
/// ```
///
/// ```rust,no_run
/// // Without arguments:
/// let result = invoke!("greet");
/// ```
#[macro_export]
macro_rules! invoke {
    ($cmd:expr) => {{
        let result = crate::invoke($cmd, wasm_bindgen::JsValue::NULL).await;
        serde_wasm_bindgen::from_value(result).unwrap()
    }};
    ($cmd:expr, $($args:tt)+) => {{
        let args = serde_json::json!($($args)+);
        let args = serde_wasm_bindgen::to_value(&args).expect("invalid arguments");
        let result = crate::invoke($cmd, args).await;
        serde_wasm_bindgen::from_value(result).unwrap()
    }};
}

pub use app::App;
