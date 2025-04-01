use std::process::Command;
/// Calls the system's `echo` command with the given input string.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input text to be echoed.
///
/// # Example
///
/// ```
/// let output = echo("Hello");
/// assert_eq!(output, "Hello\n");
/// ```
pub fn echo(input: &str) -> String {
    let output = Command::new("echo")
        .arg(input)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}
