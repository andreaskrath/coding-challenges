use clap::{Args, Parser};

#[derive(Parser)]
#[command(name = "rswc", author = "Andreas Krath <andreas.krath@gmail.com>")]
/// A Rust rewrite of the Unix CLI tool 'wc'.
///
/// Print newline, word, and byte counts for each file, and a total line if more than one file is specified.
///
/// A word is a non-zero-length sequence of printable chracters delimited by whitespace.
pub struct Cli {
    /// The file name(s) to be affected by the tool
    files: Vec<String>,

    #[command(flatten)]
    flags: Flags,
}

#[derive(Args)]
#[group(required = true, multiple = true)]
struct Flags {
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

impl Cli {
    #[inline]
    /// The `get_args` function parses command line arguments from
    /// `std::env::args_os()`, and returns an instance of `Args`
    /// containing the parsed command line arguments, reacheable
    /// via their appropriate getters.
    pub fn get_args() -> Self {
        Cli::parse()
    }

    #[inline]
    /// A getter for the `bytes` CLI flag.
    pub fn bytes(&self) -> bool {
        self.flags.bytes
    }

    #[inline]
    /// A getter for the `words` CLI flag.
    pub fn words(&self) -> bool {
        self.flags.words
    }

    #[inline]
    /// A getter for the `lines` CLI flag.
    pub fn lines(&self) -> bool {
        self.flags.lines
    }

    #[inline]
    /// A getter for the file names passed as command line arguments.
    ///
    /// **It is not validated whether the file names refer to valid files.**
    pub fn files(&self) -> &Vec<String> {
        &self.files
    }
}

// The 0th argument does not matter, but it must be present for parsing to function accordingly.
// During execution the 0th argument is typically the name of the binary.
#[cfg(test)]
mod args_cli {
    use crate::Cli;
    use clap::Parser;

    #[test]
    fn bytes_short_flag() {
        let args = Cli::parse_from(["", "-c"]);
        assert!(args.flags.bytes);
    }

    #[test]
    fn bytes_long_flag() {
        let args = Cli::parse_from(["", "--bytes"]);
        assert!(args.flags.bytes);
    }

    #[test]
    fn words_short_flag() {
        let args = Cli::parse_from(["", "-w"]);
        assert!(args.flags.words);
    }

    #[test]
    fn words_long_flag() {
        let args = Cli::parse_from(["", "--words"]);
        assert!(args.flags.words);
    }

    #[test]
    fn lines_short_flag() {
        let args = Cli::parse_from(["", "-l"]);
        assert!(args.flags.lines);
    }

    #[test]
    fn lines_long_flag() {
        let args = Cli::parse_from(["", "--lines"]);
        assert!(args.flags.lines);
    }

    #[test]
    fn multiple_flags() {
        let args = Cli::parse_from(["", "-c", "-l", "-w"]);
        assert!(args.flags.bytes);
        assert!(args.flags.lines);
        assert!(args.flags.words);
    }

    #[test]
    fn single_file() {
        // flag is not important, but must be present
        let args = Cli::parse_from(["", "-c", "file.txt"]);
        assert_eq!(args.files, ["file.txt"]);
    }

    #[test]
    fn multiple_files() {
        // flag is not important, but must be present
        let args = Cli::parse_from(["", "-c", "file.txt", "another_file.rs"]);
        assert_eq!(args.files, ["file.txt", "another_file.rs"]);
    }

    #[test]
    fn flags_and_files() {
        let args = Cli::parse_from(["", "-c", "-w", "-l", "file.txt", "newfile.txt", "main.rs"]);
        assert_eq!(args.files, ["file.txt", "newfile.txt", "main.rs"]);
        assert!(args.flags.bytes);
        assert!(args.flags.words);
        assert!(args.flags.lines);
    }
}
