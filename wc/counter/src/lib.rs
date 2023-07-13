//! The `counter` crate provides functionality for counting bytes, lines and words of an input string.

mod counters;

use cli::Args;
use counters::{byte_count, line_count, word_count};

/// The `run` function runs the program in its entirety, parsing command line arguments,
/// performing the specified tasks and printing to the user.
pub fn run() {
    let args = Args::get_args();

    let mut file_table = Vec::new();
    for file_path in args.files() {
        let mut new_line = PrintLine::new(file_path);
        let file_content = match std::fs::read_to_string(file_path) {
            Ok(fc) => fc,
            Err(e) => {
                println!("{file_path}: {e}");
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

    if file_table.len() > 1 {
        let mut max_length = 0;
        let mut total = PrintLine::new("total");
        if args.lines() {
            let mut total_lines = 0;
            for file in file_table.iter() {
                total_lines += file.lines.unwrap();
            }
            max_length = max_length.max(total_lines.to_string().len());
            total.lines = Some(total_lines);
        }
        if args.words() {
            let mut total_words = 0;
            for file in file_table.iter() {
                total_words += file.words.unwrap();
            }
            max_length = max_length.max(total_words.to_string().len());
            total.words = Some(total_words);
        }
        if args.bytes() {
            let mut total_bytes = 0;
            for file in file_table.iter() {
                total_bytes += file.bytes.unwrap();
            }
            max_length = max_length.max(total_bytes.to_string().len());
            total.bytes = Some(total_bytes);
        }
        file_table.push(total);

        for line in file_table {
            println!("{}", line.format(max_length));
        }
    } else {
        println!("{}", file_table[0].format(0));
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
