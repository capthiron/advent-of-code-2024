use std::env;

mod utils;
mod day01;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a day argument is provided
    if args.len() < 2 {
        eprintln!("Usage: aoc-rust <day>");
        return;
    }

    // Parse the day argument
    let day = &args[1];

    // Match the day argument to run the corresponding day's solution
    match day.as_str() {
        "1" => {
            let input = utils::read_file_to_string("input/day01.txt").unwrap();
            println!("Day 1 - Part 1: {}", day01::solve_part1(&input));
            println!("Day 1 - Part 2: {}", day01::solve_part2(&input));
        },
        // Add more cases for other days
        _ => {
            eprintln!("Invalid day: {}. Please enter a valid day number.", day);
        }
    }
}
