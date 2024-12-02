use std::collections::HashMap;

pub fn solve_part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    let mut sum_of_differences = 0;
    for (i, v) in left.iter().enumerate() {
        sum_of_differences += (v - right.get(i).unwrap()).abs()
    }

    sum_of_differences
}

pub fn solve_part2(input: &str) -> i32 {
    let (left, right) = parse_input(input);
    let mut total_amount_right = HashMap::new();

    let mut sum_of_similarity = 0;
    for v in left {
        match total_amount_right.get(&v) {
            Some(&amount) => sum_of_similarity += v * amount,
            None => {
                let mut amount_right = 0;
                for rv in &right {
                    if *rv == v {
                        amount_right += 1
                    }
                }
                total_amount_right.insert(v, amount_right);
                sum_of_similarity += v * amount_right
            }
        }
    }

    sum_of_similarity
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
