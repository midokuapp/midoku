use html_escape::encode_text;
use leptos::*;

#[component]
pub fn Tile(id: String, title: String, cover_src: String, unread_chapters: u32) -> impl IntoView {
    let manga_url = format!("/manga?id={}", encode_text(&id));

    view! {
        <div class="relative">
            <div class="mx-1 mt-1">
                <Image src=cover_src alt="{title} cover".to_string() />
                <Title text=title />
                <Show when=move || { unread_chapters > 0 }>
                    <Count count=unread_chapters />
                </Show>
            </div>
        </div>
    }
}

#[component]
fn Image(src: String, alt: String) -> impl IntoView {
    view! {
        <figure class="relative aspect-[3/4]">
            <div class="absolute top-0 left-0 w-full h-full rounded-md animate-pulse bg-muted"></div>
            <img src=src alt=alt loading="lazy" class="object-cover absolute rounded-md" />
        </figure>
    }
}

#[component]
fn Title(text: String) -> impl IntoView {
    view! { <h1 class="h-10 text-sm font-medium line-clamp-2 text-start text-foreground">{text}</h1> }
}

#[component]
fn Count(count: u32) -> impl IntoView {
    view! {
        <span class="absolute top-0 left-0 py-1 px-2 text-sm font-semibold rounded-md bg-primary text-primary-foreground">
            {count}
        </span>
    }
}
