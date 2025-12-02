use std::{fs::File, io::{BufRead, BufReader}};

pub fn read_file(file : &str) -> Vec<String> {
    let file = File::open(file).expect("file not real");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("line parse fail"))
        .collect()
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
