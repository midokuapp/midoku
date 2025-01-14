use dioxus::prelude::*;

use crate::hook::use_extensions;
use crate::Route;

#[component]
pub fn Browse() -> Element {
    let extensions = use_extensions();

    rsx! {
        ul {
            {
                extensions
                    .read()
                    .iter()
                    .map(|(_, extension)| {
                        let extension_id = &extension.id;
                        rsx! {
                            li {
                                Link {
                                    to: Route::BrowseExtension {
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
