use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn load_file_to_2d_array(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let array: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    Ok(array)
}

fn compute_cost_of_plot_2(garden: &Vec<Vec<char>>, coords: &Vec<(i32, i32)>, plant: char) -> i32{

    let area = coords.len() as i32;

    let corner_coords = [((-1, 0), (0, -1), (-1, -1)),
                                    ((0, -1), (1, 0), (1, -1)),
                                    ((1, 0), (0, 1), (1, 1)),
                                    ((0, 1), (-1, 0), (-1, 1))];

    let width = garden[0].len() as i32;
    let height = garden.len() as i32;
    let mut corner_count = 0;

    // Loop through all coords
    for coord in coords {
        // Check if each coord is a corner, by checking for convex and concave corners
        for corner in &corner_coords {
            let mut new_coord_1 = coord.clone();
            let mut new_coord_2 = coord.clone();
            let mut new_coord_3 = coord.clone();
            new_coord_1.0 += corner.0.0;
            new_coord_1.1 += corner.0.1;
            new_coord_2.0 += corner.1.0;
            new_coord_2.1 += corner.1.1;
            new_coord_3.0 += corner.2.0;
            new_coord_3.1 += corner.2.1;

            let mut coord1_same = false;
            let mut coord2_same = false;
            let mut coord3_same = false;

            // Original coordinate is only a corner if both new coords are not valid

            // ------------ Coord 1 check ----------------------
            if new_coord_1.0 < width && new_coord_1.0 >= 0 {
                if new_coord_1.1 < height && new_coord_1.1 >= 0 {
                    if garden[new_coord_1.0 as usize][new_coord_1.1 as usize] == plant {
                        coord1_same = true;
                    }
                }
            }

            // --------- Coord 2 check ------------------------
            if new_coord_2.0 < width && new_coord_2.0 >= 0 {
                if new_coord_2.1 < height && new_coord_2.1 >= 0 {
                    if garden[new_coord_2.0 as usize][new_coord_2.1 as usize] == plant {
                        coord2_same = true;;
                    }
                }
            }

            // --------- Coord 3 check ------------------------
            if new_coord_3.0 < width && new_coord_3.0 >= 0 {
                if new_coord_3.1 < height && new_coord_3.1 >= 0 {
                    if garden[new_coord_3.0 as usize][new_coord_3.1 as usize] == plant {
                        coord3_same = true;
                    }
                }
            }

            // It is a corner if 1 and 2 false OR
            // if corner 1 and 2 are true and 3 is false
            if !coord1_same && !coord2_same{
                corner_count += 1;
            }
            else if coord1_same && coord2_same && !coord3_same{
                corner_count += 1;
            }
        }
    }

    area * corner_count
}

fn compute_cost_of_plot(garden: &Vec<Vec<char>>, coords: &Vec<(i32, i32)>, plant: char) -> i32{
    let mut cost:i32 = 0;

    let area = coords.len() as i32;
    let mut perimeter = 0;
    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let width = garden[0].len() as i32;
    let height = garden.len() as i32;

    for coord in coords {
        // check each coord cardinal directions. If one direction is not the same character
        // or out of bounds, then it is a perimeter
        for direction in &directions {
            let mut new_coord = coord.clone();
            new_coord.0 += direction.0;
            new_coord.1 += direction.1;

            // check if new coord is within boundaries
            if new_coord.0 >= width || new_coord.0 < 0 {
                perimeter += 1;
                continue;
            }
            if new_coord.1 >= height || new_coord.1 < 0 {
                perimeter += 1;
                continue;
            }

            // Check if character is the same as the current patch
            if garden[new_coord.0 as usize][new_coord.1 as usize] != plant{
                perimeter += 1;
            }
        }
    }

    cost = perimeter * area;
    cost
}

fn bfs_find_patch(garden: &Vec<Vec<char>>, plant: char, starting_coord: (i32, i32)) -> Vec<(i32, i32)> {

    let mut patch_coords:Vec<(i32, i32)> = Vec::new();
    let mut boundary_coords:Vec<(i32, i32)> = Vec::new();
    let width = garden[0].len() as i32;
    let height = garden.len() as i32;

    patch_coords.push(starting_coord);
    boundary_coords.push(starting_coord);

    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    loop {
        // Get the neighbours of the boundary
        let mut neighbours: Vec<(i32, i32)> = Vec::new();

        for coord in &boundary_coords {
            'direction: for direction in &directions {
                let mut new_coord = coord.clone();
                new_coord.0 += direction.0;
                new_coord.1 += direction.1;

                // check if new coord is within boundaries
                if new_coord.0 >= width || new_coord.0 < 0 {
                    continue;
                }
                if new_coord.1 >= height || new_coord.1 < 0 {
                    continue;
                }

                // Check if character is the same as the current patch
                if garden[new_coord.0 as usize][new_coord.1 as usize] != plant{
                   continue;
                }


                // Check if neighbour already in the patch
                for patch_coord in &patch_coords {
                    if new_coord.0 == patch_coord.0 && new_coord.1 == patch_coord.1 {
                        continue 'direction;
                    }
                }

                neighbours.push(new_coord);
                patch_coords.push(new_coord);

            }
        }

        boundary_coords.clear();
        for neighbour in &neighbours{
            boundary_coords.push(neighbour.clone());
        }

        if boundary_coords.len() == 0 {
            break;
        }
    }

    patch_coords
}

fn main() {
    let filename = "data.txt";
    let data = match load_file_to_2d_array(filename) {
        Ok(array) => array,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let width = data[0].len();
    let height = data.len();

    let mut checked: Vec<Vec<bool>> = vec![vec![false; width]; height];

    let mut cost = 0;

    let start = Instant::now();
    // Part 1
    // Loop through each character and create the patches
    for (x, line) in data.iter().enumerate(){
        for (y, character) in line.iter().enumerate() {

            // Check if this coordinate has already been assigned to a patch
            if checked[y][x] == true {
                continue;
            }

            let new_patch = bfs_find_patch(&data, *character, (x as i32, y as i32));

            let plot_cost = compute_cost_of_plot(&data, &new_patch, *character);
            cost += plot_cost;


            for coordinate in &new_patch {
                checked[coordinate.1 as usize][coordinate.0 as usize] = true;
            }
        }
    }
    let duration = start.elapsed();

    println!("Total cost of garden - part 1: {}", cost);
    println!("Total time for part 1: {:?}", duration);

    cost = 0;
    let mut checked_2: Vec<Vec<bool>> = vec![vec![false; width]; height];

    let start = Instant::now();
    // Part 2
    // No longer use the perimeter, use the number of sides
    for (x, line) in data.iter().enumerate(){
        for (y, character) in line.iter().enumerate() {

            // Check if this coordinate has already been assigned to a patch
            if checked_2[y][x] == true {
                continue;
            }

            let new_patch = bfs_find_patch(&data, *character, (x as i32, y as i32));

            let plot_cost = compute_cost_of_plot_2(&data, &new_patch, *character);

            cost += plot_cost;


            for coordinate in &new_patch {
                checked_2[coordinate.1 as usize][coordinate.0 as usize] = true;
            }
        }
    }
    let duration = start.elapsed();

    println!("Total cost of garden - part 2: {}", cost);
    println!("Total time for part 2: {:?}", duration);
}
