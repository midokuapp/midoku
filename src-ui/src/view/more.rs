use leptos::*;

use crate::header::Header;
use crate::navigation::NavigationBar;

#[component]
pub fn More() -> impl IntoView {
    view! {
        <div class="flex flex-col w-screen h-screen">
            <Header>
                <h1 class="mr-auto text-2xl">More</h1>
            </Header>
            <main class="overflow-y-scroll grow">
            </main>
            <NavigationBar/>
        </div>
    }
}
