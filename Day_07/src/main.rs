use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Path to the file
    let path = "data.txt";

    // Open the file
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Vectors to store the results
    let mut answers: Vec<i64> = Vec::new();
    let mut inputs: Vec<Vec<i64>> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        if let Some((key, values)) = line.split_once(":") {
            // Parse the key (answer)
            let answer: i64 = key.trim().parse().expect("Failed to parse answer");
            answers.push(answer);

            // Parse the possible inputs
            let possible_inputs: Vec<i64> = values
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse input"))
                .collect();
            inputs.push(possible_inputs);
        }
    }

    // Part 1 - We have a set of input number. We can add + opr * symbols between numbers.
    // Operations always occur from left to right. We need to see if any combination of operators
    // can make the correct answer. Simple combinatorial search.

    let mut sum: i64 = 0;
    for (i, equation) in inputs.iter().enumerate(){
        if valid_combination(answers[i], equation){
            sum += answers[i];
        }
    }
    println!("{}", sum);


    let mut sum: i64 = 0;
    for (i, equation) in inputs.iter().enumerate(){
        if valid_combination_part_2(answers[i], equation){
            sum += answers[i];
        }
    }
    println!("{}", sum);
    // Print the results
    // println!("Answers: {:?}", answers);
    // println!("Inputs: {:?}", inputs);

    Ok(())
}

fn valid_combination_part_2(answer: i64, inputs: &Vec<i64>) -> bool{
    // Now we also have the concatenation operator - so trinary
    let mut valid = false;

    let n: usize = inputs.len() - 1; // Number of trinary digits
    let mut trinary_variable: u64 = 0; // Initialize to all zeros

    // Calculate the maximum value for n trinary digits (3^n - 1)
    let max_value = 3_u64.pow(n as u32) - 1;

    // Iterate through all possible values of the trinary variable
    for _ in 0..=max_value {
        // Process each trinary digit
        let mut result = inputs[0];
        for i in (0..n) {
            let digit = (trinary_variable / 3_u64.pow(i as u32)) % 3;

            if digit == 0{
                // concatenation
                let left = result.to_string();
                let right = inputs[i+1].to_string();
                let result_string = format!("{}{}", left, right);
                result = result_string.parse::<i64>().unwrap();

            }
            else if digit == 1 {
                // addition
                result = result + inputs[i+1];
            }
            else{
                // multiplication
                result = result * inputs[i+1];
            }
        }

        // check if correct
        if(result == answer){
            valid = true;
            break;
        }

        // Increment the trinary variable
        trinary_variable += 1;
    }

    valid
}

fn valid_combination(answer: i64, inputs: &Vec<i64>) -> bool {
    let mut valid = false;

    let num_operators = inputs.len() - 1;

    let n: usize = num_operators; // Number of bits
    let mut bit_map: u64 = 0; // Initialize to all zeros

    // Calculate the maximum value for n bits
    let max_value = (1 << n) - 1;

    // Iterate through all possible values of the binary variable
    for _ in 0..=max_value {
        // Process each bit in the binary variable
        let mut result = inputs[0];
        for i in (0..n) {
            let bit = (bit_map >> i) & 1;
            if bit == 0 {
                result = result + inputs[i+1];
                // println!("Bit at position {} is 0", n - i - 1);
            }
            else {
                result = result * inputs[i+1];
                // println!("Bit at position {} is 1", n - i - 1);
            }
        }

        // check if correct
        if(result == answer){
            valid = true;
            break;
        }

        // Increment the binary variable
        bit_map += 1;
    }

    valid
}

