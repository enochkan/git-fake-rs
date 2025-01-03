# git-fake-rs

GitHub contribution graph looking a little barren? Fear not—**git-fake-rs** is here to save the day! This nifty Rust tool generates random commits to make your profile look like you're a coding machine.

## Features

- Randomly generates commits to fill up your GitHub contribution graph.
- Configurable number of commits per run.
- Automatically stages, commits, and pushes changes to your repo.
- Written entirely in Rust for blazing-fast fakery.

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/git-fake-rs.git
   cd git-fake-rs
   ```

2. Run the random committer:
   ```bash
   cargo run -- -min 3 --max 10
   ```

3. Add it to `crontab` as a background task and enjoy looking busy!
