use std::{fs::File, io::{BufRead, BufReader}};

pub fn read_file(file : &str) -> Vec<String> {
    let file = File::open(file).expect("file not real");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("line parse fail"))
        .collect()
}