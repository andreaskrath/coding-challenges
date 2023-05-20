pub struct Uniq<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Uniq<'a> {
    pub fn new(input: &'a str) -> Self {
        let lines: Vec<&str> = input.split('\n').collect();
        Self { lines }
    }

    pub fn get_uniq_lines(&self) -> Vec<&str> {
        let mut unique_lines = vec![*self.lines.first().unwrap()];

        // Starting off the first iteration with the very first line being trivially unique.
        // Thereby when unique lines are found, the latter of the two is added to ensure
        // that each iteration holds its own, and that the very line last will also be considered.
        for line in self.lines.windows(2) {
            if (!line[0].is_empty() && !line[1].is_empty()) && (line[0] != line[1]) {
                unique_lines.push(line[1]);
            }
        }

        unique_lines
    }
}

#[cfg(test)]
#[allow(clippy::derivable_impls)]
impl<'a> Default for Uniq<'a> {
    fn default() -> Self {
        Self { lines: Vec::new() }
    }
}

#[cfg(test)]
mod uniq {
    use super::Uniq;

    #[test]
    fn two_duplicates() {
        let mut unique_finder = Uniq::default();
        unique_finder.lines.push("abc");
        unique_finder.lines.push("abc");
        unique_finder.lines.push("bbc");
        unique_finder.lines.push("aaa");
        unique_finder.lines.push("ddd");
        unique_finder.lines.push("ddd");
        unique_finder.lines.push("abc");

        let expected = unique_finder.lines.len() - 2;
        let actual = unique_finder.get_uniq_lines().len();

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_duplicates() {
        let mut unique_finder = Uniq::default();
        unique_finder.lines.push("aaa");
        unique_finder.lines.push("bbb");
        unique_finder.lines.push("ccc");
        unique_finder.lines.push("ddd");
        unique_finder.lines.push("eee");
        unique_finder.lines.push("fff");
        unique_finder.lines.push("ggg");

        let expected = unique_finder.lines.len();
        let actual = unique_finder.get_uniq_lines().len();

        assert_eq!(actual, expected);
    }
}
