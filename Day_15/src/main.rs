use std::fs;

fn print_map_state(map: &Vec<Vec<u8>> ){
    for line in map.iter(){
        for char in line.iter(){
            if(*char == 0){
                print!("O");
            }
            else if(*char == 1){
                print!("@");
            }
            else if(*char == 2){
                print!("#");
            }
            else{
                print!(".");
            }
        }
        println!();
    }
}

fn step_scene(map: &mut Vec<Vec<u8>>, robot_pos: &mut (i32, i32), direction: u8){

    let old_robot_pos = *robot_pos;

    let mut coord_movement: (i32, i32) = (0, 0);

    // Move robot in correct direction
    if(direction == 0){
        coord_movement.0 = -1;
        coord_movement.1 = 0;
    }
    else if(direction == 1){
        coord_movement.0 = 0;
        coord_movement.1 = -1;
    }
    else if(direction == 2){
        coord_movement.0 = 1;
        coord_movement.1 = 0;
    }
    else{
        coord_movement.0 = 0;
        coord_movement.1 = 1;
    }

    let new_robot_pos = (robot_pos.0 + coord_movement.0, robot_pos.1 + coord_movement.1);
    let mut tracking_tile = new_robot_pos;

    // Check tile where were moving to
    let mut blank_found = false;

    let mut number_boxes = 0;

    while(!blank_found){
        if(map[tracking_tile.1 as usize][tracking_tile.0 as usize] == 3){
            // Move the robot and all objects appropriately
            blank_found = true;
        }
        else if(map[tracking_tile.1 as usize][tracking_tile.0 as usize] == 0){
            // Add object to list
            number_boxes += 1;
        }
        else{
            // Wall do not move anything
            return
        }
        tracking_tile.0 = tracking_tile.0 + coord_movement.0;
        tracking_tile.1 = tracking_tile.1 + coord_movement.1;
    }

    // Update map
    // Move boxes as needed
    map[(old_robot_pos.1 + ((number_boxes+1) * coord_movement.1)) as usize][(old_robot_pos.0 + ((number_boxes+1) * coord_movement.0)) as usize] = 0;

    // Clear old robot position
    map[old_robot_pos.1 as usize][old_robot_pos.0 as usize] = 3;

    // Update new robot position
    map[new_robot_pos.1 as usize][new_robot_pos.0 as usize] = 1;
    robot_pos.0 = new_robot_pos.0;
    robot_pos.1 = new_robot_pos.1;
}

fn compute_gps_coordinate(map: &Vec<Vec<u8>>) -> i32{
    let mut count:i32 = 0;
    for (i, line) in map.iter().enumerate(){
        for (j, element) in line.iter().enumerate(){
            if *element == 0{
                count += (100 * i as i32) + (j as i32);
            }
        }
    }
    count
}

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Failed to read file");

    // Split into logical blocks: one for the grid, one for the arrow string
    let mut blocks = Vec::new();
    let mut current = Vec::new();

    for line in contents.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                blocks.push(current.join("\n"));
                current.clear();
            }
        } else {
            current.push(line);
        }
    }
    if !current.is_empty() {
        blocks.push(current.join("\n"));
    }

    if blocks.len() < 2 {
        panic!("Expected at least two blocks separated by empty lines");
    }

    let map_str = &blocks[0];
    let arrows_str = &blocks[1];

    // Convert map into a 2D grid of integers
    let mut grid: Vec<Vec<u8>> = map_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => 0,
                    '@' => 1,
                    '#' => 2,
                    _ => 3, // Or handle '.' separately if needed
                })
                .collect()
        })
        .collect();

    // Convert arrow block into 1D vector of directions
    let arrows: Vec<u8> = arrows_str
        .chars()
        .filter_map(|c| match c {
            '<' => Some(0),
            '^' => Some(1),
            '>' => Some(2),
            'v' => Some(3),
            _ => None, // skip newlines, spaces, etc.
        })
        .collect();

    let mut robot_pos: (i32, i32) = (0, 0);
    for (i, line) in grid.iter().enumerate(){
        for (j, element) in line.iter().enumerate(){
            if *element == 1{
                robot_pos = (i as i32, j as i32);
            }
        }
    }
    print_map_state(&grid);
    let mut answer = 0;

    for direction in arrows.iter(){
        step_scene(&mut grid, &mut robot_pos, *direction);
        answer = compute_gps_coordinate(&grid);
    }

    print_map_state(&grid);
    println!("Answer is {:?}", answer);
}
