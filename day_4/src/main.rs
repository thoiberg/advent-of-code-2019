use std::ops::Range;

fn main() {
    println!("Part One: {}", part_one_solution());
    println!("Part Two: {}", part_two_solution());
}

fn part_one_solution() -> usize {
    let input_data = input_data();
    input_data.into_iter().fold(0, |acc, number| {
        if number_meets_requirements(&number.to_string()) {
            acc + 1
        } else {
            acc
        }
    })
}

fn part_two_solution() -> usize {
    let input_data = input_data();
    input_data.into_iter().fold(0, |acc, number| {
        if number_meets_part_two_requirements(&number.to_string()) {
            acc + 1
        } else {
            acc
        }
    })
}

fn number_meets_requirements(number: &str) -> bool {
    number_has_repeating_digits(number) && number_increases_or_stays_the_same(number)
}

fn number_meets_part_two_requirements(number: &str) -> bool {
    number_has_correct_repeating_digits(number) && number_increases_or_stays_the_same(number)
}

fn number_has_correct_repeating_digits(number: &str) -> bool {
    for (position, digit) in number.bytes().enumerate() {
        if position == 0 {
            continue;
        } else if position == 1 {
            let previous_digit = number.as_bytes().get(position - 1).cloned().unwrap_or(b'a');
            let next_digit = number.as_bytes().get(position + 1).cloned().unwrap_or(b'a');
            let more_next_digit = number.as_bytes().get(position + 2).cloned().unwrap_or(b'a');

            if digit == previous_digit && digit != next_digit && digit != more_next_digit {
                return true;
            }
            if digit == next_digit && digit != more_next_digit && digit != previous_digit {
                return true;
            }
        } else {
            let previous_digit = number.as_bytes().get(position - 1).cloned().unwrap_or(b'a');
            let more_previous_digit = number.as_bytes().get(position - 2).cloned().unwrap_or(b'a');
            let next_digit = number.as_bytes().get(position + 1).cloned().unwrap_or(b'a');
            let more_next_digit = number.as_bytes().get(position + 2).cloned().unwrap_or(b'a');

            if digit == previous_digit && digit == more_previous_digit {
                continue;
            } else if digit == next_digit && digit == more_next_digit {
                continue;
            } else if digit == next_digit && digit == previous_digit {
                continue;
            } else if digit == previous_digit || digit == next_digit {
                return true;
            } else {
                continue;
            }
        }
    }

    false
}

fn number_has_repeating_digits(number: &str) -> bool {
    for (position, digit) in number.bytes().enumerate() {
        if position == 0 {
            continue;
        }
        let previous_digit = number.as_bytes()[position - 1];
        if previous_digit == digit {
            return true;
        }
    }

    false
}

fn number_increases_or_stays_the_same(number: &str) -> bool {
    for (position, digit) in number.bytes().enumerate() {
        if position == 0 {
            continue;
        }

        let previous_digit = number.as_bytes()[position - 1];
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
    fn test_part_two_solution() {
        assert_eq!(part_two_solution(), 1142);
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

    #[test]
    fn test_number_meets_part_two_requirements_returns_true() {
        assert_eq!(number_meets_part_two_requirements(112233), true);
        assert_eq!(number_meets_part_two_requirements(111122), true);
        assert_eq!(number_meets_part_two_requirements(111223), true);
        assert_eq!(number_meets_part_two_requirements(112345), true);
        assert_eq!(number_meets_part_two_requirements(123445), true);
        assert_eq!(number_meets_part_two_requirements(122333), true);
        assert_eq!(number_meets_part_two_requirements(122233), true);
        assert_eq!(number_meets_part_two_requirements(123345), true);
    }

    #[test]
    fn test_number_meets_part_two_requirements_returns_false() {
        assert_eq!(number_meets_part_two_requirements(123444), false);
        assert_eq!(number_meets_part_two_requirements(111111), false);
        assert_eq!(number_meets_part_two_requirements(111112), false);
        assert_eq!(number_meets_part_two_requirements(211333), false);
        assert_eq!(number_meets_part_two_requirements(111333), false);
        assert_eq!(number_meets_part_two_requirements(123333), false);
        assert_eq!(number_meets_part_two_requirements(123456), false);
        assert_eq!(number_meets_part_two_requirements(123334), false);
        assert_eq!(number_meets_part_two_requirements(111222), false);
        assert_eq!(number_meets_part_two_requirements(111123), false);
        assert_eq!(number_meets_part_two_requirements(122234), false);
        assert_eq!(number_meets_part_two_requirements(122223), false);
        assert_eq!(number_meets_part_two_requirements(123334), false);
        assert_eq!(number_meets_part_two_requirements(123444), false);
    }
}
