//! The `counter` crate provides functionality for counting bytes, lines and words of an input string.

mod counters;

use cli::Args;
use counters::{byte_count, line_count, word_count};

/// The `run` function runs the program in its entirety, parsing command line arguments,
/// performing the specified tasks and printing to the user.
pub fn run() {
    // let args = Args::get_args();

    // let mut file_table = Vec::new();
    // for file_path in args.files() {
    //     let mut temp_str = Vec::new();
    //     let file_content = match std::fs::read_to_string(file_path) {
    //         Ok(fc) => fc,
    //         Err(e) => {
    //             println!("{file_path}: {e}");
    //             continue;
    //         }
    //     };

    //     if args.lines() {
    //         temp_str.push(line_count(&file_content).to_string());
    //     }
    //     if args.words() {
    //         temp_str.push(word_count(&file_content).to_string());
    //     }
    //     if args.bytes() {
    //         temp_str.push(byte_count(&file_content).to_string());
    //     }
    //     temp_str.push(file_path.to_owned());

    //     file_table.push(temp_str.join(" "));
    // }

    // for file in file_table {
    //     println!("{file}");
    // }
}
