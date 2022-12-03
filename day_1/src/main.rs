// import utils module
// can only access public methods of this module
mod utils;

// Elf struct
// For sorting, the struct needs the traits Eq, PartialEq, Ord and PartialOrd
// Simply derive them
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    calories: u32,
    idx: u16
}

// Main function
fn main() {
    // Read and parse input
    let input = utils::get_input();
    let input_split = input.split("\r");

    // Vector to store all elves
    let mut elves = vec![];

    // Tracker variable: Total calories held by an elf
    let mut total = 0 as u32;
    // Tracker variable: Elf index
    let mut elv_idx = 0;

    // Read each line
    for (_i, line) in input_split.enumerate() {
        let calories = line.trim();

        if !calories.is_empty() {
            // calories is str, we need to parse it to an integer.
            // any better method?
            total = total + calories.parse::<u32>().unwrap();
        } else {
            // finished processing an elf
            elves.push(Elf { calories: total, idx: elv_idx });

            // Reset tracker variables
            elv_idx = elv_idx + 1;
            total = 0;
        }
    }
    // Final elf
    elves.push(Elf { calories: total, idx: elv_idx });

    // Sort elves by calories
    // in ascending order
    elves.sort_by_key(|elf| elf.calories);

    // Pop the last element
    let elf_1 = elves.pop().unwrap();
    let elf_2 = elves.pop().unwrap();
    let elf_3 = elves.pop().unwrap();

    println!("Largest calories {} for elf {}", elf_1.calories, elf_1.idx);
    println!("2nd largest calories {} for elf {}", elf_2.calories, elf_2.idx);
    println!("3rd largest calories {} for elf {}", elf_3.calories, elf_3.idx);
    println!("Total calories {}", elf_3.calories + elf_2.calories + elf_1.calories);
}

// How to run the file?
// cargo build && cargo run