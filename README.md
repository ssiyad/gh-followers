# GitHub Followers CLI

A Rust command-line tool to query GitHub user relationships (ghosts, lurkers,
followers, followees) using the official GitHub CLI (`gh`). 

## Features

- List types: `ghosts`, `lurkers`, `followers`, `followees`
- Specify GitHub username and token via CLI or environment
- Automatically fetch username and token using `gh` if not provided

## Requirements

- Rust (edition 2021 or later)
- [GitHub CLI (`gh`)](https://cli.github.com/) installed and authenticated

## Usage

```
cargo run -- [OPTIONS]
```

### Options

- `--list <TYPE>`: Type of data to get. Possible values: `ghosts`, `lurkers`, `followers`, `followees`. Default: `ghosts`
- `--user <USERNAME>`: GitHub username. If omitted, uses `gh api /user`.
- `--token <TOKEN>`: GitHub token. If omitted, uses `gh auth token`.

### Examples

List followers for the authenticated user:
```
cargo run -- --list followers
```

List ghosts for a specific user:
```
cargo run -- --list ghosts --user octocat
```

Provide a token explicitly:
```
cargo run -- --token <your_token>
```

## Development

Build:
```
cargo build
```

Run:
```
cargo run -- [OPTIONS]
```

Test:
```
cargo test
```

## Self Promotion

Like this project? Give it a star! ⭐, and spread the word! 🚀. And if you are
feeling especially charitable, follow [Sabu Siyad](https://ssiyad.com) on
[GitHub](https://github.com/ssiyad). Thanks!
