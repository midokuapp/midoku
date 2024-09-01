use leptos::*;
use leptos_router::*;

use crate::view::{Browse, Library, More};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Library/>
                <Route path="/browse" view=Browse/>
                <Route path="/more" view=More/>
            </Routes>
        </Router>
    }
}
