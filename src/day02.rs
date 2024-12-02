pub fn solve_part1(input: &str) -> i32 {
    let reports = input.lines();

    let mut safe_reports = 0;
    for report in reports {
        let levels = extract_levels_from(report);
        if is_safe(&levels) {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn solve_part2(input: &str) -> i32 {
    let reports = input.lines();

    let mut safe_reports = 0;
    for report in reports {
        let levels = extract_levels_from(report);
        if is_safe(&levels) {
            safe_reports += 1;
            continue;
        }

        for i in 0..levels.len() {
            if is_safe(&[&levels[..i], &levels[i + 1..]].concat()) {
                safe_reports += 1;
                break;
            }
        }
    }

    safe_reports
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

fn extract_levels_from(report: &str) -> Vec<i32> {
    let levels: Vec<i32> = report
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();
    levels
}

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
