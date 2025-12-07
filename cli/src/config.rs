use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Data to get
    #[arg(default_value("ghosts"), value_parser = ["ghosts", "lurkers", "followers", "followees"])]
    list: String,

    /// GitHub username
    #[arg(long)]
    user: Option<String>,

    /// GitHub token
    #[arg(long)]
    token: Option<String>,
}

fn parse() -> Args {
    Args::parse()
}

pub fn list() -> String {
    let args = parse();
    args.list
}

pub fn user() -> String {
    let args = parse();
    match args.user {
        Some(username) => username,
        None => String::from_utf8(
            Command::new("gh")
                .args(["api", "/user", "--jq", ".login"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .strip_suffix('\n')
        .unwrap()
        .to_string(),
    }
}

pub fn token() -> String {
    let args = parse();
    match args.token {
        Some(token) => token,
        None => String::from_utf8(
            Command::new("gh")
                .args(["auth", "token"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .strip_suffix('\n')
        .unwrap()
        .to_string(),
    }
}
