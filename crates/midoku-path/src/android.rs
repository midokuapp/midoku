use std::path::PathBuf;

use dioxus::mobile::wry;
use jni::objects::{JObject, JString};
use jni::JNIEnv;

use crate::error::Result;

fn resolve<F>(f: F) -> Result<PathBuf>
where
    F: FnOnce(&mut JNIEnv, &JObject) -> Result<PathBuf> + Send + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel();
    wry::prelude::dispatch(move |env, activity, _webview| tx.send(f(env, activity)).unwrap());
    rx.recv().unwrap()
}

macro_rules! resolve {
    (file $method:expr) => {
        resolve(move |env, activity| {
            let file = call_method!(env, activity, file $method)?;
            let string = call_method!(env, file, string "getAbsolutePath")?;
            Ok(PathBuf::from(string))
        })
    };
    (string $method:expr) => {
        resolve(move |env, activity| {
            let string = call_method!(env, activity, string $method)?;
            Ok(PathBuf::from(string))
        })
    };
}

macro_rules! call_method {
    ($env:expr, $object:expr, file $method:expr) => {
        $env.call_method($object, $method, "()Ljava/io/File;", &[])
            .and_then(|obj| obj.l())
    };
    ($env:expr, $object:expr, string $method:expr) => {
        $env.call_method($object, $method, "()Ljava/lang/String;", &[])
            .and_then(|obj| obj.l())
            .map(JString::from)
            .and_then(|obj| $env.get_string(&obj).map(|java_str| String::from(java_str)))
    };
}

/// Returns the path to the suggested directory for the app's config files.
pub fn app_config_dir() -> Result<PathBuf> {
    resolve!(file "getDataDir")
}

/// Returns the path to the suggested directory for the app's data files.
pub fn app_data_dir() -> Result<PathBuf> {
    resolve!(file "getDataDir")
}

/// Returns the path to the suggested directory for the app's local data files.
pub fn app_local_data_dir() -> Result<PathBuf> {
    resolve!(file "getDataDir")
}

/// Returns the path to the suggested directory for the app's cache files.
pub fn app_cache_dir() -> Result<PathBuf> {
    resolve!(file "getCacheDir")
}

/// Returns the path to the suggested directory for the app's log files.
pub fn app_log_dir() -> Result<PathBuf> {
    resolve!(file "getDataDir").map(|dir| dir.join("logs"))
}
