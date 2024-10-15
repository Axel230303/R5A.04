use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    let moose1: i32 = parts[0].parse().expect("Please enter a valid number");
    let moose2: i32 = parts[1].parse().expect("Please enter a valid number");

    if (moose1 == 0) && (moose2 == 0) {
        println!("Not a moose");
        return;
    }
    else if moose1 == moose2 {
        println!("Even {}", moose1 * 2);
        return;
    }
    else {
        let max = std::cmp::max(moose1, moose2);
        println!("Odd {}", max * 2);
    }
}