pub fn is_palindrome(s: &str) -> bool {
    let sanitized: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect();

    sanitized == sanitized.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(is_palindrome("Racecar"));
        assert!(is_palindrome("Able was I, I saw elba"));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!is_palindrome("This is not a palindrome"));
        assert!(!is_palindrome("Hello, World!"));
    }
}
