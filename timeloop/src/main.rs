use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: i32 = input.trim().parse().expect("Please enter a number");

    for i in 1..=n {
        println!("{} Abracadabra", i);
    }
}
