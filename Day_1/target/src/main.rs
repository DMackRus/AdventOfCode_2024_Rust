use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    // Define the path
    let path = "../data.txt";

    // Open the file
    let file = File::open(&path)?;

    // Create a reader
    let reader = io::BufReader::new(file);

    // Data storage - two columns of data
    let mut data: Vec<(i64, i64)> = Vec::new();

    // Read each line
    for line in reader.lines(){
        let line = line?;

        // Split the line into parts and parse the values
        if let Some((x,y)) = parse_line(&line){
            data.push((x,y));
        }
    }

    // Print the data
    // println!("{:?}", data);


    // We have the data loaded into a Vector now to do the challenge
    // Loop through both lists, find min val of both and compare the difference
    // Keep track of cumulative sum of the differences

    // Since we want to constantly find min values, lets sort these lists

    // Split 2d vector into two 1d vectors
    let (mut x_values, mut y_values): (Vec<i64>, Vec<i64>) = data.iter().cloned().unzip();

    // Sort both lists
    x_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    y_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("{:?}", x_values);

    let mut cumulative_difference: i64 = 0;
    // Loop through length of list and sum differences
    for (i, x) in x_values.iter().enumerate() {
        let diff = x_values[i] - y_values[i];
        cumulative_difference += diff.abs();
    }

    println!("Cumulative diff is: {}", cumulative_difference);

    // Second part of the puzzle - Compute a similarity score between the two lists
    // For each value in the left list, multiply it by the number of times it appears in the
    // right list. Cumulatively add this.

    let mut cumulative_similarity: i64 = 0;
    // Loop through length of list
    for (i, x) in x_values.iter().enumerate() {

        for (i, y) in y_values.iter().enumerate(){
            if x == y {
                cumulative_similarity += x;
            }
        }
    }

    println!("Cumulative similairty is: {}", cumulative_similarity);



    Ok(())
}

fn parse_line(line: &str) -> Option<(i64, i64)>{
    // Split the line by whitespace and collect results into a vector
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 2 {
        if let (Ok(x), Ok(y)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()){
            return Some((x,y));
        }
    }
    None
}

