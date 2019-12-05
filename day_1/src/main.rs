use std::fs::File;
use std::io::prelude::*;

fn main() {
    // TODO fix file pathing. Relative to the cwd, not this file itself
    let input_data = read_input_data_from_file("src/example_data").unwrap();
    let total_fuel_needed: i32 = input_data
        .into_iter()
        .map(|module_mass| fuel_for_mass(module_mass))
        .fold(0, |acc, x| acc + x);

    println!("{:?}", total_fuel_needed);
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

fn fuel_for_mass(module: i32) -> i32 {
    return module / 3 - 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(fuel_for_mass(12), 2);
        assert_eq!(fuel_for_mass(14), 2);
        assert_eq!(fuel_for_mass(1969), 654);
        assert_eq!(fuel_for_mass(100756), 33583);
    }
}
