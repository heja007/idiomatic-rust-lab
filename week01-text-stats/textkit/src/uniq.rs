use crate::errors::TextkitError;
use std::collections::HashSet;

pub fn uniq_lines(text: &str, all: bool) -> Result<Vec<String>, TextkitError> {
    let mut out: Vec<String> = Vec::new();

    if all {
        let mut hset: HashSet<&str> = HashSet::new();

        for line in text.lines() {
            if !hset.contains(line) {
                out.push(line.to_string());
                hset.insert(line);
            }
        }

        return Ok(out);
    }

    let mut prev: Option<&str> = None;
    for line in text.lines() {
        if prev != Some(line) {
            out.push(line.to_string());
            prev = Some(line);
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::uniq_lines;

    #[test]
    fn uniq_all_keeps_first_occurrence_of_each_line() {
        let input = "one\none two\none\none two\nthree\n";
        let result = uniq_lines(input, true).unwrap();

        let expected = vec![
            "one".to_string(),
            "one two".to_string(),
            "three".to_string(),
        ];

        assert_eq!(expected, result);
    }

    #[test]
    fn uniq_adjacent_only_collapses_consecutive_duplicates() {
        let input = "one\none\none two\none\none\n";
        let result = uniq_lines(input, false).unwrap();

        let expected = vec!["one".to_string(), "one two".to_string(), "one".to_string()];

        assert_eq!(expected, result);
    }
}
