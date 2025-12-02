pub mod day01;

#[cfg(test)]
mod tests {
    use crate::{day01::*};

    use super::*;

    #[test]
    fn day_one_part_one() {
        let input = d01_test_input();
        let result = day01::part_one(input);

        assert_eq!(result, 3)
    }
}
