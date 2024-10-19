use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut max_score = 0;
    let mut winner = 0;

    for (i, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        let grades: Vec<i32> = line.split_whitespace()
                                   .map(|s| s.parse().unwrap())
                                   .collect();
        let sum: i32 = grades.iter().sum();

        if sum > max_score {
            max_score = sum;
            winner = i + 1; // Contestants are 1-indexed
        }
    }

    println!("{} {}", winner, max_score);
}
