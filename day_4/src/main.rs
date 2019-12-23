use std::ops::Range;

fn main() {
    println!("Part One: {}", part_one_solution());
}

fn part_one_solution() -> usize {
    let input_data = input_data();
    input_data.into_iter().fold(0, |acc, number| {
        if number_meets_requirements(number) {
            acc + 1
        } else {
            acc
        }
    })
}

fn number_meets_requirements(number: usize) -> bool {
    // number meets requirements if:
    // digits only increase or stay the same as they go from left to right
    // there is at least one repeat in the number
    number_has_repeating_digits(number) && number_increases_or_stays_the_same(number)
}

fn number_has_repeating_digits(number: usize) -> bool {
    for (position, digit) in number.to_string().chars().enumerate() {
        if position == 0 {
            // then we're at the start of the number, and should
            // automatically continue to the next digit
            continue;
        }
        let previous_digit = number.to_string().chars().nth(position - 1).unwrap();
        if previous_digit == digit {
            return true;
        }
    }

    false
}

fn number_increases_or_stays_the_same(number: usize) -> bool {
    for (position, digit) in number.to_string().chars().enumerate() {
        if position == 0 {
            // then we're at the start of the number, and should
            // automatically continue to the next digit
            continue;
        }

        let previous_digit = number.to_string().chars().nth(position - 1).unwrap();
        if digit < previous_digit {
            return false;
        }
    }

    true
}

fn input_data() -> Range<usize> {
    172930..683082
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(), 1675);
    }

    #[test]
    fn test_number_meets_requirements_returns_true() {
        assert_eq!(number_meets_requirements(111111), true);
    }

    #[test]
    fn test_number_does_not_meet_requirements_returns_false() {
        assert_eq!(number_meets_requirements(223450), false);
        assert_eq!(number_meets_requirements(123789), false);
    }

    #[test]
    fn test_number_has_repeating_digits_returns_true() {
        assert_eq!(number_has_repeating_digits(111111), true);
        assert_eq!(number_has_repeating_digits(112345), true);
    }

    #[test]
    fn test_number_has_no_repeating_digits_returns_false() {
        assert_eq!(number_has_repeating_digits(123456), false);
    }

    #[test]
    fn test_number_increases_or_stays_the_same_returns_true() {
        assert_eq!(number_increases_or_stays_the_same(111111), true);
        assert_eq!(number_increases_or_stays_the_same(123456), true);
    }

    #[test]
    fn test_number_increases_or_stays_the_same_returns_false() {
        assert_eq!(number_increases_or_stays_the_same(987654), false);
        assert_eq!(number_increases_or_stays_the_same(123451), false);
    }
}
