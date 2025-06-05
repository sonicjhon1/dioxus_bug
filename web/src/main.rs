use dioxus::prelude::*;

fn main() {
    dioxus::LaunchBuilder::web().launch(|| {
        rsx! {
            Router::<web::Route> {}
        }
    });
}
