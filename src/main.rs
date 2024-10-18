mod cli;

use clap::{arg, Parser};
use cli::Args;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Args = Args::parse();
    let n_threads = args.n_threads;
    let batch_size = args.batch_size;
    let pgn_file = args.pgn;
    println!(
        "file: {}; n threads: {}; batch size: {}",
        pgn_file, n_threads, batch_size
    );
    let mut found_game_string = false;
    let mut count = 0usize;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(pgn_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let stripped = line.trim();
            if stripped.is_empty() {
                found_game_string = !found_game_string;
            } else {
                count += 1;
                if found_game_string {
                    println!("{}", stripped);
                }
            }
        }
        println!("Found {count} games")
    }
}
