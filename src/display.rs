use colored::Colorize;
use octocrab::models::{Followee, Follower};

pub fn following(followees: &[Followee], followers: &[Follower]) {
    println!(
        "{header} ({count})",
        header = "Following".blue().bold(),
        count = followees.len().to_string().bold()
    );

    for user in followees {
        let is_followed_back = followers.iter().any(|f| f.login == user.login);
        let login = match is_followed_back {
            true => user.login.bright_yellow(),
            false => user.login.bright_red(),
        };
        print!("@{} ", login);
    }

    println!();
}

pub fn followers(followers: &[Follower], followees: &[Followee]) {
    println!(
        "{header} ({count})",
        header = "Followers".blue().bold(),
        count = followers.len().to_string().bold()
    );

    for user in followers {
        let is_following_back = followees.iter().any(|f| f.login == user.login);
        let login = match is_following_back {
            true => user.login.bright_yellow(),
            false => user.login.bright_green(),
        };
        print!("@{} ", login);
    }

    println!();
}
