use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

const WIDTH:usize = 101;
const HEIGHT:usize = 103;

// const WIDTH:usize = 11;
// const HEIGHT:usize = 7;


fn load_robots_from_file(filename: &str) -> io::Result<Vec<Robot>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut robots = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        // Expect each line to be in the format "p=x,y v=vx,vy"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Skipping invalid line: {}", line);
            continue;
        }

        // Parse position (remove "p=" and split by comma)
        let pos_str = parts[0].trim_start_matches("p=");
        let pos_coords: Vec<&str> = pos_str.split(',').collect();
        if pos_coords.len() != 2 {
            eprintln!("Skipping invalid position in line: {}", line);
            continue;
        }
        let x = pos_coords[0].parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Failed to parse x in line: {}", line);
            0
        });
        let y = pos_coords[1].parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Failed to parse y in line: {}", line);
            0
        });

        // Parse velocity (remove "v=" and split by comma)
        let vel_str = parts[1].trim_start_matches("v=");
        let vel_coords: Vec<&str> = vel_str.split(',').collect();
        if vel_coords.len() != 2 {
            eprintln!("Skipping invalid velocity in line: {}", line);
            continue;
        }
        let vx = vel_coords[0].parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Failed to parse vx in line: {}", line);
            0
        });
        let vy = vel_coords[1].parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Failed to parse vy in line: {}", line);
            0
        });

        robots.push(Robot { pos: (x, y), vel: (vx, vy) });
    }
    Ok(robots)
}

fn display_scene_state(robots: &Vec<Robot>) {
    // Make a 2D Grid of size width and height
    let mut scene: [[i32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    // Iterate through all robots and increment their positions by 1
    for robot in robots{
        // println!("{:?}", robot);
        scene[robot.pos.1 as usize][robot.pos.0 as usize] += 1;
    }

    for i in 0..HEIGHT{
        for j in 0..WIDTH{
            print!("{}", scene[i as usize][j as usize]);
        }
        println!();
        // println!("{:?}", scene[i]);
    }
}

fn step_scene(robots: &mut Vec<Robot>) {
    // Loop through all robots, increment their position based on their velocity

    for robot in robots{
        robot.pos.0 += robot.vel.0;
        robot.pos.1 += robot.vel.1;

        robot.pos.0 = robot.pos.0 % WIDTH as i32;
        robot.pos.1 = robot.pos.1 % HEIGHT as i32;

        if robot.pos.0 < 0{
            robot.pos.0 = WIDTH as i32 + robot.pos.0;
        }
        if robot.pos.1 < 0{
            robot.pos.1 = HEIGHT as i32 + robot.pos.1;
        }

        // if robot.pos.0 >= WIDTH as i32{
        //     robot.pos.0 = robot.pos.0 - WIDTH as i32;
        // }
        //
        // if robot.pos.1 >= HEIGHT as i32{
        //     robot.pos.1 = robot.pos.1 - HEIGHT as i32;
        // }
    }
}

fn compute_answer_quadrants(robots: &Vec<Robot>) -> i32 {
    let mut answer: i32 = 0;

    // Calculate middle indices:
    let mid_row = HEIGHT / 2;       // y-coordinate for the middle row
    let mid_column = WIDTH / 2;     // x-coordinate for the middle column

    println!("mid_row: {}, mid_column: {}", mid_row, mid_column);

    let mut robots_per_quadrant: [i32; 4] = [0; 4];

    for robot in robots {
        // Only count robots not on the middle row or column.
        // Compare x to mid_column and y to mid_row.
        if robot.pos.0 < mid_column as i32 && robot.pos.1 < mid_row as i32 {
            // Top-left quadrant
            robots_per_quadrant[0] += 1;
        }
        if robot.pos.0 < mid_column as i32 && robot.pos.1 > mid_row as i32 {
            // Bottom-left quadrant
            robots_per_quadrant[1] += 1;
        }
        if robot.pos.0 > mid_column as i32 && robot.pos.1 > mid_row as i32 {
            // Bottom-right quadrant
            robots_per_quadrant[2] += 1;
        }
        if robot.pos.0 > mid_column as i32 && robot.pos.1 < mid_row as i32 {
            // Top-right quadrant
            robots_per_quadrant[3] += 1;
        }
    }

    answer = robots_per_quadrant[0]
        * robots_per_quadrant[1]
        * robots_per_quadrant[2]
        * robots_per_quadrant[3];

    answer
}

fn heuristic_guess_christmas_tree(robots: &Vec<Robot>) -> bool{
    // Make a 2D Grid of size width and height
    let mut scene: [[i32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    // Iterate through all robots and increment their positions by 1
    for robot in robots{
        // println!("{:?}", robot);
        scene[robot.pos.1 as usize][robot.pos.0 as usize] += 1;
    }

    // If > x% of robots near middle of image
    // let left_barrier = 30;
    // let right_barrier = WIDTH - 30;
    // let top_barrier = 30;
    // let bottom_barrier = HEIGHT - 30;

    let mut stack = false;
    // let mut count = 0;
    for i in 0.. HEIGHT{
        for j in 0.. WIDTH{
            if scene[i][j] > 1{
                stack = true;
                // count += 1;
            }
        }
    }


    stack
    //
    // let percentage:f32 = count as f32 / robots.len() as f32;
    //
    // let mut answer = false;
    //
    // println!("percentage: {}", percentage);
    // if percentage > 0.2{
    //     answer = true;
    // }
    //
    // answer

    // If there are no twos in the image
}

fn main() -> io::Result<()> {
    // Replace "data.txt" with the path to your file
    let mut robots = load_robots_from_file("data.txt")?;

    println!("number of robots: {}", robots.len());

    // display_scene_state(&robots);
    // for i in 0..171 {
    //     step_scene(&mut robots);
    // }

    // Update system state 100 times
    for i in 0..10000{

        step_scene(&mut robots);


        if !heuristic_guess_christmas_tree(&robots){
            println!("------------------------------------ Iteration {} --------------------------------------------------", i);
            display_scene_state(&robots);
        }
    }
    // display_scene_state(&robots);
    let answer = compute_answer_quadrants(&robots);

    println!("Answer quadrants: {}", answer);

    Ok(())
}