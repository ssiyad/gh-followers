use std::process::Command;

use cli_table::{
    Cell, Style, Table, TableStruct,
    format::{Border, HorizontalLine, Justify, Separator, VerticalLine},
    print_stdout,
};
use futures_util::TryStreamExt;
use octocrab::models::{Followee, Follower};

static USER: &str = "ssiyad";

#[tokio::main]
async fn main() {
    octo_init();
    print_stdout(followers_table(followers().await)).ok();
    print_stdout(following_table(following().await)).ok();
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

fn followers_table(users: Vec<Follower>) -> TableStruct {
    users
        .iter()
        .map(|user| vec![format!("@{}", user.login).cell().justify(Justify::Left)])
        .table()
        .title(vec![
            format!("Username ({})", users.len()).cell().bold(true),
        ])
        .bold(true)
        .border(table_border())
        .separator(table_seperator())
}

fn following_table(users: Vec<Followee>) -> TableStruct {
    users
        .iter()
        .map(|user| {
            vec![
                user.login.clone().cell().justify(Justify::Left),
                user.id.cell().justify(Justify::Right),
            ]
        })
        .table()
        .title(vec!["Username".cell().bold(true), "ID".cell().bold(true)])
        .bold(true)
        .border(table_border())
        .separator(table_seperator())
}

fn table_border() -> Border {
    Border::builder()
        .top(HorizontalLine::new('┌', '┐', '┬', '─'))
        .bottom(HorizontalLine::new('└', '┘', '┴', '─'))
        .left(VerticalLine::new('│'))
        .right(VerticalLine::new('│'))
        .build()
}

fn table_seperator() -> Separator {
    Separator::builder()
        .column(Some(VerticalLine::new('│')))
        .row(Some(HorizontalLine::new('├', '┤', '┼', '─')))
        .build()
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
