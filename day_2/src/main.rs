use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let input_data = read_input_data_from_file(String::from("src/input_data")).unwrap();

    println!("input data: {:?}", input_data);
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
