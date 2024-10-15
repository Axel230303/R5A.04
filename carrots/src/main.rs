use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    let carrots: i32 = parts[1].parse().expect("Please enter a valid number");

    println!("{}", carrots);
}