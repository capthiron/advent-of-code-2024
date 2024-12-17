use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn read_file_to_string<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_string_to_2d_vector(input: &str) -> Vec<Vec<char>> {
    input
        .lines() // Create an iterator over the lines of the string
        .map(|line| line.chars().collect()) // Map each line to a vector of chars
        .collect() // Collect the results into a 2D vector
}
