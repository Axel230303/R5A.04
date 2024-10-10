use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut free_food_days = HashSet::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let s: i32 = parts.next().unwrap().parse().unwrap();
        let t: i32 = parts.next().unwrap().parse().unwrap();
        
        for day in s..=t {
            free_food_days.insert(day);
        }
    }

    println!("{}", free_food_days.len());
}