use std::process::Command;
/// Executes a sequence of Git commands to initialize a repository, add files, and commit changes.
///
/// This function runs the following sequence of Git commands:
/// 1. `git init`
/// 2. `git add .`
/// 3. `git commit -m "Initial commit"`
///
/// # Example
///
/// ```
/// git_commands(); // This will run the sequence of Git commands.
/// ```
pub fn git_commands() {
    // Initialize git repository
    let _output = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to initialize git repository");

    // Add files to git
    let _output = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to add files to git");

    // Commit with a message
    let _output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Initial commit")
        .output()
        .expect("Failed to commit changes");
}
