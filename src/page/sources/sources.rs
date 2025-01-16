use dioxus::prelude::*;

use crate::hook::use_state;
use crate::Route;

#[component]
pub fn SourceList() -> Element {
    let state = use_state();
    let extensions = state.extensions.read();

    rsx! {
        ul {
            {
                extensions
                    .iter()
                    .map(|(_, extension)| {
                        let extension_id = extension.id();
                        rsx! {
                            li {
                                Link {
                                    to: Route::MangaList {
                                        extension_id: extension_id.to_string(),
                                    },
                                    "{extension_id}"
                                }
                            }
                        }
                    })
            }
        }
    }
}
