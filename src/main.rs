pub mod backend;
use dioxus::{logger::tracing::warn, prelude::*};

use crate::backend::weather_get;

const MAIN_CSS: Asset = asset!("/assets/ebook.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Echo {}
    }
}
#[component]
fn Echo() -> Element {
    let Some(weather) = use_resource(move || async move { weather_get().await.unwrap() })
        .read_unchecked()
        .clone()
    else {
        return rsx! {
            h1 { "Server Error :/" }
        };
    };

    let mut weather = use_signal(|| weather);

    let (current, max) = (
        format!("{:.1}", weather.read().result.uv),
        format!("{:.1}", weather.read().result.uv_max),
    );

    let mut is_loading = use_signal(|| false);

    rsx! {


        div { class: "boxes",
            h1 { class: "box", "UV Index for Fremantle" }
        }
        br {}

        div { class: "boxes",

            if !*is_loading.read() {
                h2 { class: "box bx-line", "Max: {max}" }

                h2 { class: "box bx-line", "Current: {current}" }
            } else {
                h2 { class: "box bx-line", ". . . " }

                h2 { class: "box bx-line", ". . ." }
            }
        }

        div { class: "boxes",
            a {
                class: "box bx-line",
                onclick: move |_| async move {
                    is_loading.set(true);
                    let uv = weather_get().await.unwrap();
                    weather.set(uv);
                    is_loading.set(false);
                },
                "Refresh"
            }
        }
    }
}
