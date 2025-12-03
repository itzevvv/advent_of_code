pub const D02_INPUT : &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

pub fn part_one(input : &str) -> u128 {
    let mut total = 0;

    let splits = input.split(",");

    for split in splits {
        let ranges = split.split("-").collect::<Vec<&str>>();

        let start = if let Ok(num) = ranges[0].parse::<u128>() {
                num
        } else {
            panic!("start parse fail")
        };

        let end = if let Ok(num) = ranges[1].parse::<u128>() {
            num
        } else {
            panic!("end parse fail")
        };

        for val in start..end+1 {
            let middle = &val.to_string().len() / 2;
            
            let first_half = val.to_string().chars().take(middle).collect::<String>();
            let second_half = val.to_string().chars().skip(middle).collect::<String>();

            if first_half == second_half {

                total += val;
            }
        }
    }

    total
}

pub fn day_two() {
    println!("=== Day 02");

    let res_part01 = part_one(&common::read_file_as_string("input/y2025/day02.txt"));
    println!("{res_part01}")
}