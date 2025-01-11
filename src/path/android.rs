use std::path::PathBuf;

use dioxus::mobile::wry;
use jni::objects::{JObject, JString};
use jni::JNIEnv;

use super::{Config, Result};

pub struct PathResolver<'a>(pub(crate) &'a Config<'a>);

impl PathResolver<'_> {
    fn resolve<F>(&self, f: F) -> Result<PathBuf>
    where
        F: FnOnce(&mut JNIEnv, &JObject) -> Result<PathBuf> + Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();
        wry::prelude::dispatch(move |env, activity, _webview| tx.send(f(env, activity)).unwrap());
        rx.recv().unwrap()
    }

    pub fn app_local_data_dir(&self) -> Result<PathBuf> {
        self.resolve(move |env, activity| {
            let files_dir = env
                .call_method(activity, "getFilesDir", "()Ljava/io/File;", &[])?
                .l()?;
            let files_dir: JString<'_> = env
                .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?
                .l()?
                .into();
            let files_dir: String = env.get_string(&files_dir)?.into();
            Ok(PathBuf::from(files_dir))
        })
    }
}
