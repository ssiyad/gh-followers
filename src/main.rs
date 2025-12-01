use std::process::Command;

use cli_table::{Cell, Style, Table, format::Justify, print_stdout};
use futures_util::TryStreamExt;
use octocrab::models::{Followee, Follower};

static USER: &str = "ssiyad";

#[tokio::main]
async fn main() {
    octo_init();
    let followers_table = followers()
        .await
        .iter()
        .map(|user| {
            vec![
                user.login.clone().cell().justify(Justify::Left),
                user.id.cell().justify(Justify::Right),
            ]
        })
        .table()
        .title(vec!["Username".cell().bold(true), "ID".cell().bold(true)])
        .bold(true);
    print_stdout(followers_table).ok();
    let following_table = following()
        .await
        .iter()
        .map(|user| {
            vec![
                user.login.clone().cell().justify(Justify::Left),
                user.id.cell().justify(Justify::Right),
            ]
        })
        .table()
        .title(vec!["Username".cell().bold(true), "ID".cell().bold(true)])
        .bold(true);
    print_stdout(following_table).ok();
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

fn octo_init() {
    octocrab::initialise(
        octocrab::OctocrabBuilder::default()
            .personal_token(token())
            .build()
            .unwrap(),
    );
}

fn token() -> String {
    String::from_utf8(
        Command::new("gh")
            .args(["auth", "token"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .strip_suffix('\n')
    .unwrap()
    .to_string()
}
