use leptos::*;
use leptos_router::*;

use crate::view::Library;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Library/>
            </Routes>
        </Router>
    }
}
