use std::collections::HashMap;

pub fn solve_part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    left.iter()
        .enumerate()
        .map(|(i, left_value)| (left_value - right.get(i).unwrap()).abs())
        .sum()
}

pub fn solve_part2(input: &str) -> i32 {
    let (left, right) = parse_input(input);
    let mut total_amount_in_right: HashMap<&i32, i32> = HashMap::new();

    right
        .iter()
        .for_each(|value| *total_amount_in_right.entry(value).or_insert(0) += 1);

    left.iter()
        .map(|value| value * total_amount_in_right.get(value).unwrap_or(&0))
        .sum()
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left.push(iter.next().unwrap().parse().unwrap());
        right.push(iter.next().unwrap().parse().unwrap());
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3"
            .to_string();
        assert_eq!(solve_part1(&input), 11);
    }

    #[test]
    fn test_solve_part2() {
        let input = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3"
            .to_string();
        assert_eq!(solve_part2(&input), 31)
    }
}
