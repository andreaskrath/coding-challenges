/// The `line_count` function returns the amount of lines in the input string.
///
/// Newlines are defined with the newline character `\n`.
pub fn line_count(input: &str) -> usize {
    // all input has at least one line,
    // no matter if it contains a newline character
    1 + input.matches('\n').count()
}

#[cfg(test)]
mod unit_tests {
    use crate::line_count;

    #[test]
    fn lines() {
        let input = "Hello, world!";
        let expected = 1;
        let actual = line_count(input);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod file_tests {
    use std::fs::read_to_string;

    use crate::line_count;

    #[test]
    fn lines() {
        let input = read_to_string("test.txt").expect("should find test file");
        let expected = 7137;
        let actual = line_count(input.as_str());
        assert_eq!(actual, expected);
    }
}
