use dioxus::prelude::*;
use midoku_store::Store;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

use crate::model::Extensions;
use crate::state::State;

pub fn use_state_provider() {
    let app_store = Store::open("app_data");
    let extensions = block_in_place(|| {
        let handle = Handle::current();
        handle.block_on(Extensions::init())
    });
    let manifests = vec![];

    use_context_provider(|| State {
        app_store: Signal::new(app_store),
        extensions,
        manifests: Signal::new(manifests),
    });
}

pub fn use_state() -> State {
    use_context::<State>()
}
