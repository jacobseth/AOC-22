use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashMap;

fn main() {

    let mut total_encapsulated_work = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./work_detail") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(pair_assignment) = line {
                let mut parsed_assignment = Vec::new();

                let assignments = pair_assignment.split(",");
                for assignment in assignments {
                    let mut individual_assignment: Vec<i32> = Vec::new();
                    let section_range = assignment.split("-");

                    for section_point in section_range {
                        individual_assignment.push(section_point.parse().unwrap());
                    }
                    parsed_assignment.push(individual_assignment);
                }

                if parsed_assignment[0][0] <= parsed_assignment[1][0] && parsed_assignment[0][1] >= parsed_assignment[1][0]{
                    println!("{}", pair_assignment);
                    total_encapsulated_work = total_encapsulated_work + 1
                } else if parsed_assignment[0][0] > parsed_assignment[1][0] && parsed_assignment[0][0] <= parsed_assignment[1][1] {
                    println!("{}", pair_assignment);
                    total_encapsulated_work = total_encapsulated_work + 1
                }
            }
        }
    }
    println!("Score: {}", total_encapsulated_work);
}

fn print_assignment(parsed_assignment: Vec<Vec<i32>>) {
    println!("hello: {:?}", parsed_assignment);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}