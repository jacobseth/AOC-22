use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elves: Vec<(i64, u64)> = vec![];
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./elf_calories") {
        // Consumes the iterator, returns an (Optional) String
        let mut current_elf: (i64, u64) = (1, 0);

        for line in lines {
            if let Ok(maybe_calorie) = line {
                if maybe_calorie.is_empty() {
                    let idx = elves.partition_point(|&x| x.1 < current_elf.1);
                    elves.insert(idx, current_elf);

                    current_elf.0 = current_elf.0 + 1;
                    current_elf.1 = 0;
                } else {
                    match maybe_calorie.parse::<u64>() {
                        Ok(n) => {
                            current_elf.1 = current_elf.1 + n;
                        }
                        Err(e) => println!("bad input: {}", e),
                    }
                }
            }
        }

        // Last elf wasn't counted
        if current_elf.1 != 0 {
            let idx = elves.partition_point(|&x| x.1 < current_elf.1);
            elves.insert(idx, current_elf);

            current_elf.0 = current_elf.0 + 1;
            current_elf.1 = 0;
        }
    }

    println!("elves: {:?}", elves);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}