use chrono::Utc;
use clap::Parser;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

/// CLI for generating random Git commits.
///
/// This tool appends dummy content to a single file, stages it, and creates random commits to
/// make your GitHub contribution graph look busy!
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Minimum number of commits (default: 1)
    #[arg(short = 'm', long, default_value = "1")]
    min: u32,

    /// Maximum number of commits (default: 5)
    #[arg(short = 'x', long, default_value = "5")]
    max: u32,
}

fn main() {
    // Parse command-line arguments using `clap`
    let args = Cli::parse();
    generate_commits(args.min, args.max);
}

/// Generates and pushes random commits.
///
/// # Arguments
///
/// * `min` - Minimum number of commits to generate.
/// * `max` - Maximum number of commits to generate.
///
/// This function calculates a random number of commits between `min` and `max`,
/// appends dummy content to a file, stages the file, creates commits, and pushes them.
fn generate_commits(min: u32, max: u32) {
    let commit_count = rand::thread_rng().gen_range(min..=max);

    // Define the dummy file name
    let file_name = "dummy_commits.txt";

    for i in 1..=commit_count {
        append_to_file(file_name, i);
        stage_file(file_name);
        create_commit(i);
    }

    push_commits();

    println!("Pushed {} random commits to the repository!", commit_count);
}

/// Appends dummy content to a file.
///
/// # Arguments
///
/// * `file_name` - The name of the file to append content to.
/// * `commit_number` - The current commit number, used for logging.
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

/// Stages a file for Git commit.
///
/// # Arguments
///
/// * `file_name` - The name of the file to stage.
fn stage_file(file_name: &str) {
    Command::new("git")
        .args(["add", file_name])
        .output()
        .expect("Failed to stage file");
}

/// Creates a Git commit with a random message.
///
/// # Arguments
///
/// * `commit_number` - The current commit number, used for the commit message.
fn create_commit(commit_number: u32) {
    Command::new("git")
        .args(["commit", "-m", &format!("Random commit {}", commit_number)])
        .output()
        .expect("Failed to commit");
}

/// Pushes all committed changes to the remote Git repository.
///
/// This function assumes the user has configured the remote repository and authentication.
fn push_commits() {
    Command::new("git")
        .args(["push"])
        .output()
        .expect("Failed to push commits");
}
