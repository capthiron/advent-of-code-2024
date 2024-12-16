pub fn solve_part1(input: &str) -> i64 {
    solve(input, &[Operation::Add, Operation::Multiply])
}

pub fn solve_part2(input: &str) -> i64 {
    solve(
        input,
        &[Operation::Add, Operation::Multiply, Operation::Concatenate],
    )
}

fn solve(input: &str, operations: &[Operation]) -> i64 {
    let equations = parse_input(input);
    equations
        .iter()
        .map(|(test_value, numbers)| {
            let configs = create_configurations(numbers.len() - 1, operations);
            for config in configs {
                let mut result = numbers[0];
                for (i, op) in config.iter().enumerate() {
                    result = op.apply(result, numbers[i + 1]);
                }

                if result == *test_value {
                    return *test_value; // Return the test value if the equation is satisfied
                }
            }

            0 // Return 0 if no configuration satisfies the equation
        })
        .sum()
}

fn create_configurations(amount: usize, operations: &[Operation]) -> Vec<Vec<Operation>> {
    let mut results = vec![];

    generate_combinations(operations, amount, &mut vec![], &mut results);

    results
}

fn generate_combinations(
    operations: &[Operation],
    amount: usize,
    current: &mut Vec<Operation>,
    results: &mut Vec<Vec<Operation>>,
) {
    if current.len() == amount {
        results.push(current.clone());
        return;
    }

    for op in operations {
        current.push(op.clone());
        generate_combinations(operations, amount, current, results);
        current.pop();
    }
}

#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => format!("{}{}", a, b).parse::<i64>().unwrap(),
        }
    }
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|equation| {
            let parts: Vec<&str> = equation.split(": ").collect();
            let test_value = parts[0].parse::<i64>().unwrap();
            let numbers = parts[1]
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            (test_value, numbers)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(solve_part1(input), 3749);
    }

    #[test]
    fn test_solve_part2() {}
}
