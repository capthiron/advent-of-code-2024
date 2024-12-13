use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> i32 {
    let (rule_map, print_vec) = parse_input(input);

    print_vec
        .iter()
        .filter(|print| is_ordered(print, &rule_map))
        .map(|print| get_middle_page(print))
        .sum()
}

pub fn solve_part2(input: &str) -> i32 {
    let (rule_map, print_vec) = parse_input(input);

    print_vec
        .iter()
        .filter(|print| !is_ordered(print, &rule_map))
        .map(|print| {
            let mut ordered_print = print.clone();
            ordered_print.sort_by(|a, b| {
                if rule_map.get(a).map_or(false, |set| set.contains(b)) {
                    return Ordering::Less;
                }
                if rule_map.get(b).map_or(false, |set| set.contains(a)) {
                    return Ordering::Greater;
                }
                Ordering::Equal
            });
            ordered_print
        })
        .map(|print| get_middle_page(&print))
        .sum()
}

fn is_ordered(print: &[i32], rule_map: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut pages_before: HashSet<i32> = HashSet::new();
    for page in print {
        let page_rule = rule_map.get(page);
        if page_rule.is_some() && !pages_before.is_disjoint(page_rule.unwrap()) {
            return false;
        }
        pages_before.insert(*page);
    }
    true
}

fn get_middle_page(print: &[i32]) -> i32 {
    if print.is_empty() {
        return 0;
    }

    print[print.len() / 2]
}

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let binding = input.split("\n\n").collect::<Vec<&str>>();
    let rules = binding.first().unwrap();
    let prints = binding.get(1).unwrap();

    let mut rule_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for rule in rules.lines() {
        let source = rule
            .split("|")
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let target = rule
            .split("|")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        rule_map
            .entry(source)
            .and_modify(|s| {
                s.insert(target);
            })
            .or_insert({
                let mut set = HashSet::new();
                set.insert(target);
                set
            });
    }

    let mut print_vec: Vec<Vec<i32>> = vec![];
    for print in prints.lines() {
        print_vec.push(
            print
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        );
    }

    (rule_map, print_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(solve_part1(input), 143)
    }

    #[test]
    fn test_solve_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(solve_part2(input), 123)
    }
}
