//! The `counter` crate provides functionality for counting bytes, lines and words of an input string.

mod counters;

use cli::Cli;
use counters::{byte_count, line_count, word_count};

/// The `run` function runs the program in its entirety, parsing command line arguments,
/// performing the specified tasks and printing to the user.
pub fn run() {
    let args = Cli::get_args();

    let mut file_table = Vec::new();
    for file_path in args.files() {
        let mut new_line = PrintLine::new(file_path);
        let file_content = match std::fs::read_to_string(file_path) {
            Ok(fc) => fc,
            Err(e) => {
                println!("rswc: {file_path}: {e}");
                continue;
            }
        };

        if args.lines() {
            new_line.lines = Some(line_count(&file_content));
        }
        if args.words() {
            new_line.words = Some(word_count(&file_content));
        }
        if args.bytes() {
            new_line.bytes = Some(byte_count(&file_content));
        }

        file_table.push(new_line);
    }

    match file_table.len() {
        0 => {}
        1 => println!("{}", file_table[0].format(0)),
        _ => {
            let mut total = PrintLine::new("total");
            for file in file_table.iter() {
                if args.lines() {
                    total.lines = combine(total.lines, file.lines);
                }
                if args.words() {
                    total.words = combine(total.words, file.words);
                }
                if args.bytes() {
                    total.bytes = combine(total.bytes, file.bytes);
                }
            }

            let max_length = (total.lines.max(total.words).max(total.bytes))
                .unwrap()
                .to_string()
                .len();

            file_table.push(total);
            for line in file_table {
                println!("{}", line.format(max_length));
            }
        }
    }
}

fn combine(first: Option<usize>, second: Option<usize>) -> Option<usize> {
    match (first, second) {
        (None, None) => None,
        (None, Some(a)) | (Some(a), None) => Some(a),
        (Some(a), Some(b)) => Some(a + b),
    }
}

struct PrintLine<'a> {
    file_name: &'a str,
    bytes: Option<usize>,
    words: Option<usize>,
    lines: Option<usize>,
}

impl<'a> PrintLine<'a> {
    #[inline]
    fn new(file_name: &'a str) -> Self {
        Self {
            file_name,
            bytes: None,
            words: None,
            lines: None,
        }
    }

    #[inline]
    /// The `format` function returns a formatted string based on the contents of the `PrintLine` object.
    ///
    /// Only defines values are printed (ie. `None` values are ignored).
    ///
    /// Furthermore, the string contents are right-aligned based on the passed input argument.
    fn format(&self, max_length: usize) -> String {
        // The format string used does the following:
        // - the leading space after the colon, means that the fill character is whitespace
        // - '>' right-aligns the contents of the print
        // - 'ml$' specifies the length of the print, and thereby the amount of fill
        match (self.lines, self.words, self.bytes) {
            (None, None, None) => todo!(),
            (None, None, Some(a)) | (None, Some(a), None) | (Some(a), None, None) => {
                format!("{: >ml$} {}", a, self.file_name, ml = max_length)
            }
            (None, Some(a), Some(b)) | (Some(a), None, Some(b)) | (Some(a), Some(b), None) => {
                format!(
                    "{: >ml$} {: >ml$} {}",
                    a,
                    b,
                    self.file_name,
                    ml = max_length
                )
            }
            (Some(a), Some(b), Some(c)) => {
                format!(
                    "{: >ml$} {: >ml$} {: >ml$} {}",
                    a,
                    b,
                    c,
                    self.file_name,
                    ml = max_length
                )
            }
        }
    }
}
