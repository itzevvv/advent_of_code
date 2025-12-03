pub fn d01_test_input() -> Vec<String> {
    vec![
        "L68".to_string(),
        "L30".to_string(),
        "R48".to_string(),
        "L5".to_string(),
        "R60".to_string(),
        "L55".to_string(),
        "L1".to_string(),
        "L99".to_string(),
        "R14".to_string(),
        "L82".to_string()
    ]
}

pub fn part_one(input : &Vec<String>) -> i32 {
    let mut dial = 50;
    let mut zeroes = 0;
    
    for rotation in input {
        let mut offset = if let Ok(num) = rotation.chars().skip(1).collect::<String>().parse::<i32>() {
            num
        } else {
            panic!("Value could not be parsed to i32")
        };

        if rotation.starts_with('L') {
            offset = -offset;
        }

        dial = (dial + offset).rem_euclid(100);
        
        if dial < 0 || dial > 99 { panic!("uh oh!") }

        if dial == 0 {
            zeroes += 1;
        }
    }

    return zeroes
}

pub fn part_two(input : &Vec<String>) -> i32 {
    let mut dial = 50;
    let mut zeroes = 0;
    
    for rotation in input {
        let mut offset = if let Ok(num) = rotation.chars().skip(1).collect::<String>().parse::<i32>() {
            num
        } else {
            panic!("Value could not be parsed to i32")
        };

        if rotation.starts_with('L') {
            offset = -offset;
        }

        if dial == 0 && offset < 0 {
            zeroes -= 1;
        }

        dial += offset;

        zeroes += dial.div_euclid(100).abs();
        dial = dial.rem_euclid(100);

        if dial == 0 && offset < 0 {
            zeroes += 1;
        }
    }

    return zeroes
}

pub fn day_one() {
    println!("=== Day 01");
    let input = common::read_file_as_lines("input/y2025/day01.txt");
    let res_d01p01 = part_one(&input);
    println!("day 01 part 01 result: {res_d01p01}");

    let res_d01p02 = part_two(&input);
    println!("day 01 part 02 result: {res_d01p02}");
}