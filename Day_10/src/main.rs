use std::fs::File;
use std::io::{self, Read, BufRead};

fn main() -> io::Result<()> {
    let file_path = "test.txt";

    // Open the file
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    // Create a BufReader for efficient reading
    // let reader = io::BufReader::new(file);

    file.read_to_string(&mut contents)?;

    // Collect the lines into a vector of strings
    // Parse the string into a Vec<Vec<i64>>
    let data: Vec<Vec<i64>> = contents
        .lines() // Split the content into lines
        .map(|line| {
            line.chars() // Convert the line into individual characters
                .filter_map(|ch| ch.to_string().parse::<i64>().ok()) // Parse each character as i64
                .collect()
        })
        .collect();

    // Loop through vector of vectors and search for 0, when zero found, search all adjacent tiles
    // for next number in chain. Add valid directions to vector until you reach 9 or all trails die.
    for (i, line) in data.iter().enumerate(){
        for (j, number) in line.iter().enumerate(){


            if *number == 0 {
                // Start a hike

                // Get valid indices
                let valid_indices

            }
        }

        println!("{:?}", line);
    }

    Ok(())
}

fn get_valid_indices(width: i64, height: i64, i: i64, j: i64) -> Vec<(i64, i64)>{
    let mut valid_indices: Vec<(i64, i64)> = Vec::new();

    if(i - 1 >= 0){
        valid_indices.push((i - 1, j));
    }

    if(i+1 < width){
        valid_indices.push((i+1, j));
    }

    if (j - 1 >= 0){
        valid_indices.push((i, j-1));
    }

    if (j+1) < height{
        valid_indices.push((i, j+1));
    }

    valid_indices
}