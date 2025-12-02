use std::process::Command;

use cli_table::print_stdout;
use gh_followers::{sources, table};

static USER: &str = "ssiyad";

#[tokio::main]
async fn main() {
    sources::init(token().as_ref());
    print_stdout(table::followers_table(sources::followers(USER).await)).ok();
    print_stdout(table::following_table(sources::following(USER).await)).ok();
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
