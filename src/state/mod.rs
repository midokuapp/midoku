mod extensions;
mod repository_url;

pub use extensions::StateExtensions;
pub use repository_url::StateRepositoryUrl;

use dioxus::prelude::*;
use midoku_store::Store;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

use crate::model::{init_extensions, Extensions, Manifests};

pub fn use_state_provider() {
    let app_store = Store::open("app_data");
    let extensions = block_in_place(|| {
        let handle = Handle::current();
        handle.block_on(init_extensions())
    });
    let manifests = vec![];

    use_context_provider(|| State {
        app_store: Signal::new(app_store),
        extensions: Signal::new(extensions),
        manifests: Signal::new(manifests),
    });
}

#[derive(Clone, Copy)]
pub struct State {
    pub app_store: Signal<Store>,
    pub extensions: Signal<Extensions>,
    pub manifests: Signal<Manifests>,
}
