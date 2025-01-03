use clap::Parser;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;
use rand::Rng;
use chrono::Utc;

/// CLI for generating random Git commits
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Minimum number of commits
    #[arg(short, long, default_value = "1")]
    min: u32,

    /// Maximum number of commits
    #[arg(short, long, default_value = "5")]
    max: u32,
}

fn main() {
    let args = Cli::parse();
    generate_commits(args.min, args.max);
}

/// Generates and pushes random commits
fn generate_commits(min: u32, max: u32) {
    let commit_count = rand::thread_rng().gen_range(min..=max);

    // Define a single dummy file
    let file_name = "dummy_commits.txt";

    for i in 1..=commit_count {
        append_to_file(&file_name, i);
        stage_file(&file_name);
        create_commit(i);
    }

    push_commits();

    println!("Pushed {} random commits to the repository!", commit_count);
}

/// Appends dummy content to the file
fn append_to_file(file_name: &str, commit_number: u32) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .expect("Failed to open or create file");

    writeln!(
        file,
        "Commit {}: This is a dummy commit created at {}",
        commit_number,
        Utc::now()
    )
    .expect("Failed to write to file");
}

/// Stages a file for commit
fn stage_file(file_name: &str) {
    Command::new("git")
        .args(["add", file_name])
        .output()
        .expect("Failed to stage file");
}

/// Creates a Git commit with a random message
fn create_commit(commit_number: u32) {
    Command::new("git")
        .args(["commit", "-m", &format!("Random commit {}", commit_number)])
        .output()
        .expect("Failed to commit");
}

/// Pushes commits to the remote repository
fn push_commits() {
    Command::new("git")
        .args(["push"])
        .output()
        .expect("Failed to push commits");
}