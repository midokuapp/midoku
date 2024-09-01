use leptos::*;
use leptos_remix_icon::Icon;
use leptos_router::*;

#[component]
pub fn NavigationBar() -> impl IntoView {
    view! {
        <nav class="flex flex-row items-center justify-around bg-muted px-3 py-4">
            <NavigationElement href="/" icon="book-marked-line" text="Library"/>
            <NavigationElement href="/browse" icon="compass-3-line"  text="Browse"/>
            <NavigationElement href="/more" icon="more-line" text="More"/>
        </nav>
    }
}

#[component]
pub fn NavigationElement(
    href: &'static str,
    icon: &'static str,
    text: &'static str,
) -> impl IntoView {
    let class = move || {
        format!(
            "flex flex-col items-center gap-1 {}",
            href.ne(&use_location().pathname.get())
                .then_some("text-muted-foreground")
                .unwrap_or_default()
        )
    };

    view! {
        <A href={href} class={class}>
            <Icon icon={icon} class="text-2xl"/>
            <span>{text}</span>
        </A>
    }
}
