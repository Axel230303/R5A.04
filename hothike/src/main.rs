use std::cmp::min;
use std::io::{self, BufRead};

fn main() {
    // Reading input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of days
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the temperature forecast
    let temps: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    // Variables to track the best day and the minimum maximum temperature
    let mut best_day = 0;
    let mut min_max_temp = i32::MAX;

    // Loop over each valid starting day
    for d in 0..(n - 2) {
        // The temperature on the two hiking days (d and d+2)
        let max_temp = temps[d].max(temps[d + 2]);

        // Update the best day if we find a smaller maximum temperature
        if max_temp < min_max_temp {
            min_max_temp = max_temp;
            best_day = d + 1;  // Convert to 1-based index
        }
    }

    // Output the result
    println!("{} {}", best_day, min_max_temp);
}
