use std::collections::HashSet;

use crate::utils;

pub fn solve_part1(input: &str) -> i32 {
    solve(input, false)
}

pub fn solve_part2(input: &str) -> i32 {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> i32 {
    let map = utils::parse_string_to_2d_vector(input);
    let mut locations: HashSet<(usize, usize)> = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if is_antenna(*c) {
                locations = locations
                    .union(&get_antidode_locations(&map, *c, (x, y), part2))
                    .cloned()
                    .collect();
            }
        }
    }

    locations.len().try_into().unwrap()
}

fn get_antidode_locations(
    map: &[Vec<char>],
    c: char,
    start_pos: (usize, usize),
    many_antidodes: bool,
) -> HashSet<(usize, usize)> {
    let (x, y) = start_pos;
    let ry = get_radius_per_dimension(map.len(), y);
    let rx = get_radius_per_dimension(map.first().unwrap().len(), x);

    let mut locations: HashSet<(usize, usize)> = HashSet::new();
    for y_prime in y - ry..=y + ry {
        for x_prime in x - rx..=x + rx {
            if *map.get(y_prime).unwrap().get(x_prime).unwrap() == c && y_prime != y && x_prime != x
            {
                let antidode = mirror_point_through((x_prime, y_prime), start_pos);
                locations.insert(antidode);
                if many_antidodes {
                    locations.insert(start_pos);
                    locations.insert((x_prime, y_prime));

                    let (vec_x, vec_y) =
                        (antidode.0 as i32 - x as i32, antidode.1 as i32 - y as i32);
                    let mut i = 1;
                    loop {
                        let y_prime_prime = (antidode.1 as i32 + i * vec_y) as usize;
                        let x_prime_prime = (antidode.0 as i32 + i * vec_x) as usize;

                        if map
                            .get(y_prime_prime)
                            .and_then(|row| row.get(x_prime_prime))
                            .is_none()
                        {
                            break;
                        }

                        locations.insert((x_prime_prime, y_prime_prime));
                        i += 1;
                    }
                }
            }
        }
    }

    locations
}

fn mirror_point_through(point: (usize, usize), through: (usize, usize)) -> (usize, usize) {
    let (x1, y1) = through;
    let (x2, y2) = point;

    let mirrored_x = 2 * x1 - x2;
    let mirrored_y = 2 * y1 - y2;

    (mirrored_x, mirrored_y)
}

fn get_radius_per_dimension(length: usize, coordinate: usize) -> usize {
    if coordinate >= (length / 2) {
        return length - coordinate - 1;
    }

    coordinate
}

fn is_antenna(c: char) -> bool {
    c != '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(solve_part1(input), 14);
    }

    #[test]
    fn test_solve_part2() {
        let input_1 = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(solve_part2(input_1), 34);

        let input_2 = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        assert_eq!(solve_part2(input_2), 9);
    }
}
