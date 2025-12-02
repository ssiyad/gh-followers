use gh_followers::{config, display, sources};

#[tokio::main]
async fn main() {
    let list = config::list();
    let user = config::user();
    let token = config::token();
    sources::init(&token);
    let followers = sources::followers(&user).await;
    let following = sources::following(&user).await;

    match list.as_str() {
        "following" => display::following(&following, &followers),
        "followers" => display::followers(&followers, &following),
        "lurkers" => display::lurkers(&followers, &following),
        "ghosts" => display::ghosts(&following, &followers),
        _ => (),
    }
}
