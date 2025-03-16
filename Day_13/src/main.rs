use std::fs;
use std::error::Error;
use regex::Regex;
use nalgebra::{Matrix2, Vector2};

#[derive(Debug)]
struct Entry {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    prize_x: i64,
    prize_y: i64,
}

fn return_tokens_2(entry: &Entry) -> i64 {
    let OFFSET = 10000000000000;
    // Construct the coefficient matrix and the prize vector, converting to f64.
    let A = Matrix2::new(
        entry.ax as f64, entry.bx as f64,
        entry.ay as f64, entry.by as f64
    );
    let y = Vector2::new((entry.prize_x + OFFSET) as f64, (entry.prize_y + OFFSET) as f64);

    // Invert the matrix.
    let A_inv = A.try_inverse().expect("Matrix is not invertible");

    // Solve for nA and nB (floating point numbers).
    let result = A_inv * y;

    // Round the computed values to the nearest integer.
    let nA_float = result[0];
    let nB_float = result[1];
    let nA = nA_float.round() as i64;
    let nB = nB_float.round() as i64;

    // Check if the rounded solution is acceptable.
    // You might also want to check that when plugging back in the solution,
    // the error is within a small tolerance.
    let epsilon = 1.0;
    let calc_prize_x = nA as f64 * entry.ax as f64 + nB as f64 * entry.bx as f64;
    let calc_prize_y = nA as f64 * entry.ay as f64 + nB as f64 * entry.by as f64;

    if (calc_prize_x - (entry.prize_x + OFFSET) as f64).abs() > epsilon ||
        (calc_prize_y - (entry.prize_y + OFFSET) as f64).abs() > epsilon {
        // The solution doesn't satisfy the equations accurately.
        return 0;
    }

    // Calculate tokens: A costs 3 tokens, B costs 1 token.
    let tokens = 3 * nA + nB;
    tokens
}

fn return_tokens(entry: &Entry) -> i64 {
    // Construct the coefficient matrix and the prize vector, converting to f64.
    let A = Matrix2::new(
        entry.ax as f64, entry.bx as f64,
        entry.ay as f64, entry.by as f64
    );
    let y = Vector2::new(entry.prize_x as f64, entry.prize_y as f64);

    // Invert the matrix.
    let A_inv = A.try_inverse().expect("Matrix is not invertible");

    // Solve for nA and nB (floating point numbers).
    let result = A_inv * y;

    // Round the computed values to the nearest integer.
    let nA_float = result[0];
    let nB_float = result[1];
    let nA = nA_float.round() as i64;
    let nB = nB_float.round() as i64;

    // Check if the rounded solution is acceptable.
    // You might also want to check that when plugging back in the solution,
    // the error is within a small tolerance.
    let epsilon = 1e-6;
    let calc_prize_x = nA as f64 * entry.ax as f64 + nB as f64 * entry.bx as f64;
    let calc_prize_y = nA as f64 * entry.ay as f64 + nB as f64 * entry.by as f64;

    if (calc_prize_x - entry.prize_x as f64).abs() > epsilon ||
        (calc_prize_y - entry.prize_y as f64).abs() > epsilon {
        // The solution doesn't satisfy the equations accurately.
        return 0;
    }

    // Check if either solution is out of bounds.
    if nA < 0 || nA > 100 || nB < 0 || nB > 100 {
        return 0;
    }

    // Calculate tokens: A costs 3 tokens, B costs 1 token.
    let tokens = 3 * nA + nB;
    tokens
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the file into a string.
    let content = fs::read_to_string("data.txt")?;
    // Normalize newlines to '\n' (this handles Windows line endings)
    let content = content.replace("\r\n", "\n");

    // Regex to capture "Button A", "Button B", or "Prize" and the corresponding X and Y values.
    let re = Regex::new(r"(Button A|Button B|Prize):\s*X[=+](\d+),\s*Y[=+](\d+)")?;

    let mut entries = Vec::new();

    // Split the file into blocks by empty lines.
    for block in content.split("\n\n").filter(|b| !b.trim().is_empty()) {
        let mut ax = 0;
        let mut ay = 0;
        let mut bx = 0;
        let mut by = 0;
        let mut prize_x = 0;
        let mut prize_y = 0;

        // Process each block with the regex.
        for cap in re.captures_iter(block) {
            let key = &cap[1];
            let x: i64 = cap[2].parse()?;
            let y: i64 = cap[3].parse()?;

            match key {
                "Button A" => { ax = x; ay = y; },
                "Button B" => { bx = x; by = y; },
                "Prize"    => { prize_x = x; prize_y = y; },
                _ => {},
            }
        }

        entries.push(Entry { ax, ay, bx, by, prize_x, prize_y });
    }

    // Print the number of parsed entries and their details.
    println!("Parsed {} entries", entries.len());
    let mut total_tokens = 0;
    for entry in &entries {
        let tokens = return_tokens(entry);
        total_tokens += tokens;
        // println!("{:?}", entry);
    }

    println!("Total tokens: {}", total_tokens);


    // Part 2
    total_tokens = 0;
    for entry in entries {
        let tokens = return_tokens_2(&entry);
        total_tokens += tokens;
    }

    println!("Total tokens, part 2: {}", total_tokens);

    Ok(())
}
