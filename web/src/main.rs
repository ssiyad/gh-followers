use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, routing::get};

#[derive(Template, WebTemplate)]
#[template(path = "index.html", ext = "html")]
struct Index {}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(index));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Index {
    Index {}
}
