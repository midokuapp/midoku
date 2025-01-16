use std::sync::Arc;

use dioxus::prelude::*;

use crate::model::Extension;
use crate::state::State;

pub fn use_extension<S: AsRef<str>>(extension_id: S) -> Signal<Arc<Extension>> {
    let state = use_context::<State>();
    let extensions = state.extensions.read();
    use_signal(|| extensions.get(extension_id.as_ref()).unwrap().clone())
}
