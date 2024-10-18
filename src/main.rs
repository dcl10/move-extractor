use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, Write};
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
    let pattern = Regex::new(r"\d+\.\s*").unwrap();

    let result = pattern.replace_all(&haystack, "");
    result.to_string()
}

fn parse_winner(game: &str) -> (String, isize) {
    // Compile the regular expression
    let pattern = Regex::new(r"(?P<game>.+)\s+(?P<winner>1-0|0-1|1/2-1/2)").unwrap();

    // Capture the game moves and winner
    if let Some(captures) = pattern.captures(game) {
        let game_moves = captures.name("game").unwrap().as_str().to_string(); // Capture moves as string
        let winner_str = captures.name("winner").unwrap().as_str(); // Capture winner

        // Map the winner to an integer
        let winner = match winner_str {
            "1-0" => 1,     // White wins
            "0-1" => -1,    // Black wins
            "1/2-1/2" => 0, // Draw
            _ => 0,         // Default (shouldn't happen in a valid game)
        };

        return (game_moves, winner); // Return the tuple with game and winner
    }

    // If the regex doesn't match, return an empty game and draw as default
    ("".to_string(), 0)
}

fn make_file<P>(path: P) -> io::Result<File>
where
    P: AsRef<Path>,
{
    OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
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

    if let Ok(lines) = read_lines(pgn_file) {
        let mut output: Vec<String> = Vec::new();
        // Extract games
        println!("Extracting games...");
        for line in lines.flatten() {
            let stripped = line.trim();
            if stripped.is_empty() {
                found_game_string = !found_game_string;
            }
            if found_game_string {
                let game_string = remove_move_counts(stripped);
                output.push(game_string);
            } else {
                // write output to file
                let file = make_file("./results.txt");
                match file {
                    Ok(mut f) => {
                        if !output.is_empty() {
                            let mut text = output.join(" ");
                            // Sometimes there are 2 spaces instead of just one
                            text = text.replace("  ", " ");
                            writeln!(f, "{}", text).unwrap()
                        }
                    }
                    Err(e) => println!("{e}"),
                }
                // reset output
                output = Vec::new()
            }
        }
        // Split into game moves and winners
        println!("Separating games and winners...");
        if let Ok(games) = read_lines("./results.txt") {
            for game in games.flatten() {
                let (g, w) = parse_winner(&game);
                if let Ok(mut gf) = make_file("./games.txt") {
                    writeln!(gf, "{g}").unwrap()
                }
                if let Ok(mut wf) = make_file("./winners.txt") {
                    writeln!(wf, "{w}").unwrap()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_winner, remove_move_counts};

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

    #[test]
    fn test_parse_winner() {
        // Arrange
        let input = "d4 Nf6 c4 e6 Nc3 Bb4 a3 Bxc3+ bxc3 c5 e3 Nc6 Bd3 b6 e4 e5 d5 Na5 f4 d6 fxe5 dxe5 Nf3 Qe7 Bg5 h6 Bh4 g5 Bg3 Nd7 O-O h5 Be2 Nb7 d6 Nxd6 Nxe5 Nxe5 Qd5 Bb7 Qxe5 Rh6 Qg7 Rg6 Qh8+ Kd7 Qxh5 Qxe4 Bg4+ f5 Qh7+ 1-0";
        let (expected_game, expected_winner) = ("d4 Nf6 c4 e6 Nc3 Bb4 a3 Bxc3+ bxc3 c5 e3 Nc6 Bd3 b6 e4 e5 d5 Na5 f4 d6 fxe5 dxe5 Nf3 Qe7 Bg5 h6 Bh4 g5 Bg3 Nd7 O-O h5 Be2 Nb7 d6 Nxd6 Nxe5 Nxe5 Qd5 Bb7 Qxe5 Rh6 Qg7 Rg6 Qh8+ Kd7 Qxh5 Qxe4 Bg4+ f5 Qh7+", 1);

        // Act
        let (actual_game, actual_winner) = parse_winner(input);

        // Assert
        assert_eq!(expected_game, actual_game);
        assert_eq!(expected_winner, actual_winner)
    }
}
