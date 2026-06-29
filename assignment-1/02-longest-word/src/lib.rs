pub fn longest_word(sentence: &str) -> Option<&str> {
    sentence
        .split_whitespace()
        .max_by_key(|word| word.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_longest_of_four() {
        assert_eq!(longest_word("the quick brown fox"), Some("quick"));
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(longest_word("   "), None);
    }
}
