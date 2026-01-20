use crate::errors::TextkitError;

pub struct TextStats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
}

pub fn analyze(text: &str) -> Result<TextStats, TextkitError> {
    Ok(TextStats {
        lines: text.lines().count(),
        words: text.split_whitespace().count(),
        chars: text.chars().count(),
        bytes: text.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::analyze;

    #[test]
    fn stats_empty_text_is_zeroes() {
        let s = analyze("").unwrap();
        assert_eq!(0, s.lines);
        assert_eq!(0, s.words);
        assert_eq!(0, s.chars);
        assert_eq!(0, s.bytes);
    }

    #[test]
    fn stats_unicode_counts_chars_vs_bytes() {
        // "é" is 2 bytes in UTF-8, 1 char
        let s = analyze("é").unwrap();
        assert_eq!(1, s.lines);
        assert_eq!(1, s.words);
        assert_eq!(1, s.chars);
        assert_eq!(2, s.bytes);
    }

    #[test]
    fn stats_counts_words_with_whitespace() {
        let s = analyze("  one\t two\nthree  ").unwrap();
        assert_eq!(2, s.lines); // two lines: "  one\t two" and "three  "
        assert_eq!(3, s.words);
    }
}
