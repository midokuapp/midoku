use serde::{de::DeserializeOwned, Serialize};

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
        pub(super) async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    }
}

/// Send a command to the Tauri backend.
#[inline(always)]
pub async fn invoke<A: Serialize, R: DeserializeOwned>(
    cmd: &str,
    args: &A,
) -> Result<R, Box<dyn std::error::Error>> {
    let args = serde_wasm_bindgen::to_value(args)?;
    let result = inner::invoke(cmd, args).await;
    let result = serde_wasm_bindgen::from_value(result)?;
    Ok(result)
}
