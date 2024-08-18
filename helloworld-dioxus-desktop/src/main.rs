#![allow(non_snake_case)]

use dioxus::prelude::*;


fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "display: flex; justify-content: center; align-items: center; height: 100vh; flex-direction: column;",
            h1 { "Hello, world!" }
            p { "This is a simple example of a Dioxus desktop app." }
            p { "You can edit this code in the /src/main.rs file." }
        }
    }
}
