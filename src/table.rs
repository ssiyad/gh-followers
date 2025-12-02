use cli_table::{
    Cell, Style, Table, TableStruct,
    format::{Border, HorizontalLine, Justify, Separator, VerticalLine},
};
use octocrab::models::{Followee, Follower};

pub fn followers_table(users: Vec<Follower>) -> TableStruct {
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

pub fn following_table(users: Vec<Followee>) -> TableStruct {
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
