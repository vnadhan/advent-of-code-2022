// import utils module
// can only access public methods of this module
mod utils;

fn get_shape_score(shape: &char) -> u16 {
    let score = match shape {
        'X' | 'A' => 1,
        'Y' | 'B' => 2,
        'Z' | 'C' => 3,
        value => panic!("Value should be one of [A, B, C, X, Y, Z], got {}.", value),
    };
    score
}

enum Outcome {
    Win,
    Draw,
    Loss
}

fn get_outcome_score(outcome: &Outcome) -> u16 {
    let score = match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    };
    score
}

// Main function
fn main() {
    // Read and parse input
    let input = utils::get_input();
    let input_split = input.split('\n');

    // Rock -> Paper -> Scissor
    // elf 1
    let winning_links_abc = ['A', 'B', 'C'];
    // elf 2
    let winning_links_xyz = ['X', 'Y', 'Z'];

    // Tracker variable: Total score by an elf
    let mut total = 0 as u16;

    // Read each line
    for (_i, line) in input_split.enumerate() {
        let mut choices = line.trim().split::<char>(' ');
        let elf_1_choice: Vec<char> = choices.next().unwrap().chars().collect();
        let elf_2_choice: Vec<char> = choices.next().unwrap().chars().collect();

        let elf_1_choice_idx = winning_links_abc.binary_search(&elf_1_choice[0]).unwrap();
        let elf_2_choice_idx = winning_links_xyz.binary_search(&elf_2_choice[0]).unwrap();

        // Part 1
        if elf_2_choice_idx < elf_1_choice_idx {
            if elf_1_choice_idx - elf_2_choice_idx == 2 {
                // rock and scissors
                total = total + get_shape_score(&elf_2_choice[0]) + get_outcome_score(&Outcome::Win);
            } else {
                total = total + get_shape_score(&elf_2_choice[0]) + get_outcome_score(&Outcome::Loss);
            }
        } else {
            if elf_2_choice_idx == elf_1_choice_idx {
                total = total + get_shape_score(&elf_2_choice[0]) + get_outcome_score(&Outcome::Draw);
            } else if elf_2_choice_idx - elf_1_choice_idx == 1 {
                total = total + get_shape_score(&elf_2_choice[0]) + get_outcome_score(&Outcome::Win);
            } else {
                total = total + get_shape_score(&elf_2_choice[0]) + get_outcome_score(&Outcome::Loss);
            }
        }

        // Part 2
        // if elf_2_choice[0] == 'X' {
        //     // Loss
        //     if elf_1_choice_idx == 0 {
                   // We need a scissor to lose to a rock
        //         total = total + get_shape_score(&winning_links_abc[2]) + get_outcome_score(&Outcome::Loss);
        //     } else {
        //         total = total + get_shape_score(&winning_links_abc[elf_1_choice_idx - 1]) + get_outcome_score(&Outcome::Loss);
        //     }
        // } else if elf_2_choice[0] == 'Y' {
        //     // Draw
        //     total = total + get_shape_score(&elf_1_choice[0]) + get_outcome_score(&Outcome::Draw);
        // } else if elf_2_choice[0] == 'Z' {
        //     // Win
        //     if elf_1_choice_idx == 2 {
                   // We need a rock to beat a scissor
        //         total = total + get_shape_score(&winning_links_abc[0]) + get_outcome_score(&Outcome::Win);
        //     } else {
        //         total = total + get_shape_score(&winning_links_abc[elf_1_choice_idx + 1]) + get_outcome_score(&Outcome::Win);
        //     }
        // }
    }

    println!("Total score {}", total)
}