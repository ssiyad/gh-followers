use axum::{Router, routing::get};
use ghf_web::components::{Body, Header, Index, SearchBox};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/body", get(body))
        .route("/search_box", get(search_box))
        .route("/header", get(header));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Index {
    Index {}
}

async fn body() -> Body {
    Body {}
}

async fn search_box() -> SearchBox {
    SearchBox {}
}

async fn header() -> Header {
    Header {}
}
