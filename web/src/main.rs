use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| {
        rsx! {
            Router::<web::Route> {}
        }
    });
}
