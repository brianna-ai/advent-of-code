use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static ROCK: i32 = 1;
static PAPER: i32 = 2;
static SCISSORS: i32 = 3;
static LOSE: i32 = 0;
static DRAW: i32 = 3;
static WIN: i32 = 6;

// from rust-lang docs
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fixed shape inputs
fn points_scored_fixed(opp_play: char, your_play: char) -> i32 {
    match your_play {
        'X' => {
            match opp_play {
                'A' => ROCK + DRAW,
                'B' => ROCK + LOSE,
                'C' => ROCK + WIN,
                _ => 0, // shouldn't be possible
            }
        },
        'Y' => {
            match opp_play {
                'A' => PAPER + WIN,
                'B' => PAPER + DRAW,
                'C' => PAPER + LOSE,
                _ => 0, // shouldn't be possible
            }
        },
        'Z' => {
            match opp_play {
                'A' => SCISSORS + LOSE,
                'B' => SCISSORS + WIN,
                'C' => SCISSORS + DRAW,
                _ => 0, // shouldn't be possible
            }
        },
        _ => 0, // shouldn't be possible
    }
}

// dyn shape inputs (for your plays)
fn points_scored_dyn(opp_play: char, your_play: char) -> i32 {
    match your_play {
        'X' => {
            match opp_play {
                'A' => LOSE + SCISSORS,
                'B' => LOSE + ROCK,
                'C' => LOSE + PAPER,
                _ => 0, // shouldn't be possible
            }
        },
        'Y' => {
            match opp_play {
                'A' => DRAW + ROCK,
                'B' => DRAW + PAPER,
                'C' => DRAW + SCISSORS,
                _ => 0, // shouldn't be possible
            }
        },
        'Z' => {
            match opp_play {
                'A' => WIN + PAPER,
                'B' => WIN + SCISSORS,
                'C' => WIN + ROCK,
                _ => 0, // shouldn't be possible
            }
        },
        _ => 0, // shouldn't be possible
    }
}

// For part 1: 
// A = X = Rock     (1)
// B = Y = Paper    (2)
// C = Z = Scissors (3)
// Loss = 0, Draw = 3, Win = 6
// For part 2:
// X = LOSE
// Y = DRAW
// Z = WIN
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // src/[file] if running from day01

    let mut total_score = 0; 
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                // String to Vec<char>
                let chars: Vec<char> = ip.chars().collect::<Vec<_>>();
                // total_score = total_score + points_scored_fixed(chars[0], chars[2]);
                total_score = total_score + points_scored_dyn(chars[0], chars[2]);
            }
        }
    }
    println!("{}", total_score);
}
