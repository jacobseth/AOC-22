use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Score of shape _YOU_ select plus the outcome of the round

#[derive(Copy, Clone)] 
#[derive(PartialEq)]
#[derive(Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn get_shape_score(shape: Shape) -> u64 {
    return match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    }
}

fn outcome_score(outcome: Outcome) -> u64 {
    return match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6
    }
}

fn outcome_char_to_enum(outcome_char: char) -> Option<Outcome> {
    return match outcome_char {
        'X' => Some(Outcome::Lose),
        'Y' => Some(Outcome::Draw),
        'Z' => Some(Outcome::Win),
        _ => None
    }
}

fn main() {
    let mut total = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./guide") {
        // Consumes the iterator, returns an (Optional) String

        for line in lines {
            if let Ok(moves) = line {
                let opp_move_char = moves.chars().nth(0).unwrap();
                let my_outcome_char = moves.chars().nth(2).unwrap();
                let opp_move = match opp_move_char {
                    'A' => Some(Shape::Rock),
                    'B' => Some(Shape::Paper),
                    'C' => Some(Shape::Scissors),
                    _ => None
                };
                
                let outcome_opt = outcome_char_to_enum(my_outcome_char);
                let outcome = outcome_opt.unwrap(); 
                
                let my_move = calculate_shape(opp_move.unwrap(), outcome);

                total = total + get_shape_score(my_move);

                total = total + outcome_score(outcome)
            }
        }
    }

    println!("Total: {}", total);
}

fn calculate_shape(opponent_move: Shape, outcome: Outcome) -> Shape {
    match outcome {
        Outcome::Win => {
            return match opponent_move {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock
            }
        }
        Outcome::Lose => {
            return match opponent_move {
                Shape::Scissors => Shape::Paper,
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock
            }
        }
        Outcome::Draw => {
            return opponent_move
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}