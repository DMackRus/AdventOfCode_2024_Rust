use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Path to the file
    let file_path = "data.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a BufReader for efficient reading
    let reader = io::BufReader::new(file);

    // Collect the lines into a vector of strings
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok()) // Filter out errors (optional)
        .collect();

    // Print the vector
    for line in &lines {
        println!("{}", line);
    }

    // --------------------------------- PART 1 -----------------------------------------------
    // Find all the antinodes
    let mut answer = find_all_antinodes(&lines);

    println!("{}", answer);

    // Part 2
    answer = find_all_antinodes_part_2(&lines);

    println!("{}", answer);

    Ok(())
}

fn find_all_antinodes_part_2(input: &Vec<String>) -> i64 {
    let mut num_antinodes: i64 = 0;

    // Store width and height of input
    let width = input[0].chars().count();
    let height = input.len();

    // Create 2D array of booleans, all set to false
    let mut antinodes: Vec<Vec<i64>> = vec![vec![0; width]; height];

    for (i1, line) in input.iter().enumerate() {
        for (j1, char) in line.chars().enumerate() {

            for (i2, line_2) in input.iter().enumerate(){
                for (j2, char_2) in line_2.chars().enumerate() {

                    if char_2 == char && char != '.' && !((i1 == i2) && (j1 == j2)) {
                        let all_antinodes = compute_all_antinodes_indices((j1 as i64, i1 as i64),
                                                                          (j2 as i64, i2 as i64),
                                                                                    width as i64, height as i64);

                        for antinode in all_antinodes.iter(){
                            antinodes[antinode.1 as usize][antinode.0 as usize] = 1;
                        }

                        println!("Pair is {:?} and {:?}", (j1, i1), (j2, i2));
                        pretty_print(&antinodes);
                    }
                }
            }
        }
    }

    // Print the antinode 2D grid
    pretty_print(&antinodes);

    // Sum up the antinodes
    num_antinodes = antinodes.iter()
        .flat_map(|row| row.iter()) // Flatten the rows into a single iterator
        .sum();


    num_antinodes
}

fn compute_all_antinodes_indices(position_1: (i64, i64),
                                 position_2: (i64, i64),
                                 width: i64,
                                 height: i64) -> Vec<(i64, i64)> {

    let mut antinode_coords: Vec<(i64, i64)> = Vec::new();



    let x_0 = position_1.0;
    let y_0 = position_1.1;

    let x_diff = position_2.0 - position_1.0;
    let y_diff = position_2.1 - position_1.1;

    antinode_coords.push((x_0, y_0));

    // Iterate along the line
    let mut in_bound = true;
    let (mut new_x, mut new_y) : (i64, i64) = (x_0, y_0);
    while(in_bound){
        new_x += x_diff;
        new_y += y_diff;

        if check_in_bounds((new_x, new_y), width, height) {
            antinode_coords.push((new_x, new_y));
        }
        else{
            in_bound = false;
        }
    }

    // Iterate backwards along the line
    in_bound = true;
    new_x = x_0;
    new_y = y_0;
    while(in_bound){
        new_x -= x_diff;
        new_y -= y_diff;

        if check_in_bounds((new_x, new_y), width, height) {
            antinode_coords.push((new_x, new_y));
        }
        else{
            in_bound = false;
        }
    }

    antinode_coords
}

fn find_all_antinodes(input: &Vec<String>) -> i64 {
    let mut num_antinodes: i64 = 0;

    // Store width and height of input
    let width = input[0].chars().count();
    let height = input.len();

    // Create 2D array of booleans, all set to false
    let mut antinodes: Vec<Vec<i64>> = vec![vec![0; width]; height];

    for (i1, line) in input.iter().enumerate() {
        for (j1, char) in line.chars().enumerate() {

            for (i2, line_2) in input.iter().enumerate(){
                for (j2, char_2) in line_2.chars().enumerate() {

                    // Check if second char is the same as the first.

                    // If they are draw a straight line between them and compute indices
                    // that are twice as far from one as the other

                    if char_2 == char && char != '.' && !((i1 == i2) && (j1 == j2)) {
                        let antinode_pair = compute_antinodes_indices((j1 as i64, i1 as i64), (j2 as i64, i2 as i64));

                        // Need to check if antinode coordinate is in range

                        if(check_in_bounds(antinode_pair[0], width as i64, height as i64)){
                            antinodes[antinode_pair[0].1 as usize][antinode_pair[0].0 as usize] = 1;
                        }

                        if(check_in_bounds(antinode_pair[1], width as i64, height as i64)){
                            antinodes[antinode_pair[1].1 as usize][antinode_pair[1].0 as usize] = 1;
                        }
                    }
                }

            }
        }
    }

    // Print the antinode 2D grid
    pretty_print(&antinodes);

    // Sum up the antinodes
    num_antinodes = antinodes.iter()
        .flat_map(|row| row.iter()) // Flatten the rows into a single iterator
        .sum();


    num_antinodes
}

fn pretty_print(grid: &Vec<Vec<i64>>) {
    for row in grid {
        let row_string: String = row
            .iter()
            .map(|num| num.to_string()) // Convert each number to a string
            .collect::<Vec<_>>()        // Collect into a vector of strings
            .join(" ");                // Join with spaces
        println!("{}", row_string);   // Print the row as a single string
    }
}

fn check_in_bounds(coord: (i64, i64), width: i64, height: i64) -> bool {
    let mut valid = true;

    if coord.0 < 0 || coord.0 > width - 1 {
        valid = false;
    }

    if coord.1 < 0 || coord.1 > height - 1 {
        valid = false;
    }

    valid
}

fn compute_antinodes_indices(position_1: (i64, i64), position_2: (i64, i64)) -> Vec<(i64, i64)> {
    let mut antinodes_pair: Vec<(i64, i64)> = Vec::new();

    let x_1 = position_1.0;
    let x_2 = position_2.0;
    let y_1 = position_1.1;
    let y_2 = position_2.1;

    let x_diff = x_2 - x_1;
    let y_diff = y_2 - y_1;

    let x_3 = x_1 + (2 * x_diff);
    let y_3 = y_1 + (2 * y_diff);

    let x_4 = x_1 - x_diff;
    let y_4 = y_1 - y_diff;

    antinodes_pair.push((x_3, y_3));
    antinodes_pair.push((x_4, y_4));

    antinodes_pair
}