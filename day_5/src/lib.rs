use std::fs::File;
use std::io;
use std::io::Read;

pub fn read_input_data_from_file(filepath: String) -> Result<Vec<i32>, io::Error> {
    let mut raw_input_data = File::open(filepath)?;
    let mut contents = String::new();
    raw_input_data.read_to_string(&mut contents)?;
    Ok(contents
        .split(",")
        .filter_map(|int| int.parse::<i32>().ok())
        .collect())
}

pub fn process_intcodes(mut intcodes: Vec<i32>) -> Vec<i32> {
    for opcode_position in (0..intcodes.len()).step_by(4) {
        let opcode = intcodes[opcode_position];

        match opcode {
            1 => {
                let (position_to_insert, new_value) = new_value_and_position_to_insert(
                    opcode_position,
                    &intcodes,
                    OpcodeActions::Add,
                );
                intcodes[position_to_insert] = new_value;
            }
            2 => {
                let (position_to_insert, new_value) = new_value_and_position_to_insert(
                    opcode_position,
                    &intcodes,
                    OpcodeActions::Multiply,
                );
                intcodes[position_to_insert] = new_value;
            }
            99 => break,
            _ => panic!("Unexpected Opcode, exiting"),
        }
    }

    intcodes
}

pub enum OpcodeActions {
    Add,
    Multiply,
}

pub fn new_value_and_position_to_insert(
    opcode_position: usize,
    intcodes: &Vec<i32>,
    action: OpcodeActions,
) -> (usize, i32) {
    let first_value_index = intcodes[opcode_position + 1] as usize;
    let second_value_index = intcodes[opcode_position + 2] as usize;
    let position_to_insert_new_value = intcodes[opcode_position + 3] as usize;

    let new_value = match action {
        OpcodeActions::Add => intcodes[first_value_index] + intcodes[second_value_index],
        OpcodeActions::Multiply => intcodes[first_value_index] * intcodes[second_value_index],
    };

    return (position_to_insert_new_value, new_value);
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
