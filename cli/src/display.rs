use colored::Colorize;
use ghf::{Followee, Follower};

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

pub fn lurkers(followers: &[Follower], followees: &[Followee]) {
    let users = followers
        .iter()
        .filter(|f| !followees.iter().any(|fe| fe.login == f.login))
        .collect::<Vec<&Follower>>();

    println!(
        "{header} ({count})",
        header = "Lurkers".blue().bold(),
        count = users.len().to_string().bold()
    );

    for user in users {
        print!("@{} ", user.login.bright_green());
    }

    println!();
}

// Lurkers: users who follow you but you do not follow back
pub fn ghosts(followees: &[Followee], followers: &[Follower]) {
    let users = followees
        .iter()
        .filter(|fe| !followers.iter().any(|f| f.login == fe.login))
        .collect::<Vec<&Followee>>();

    println!(
        "{header} ({count})",
        header = "Ghosts".blue().bold(),
        count = users.len().to_string().bold()
    );

    for user in users {
        print!("@{} ", user.login.bright_red());
    }

    println!();
}
