use std::ops::Range;

fn main() {
    let input_data = input_data();
    println!("input data range: {:?}", input_data);
}

fn number_meets_requirements(number: usize) -> bool {
    unimplemented!();
}

fn input_data() -> Range<u32> {
    172930..683082
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_meets_requirements_returns_true() {
        assert_eq!(number_meets_requirements(111111), true);
    }

    #[test]
    fn test_number_does_not_meet_requirements_returns_false() {
        assert_eq!(number_meets_requirements(223450), false);
        assert_eq!(number_meets_requirements(123789), false);
    }
}
