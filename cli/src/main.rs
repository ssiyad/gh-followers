use ghf::sources;
use ghf_cli::{config, display};

#[tokio::main]
async fn main() {
    let list = config::list();
    let user = config::user();
    let token = config::token();
    sources::init(&token);
    let followers = sources::followers(&user).await;
    let following = sources::following(&user).await;
    let lurkers = sources::lurkers(&user).await;
    let ghosts = sources::ghosts(&user).await;

    match list.as_str() {
        "following" => display::following(&following, &followers),
        "followers" => display::followers(&followers, &following),
        "lurkers" => display::lurkers(&lurkers),
        "ghosts" => display::ghosts(&ghosts),
        _ => (),
    }
}
