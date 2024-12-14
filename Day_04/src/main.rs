use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

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

    let start = Instant::now();

    // Part 1, find all instances of "XMAS" in the string, its two-dimensional
    // So the instance can be horizontal, vertical or diagonal, including backwards
    let XMAS_counter = part_1(&data);

    let duration = start.elapsed();
    println!("Number of XMAS found {}", XMAS_counter);
    println!("Part 1 took {:?}", duration);

    // Part 2 - It was actually an X-Mas puzzle
    // M * S
    // * A *    // These are what were looking for now!
    // M * S

    let start = Instant::now();

    let X_MAS_counter = part_2(&data);

    let duration = start.elapsed();
    println!("Number of XMAS found {}", X_MAS_counter);
    println!("Part 2 took {:?}", duration);
}

fn part_2(data: &Vec<String>) -> i32{
    // Get the width and height of the data input
    let width = data[0].len() as usize;
    let height = data.len() as usize;

    // N, NE, E, SE, S, SW, W, NW
    let direction_offsets: [(i32, i32); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];

    let XMAS_string = ['X', 'M', 'A', 'S'];
    let mut X_MAS_counter = 0;

    for (i, line) in data.iter().enumerate(){

        // Loop through every letter in this line
        for (j, letter) in line.chars().enumerate(){

            // Check if M or S
            let mut left_letter;
            let mut right_letter;

            if letter == 'M' || letter == 'S'{
                left_letter = letter;
            }
            else{
                continue;
            }

            // Check +2 +2 s in range of 2d array
            if (j + 2) > width-1 || (i + 2) > height-1{
                continue
            }


            // Check for letter M or S at other side
            if data[i].chars().nth(j+2) == Option::from('M') {
                right_letter = 'M';
            }
            else if data[i].chars().nth(j+2) == Option::from('S') {
                right_letter = 'S';
            }
            else{
                continue;
            }



            //Check if row below + 1 is A
            if data[i+1].chars().nth(j+1) != Option::from('A'){
                continue;
            }


            // Check if two rows below is opposite of the cross
            if right_letter == 'S'{
                // bottom left should be M
                if data[i+2].chars().nth(j) != Option::from('M'){
                    continue;
                }
            }
            else if right_letter == 'M' {
                if data[i+2].chars().nth(j) != Option::from('S'){
                    continue;
                }
            }

            if left_letter == 'S'{
                // bottom left should be M
                if data[i+2].chars().nth(j+2) != Option::from('M'){
                    continue;
                }
            }
            else if left_letter == 'M' {
                if data[i+2].chars().nth(j+2) != Option::from('S'){
                    continue;
                }
            }

            // Sum the number of valid directions to count how many XMAS
            X_MAS_counter += 1;
        }
    }

    X_MAS_counter as i32
}

fn part_1(data: &Vec<String>) -> i32{
    // Get the width and height of the data input
    let width = data[0].len() as usize;
    let height = data.len() as usize;

    // N, NE, E, SE, S, SW, W, NW
    let direction_offsets: [(i32, i32); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];

    let XMAS_string = ['X', 'M', 'A', 'S'];
    let mut XMAS_counter = 0;

    for (i, line) in data.iter().enumerate(){

        // Loop through every letter in this line
        for (j, letter) in line.chars().enumerate(){

            let mut possible = true;
            let mut directions: Vec<usize> = (0..=7).collect();   // Directions N, NE, E, SE, S, SW, W, NW
            let mut XMAS_index = 0;

            // On iteration1, check current letter = X,
            if letter != 'X'{
                continue;
            }

            // Iteration 2 - 4we check if outer ring  = M

            while possible{
                // Reverse iteration so safely can remove as we go
                for k in (0..directions.len()).rev(){
                    let mut new_row: usize;
                    let mut new_column: usize;

                    // Oh god what is this casting mess....
                    new_row = (i as i32 + (direction_offsets[directions[k]].0 * XMAS_index)) as usize;
                    new_column = (j as i32 + (direction_offsets[directions[k]].1 * XMAS_index)) as usize;

                    // Check if row is valid
                    if new_row < 0 || new_row > height-1{
                        // not valid
                        directions.remove(k);
                        continue;
                    }

                    // Check if column is valid
                    if new_column < 0 || new_column > width - 1{
                        // not valid
                        directions.remove(k);
                        continue;
                    }

                    // Check next XMAS letter
                    let character = data[new_row].chars().nth(new_column);
                    if let Some(c) = character {
                        if c != XMAS_string[XMAS_index as usize] { // Convert `i32` to `usize`
                            directions.remove(k);
                        }
                    }

                }

                XMAS_index += 1;

                if (XMAS_index > 3){
                    break;
                }
            }

            // Sum the number of valid directions to count how many XMAS
            XMAS_counter += directions.len();

        }
    }

    XMAS_counter as i32
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
