use std::fs::File;
use std::io::prelude::*;

fn main() {
    // TODO fix file pathing. Relative to the cwd, not this file itself
    let input_data = read_input_data_from_file("src/example_data").unwrap();

    println!(
        "The part one solution is: {:?}",
        part_one_solution(&input_data)
    );
    println!(
        "The part two solution is: {:?}",
        part_two_solution(&input_data)
    )
}

fn part_one_solution(input_data: &Vec<i32>) -> i32 {
    return input_data
        .into_iter()
        .map(|module_mass| fuel_required_for_mass(*module_mass))
        .fold(0, |acc, x| acc + x);
}

fn part_two_solution(input_data: &Vec<i32>) -> i32 {
    return input_data
        .into_iter()
        .map(|module_mass| fuel_required_for_mass_and_fuel(*module_mass))
        .fold(0, |acc, x| acc + x);
}

fn read_input_data_from_file(filepath: &str) -> Result<Vec<i32>, std::io::Error> {
    let mut raw_input_data = File::open(filepath)?;
    let mut contents = String::new();
    raw_input_data.read_to_string(&mut contents)?;

    let split_by_line = contents
        .split("\n")
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    Ok(split_by_line)
}

fn fuel_required_for_mass(mass: i32) -> i32 {
    return mass / 3 - 2;
}

// TODO see if I can do this recursively
fn fuel_required_for_mass_and_fuel(module: i32) -> i32 {
    let mut fuel_requirements: Vec<i32> = vec![];
    let mut fuel_remaining = module;

    while fuel_remaining > 0 {
        fuel_remaining = fuel_required_for_mass(fuel_remaining).max(0);

        fuel_requirements.push(fuel_remaining);
    }

    return fuel_requirements
        .into_iter()
        .fold(0, |acc, fuel| acc + fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_required_for_mass() {
        assert_eq!(fuel_required_for_mass(12), 2);
        assert_eq!(fuel_required_for_mass(14), 2);
        assert_eq!(fuel_required_for_mass(1969), 654);
        assert_eq!(fuel_required_for_mass(100756), 33583);
    }

    #[test]
    fn test_fuel_required_for_mass_and_fuel() {
        assert_eq!(fuel_required_for_mass_and_fuel(14), 2);
        assert_eq!(fuel_required_for_mass_and_fuel(1969), 966);
        assert_eq!(fuel_required_for_mass_and_fuel(100756), 50346);
    }
}
