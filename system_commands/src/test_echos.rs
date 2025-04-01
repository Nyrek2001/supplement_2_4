#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo() {
        // This test should check if echo command outputs the string correctly.
        // Since we can't directly capture stdout easily here, we'll focus on the command run logic.
        assert_eq!(echo("Hello"), "Hello\n");
    }
}
