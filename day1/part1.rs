use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut most_caloric_elf: i64 = -1;
    let mut most_caloric_elf_count: u64 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./elf_calories") {
        // Consumes the iterator, returns an (Optional) String
        let mut current_elf: i64 = 1;
        let mut current_elf_calories: u64 = 0;

        for line in lines {
            if let Ok(maybe_calorie) = line {
                if maybe_calorie.is_empty() {
                    if current_elf_calories > most_caloric_elf_count {
                        most_caloric_elf = current_elf;
                        most_caloric_elf_count = current_elf_calories;
                    }

                    current_elf = current_elf + 1;
                    current_elf_calories = 0;
                } else {
                    match maybe_calorie.parse::<u64>() {
                        Ok(n) => {
                            current_elf_calories = current_elf_calories + n;
                        }
                        Err(e) => println!("bad input: {}", e),
                    }
                }
            }
        }
    }

    println!("most caloric elf: {}", most_caloric_elf);
    println!("calories: {}", most_caloric_elf_count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}