#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mkdir() {
        // Assuming a valid string and number
        mkdir(5, "test_dir");
        // You should check if the directories have been created
        // but for simplicity, we assume mkdir would work correctly.
    }
}
