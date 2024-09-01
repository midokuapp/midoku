use leptos::*;
use leptos_router::*;

#[component]
pub fn NavigationBar() -> impl IntoView {
    view! {
        <nav class="flex flex-row items-center justify-around bg-muted px-3 py-4">
            <NavigationElement href="/" text="Library"/>
            <NavigationElement href="/browse" text="Browse"/>
            <NavigationElement href="/more" text="More"/>
        </nav>
    }
}

#[component]
pub fn NavigationElement(
    #[prop(into, default = "#".to_string())] href: String,
    #[prop(into)] text: String,
) -> impl IntoView {
    let href_clone = href.clone();

    let class = move || {
        if href_clone == use_location().pathname.get() {
            "flex flex-col items-center gap-1"
        } else {
            "flex flex-col items-center gap-1 text-muted-foreground"
        }
    };

    view! {
        <A href={href} class={class}>
            <span>{text}</span>
        </A>
    }
}
