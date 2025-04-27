use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines_from_file<P>(filename: P) -> io::Result<Vec<Vec<u8>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    let mut data = Vec::new();

    for line in buf_reader.lines() {
        let line = line?;
        data.push(line.into_bytes());
    }

    Ok(data)
}

pub fn read_file_as_strings<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file_data = read_lines_from_file(filename)?;
    Ok(file_data
        .iter()
        .map(|bytes| String::from_utf8(bytes.clone()).expect("invalid UTF-8"))
        .collect())
}
