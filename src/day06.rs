use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> i32 {
    let map: Vec<Vec<char>> = parse_map(input);
    scrape_map_for_guard(&map).0
}

pub fn solve_part2(input: &str) -> i32 {
    let map = parse_map(input);
    let path_to_walk = scrape_map_for_guard(&map).1;

    let mut map_variants: Vec<Vec<Vec<char>>> = vec![];
    for (x, y) in path_to_walk {
        let c = map[y as usize][x as usize];
        if c != '^' || c != 'v' || c != '<' || c != '>' {
            let mut map_variant = map.clone();
            map_variant[y as usize][x as usize] = '#';
            map_variants.push(map_variant);
        }
    }

    map_variants
        .iter()
        .filter(|map_variant| scrape_map_for_guard(&map_variant.to_vec()).2)
        .count() as i32
}

fn scrape_map_for_guard(map: &[Vec<char>]) -> (i32, HashSet<(i32, i32)>, bool) {
    let mut direction: Direction = Direction::Up;
    let mut position: (i32, i32) = (0, 0);
    let mut dist_positions: HashMap<(i32, i32), i32> = HashMap::new();
    let mut path_walked = HashSet::new();
    let mut infinite_loop = false;

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c {
                '<' => {
                    direction = Direction::Left;
                    position = (x as i32, y as i32);
                }
                '>' => {
                    direction = Direction::Right;
                    position = (x as i32, y as i32);
                }
                '^' => {
                    direction = Direction::Up;
                    position = (x as i32, y as i32);
                }
                'v' => {
                    direction = Direction::Down;
                    position = (x as i32, y as i32);
                }
                _ => {
                    direction = Direction::Up;
                }
            }
        }
    }

    dist_positions.insert(position, 1);

    let mut patrol_over = false;
    while !patrol_over {
        let (x, y) = position;

        let mut new_position = direction.move_position(x, y);
        if new_position.0 < 0
            || new_position.1 < 0
            || new_position.0 >= map[0].len() as i32
            || new_position.1 >= map.len() as i32
        {
            patrol_over = true;
            continue;
        }

        if map
            .get(new_position.1 as usize)
            .unwrap()
            .get(new_position.0 as usize)
            .unwrap()
            == &'#'
        {
            let mut turn_again = true;
            while turn_again {
                direction = direction.turn();
                new_position = direction.move_position(x, y);
                if map
                    .get(new_position.1 as usize)
                    .unwrap()
                    .get(new_position.0 as usize)
                    .unwrap()
                    != &'#'
                {
                    turn_again = false;
                }
            }
        }

        *dist_positions.entry(position).or_insert(0) += 1;
        if dist_positions.get(&position).unwrap() > &4 {
            infinite_loop = true;
            patrol_over = true;
            continue;
        }

        position = new_position;
        path_walked.insert(position);
    }

    (
        (dist_positions.len() + 1) as i32,
        path_walked,
        infinite_loop,
    )
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    let mut map = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }

    map
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const UP_DELTA: (i32, i32) = (0, -1);
    const DOWN_DELTA: (i32, i32) = (0, 1);
    const LEFT_DELTA: (i32, i32) = (-1, 0);
    const RIGHT_DELTA: (i32, i32) = (1, 0);

    fn dx(&self) -> i32 {
        match self {
            Direction::Up => Direction::UP_DELTA.0,
            Direction::Down => Direction::DOWN_DELTA.0,
            Direction::Left => Direction::LEFT_DELTA.0,
            Direction::Right => Direction::RIGHT_DELTA.0,
        }
    }

    fn dy(&self) -> i32 {
        match self {
            Direction::Up => Direction::UP_DELTA.1,
            Direction::Down => Direction::DOWN_DELTA.1,
            Direction::Left => Direction::LEFT_DELTA.1,
            Direction::Right => Direction::RIGHT_DELTA.1,
        }
    }

    fn move_position(&self, x: i32, y: i32) -> (i32, i32) {
        (x + self.dx(), y + self.dy())
    }

    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve_part1(input), 41);
    }

    #[test]
    fn test_solve_part2() {
        let input_1 = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve_part2(input_1), 6);
        let input_2 = ".#..#..
#....#.
.......
....^..
.......
.......
.......";
        assert_eq!(solve_part2(input_2), 0);
    }
}
