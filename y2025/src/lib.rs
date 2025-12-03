pub mod day01;
pub mod day02;
pub mod day03;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_one_part_one() {
        let input = day01::d01_test_input();
        let result = day01::part_one(&input);

        assert_eq!(result, 3)
    }

    #[test]
    fn day_one_part_two() {
        let input = vec![
            "L50".to_string(),
            "L100".to_string(),
        ];
        let result = day01::part_two(&input);
        println!("\n");
        
        assert_eq!(result, 2);

        let input = vec![
            "L1000".to_string(),
        ];
        let result = day01::part_two(&input);
        println!("\n");
        
        assert_eq!(result, 10);


        let result = day01::part_two(&day01::d01_test_input());
        println!("\n");
        
        assert_eq!(result, 6)
    }

    #[test]
    fn day_two_part_one() {
        let res_part01 = day02::part_one(day02::D02_INPUT);
        assert_eq!(res_part01, 1227775554);
    }

    #[test]
    fn day_two_part_two() {
        let res_part02 = day02::part_two(day02::D02_INPUT);
        assert_eq!(res_part02, 4174379265);
    }

    #[test]
    fn day_three_part_one() {
        let result = day03::part_one(day03::day03_input());
        assert_eq!(result, 357);
    }
}
