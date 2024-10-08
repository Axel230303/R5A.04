use std::io;

fn main() {
    let mut input = String::new();
    
    // Lire le nombre de températures
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    let _n: usize = input.trim().parse().expect("Erreur de parsing");
    
    input.clear();
    
    // Lire les températures
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    let temperatures: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Erreur de parsing"))
        .collect();
    
    // Compter les températures inférieures à 0
    let count = temperatures.iter().filter(|&&t| t < 0).count();
    
    // Afficher le résultat
    println!("{}", count);
}
