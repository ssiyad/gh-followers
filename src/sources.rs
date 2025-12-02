use futures_util::TryStreamExt;
use octocrab::models::{Followee, Follower};

pub async fn followers(user: &str) -> Vec<Follower> {
    let octo = octocrab::instance();
    octo.users(user)
        .followers()
        .send()
        .await
        .unwrap()
        .into_stream(&octo)
        .try_collect::<Vec<Follower>>()
        .await
        .unwrap()
}

pub async fn following(user: &str) -> Vec<Followee> {
    let octo = octocrab::instance();
    octo.users(user)
        .following()
        .send()
        .await
        .unwrap()
        .into_stream(&octo)
        .try_collect::<Vec<Followee>>()
        .await
        .unwrap()
}

pub fn init(token: &str) {
    octocrab::initialise(
        octocrab::OctocrabBuilder::default()
            .personal_token(token)
            .build()
            .unwrap(),
    );
}
