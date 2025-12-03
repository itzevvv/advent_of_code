pub mod day01;

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
}
