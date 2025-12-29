use std::collections::HashMap;

use crate::errors::StatsError;

pub struct TextStats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub non_empty_lines: usize,
    pub top_word: Option<String>,
}

pub fn analyze(text: &str) -> Result<TextStats, StatsError> {
    if text.is_empty() {
        return Err(StatsError::EmptyInput);
    }

    let h: HashMap<String, usize> =
        text.split_whitespace()
            .fold(HashMap::new(), |mut acc, word| {
                let lower = word.to_lowercase();
                let trimmed: String = lower
                    .chars()
                    .filter(|c| !matches!(c, '.' | ',' | '!' | '?' | ';' | ':'))
                    .collect();

                if !trimmed.is_empty() {
                    *acc.entry(trimmed).or_insert(0) += 1;
                }
                acc
            });

    let top_word = h
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(word, _)| word.to_string());

    let out = TextStats {
        lines: text.matches("\n").count() + 1,
        words: text.split_whitespace().count(),
        chars: text.chars().count(),
        non_empty_lines: text.lines().filter(|l| !l.trim().is_empty()).count(),
        top_word: top_word,
    };

    return Ok(out);
}

#[cfg(test)]
mod tests {
    use super::analyze;

    #[test]
    fn empty_input_returns_errors() {
        let result = analyze("");
        assert!(result.is_err());
    }

    #[test]
    fn two_lines() {
        let result = analyze("line1\nline2").unwrap();
        assert_eq!(2, result.lines);
    }

    #[test]
    fn three_words() {
        let result = analyze("word1 word2 word3").unwrap();
        assert_eq!(3, result.words);
    }

    #[test]
    fn five_chars() {
        let result = analyze("12345").unwrap();
        assert_eq!(5, result.chars);
    }

    #[test]
    fn non_empty_lines() {
        let result = analyze("line1\n \nline2").unwrap();
        assert_eq!(2, result.non_empty_lines);
    }

    #[test]
    fn top_words() {
        let result = analyze("line1\n \nline1").unwrap();
        assert_eq!("line1", result.top_word.unwrap());
    }

    #[test]
    fn hello_word() {
        let result = analyze("Привет мир").unwrap();
        assert_eq!(1, result.lines);
        assert_eq!(2, result.words);
        assert_eq!(10, result.chars);
        assert_eq!(1, result.non_empty_lines);

        assert!(matches!(
            result.top_word.as_deref(),
            Some("привет") | Some("мир")
        ));
    }

    #[test]
    fn hello2_word() {
        let result = analyze("Hello, hello! HELLO, word").unwrap();
        assert_eq!(1, result.lines);
        assert_eq!(4, result.words);
        assert_eq!(25, result.chars);
        assert_eq!(1, result.non_empty_lines);
        assert_eq!("hello", result.top_word.unwrap());
    }
}
