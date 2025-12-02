use std::collections::HashMap;

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

pub fn part_one(input : Vec<String>) -> i32 {
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
        
        println!("rotating {offset}");
        
        dial = mod_wrap(dial + offset);
        println!("{dial}");

        if dial < 0 || dial > 99 { panic!("uh oh!") }

        if dial == 0 {
            zeroes += 1;

            println!("{zeroes} zeroes")
        }
    }

    return zeroes
}

pub fn part_two(input : Vec<String>) -> i32 {
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
        
        println!("rotating {offset}");
        
        let mut full_rotations = (dial + offset) / 100;
        let remainder = (dial + offset) % 100;

        if remainder < 0 || remainder >= 100 {
            full_rotations += 1;
        }

        println!("ok: {full_rotations}");

        dial = mod_wrap(dial + offset);
        println!("{dial}");

        if dial < 0 || dial > 99 { panic!("uh oh!") }

        zeroes += full_rotations;
    }

    return zeroes
}

pub fn day_one() {
    println!("=== Year 2025 Day 1 ===\n");
    let a = ((345 % 50) + 50) % 50;
    println!("{a}\nboobs\n\n\n");

    //let res_d01p01 = part_one(common::read_file("input/y2025/day01.txt"));
    //println!("day 01 part 01 result: {res_d01p01}");

    let res_d01p02 = part_two(common::read_file("input/y2025/day01.txt"));
    println!("day 01 part 02 result: {res_d01p02}");
}

fn mod_wrap(input : i32) -> i32 {
   let mut result = input % 100;

   if result < 0 {
    // do something?
    result = 100 - result.abs()
   }

   result
}

fn mod_wrap_zeroes(input : i32) -> (i32, i32) {
    let mut result = input % 100;
    let mut zeroes = result / 100;

    if zeroes < 1 && result < 0 {
        zeroes += 1;
    }

    println!("{zeroes} zeroes");
    
    if result < 0 {
        result = 100 - result.abs()
    }

    (result, zeroes)
}
/*

    The dial starts by pointing at 50.
    The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
    The dial is rotated L30 to point at 52.
    The dial is rotated R48 to point at 0.
    The dial is rotated L5 to point at 95.
    The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
    The dial is rotated L55 to point at 0.
    The dial is rotated L1 to point at 99.
    The dial is rotated L99 to point at 0.
    The dial is rotated R14 to point at 14.
    The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.

*/

/*

    - Check if operation goes under 0 or goes over 99
    - Do something to figure out how many times ( div? mod? )
    - Make sure that 0 passes + 0 totals = right amount

*/

/*
    50
    68 - 50 -> goes under, so at least one 0.
        - raw res = 18, under 99. only once
    82
    30 - 82 -> does NOT go under or over.
    52

*/