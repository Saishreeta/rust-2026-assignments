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
    fn three_words() {
        assert_eq!(reverse_words("hello world rust"), "rust world hello");
    }

    #[test]
    fn collapses_inner_whitespace() {
        assert_eq!(reverse_words("   one    two   "), "two one");
    }

    #[test]
    fn empty_input() {
        assert_eq!(reverse_words(""), "");
    }
}
    
