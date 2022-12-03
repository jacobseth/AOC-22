use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// Score of shape _YOU_ select plus the outcome of the round

#[derive(Hash)]
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


fn main() {
    let shape_selected_score = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3)
    ]);
    let outcome_score = HashMap::from([
        (Outcome::Lose, 0),
        (Outcome::Draw, 3),
        (Outcome::Win, 6),
    ]);

    let mut total = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./guide") {
        // Consumes the iterator, returns an (Optional) String

        for line in lines {
            if let Ok(moves) = line {
                let opp_move_char = moves.chars().nth(0).unwrap();
                let my_move_char = moves.chars().nth(2).unwrap();
                let opp_move = match opp_move_char {
                    'A' => Some(Shape::Rock),
                    'B' => Some(Shape::Paper),
                    'C' => Some(Shape::Scissors),
                    _ => None
                };
                
                let my_move = match my_move_char {
                    'X' => Some(Shape::Rock),
                    'Y' => Some(Shape::Paper),
                    'Z' => Some(Shape::Scissors),
                    _ => None
                };

                match shape_selected_score.get(&my_move_char) {
                    Some(score) => total = total + score,
                    None => println!("missed move score")
                }

                match outcome_score.get(&calculate_outcome(opp_move.unwrap(), my_move.unwrap())) {
                    Some(score) => total = total + score,
                    None => println!("missed outcome score")
                }
            }
        }
    }

    println!("Total: {}", total);
}

fn calculate_outcome(opponent_move: Shape, my_move: Shape) -> Outcome {
    if opponent_move == my_move {
        return Outcome::Draw
    }

    match opponent_move {
        Shape::Rock => if my_move == Shape::Paper {return Outcome::Win} else {return Outcome::Lose}
        Shape::Paper => if my_move == Shape::Scissors {return Outcome::Win} else {return Outcome::Lose}
        Shape::Scissors => if my_move == Shape::Rock {return Outcome::Win} else {return Outcome::Lose}
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}