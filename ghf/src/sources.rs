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

pub async fn ghosts(user: &str) -> Vec<Followee> {
    let followers = followers(user).await;
    let following = following(user).await;
    let followers_logins = followers
        .into_iter()
        .map(|x| x.login)
        .collect::<Vec<String>>();

    following
        .into_iter()
        .filter(|f| !followers_logins.contains(&f.login))
        .collect()
}

pub async fn lurkers(user: &str) -> Vec<Follower> {
    let followers = followers(user).await;
    let following = following(user).await;
    let following_logins = following
        .into_iter()
        .map(|x| x.login)
        .collect::<Vec<String>>();

    followers
        .into_iter()
        .filter(|f| !following_logins.contains(&f.login))
        .collect()
}

pub fn init(token: &str) {
    octocrab::initialise(
        octocrab::OctocrabBuilder::default()
            .personal_token(token)
            .build()
            .unwrap(),
    );
}
