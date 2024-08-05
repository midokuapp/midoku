mod binding;

use leptos::{html::Input, *};
use serde::Serialize;

use crate::binding::invoke;

async fn greet(name: String) -> String {
    #[derive(Serialize)]
    struct GreetCmdArgs {
        name: String,
    }

    invoke("greet", &GreetCmdArgs { name }).await.unwrap()
}

#[component]
pub fn App() -> impl IntoView {
    let greet_message = create_action(|name: &String| {
        let name = name.to_owned();
        async move { greet(name).await }
    });

    let input_ref = create_node_ref::<Input>();

    view! {
        <div>
            <form
                on:submit=move |e| {
                    e.prevent_default();
                    let input = input_ref.get().expect("input does not exist");
                    greet_message.dispatch(input.value());
                }
            >
                <label for="name">"Name:"</label>
                <div class="mt-2">
                    <input type="text" name="name" node_ref=input_ref class="input input-bordered w-full max-w-xs"/>
                </div>
                <button type="submit" class="btn btn-primary"> "Greet" </button>
            </form>

            <span>{move || greet_message.value()}</span>
        </div>
    }
}
