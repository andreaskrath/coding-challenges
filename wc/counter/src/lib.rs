//! The `counter` crate provides functionality for counting bytes, lines and words of an input string.

/// The `line_count` function returns the amount of lines in the input string.
///
/// Newlines are defined with the newline character `\n`.
///
/// # Examples
/// ```
/// # use counter::line_count;
/// let line_amount = line_count("Hello!\nHow are you doing?");
/// assert_eq!(line_amount, 2);
/// ```
pub fn line_count(input: &str) -> usize {
    input.lines().count()
}

/// The `word_count` function returns the amount of words in the input string.
///
/// Words are seperated with a whitespace character,
/// meaning `Hello!` is a single word, while `Hello !` represents two words.
///
/// # Examples
/// ```
/// # use counter::word_count;
/// // newline counts as whitespace
/// let word_amount = word_count("Hello!\nHow are you doing?");
/// assert_eq!(word_amount, 5);
/// ```
pub fn word_count(input: &str) -> usize {
    input.split_whitespace().count()
}

/// The `byte_count` function returns the amount of bytes in the input string.
///
/// # Examples
/// For ASCII characters, a character is the size of a byte, meaning the amount
/// of bytes is equal to the length of the string.
/// ```
/// # use counter::byte_count;
/// let byte_amount = byte_count("Hello, how are you doing?");
/// assert_eq!(byte_amount, 25);
/// ```
pub fn byte_count(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod unit_tests {
    use crate::{byte_count, line_count, word_count};
    use rstest::rstest;

    #[rstest]
    #[case("Hello, world!", 1)]
    #[case("Hello\n world!", 2)]
    #[case("\nHello,\n world!", 3)]
    #[case("Hello, world!\n\n\nGoodbye!", 4)]
    #[case("\n", 1)]
    fn lines(#[case] input: &str, #[case] expected: usize) {
        let actual = line_count(input);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("", 0)]
    #[case(" ", 0)]
    #[case("Hello world!", 2)]
    #[case("Hello, world!\nGoodbye!", 3)]
    fn words(#[case] input: &str, #[case] expected: usize) {
        let actual = word_count(input);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("", 0)]
    #[case("Hello, world! How are you feeling today?", 40)]
    #[case(" ", 1)]
    #[case("\n\n\n", 3)]
    fn bytes(#[case] input: &str, #[case] expected: usize) {
        let actual = byte_count(input);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod file_tests {
    use crate::{byte_count, line_count, word_count};
    use std::fs::read_to_string;

    #[test]
    fn words() {
        let input = read_to_string("test.txt").expect("should find test file");
        let expected = 58159; // what `wc -w test.txt` produces locally
        let actual = word_count(input.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn lines() {
        let input = read_to_string("test.txt").expect("should find test file");
        let expected = 7136; // what `wc -l test.txt` produces locally
        let actual = line_count(input.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn bytes() {
        let input = read_to_string("test.txt").expect("should find test file");
        let expected = 334695; // what `wc -c test.txt` produces locally
        let actual = byte_count(input.as_str());
        assert_eq!(actual, expected);
    }
}
