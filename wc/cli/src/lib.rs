use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rswc",
    author = "Andreas Krath <andreas.krath@gmail.com>",
    version = "1.0",
    about = "A Rust rewrite of the Unix CLI tool 'wc'."
)]
pub struct Args {
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
    pub fn get_args() -> Self {
        Args::parse()
    }

    pub fn bytes(&self) -> bool {
        self.bytes
    }

    pub fn words(&self) -> bool {
        self.words
    }

    pub fn lines(&self) -> bool {
        self.lines
    }
}
