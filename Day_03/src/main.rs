use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn main() {

    // Read the data in with error checking
    let mut data: Vec<String> = Vec::new();
    match read_data("data.txt") {
        Ok(loaded_data) => {
            data = loaded_data;
            println!("Data loaded!");
        },
        Err(e) => eprintln!("Error reading data: {}", e),
    }

    // ------------ Part 1 ----------------
    // Find all valid multiples in all the lines
    // Multiply them all and sum them up

    let mut result: i64 = 0;
    for line in data.iter(){
        let valid_multiples = find_all_mul_instances(line);

        for multiple in valid_multiples.iter(){
            let product = multiple.0 * multiple.1;
            result += product;
        }
    }

    println!("result is {}", result);

    // println!("valid multiples {:?}",valid_multiples)
}

fn find_all_mul_instances(input_string : &str) -> Vec<(i64, i64)>{

    let pattern = r"mul\(\s*(-?\d+\.?\d*)\s*,\s*(-?\d+\.?\d*)\s*\)";
    let re = Regex::new(pattern).unwrap();

    // Create a vector to store the results
    let mut results = Vec::new();

    // Iterate over all matches
    for cap in re.captures_iter(input_string) {
        // Parse the numbers from the regex captures
        if let (Ok(first), Ok(second)) = (cap[1].parse::<i64>(), cap[2].parse::<i64>()) {
            results.push((first, second));
        }
    }

    results
}

fn read_data(file_path: &str) -> io::Result<Vec<String>>{
    let path = Path::new(file_path);

    // Open the file
    let file = File::open(&path)?;

    // Create a reader
    // let reader = io::BufReader::new(file);


    let lines: Vec<String> = io::BufReader::new(file)
        .lines() // Iterator of Result<String>
        .filter_map(|line| line.ok()) // Filter out lines that might error
        .collect(); // Collect the valid lines into a Vec<String>

    // Print the lines to verify
    for line in &lines {
        println!("{}", line);
    }

    Ok(lines)
}
