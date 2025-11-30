use futures_util::TryStreamExt;
use octocrab::models::{Followee, Follower, SimpleUser};

static USER: &str = "ssiyad";

#[tokio::main]
async fn main() {
    // let octo = octocrab::instance();
    let _followers = followers().await;
    let _following = following().await;
    // let following = octo.users("ssiyad").following().send().await.unwrap();
}

async fn followers() -> Vec<Follower> {
    let octo = octocrab::instance();
    octo.users(USER)
        .followers()
        .send()
        .await
        .unwrap()
        .into_stream(&octo)
        .try_collect::<Vec<Follower>>()
        .await
        .unwrap()
}

async fn following() -> Vec<Followee> {
    let octo = octocrab::instance();
    octo.users(USER)
        .following()
        .send()
        .await
        .unwrap()
        .into_stream(&octo)
        .try_collect::<Vec<Followee>>()
        .await
        .unwrap()
}

#[allow(dead_code)]
async fn ghosts(followers: &Vec<Follower>, following: &Vec<Followee>) -> Vec<SimpleUser> {
    todo!()
}

#[allow(dead_code)]
async fn lurkers(followers: &Vec<Follower>, following: &Vec<Followee>) -> Vec<SimpleUser> {
    todo!()
}
