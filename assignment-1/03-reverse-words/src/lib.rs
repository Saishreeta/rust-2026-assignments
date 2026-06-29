pub fn reverse_words(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(reverse_words(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(reverse_words("hello"), "hello");
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(reverse_words("hello world"), "world hello");
        assert_eq!(reverse_words("the quick brown fox"), "fox brown quick the");
    }
}
