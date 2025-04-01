use std::process::Command;
/// Creates directories using the system's `mkdir` command. The directories are named by concatenating
/// the `base` string with the numbers from `0` to `n-1`.
///
/// # Arguments
///
/// * `n` - The number of directories to create.
/// * `base` - A string slice that is used as the base name for each directory.
///
/// # Example
///
/// ```
/// mkdir(3, "dir_");
/// // This will create directories named dir_0, dir_1, and dir_2.
/// ```
pub fn mkdir(n: u32, base: &str) {
    for i in 0..n {
        let dir_name = format!("{}{}", base, i);
        let _output = Command::new("mkdir")
            .arg(&dir_name)
            .output()
            .expect("Failed to create directory");
    }
}
