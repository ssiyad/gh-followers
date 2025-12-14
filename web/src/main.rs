use axum::{Router, extract::Query, routing::get};
use ghf_web::components::{Body, Header, Index, Search, TableGhosts};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/body", get(body))
        .route("/search_box", get(search_box))
        .route("/header", get(header))
        .route("/table_ghosts", get(table_ghosts));

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

async fn search_box() -> Search {
    Search {}
}

async fn header() -> Header {
    Header {}
}

#[derive(Debug, Deserialize)]
struct GhostsQuery {
    username: String,
}

async fn table_ghosts(params: Query<GhostsQuery>) -> TableGhosts {
    TableGhosts {
        ghosts: ghf::sources::ghosts(&params.0.username).await,
    }
}
