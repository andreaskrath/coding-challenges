use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rswc",
    author = "Andreas Krath <andreas.krath@gmail.com>",
    version = "1.0",
    about = "A Rust rewrite of the Unix CLI tool 'wc'."
)]
pub struct Args {
    /// F
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
    /// The `get_args` function parses command line arguments from
    /// `std::env::args_os()`, and returns an instance of `Args`
    /// containing the parsed command line arguments, reacheable
    /// via their appropriate getters.
    pub fn get_args() -> Self {
        Args::parse()
    }

    /// A getter for the `bytes` CLI flag.
    pub fn bytes(&self) -> bool {
        self.bytes
    }

    /// A getter for the `words` CLI flag.
    pub fn words(&self) -> bool {
        self.words
    }

    /// A getter for the `lines` CLI flag.
    pub fn lines(&self) -> bool {
        self.lines
    }

    /// A getter for the file names passed as command line arguments.
    ///
    /// **It is not validated whether the file names refer to valid files.**
    pub fn files(&self) -> &Vec<String> {
        &self.files
    }
}
