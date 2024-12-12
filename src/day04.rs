pub fn solve_part1(input: &str) -> i32 {
    let mut xmas_count = 0;

    let word_search_matrix = parse_string_to_2d_vector(input);

    let diagonals = collect_diagonals(&word_search_matrix);
    diagonals.iter().for_each(|diagonal| {
        xmas_count += count_word_occurrences("XMAS", &diagonal.iter().collect::<String>());
        xmas_count += count_word_occurrences("SAMX", &diagonal.iter().collect::<String>());
    });

    let mirrored_diagonals = collect_diagonals(&mirror_matrix(&word_search_matrix));
    mirrored_diagonals.iter().for_each(|diagonal| {
        xmas_count += count_word_occurrences("XMAS", &diagonal.iter().collect::<String>());
        xmas_count += count_word_occurrences("SAMX", &diagonal.iter().collect::<String>());
    });

    word_search_matrix.iter().for_each(|row| {
        xmas_count += count_word_occurrences("XMAS", &row.iter().collect::<String>());
        xmas_count += count_word_occurrences("SAMX", &row.iter().collect::<String>());
    });

    for col in 0..word_search_matrix[0].len() {
        let column: Vec<char> = word_search_matrix.iter().map(|row| row[col]).collect();
        xmas_count += count_word_occurrences("XMAS", &column.iter().collect::<String>());
        xmas_count += count_word_occurrences("SAMX", &column.iter().collect::<String>());
    }

    xmas_count
}

pub fn solve_part2(input: &str) -> i32 {
    let m = parse_string_to_2d_vector(input);

    let mut x_mas_count = 0;
    for (i, row) in m.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if i > 0 && j > 0 && i < m.len() - 1 && j < row.len() - 1 && m[i][j] == 'A' {
                let upper_left = m[(i as isize - 1).rem_euclid(m.len() as isize) as usize]
                    [(j as isize - 1).rem_euclid(m[0].len() as isize) as usize];
                let upper_right = m[(i as isize - 1).rem_euclid(m.len() as isize) as usize]
                    [(j as isize + 1).rem_euclid(m[0].len() as isize) as usize];
                let lower_left = m[(i as isize + 1).rem_euclid(m.len() as isize) as usize]
                    [(j as isize - 1).rem_euclid(m[0].len() as isize) as usize];
                let lower_right = m[(i as isize + 1).rem_euclid(m.len() as isize) as usize]
                    [(j as isize + 1).rem_euclid(m[0].len() as isize) as usize];

                if upper_left == 'M'
                    && lower_right == 'S'
                    && lower_left == 'M'
                    && upper_right == 'S'
                {
                    x_mas_count += 1;
                }

                if upper_left == 'S'
                    && lower_right == 'M'
                    && lower_left == 'M'
                    && upper_right == 'S'
                {
                    x_mas_count += 1;
                }

                if upper_left == 'S'
                    && lower_right == 'M'
                    && lower_left == 'S'
                    && upper_right == 'M'
                {
                    x_mas_count += 1;
                }

                if upper_left == 'M'
                    && lower_right == 'S'
                    && lower_left == 'S'
                    && upper_right == 'M'
                {
                    x_mas_count += 1;
                }
            }
        }
    }

    x_mas_count
}

fn count_word_occurrences(word: &str, text: &str) -> i32 {
    let mut count = 0;
    let mut start = 0;
    while let Some(pos) = text[start..].find(word) {
        count += 1;
        start += pos + 1;
    }
    count
}

fn collect_diagonals(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut diagonals = Vec::new();

    if matrix.is_empty() {
        return diagonals;
    }

    let n = matrix.len();
    let m = matrix[0].len();

    // Iterate over diagonals starting from the first row
    for col_start in 0..m {
        let mut diagonal = Vec::new();
        let mut row = 0;
        let mut col = col_start;
        while row < n && col < m {
            diagonal.push(matrix[row][col]);
            row += 1;
            col += 1;
        }
        diagonals.push(diagonal);
    }

    // Iterate over diagonals starting from the first column
    for row_start in 1..n {
        let mut diagonal = Vec::new();
        let mut row = row_start;
        let mut col = 0;
        while row < n && col < m {
            diagonal.push(matrix[row][col]);
            row += 1;
            col += 1;
        }
        diagonals.push(diagonal);
    }

    diagonals
}

fn mirror_matrix(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut mirrored_matrix = Vec::new();
    for row in matrix {
        let mut mirrored_row = row.clone();
        mirrored_row.reverse();
        mirrored_matrix.push(mirrored_row);
    }
    mirrored_matrix
}

fn parse_string_to_2d_vector(input: &str) -> Vec<Vec<char>> {
    input
        .lines() // Create an iterator over the lines of the string
        .map(|line| line.chars().collect()) // Map each line to a vector of chars
        .collect() // Collect the results into a 2D vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input1 = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        let input2 = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(solve_part1(input1), 4);
        assert_eq!(solve_part1(input2), 18);
    }

    #[test]
    fn test_solve_part2() {
        let input1 = "M.S
.A.
M.S";
        assert_eq!(solve_part2(input1), 1);

        let input2 = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(solve_part2(input2), 9);
    }
}
