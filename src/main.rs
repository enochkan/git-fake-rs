use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;
use rand::Rng;
use chrono::Utc;

fn main() {
    let x = 1; // Minimum number of commits
    let y = 5; // Maximum number of commits

    let commit_count = rand::thread_rng().gen_range(x..=y);

    // Define a single dummy file
    let file_name = "dummy_commits.txt";

    for i in 1..=commit_count {
        // Open the file in append mode, creating it if it doesn't exist
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_name)
            .expect("Failed to open or create file");

        writeln!(
            file,
            "Commit {}: This is a dummy commit created at {}",
            i,
            Utc::now()
        )
        .expect("Failed to write to file");

        // Stage the single dummy file
        Command::new("git")
            .args(["add", &file_name])
            .output()
            .expect("Failed to stage file");

        // Create a commit
        Command::new("git")
            .args(["commit", "-m", &format!("Random commit {}", i)])
            .output()
            .expect("Failed to commit");
    }

    // Push the commits to the repository
    Command::new("git")
        .args(["push"])
        .output()
        .expect("Failed to push commits");

    println!("Pushed {} random commits to the repository!", commit_count);
}