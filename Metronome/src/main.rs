use std::io;

fn main() {
    let mut input = String::new();
    let denominator = 4.0;

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    let metro:f64 = parts[0].parse().expect("Please enter a valid number");

    let tick = metro / denominator;
    let formated_tick = format!("{:.2}", tick);

    println!("{}", formated_tick);
}