use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_data(file_path: &str) -> io::Result<Vec<Vec<i32>>>{
    let path = Path::new(file_path);

    // Open the file
    let file = File::open(&path)?;

    // Create a reader
    let reader = io::BufReader::new(file);

    // Data storage - Vector of vectors (each list can have different lengths)
    let mut data: Vec<Vec<i32>> = Vec::new();

    // Read each line
    for line_result in reader.lines(){
        let line = line_result?;

        // Fancy code to collect the data
        let numbers: Vec<i32> = line
            .split_whitespace()                            // Splits by whitespace
            .filter_map(|s| s.parse::<i32>().ok())     // Checks for valid entries only i32 numbers
            .collect();

        data.push(numbers);

    }

    Ok(data)
}