use std::collections::HashMap;
use std::rc::Rc;

use dioxus::html::geometry::PixelsVector2D;
use dioxus::prelude::*;

use crate::component::{extract_attributes, merge_attributes};
use crate::Route;

static SCROLL: GlobalSignal<HashMap<Route, PixelsVector2D>> = Signal::global(|| HashMap::new());

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Horizontal,
    Vertical,
}

#[component]
pub fn ScrollArea(
    direction: ScrollDirection,
    #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (attributes, classes): (Vec<_>, Vec<_>) = extract_attributes!(attributes, "class");
    let class = merge_attributes!(classes, " ");

    let mut element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    _ = use_resource(move || async move {
        if let (Some(element), Some(scroll)) = (
            element.read().as_ref(),
            SCROLL.peek().get(&use_route()).cloned(),
        ) {
            let t = element.scroll(scroll, ScrollBehavior::Instant).await;
            dioxus::logger::tracing::debug!("{:?}", t);
        }
    });

    rsx! {
        div {
            class: if direction == ScrollDirection::Horizontal { "overflow-x-auto" },
            class: if direction == ScrollDirection::Vertical { "overflow-y-auto" },
            class: "{class}",

            onmounted: move |event| element.set(Some(event.data())),

            onscroll: move |_| async move {
                if let Some(element) = element.read().as_ref() {
                    let scroll = element.get_scroll_offset().await.unwrap();
                    SCROLL.write().insert(use_route(), scroll);
                }
            },

            ..attributes,
            {children}
        }
    }
}
