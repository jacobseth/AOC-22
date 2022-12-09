use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {

    let mut all_triplicate_items = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./rucksacks") {
        // Consumes the iterator, returns an (Optional) String
        let mut line_count = 0;
        let mut first_rucksack = HashMap::new();
        let mut second_rucksack = HashMap::new();
        for line in lines {
            if let Ok(rucksack_content) = line {
                for item in rucksack_content.chars() {
                    match line_count % 3 {
                        0 => {
                            first_rucksack.insert(item, true)
                        }
                        1 => {
                            match first_rucksack.get(&item) {
                                Some(_) => second_rucksack.insert(item, true),
                                _ => None
                            }
                        }
                        2 => match second_rucksack.get(&item) {
                            Some(_) => {
                                all_triplicate_items.push(item);
                                break
                            }
                            _ => None
                        }
                        _ => None // Not possible currently
                    };
                }
            }
            line_count = line_count + 1;
            if (line_count % 3) == 0 {
                // Reset all collections
                first_rucksack = HashMap::new();
                second_rucksack= HashMap::new();                
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

    let mut alphabet_scores = HashMap::new();
    let mut character_id = 1;
    for character in alphabet {
        alphabet_scores.insert(character, character_id);
        character_id = character_id + 1;
    }
    println!("{:?}", alphabet_scores);
    println!("Total: {:?}", all_triplicate_items);

    let mut final_score = 0;
    for item in all_triplicate_items {
        match alphabet_scores.get(&item) {
            Some(score) => final_score = final_score + score,
            None => ()
        }
    }

    println!("Final Score: {}", final_score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}