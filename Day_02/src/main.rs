include!("common.rs");

use std::time::Instant;

fn main() {

    // Read the data in with error checking
    let mut data: Vec<Vec<i32>> = Vec::new();
    match read_data("data.txt") {
        Ok(loaded_data) => {
            data = loaded_data;
            println!("Data loaded!");
        },
        Err(e) => eprintln!("Error reading data: {}", e),
    }

    // Part 1 - Loop through data line by line and check each entry versus the rules

    let start = Instant::now();
    let mut safe_report_count : i32 = 0;
    for nuclear_report in data.iter(){
        let (_, safe_report) = is_safe_report(nuclear_report);
        if safe_report{
            safe_report_count += 1;
        }
    }
    let duration = start.elapsed();

    println!("Number of safe nuclear reactor reports: {}", safe_report_count);
    println!("Part 1 took {:?}", duration);

    // Part 2 - Test reports but allow for ONE power dampener (remove one index from list)
    let start = Instant::now();
    let mut safe_report_count : i32 = 0;
    for nuclear_report in data.iter(){

        let (mut fail_index, safe_report) = is_safe_report(nuclear_report);

        if safe_report{
            safe_report_count += 1;
        }
        else{
            while fail_index > -1{
                let mut copy_report = nuclear_report.clone();

                copy_report.remove(fail_index as usize);

                let (_, safe_report) = is_safe_report(&copy_report);

                if(safe_report){
                    safe_report_count += 1;
                    break;
                }

                fail_index -= 1;
            }
        }
    }
    let duration = start.elapsed();

    println!("Number of safe nuclear reactor reports: {}", safe_report_count);
    println!("Part 2 took {:?}", duration);
}

fn is_safe_report(report: &Vec<i32>) -> (i32, bool){
    let mut safe_report: bool = true;
    let mut fail_index: i32 = -1;

    // Loop through the report
    // Check if ascending or descending first
    // Check always ascending or descending
    // Check gap between numbers

    let mut ascending: bool = false;
    if report[1] - report[0] > 0 {
        ascending = true;
    }

    let mut last_val: i32 = report[0];
    for (index, value) in report.iter().enumerate().skip(1){         // Skip syntax is cool
        let index = index as i32;
        if ascending{
            // If ascending, should always be ascending
            if value - last_val < 0{
                safe_report = false;
                fail_index = index;
                break;
            }
        }
        else{
            // If descending, should always be descending
            if value - last_val > 0{
                safe_report = false;
                fail_index = index;
                break;
            }
        }

        // Check difference in numbers Need to be less than 3 and greater than 0
        let diff: i32 = value - last_val;
        if diff.abs() > 3 || diff.abs() == 0{
            safe_report = false;
            fail_index = index;        // Power dampener will do nothing here
            break;
        }

        // Update last val
        last_val = *value;

    }

    (fail_index, safe_report)
}