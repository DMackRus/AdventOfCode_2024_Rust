use std::fs;

fn main() -> Result<(), std::io::Error>{

    println!("Hello, world!");

    // Read all the data into a string
    let file_path = "data.txt";

    // Read the file content into a String
    let data = fs::read_to_string(file_path)?;

    // Split the data into the rules and print commands

    // Split the input into two parts based on the blank line
    let parts: Vec<&str> = data.split("\n\n").collect();
    if parts.len() < 2 {
        eprintln!("Input does not contain both sections!");

        // Return not ok?
    }

    // Setup the print rules
    let print_rules: Vec<(i64, i64)> = parts[0]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line|{
        let mut nums = line.split('|');
        let n1 = nums.next().unwrap().trim().parse().unwrap();
        let n2 = nums.next().unwrap().trim().parse().unwrap();
        (n1, n2)
    })
    .collect();

    println!("{:?}", print_rules);

    // Assemble the print commands

    let print_commands: Vec<Vec<i64>> = parts[1]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line|{
            line.split(',')
                .map(|num| num.trim().parse().unwrap())
                .collect()

        })
        .collect();

    println!("Print commands {:?}", print_commands);

    // ---------------------------------- Part 1 ---------------------------------------------------
    // For each print command, loop through the rule set, find any rules that contain numbers in the
    // print command, discard the rest.

    // Loop through remaining print rules, Loop through the print command number by number and ensure
    // each rule is satisfied

    // Keep all commands that abide by the rules

    // Loop through valid commands and sum up middle number

    let mut valid_commands: Vec<Vec<i64>> = Vec::new();
    for command in print_commands.iter(){

        if check_valid_command(command, &print_rules){
            valid_commands.push(command.clone());
        }

    }

    println!("valid commands: {:?}", valid_commands);

    let middle_nubers: Vec<Option<i64>> = valid_commands.iter()
        .map(|inner_vec|{
            let len = inner_vec.len();
            if len == 0{
                None
            }
            else{
                Some(inner_vec[len / 2])
            }
        }).collect();

    let sum: i64 = middle_nubers
        .iter()
        .filter_map(|&num| num) // Filter out None values and unwrap Some
        .sum();

    println!("Sum of middle numbers is: {:?}", sum);

    Ok(())
}

fn check_valid_command(command: &Vec<i64>, rules: &Vec<(i64, i64)>) -> bool{
    let mut valid_command = true;

    'outer: for rule in rules.iter(){
        let mut found_first = false;
        let mut found_second = false;

        for number in command.iter(){

            if found_first && number == &rule.1{
                break

            }

            if !found_first && number == &rule.1{
                found_second = true;
            }

            if found_second && number == &rule.0{
                // Bad - already found second number of rule before first!
                // This command is invalid
                valid_command = false;
                break 'outer    // This is cool syntax
            }


            if number == &rule.0{
                found_first = true;
            }
        }
    }

    valid_command
}