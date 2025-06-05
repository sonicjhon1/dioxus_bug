use dioxus::prelude::*;
use tokio::net::TcpListener;

pub fn main() -> () {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            let listener =
                TcpListener::bind(dioxus::cli_config::fullstack_address_or_localhost()).await?;

            let app = axum::Router::new();
            let app = app.serve_dioxus_application(
                ServeConfigBuilder::default()
                    .build()
                    .expect("Dioxus ServeConfigBuilder should build successfully"),
                || {
                    rsx! {
                        dioxus::router::prelude::Router::<web::Route> {}
                    }
                },
            );

            axum::serve(listener, app).await
        })
        .unwrap();
}
