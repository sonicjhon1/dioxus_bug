use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const IMG: Asset = asset!("/assets/dioxus-architecture-diagram.webp");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Index {},
}

#[component]
pub fn Index() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            "There should be an image:"
            img { class: "img", src: IMG }
        }
        div {
            "The background should be blue, the text should be white:"
            div { class: "blue", "Blue and white?" }
        }
    }
}
