use std::str::CharIndices;

fn lex_word<'a, 'b>(message: &'a str, start: usize, chars: &'b mut CharIndices) -> &'a str {
    let count = chars
        .position(|(_, c)| !c.is_alphanumeric())
        .unwrap_or_else(|| message.len() - start - 1)
        + 1;

    &message[start..start + count]
}

fn lex_sentence<'a, 'b>(message: &'a str, start: usize, chars: &'b mut CharIndices) -> &'a str {
    let count = chars
        .position(|(_, c)| c == '"')
        .unwrap_or_else(|| message.len() - start - 1)
        + 1;

    &message[start + 1..start + count]
}

pub fn lex_args(message: &str) -> Vec<&str> {
    // Ignore the first word
    let first_space = message.find(' ').unwrap();
    let arguments = message.split_at(first_space + 1).1;

    // Parse the arguments using the characters
    let mut chars = arguments.char_indices();
    let mut args = Vec::new();

    while let Some((i, c)) = chars.next() {
        if c.is_alphanumeric() {
            let word = lex_word(arguments, i, &mut chars);
            args.push(word);
        } else if c == '"' {
            let sentence = lex_sentence(arguments, i, &mut chars);
            args.push(sentence);
        }
    }

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_lexing() {
        let message = r#"!poll title arg1 arg2"#;
        let args = lex_args(message);
        let expected = vec!["title", "arg1", "arg2"];

        assert_eq!(args, expected);
    }

    #[test]
    fn quotation_marks() {
        let message = r#"!poll title "arg1" arg2"#;
        let args = lex_args(message);
        let expected = vec!["title", "arg1", "arg2"];

        assert_eq!(args, expected);
    }

    #[test]
    fn multi_word_quotations() {
        let message = r#"!poll title "arg1 arg2""#;
        let args = lex_args(message);
        let expected = vec!["title", "arg1 arg2"];

        assert_eq!(args, expected);
    }

    #[test]
    fn multiple_sentences() {
        let message = r#"!poll "longer title" "arg1 arg2""#;
        let args = lex_args(message);
        let expected = vec!["longer title", "arg1 arg2"];

        assert_eq!(args, expected);
    }

    #[test]
    #[should_panic]
    fn mismatched_quotations() {
        let message = r#"!poll "longer title "arg1 arg2""#;
        let args = lex_args(message);
        let expected = vec!["longer title", "arg1 arg2"];

        assert_eq!(args, expected);
    }
}
