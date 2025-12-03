use std::{fs::File, io::{BufRead, BufReader, Read}};

pub fn read_file_as_string(file : &str) -> String {
    std::fs::read_to_string(file).expect("file not real")
}

pub fn read_file_as_lines(file : &str) -> Vec<String> {
    let file = File::open(file).expect("file not real");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("line parse fail"))
        .collect()
}