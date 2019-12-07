use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let mut input_data = read_input_data_from_file(String::from("src/input_data")).unwrap();

    input_data[1] = 12;
    input_data[2] = 2;

    let processed_int_codes = process_intcodes(input_data);
    println!("output data: {:?}", processed_int_codes);
}

fn process_intcodes(intcodes: Vec<i32>) -> Vec<i32> {
    let mut new_intcodes = intcodes.clone();
    for opcode_position in (0..new_intcodes.len()).step_by(4) {
        let opcode = new_intcodes[opcode_position];

        match opcode {
            1 => {
                let first_value_index = new_intcodes[opcode_position + 1] as usize;
                let second_value_index = new_intcodes[opcode_position + 2] as usize;
                let new_value = new_intcodes[first_value_index] + new_intcodes[second_value_index];
                let position_to_insert_new_value = new_intcodes[opcode_position + 3] as usize;
                new_intcodes[position_to_insert_new_value] = new_value;
            }
            2 => {
                let first_value_index = new_intcodes[opcode_position + 1] as usize;
                let second_value_index = new_intcodes[opcode_position + 2] as usize;
                let new_value = new_intcodes[first_value_index] * new_intcodes[second_value_index];
                let position_to_insert_new_value = new_intcodes[opcode_position + 3] as usize;
                new_intcodes[position_to_insert_new_value] = new_value;
            }
            99 => break,
            _ => panic!("Unexpected Opcode exiting"),
        }
    }

    new_intcodes
}

fn read_input_data_from_file(filepath: String) -> Result<Vec<i32>, io::Error> {
    let mut raw_input_data = File::open(filepath)?;
    let mut contents = String::new();
    raw_input_data.read_to_string(&mut contents)?;

    Ok(contents
        .split(",")
        .filter_map(|int| int.parse::<i32>().ok())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_intcode_adds_correctly() {
        assert_eq!(process_intcodes(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_process_intcode_multiplies_correctly() {
        assert_eq!(process_intcodes(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_process_intcodes_breaks_on_exit_code() {
        assert_eq!(
            process_intcodes(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
    }

    #[test]
    fn test_process_intcodes_can_perform_multiple_actions() {
        assert_eq!(
            process_intcodes(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
