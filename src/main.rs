use cli_table::print_stdout;
use gh_followers::{config, sources, table};

#[tokio::main]
async fn main() {
    let user = config::user();
    let token = config::token();
    sources::init(&token);
    print_stdout(table::followers_table(sources::followers(&user).await)).ok();
    print_stdout(table::following_table(sources::following(&user).await)).ok();
}
