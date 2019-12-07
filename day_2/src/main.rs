use advent_of_code_2019_day_2::*;

fn main() {
    println!("Part one: {:?}", solve_part_one());

    match solve_part_two() {
        Some(part_two_solution) => println!("Part Two: {}", part_two_solution),
        None => println!("Unable to find a matching noun and verb for Part Two"),
    };
}

fn solve_part_one() -> i32 {
    let mut input_data = read_input_data_from_file(String::from("src/input_data")).unwrap();

    input_data[1] = 12;
    input_data[2] = 2;

    let processed_int_codes = process_intcodes(input_data);

    return processed_int_codes[0];
}

fn solve_part_two() -> Option<i32> {
    let input_data = read_input_data_from_file(String::from("src/input_data")).unwrap();

    let (noun, verb) = find_noun_and_verb_that_match_output(input_data, 19690720)?;

    return Some(noun * 100 + verb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution_is_correct() {
        assert_eq!(solve_part_one(), 4945026);
    }

    #[test]
    fn test_part_two_solution_is_correct() {
        assert_eq!(solve_part_two().unwrap(), 5296)
    }
}
