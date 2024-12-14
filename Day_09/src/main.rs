use std::fs::File;
use std::io::{self, Read, BufRead};
use std::num::ParseIntError;
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut file = File::open("data.txt")?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Trim any whitespace or newline characters at the ends
    let trimmed_contents = contents.trim();

    // Convert the string into a vector of numbers
    let number_vector: Vec<u32> = trimmed_contents.chars()
        .filter_map(|c| c.to_digit(10)) // Convert each character to a digit
        .collect();

    // Part 1
    let start = Instant::now();
    println!("Numbers: {:?}", number_vector);
    let sparse_representation = create_sparse_representation(number_vector);
    let compacted = compact_space(&sparse_representation);
    let checksum = compute_checksum(compacted);
    let duration = start.elapsed();

    println!("Checksum is {}", checksum);
    println!("Time for part 1 {:?}", duration);

    // Part 2
    let start = Instant::now();
    let compacted = compact_space_full_blocks(&sparse_representation);
    let checksum = compute_checksum(compacted);
    let duration = start.elapsed();

    println!("Checksum part 2 is {}", checksum);
    println!("Time for part 2 {:?}", duration);

    Ok(())
}

fn create_sparse_representation(compact_representation: Vec<u32>) -> Vec<i64>{
    let mut sparse_representation : Vec<i64> = Vec::new();

    let mut index = 0;
    let mut data = true;

    // Iterate through the string
    for number in compact_representation.iter() {

        if data{
            for i in 0..*number {
                sparse_representation.push(index);

            }
            index += 1;

        }
        else{
            for i in 0..*number {
                sparse_representation.push(-1);

            }
        }
        data = !data;

    }

    sparse_representation
}

fn compact_space(sparse_representation: &Vec<i64>) -> Vec<i64>{
    let mut compacted: Vec<i64> = sparse_representation.clone();

    let mut left_index = 0;

    for (index, &value) in sparse_representation.iter().enumerate().rev() {

        if value == -1{
            continue;
        }

        let mut empty = false;
        while(!empty){

            if(sparse_representation[left_index] == -1){
                empty = true;
                // move a value here
                compacted[left_index] = value;
            }
            left_index += 1;

            if index < left_index{
                break;
            }

        }

        if index < left_index{
            break;
        }
    }

    compacted.truncate(left_index);

    // length_new = left_index as i64;

    compacted
}

fn compact_space_full_blocks(sparse_representation: &Vec<i64>) -> Vec<i64>{
    let mut compacted: Vec<i64> = sparse_representation.clone();

    // Loop through in reverse

    // Find contiguous block and how big it is

    // Loop from left to right looking for a space for it

    // Insert if possible, otherwise go to next id minus 1
    // let mut left_index = 0;
    let mut current_id = sparse_representation[sparse_representation.len() - 1];
    let mut current_right_index = 0;

    for (index, &value) in sparse_representation.iter().enumerate().rev() {

        if value != current_id{
            continue;
        }

        current_right_index = index;
        let mut size_of_block = 0;

        for (index2, &value2) in sparse_representation[..=index].iter().enumerate().rev() {
            if value2 != current_id{
                break;
            }
            size_of_block += 1;
        }

        // println!("Size of required block {} for id {}", size_of_block, current_id);

        // Now loop from left to right and try find a block of free space
        let mut empty_counter = 0;
        for i in 0..index{

            if(compacted[i] == -1){
                empty_counter += 1;
            }
            else{
                empty_counter = 0;
            }

            if empty_counter == size_of_block{
                for j in 0..size_of_block{
                    // Move to empty space
                    compacted[j + i - size_of_block + 1] = current_id;

                    // clear old space
                    compacted[index + j - size_of_block + 1] = -1;
                }
                break;
            }
        }

        // if left_index > index{
        //     break;
        // }

        current_id -= 1;
        if current_id < 0{
            break;
        }
    }

    compacted
}

fn compute_checksum(disk: Vec<i64>) -> i64{
    let mut total = 0;

    for (index, &value) in disk.iter().enumerate() {
        if value == -1{
            continue;
        }
        total += value * index as i64;
    }

    total
}
