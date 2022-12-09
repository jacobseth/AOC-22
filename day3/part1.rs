use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {

    let mut all_duplicate_items = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./rucksacks") {
        // Consumes the iterator, returns an (Optional) String

        for line in lines {
            if let Ok(rucksack_content) = line {
                let mut first_rucksack = HashMap::new();
                let size = rucksack_content.chars().count();

                let mut id = 0;
                for item in rucksack_content.chars() {
                    if id < (size / 2) { // First Rucksack
                        first_rucksack.insert(item, true);
                        println!("{}", item)
                    } else { // Second Rucksack
                        match first_rucksack.get(&item) {
                            Some(_) => {
                                all_duplicate_items.push(item);
                                break
                            }
                            _ => ()
                        }
                    }

                    id = id + 1;
                }
            }
        }
    }

    let mut alphabet = (b'a'..=b'z')           // Start as u8
        .map(|c| c as char)            // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect::<Vec<_>>();          // Collect as Vec<char>

    let mut upper_alphabet = (b'A'..=b'Z')           // Start as u8
        .map(|c| c as char)            // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect::<Vec<_>>();          // Collect as Vec<char>

    alphabet.append(&mut upper_alphabet);

    let mut alpha_scores = HashMap::new();
    let mut alpha_id = 1;
    for item in alphabet {
        alpha_scores.insert(item, alpha_id);
        alpha_id = alpha_id + 1;
    }

    println!("Items: {:?}", all_duplicate_items);

    let mut total_score = 0;
    for dupe in all_duplicate_items {
        match alpha_scores.get(&dupe) {
            Some(score) => total_score = total_score + score,
            _ => ()
        }
    }

    println!("Score: {}", total_score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}