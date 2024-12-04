pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .filter(|report| is_safe(&parse_levels(report)))
        .count() as i32
}

pub fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .filter(|report| {
            let levels = parse_levels(report);
            // Check if removing one element results in a safe sequence
            (0..levels.len()).any(|i| {
                let (left, right) = levels.split_at(i);
                is_safe(&[left, &right[1..]].concat())
            })
        })
        .count() as i32
}

fn is_safe(levels: &[i32]) -> bool {
    let mut previous_level = levels[0];
    let direction = if levels[1] > levels[0] {
        Direction::Inc
    } else {
        Direction::Dec
    };
    let mut is_safe = true;

    for &level in &levels[1..] {
        match direction {
            Direction::Inc => {
                if level <= previous_level || level > previous_level + 3 {
                    is_safe = false;
                }
            }
            Direction::Dec => {
                if level >= previous_level || level < previous_level - 3 {
                    is_safe = false;
                }
            }
        }

        if !is_safe {
            break;
        }

        previous_level = level;
    }

    is_safe
}

fn parse_levels(report: &str) -> Vec<i32> {
    let levels: Vec<i32> = report
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();
    levels
}

#[derive(Debug)]
enum Direction {
    Inc,
    Dec,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(solve_part1(input), 2);
    }

    #[test]
    fn test_solve_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7"; //additional edge cases from https://www.reddit.com/r/adventofcode/comments/1h4shdu/2024_day_2_part2_edge_case_finder
        assert_eq!(solve_part2(input), 12);
    }
}
