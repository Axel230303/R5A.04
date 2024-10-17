use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");

    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Erreur lors de la conversion des valeurs"))
        .collect();
    
    let r1 = values[0];
    let s = values[1];
    
    let r2 = 2 * s - r1;

    println!("{}", r2);
}