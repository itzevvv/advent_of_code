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

        if dial == 0 && offset < 0 {
            zeroes -= 1;
        }

        dial += offset;

        zeroes += dial.div_euclid(100).abs();
        dial = dial.rem_euclid(100);

        if dial == 0 && offset < 0 {
            println!("gained a zero");
            zeroes += 1;
        }
    }

    return zeroes
}

pub fn day_one() {
    println!("=== Year 2025 Day 1 ===\n");
    let a = ((345 % 50) + 50) % 50;
    println!("{a}\nboobs\n\n\n");

    let res_d01p01 = part_one(common::read_file("input/y2025/day01.txt"));
    println!("day 01 part 01 result: {res_d01p01}");

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