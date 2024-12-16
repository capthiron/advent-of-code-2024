use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod utils;

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
        }
        "2" => {
            let input = utils::read_file_to_string("input/day02.txt").unwrap();
            println!("Day 2 - Part 1: {}", day02::solve_part1(&input));
            println!("Day 2 - Part 2: {}", day02::solve_part2(&input));
        }
        "3" => {
            let input = utils::read_file_to_string("input/day03.txt").unwrap();
            println!("Day 3 - Part 1: {}", day03::solve_part1(&input));
            println!("Day 3 - Part 2: {}", day03::solve_part2(&input));
        }
        "4" => {
            let input = utils::read_file_to_string("input/day04.txt").unwrap();
            println!("Day 4 - Part 1: {}", day04::solve_part1(&input));
            println!("Day 4 - Part 2: {}", day04::solve_part2(&input));
        }
        "5" => {
            let input = utils::read_file_to_string("input/day05.txt").unwrap();
            println!("Day 5 - Part 1: {}", day05::solve_part1(&input));
            println!("Day 5 - Part 2: {}", day05::solve_part2(&input));
        }
        "6" => {
            let input = utils::read_file_to_string("input/day06.txt").unwrap();
            println!("Day 6 - Part 1: {}", day06::solve_part1(&input));
            println!("Day 6 - Part 2: {}", day06::solve_part2(&input));
        }
        "7" => {
            let input = utils::read_file_to_string("input/day07.txt").unwrap();
            println!("Day 7 - Part 1: {}", day07::solve_part1(&input));
            println!("Day 7 - Part 2: {}", day07::solve_part2(&input));
        }
        _ => {
            eprintln!("Invalid day: {}. Please enter a valid day number.", day);
        }
    }
}
