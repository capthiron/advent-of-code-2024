use std::{fs::File, io::{self, Read}, path::Path};

pub fn read_file_to_string<P>(filename: P) -> io::Result<String> 
where P: AsRef<Path>, {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}
