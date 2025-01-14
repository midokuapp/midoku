mod extensions;
mod repository_url;

pub use extensions::StateExtensions;
pub use repository_url::StateRepositoryUrl;

use dioxus::prelude::*;
use midoku_store::Store;

use crate::model::{init_extensions, Extensions, Manifests};

pub fn use_state_provider() {
    let app_store = use_signal(|| Store::open("app_data"));
    let extensions = use_signal(|| init_extensions());
    let manifests = use_signal(|| vec![]);

    use_context_provider(|| State {
        app_store,
        extensions,
        manifests,
    });
}

#[derive(Clone, Copy)]
pub struct State {
    pub app_store: Signal<Store>,
    pub extensions: Signal<Extensions>,
    pub manifests: Signal<Manifests>,
}
