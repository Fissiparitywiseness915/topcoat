mod app;
mod components;

use topcoat::{view, view::View};

#[tokio::main]
async fn main() {
    let router = app::router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    topcoat::axum::serve(listener, router).await.unwrap();

    let content = view! {
        <html>
            <head>
                <title>"hello world"</title>
            </head>
            <body id="test">
                <form
                    // action=async || {
                    //     // runs on server
                    //     println!("{}, {}", email, password);
                    // }
                >
                    <input name="email" />
                    <input name="password" />
                </form>
            </body>
        </html>
    };

    println!("{}", content);
}
