use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use clap::Parser;
use regex::Regex;

use cli::Args;

mod cli;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn remove_move_counts(line: &str) -> String {
    let haystack = String::from(line);
    let pattern = Regex::new(r"\d+\.\s+").unwrap();

    let result = pattern.replace_all(&haystack, "");
    result.to_string()
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

#[cfg(test)]
mod tests {
    use crate::remove_move_counts;

    #[test]
    fn test_remove_move_counts() {
        // Arrange
        let input = "1. Nf3 d5 2. c4 d4 3. d3 Nc6 4. g3 e5 5. Bg2 Bb4+ 6. Bd2 a5 7. O-O Nf6 8. Na3";
        let expected_output = "Nf3 d5 c4 d4 d3 Nc6 g3 e5 Bg2 Bb4+ Bd2 a5 O-O Nf6 Na3";

        // Act
        let actual_output = remove_move_counts(input);

        // Assert
        assert_eq!(expected_output, actual_output)
    }
}
