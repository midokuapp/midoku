use leptos::*;

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
    #[prop(default = false)] active: bool,
    #[prop(into)] text: String,
) -> impl IntoView {
    let class = if active {
        "flex flex-col items-center gap-1 text-muted-foreground"
    } else {
        "flex flex-col items-center gap-1"
    };

    view! {
        <a href={href} class={class}>
            <span>{text}</span>
        </a>
    }
}
