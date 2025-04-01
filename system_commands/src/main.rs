mod echo;
mod mkdir;
mod git_commands;

use echo::echo;
use mkdir::mkdir;
use git_commands::git_commands;

fn main() {
    // Example usage
    println!("{}", echo("Hello, World!"));
    mkdir(5, "test_dir");
    git_commands();
}
