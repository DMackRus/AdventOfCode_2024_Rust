use std::fs::File;
use std::io::{self, Read, BufRead};

fn main() -> io::Result<()> {
    let file_path = "data.txt";

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


    let width = data[0].len();
    let height = data.len();

    // Loop through vector of vectors and search for 0, when zero found, search all adjacent tiles
    // for next number in chain. Add valid directions to vector until you reach 9 or all trails die.
    let mut valid_hikes = 0;
    let mut score = 0;
    for (i, line) in data.iter().enumerate(){
        for (j, number) in line.iter().enumerate(){
            if *number == 0 {
                // Start a hike
                println!("hike started at index {} {}", i, j);

                let mut all_hikes: Vec<Vec<(i64, i64)>> = Vec::new();
                let mut new_hikes: Vec<Vec<(i64, i64)>> = Vec::new();

                all_hikes.push(Vec::new());
                all_hikes[0].push((i as i64,j as i64));

                for k in 1..10 {
                    // println!("hike - {:?}", all_hikes);

                    // Loop through the current valid hikes
                    let mut valid_next_steps = 0;
                    for (m, hike) in all_hikes.iter().enumerate(){

                        // current index is last index pair in hike
                        let current_index = hike[hike.len() - 1];
                        let valid_indices = get_valid_indices(width as i64, height as i64, current_index.0, current_index.1);

                        for index in valid_indices {
                            if data[index.0 as usize][index.1 as usize] == k {
                                new_hikes.push(hike.clone());
                                new_hikes[valid_next_steps].push(index);
                                valid_next_steps += 1;
                                // println!("New hikes {:?}", new_hikes);
                            }
                        }
                    }

                    // If no valid hikes, exit.
                    if new_hikes.len() < 0 {
                        break;
                    }

                    // Copy
                    all_hikes = new_hikes.clone();
                    new_hikes.clear();
                }

                // Loop through all hikes and remove the duplicates
                let mut end_points: Vec<(i64, i64)> = Vec::new();
                let mut list_length = 0;

                // println!("Number of hikes {}", all_hikes.len());

                for hike in all_hikes.iter(){

                    if list_length == 0{
                        end_points.push((hike[hike.len() - 1].0,
                                         hike[hike.len() - 1].1));
                        list_length += 1;
                    }

                    let mut found = false;
                    for k in 0..list_length{

                        if hike[hike.len() - 1].0 == end_points[k].0 &&
                            hike[hike.len() - 1].1 == end_points[k].1{
                            // do nothing
                            found = true;
                            break;
                        }
                    }

                    if !found{
                        list_length += 1;
                        end_points.push((hike[hike.len() - 1].0,
                                         hike[hike.len() - 1].1));
                    }
                }

                // println!("{:?}", all_hikes);
                //
                // println!("trailhead score for hike that starts at {} {} is {}", i, j, list_length);

                valid_hikes += list_length;
                score += all_hikes.len();
            }
        }
    }

    println!("valid hikes: {}", valid_hikes);
    println!("score: {}", score);

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