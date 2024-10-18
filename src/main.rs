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
    )
}
