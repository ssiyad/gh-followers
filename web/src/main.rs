use axum::{Router, extract::Query, routing::get};
use ghf_web::components::{Body, Header, Index, Logo, Search, SourceCode, TableGhosts};
use serde::Deserialize;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/body", get(body))
        .route("/search", get(search))
        .route("/header", get(header))
        .route("/logo", get(logo))
        .route("/source_code", get(source_code))
        .route("/table_ghosts", get(table_ghosts))
        .nest_service("/static/main.css", ServeFile::new("web/static/main.css"));

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

async fn search() -> Search {
    Search {}
}

async fn header() -> Header {
    Header {}
}

async fn logo() -> Logo {
    Logo {}
}

async fn source_code() -> SourceCode {
    SourceCode {}
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
