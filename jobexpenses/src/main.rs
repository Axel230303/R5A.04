use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    let _n: usize = input.trim().parse().expect("Erreur de parsing");
    
    input.clear();

    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    let transactions: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Erreur de parsing"))
        .collect();

    let expenses_sum: i32 = transactions
        .iter()
        .filter(|&&x| x < 0)
        .sum();

    println!("{}", expenses_sum.abs());
}
