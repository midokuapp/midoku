mod extensions;
mod repository_url;

pub use extensions::StateExtensions;
pub use repository_url::StateRepositoryUrl;

use dioxus::prelude::*;
use midoku_store::Store;

use crate::model::{Extensions, Manifests};

#[derive(Clone, Copy)]
pub struct State {
    pub app_store: Signal<Store>,
    pub extensions: Signal<Extensions>,
    pub manifests: Signal<Manifests>,
}
