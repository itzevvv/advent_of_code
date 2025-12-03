pub const D03_INPUT : &[&str] = &[
    "987654321111111",
    "811111111111119",
    "234234234234278",
    "818181911112111",
];

pub fn day03_input() -> Vec<String> {
    vec![
        "987654321111111".to_string(),
        "811111111111119".to_string(),
        "234234234234278".to_string(),
        "818181911112111".to_string(),
    ]
}

pub fn part_one(input : Vec<String>) -> i32 {
    let mut total = 0;

    for bank in input {
        let mut largest_total = 0;
        let mut first_largest = 0;
            
        for (index, num_char) in bank.chars().enumerate() {
            if let Ok(num) = num_char.to_string().parse::<i32>() {
                //println!("{num}");
                if num > first_largest {
                    first_largest = num;

                    if index == bank.len() {
                        continue;
                    }

                    for next_num_char in bank.chars().skip(index+1) {
                        let mut concat = num_char.to_string();
                        concat.push_str(&next_num_char.to_string());
                        //println!("{concat}");
                        
                        if let Ok(next_num) = concat.parse::<i32>() {
                            if next_num > largest_total {
                                largest_total = next_num;
                            }
                        }
                    }
                }
            } else {
                panic!("first not a num")
            }
        }
        
        //println!("largest: {largest_total}");
        total += largest_total
    }

    total
}

pub fn day_three() {
    let input = common::read_file_as_lines("input/y2025/day03.txt");

    let part01 = part_one(input);
    println!("total: {part01}");
}