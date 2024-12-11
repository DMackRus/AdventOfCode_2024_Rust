use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;
use std::time::Instant;

fn main() -> io::Result<()> {

    let file_path = "data.txt";

    // Open the file
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    // Create a BufReader for efficient reading
    // let reader = io::BufReader::new(file);

    file.read_to_string(&mut contents)?;

    println!("{:?}", contents);

    let data: Vec<i64> = contents
        .split_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
    .collect();

    println!("Data: {:?}", data);

    // Part 1 and part 2 are the same, except part 2 requires 75 blinks instead of 25.
    // Brute force doesnt overly work, we need a smart way to do this.
    // Blink number y number as there is no overlap between them?

    let start = Instant::now();
    let mut stone_counter = 0;
    for number in data.iter(){
        let mut input: Vec<i64> = Vec::new();
        input.push(*number);
        let mut output: Vec<i64> = Vec::new();
        for i in 0..25{
            output = blink_part1(&input);
            input = output.clone();
        }

        stone_counter += output.len();


    }
    let duration = start.elapsed();
    println!("Answer for part 1: {:?}", stone_counter);
    println!("Time for part 1 {:?}", duration);

    let start = Instant::now();
    let mut cache = HashMap::new();
    let mut total_count = 0;
    for number in data.iter(){
        let stone_count = count(*number, 75, & mut cache);
        total_count += stone_count;
    }
    let duration = start.elapsed();

    println!("Answer for part 2{:?}", total_count);
    println!("Time for part 2 {:?}", duration);

    Ok(())
}

// Need to use dynamic programming and a cache to cache previous results
fn count(stone: i64, steps: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64{
    // Check if the result is already in the cache
    if let Some(&result) = cache.get(&(stone, steps)) {
        return result; // Return the cached result
    }

    // If no steps left return 1
    if steps == 0{
        return 1;
    }

    // Rule 0
    if stone == 0{
        let answer = count(1, steps - 1, cache);
        cache.insert((stone, steps), answer);
        return answer;
    }

    // Rule 1 - Splitting even digits
    let digit_count = stone.abs().to_string().len();
    if(digit_count % 2 == 0){
        let num_str = stone.to_string();
        let len = num_str.len();
        let mid = len / 2;

        let left = &num_str[..mid];
        let right = &num_str[mid..];

        let answer = count(left.parse::<i64>().unwrap(), steps - 1, cache) + count(right.parse::<i64>().unwrap(), steps - 1, cache);
        cache.insert((stone, steps), answer);
        return answer;
    }

    let answer = count(stone * 2024, steps - 1, cache);
    cache.insert((stone, steps), answer);
    answer
}

fn blink_part1(input: &Vec<i64>) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();

    for stone in input.iter(){

        // Rule 1
        if *stone == 0{
            output.push(1);
            continue;
        }

        // Check if stone has even number of digits
        let digit_count = stone.abs().to_string().len();
        if(digit_count % 2 == 0){
            let num_str = stone.to_string();
            let len = num_str.len();
            let mid = len / 2;

            let left = &num_str[..mid];
            let right = &num_str[mid..];

            output.push(left.parse::<i64>().unwrap());
            output.push(right.parse::<i64>().unwrap());

            continue;
        }

        // otherwise we multiply by 2024
        output.push(*stone * 2024);

    }

    output
}
