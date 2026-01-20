use crate::errors::TextkitError;

pub fn grep_lines(
    text: &str,
    pattern: &str,
    ignore_case: bool,
    line_numbers: bool,
) -> Result<Vec<String>, TextkitError> {
    let mut out: Vec<String> = Vec::new();

    let pattern_lower = pattern.to_ascii_lowercase();
    for (i, line) in text.lines().enumerate() {
        let matched = if ignore_case {
            line.to_ascii_lowercase().contains(&pattern_lower)
        } else {
            line.contains(pattern)
        };

        if matched {
            if line_numbers {
                out.push(format!("{}:{}", i + 1, line));
            } else {
                out.push(line.to_string());
            }
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::grep_lines;

    #[test]
    fn ignore_case_grep_lines_any_case() {
        let input = "one\none Two\none\none two\nthree\n";
        let result = grep_lines(input, &"two", true, false).unwrap();

        let expected = vec!["one Two".to_string(), "one two".to_string()];

        assert_eq!(expected, result);
    }

    #[test]
    fn ignore_case_grep_lines_strict_case() {
        let input = "one\none Two\none\none two\nthree\n";
        let result = grep_lines(input, &"two", false, false).unwrap();

        let expected = vec!["one two".to_string()];

        assert_eq!(expected, result);
    }

    #[test]
    fn ignore_case_grep_lines_strict_case_line_number() {
        let input = "one\none Two\none\none two\nthree\n";
        let result = grep_lines(input, &"two", false, true).unwrap();

        let expected = vec!["4:one two".to_string()];

        assert_eq!(expected, result);
    }
}
