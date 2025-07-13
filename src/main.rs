use dioxus::prelude::*;
mod backend;
mod components;

use crate::components::*;

fn main() {
    dioxus::launch(App);
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn App() -> Element {
    static CSS: Asset = asset!("/assets/main.css");
    rsx! {
        document::Stylesheet {href: CSS}
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    // ...
    // We can collect the segments of the URL into a Vec<String>
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favs")]
    Favorites,
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
             h1 {"HotDog!"}
         }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });
    // ...
    rsx! {
        div {id : "dogview",
            img {src: img_src.cloned().unwrap_or_default()}
        }
        div { id: "buttons",
            button {
                id: "save",
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = save_dog(current).await;
                },

                "save!"
            }
        }
    }
}

// The database is only available to server code
