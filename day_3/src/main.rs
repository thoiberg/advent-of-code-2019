use std::fs::File;
use std::io::Error as ioError;
use std::io::Read;

fn main() {
    println!("input data: {:?}", read_and_process_input().unwrap()[0]);
}

fn read_and_process_input() -> Result<Vec<String>, ioError> {
    let mut raw_input_data = File::open("src/input_data")?;
    let mut contents = String::new();
    raw_input_data.read_to_string(&mut contents)?;
    Ok(contents
        .split("\n")
        .map(|route| String::from(route))
        .collect())
}
