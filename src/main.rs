use gh_followers::{config, display, sources};

#[tokio::main]
async fn main() {
    let user = config::user();
    let token = config::token();
    sources::init(&token);
    let followers = sources::followers(&user).await;
    let following = sources::following(&user).await;
    display::following(&following, &followers);
    display::followers(&followers, &following);
}
