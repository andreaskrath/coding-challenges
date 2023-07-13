use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rswc", author = "Andreas Krath <andreas.krath@gmail.com>")]
/// A Rust rewrite of the Unix CLI tool 'wc'.
///
/// Print newline, word, and byte counts for each file, and a total line if more than one file is specified.
///
/// A word is a non-zero-length sequence of printable chracters delimited by whitespace.
pub struct Args {
    /// The file name(s) to be affected by the tool
    files: Vec<String>,

    #[arg(short = 'c', long = "bytes")]
    /// Print the byte count
    bytes: bool,
    #[arg(short = 'w', long = "words")]
    /// Print the word count
    words: bool,
    #[arg(short = 'l', long = "lines")]
    /// Print the newline count
    lines: bool,
}

impl Args {
    #[inline]
    /// The `get_args` function parses command line arguments from
    /// `std::env::args_os()`, and returns an instance of `Args`
    /// containing the parsed command line arguments, reacheable
    /// via their appropriate getters.
    pub fn get_args() -> Self {
        Args::parse()
    }

    #[inline]
    /// A getter for the `bytes` CLI flag.
    pub fn bytes(&self) -> bool {
        self.bytes
    }

    #[inline]
    /// A getter for the `words` CLI flag.
    pub fn words(&self) -> bool {
        self.words
    }

    #[inline]
    /// A getter for the `lines` CLI flag.
    pub fn lines(&self) -> bool {
        self.lines
    }

    #[inline]
    /// A getter for the file contents of the file names passed as command line arguments.
    pub fn files(&self) -> &Vec<String> {
        &self.files
    }
}

// The 0th argument does not matter, but it must be present for parsing to function accordingly.
// During execution the 0th argument is typically the name of the binary.
#[cfg(test)]
mod args_cli {
    use crate::Args;
    use clap::Parser;

    #[test]
    // If this test case fails, then all other test cases relating the CLI cannot be considered trustworthy
    fn default_values() {
        let args = Args::parse_from([""]);
        assert!(!args.bytes);
        assert!(!args.words);
        assert!(!args.lines);
        assert!(args.files.is_empty())
    }

    #[test]
    fn bytes_short_flag() {
        let args = Args::parse_from(["", "-c"]);
        assert!(args.bytes);
    }

    #[test]
    fn bytes_long_flag() {
        let args = Args::parse_from(["", "--bytes"]);
        assert!(args.bytes);
    }

    #[test]
    fn words_short_flag() {
        let args = Args::parse_from(["", "-w"]);
        assert!(args.words);
    }

    #[test]
    fn words_long_flag() {
        let args = Args::parse_from(["", "--words"]);
        assert!(args.words);
    }

    #[test]
    fn lines_short_flag() {
        let args = Args::parse_from(["", "-l"]);
        assert!(args.lines);
    }

    #[test]
    fn lines_long_flag() {
        let args = Args::parse_from(["", "--lines"]);
        assert!(args.lines);
    }

    #[test]
    fn multiple_flags() {
        let args = Args::parse_from(["", "-c", "-l", "-w"]);
        assert!(args.bytes);
        assert!(args.lines);
        assert!(args.words);
    }

    #[test]
    fn single_file() {
        let args = Args::parse_from(["", "file.txt"]);
        assert_eq!(args.files, ["file.txt"]);
    }

    #[test]
    fn multiple_files() {
        let args = Args::parse_from(["", "file.txt", "another_file.rs"]);
        assert_eq!(args.files, ["file.txt", "another_file.rs"]);
    }

    #[test]
    fn flags_and_files() {
        let args = Args::parse_from(["", "-c", "-w", "-l", "file.txt", "newfile.txt", "main.rs"]);
        assert_eq!(args.files, ["file.txt", "newfile.txt", "main.rs"]);
        assert!(args.bytes);
        assert!(args.words);
        assert!(args.lines);
    }
}
