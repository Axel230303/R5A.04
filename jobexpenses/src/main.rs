use std::io;

fn main() {
    let mut input = String::new();
    
    // Lire le nombre de transactions
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    let _n: usize = input.trim().parse().expect("Erreur de parsing");
    
    input.clear();
    
    // Lire les transactions
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    let transactions: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Erreur de parsing"))
        .collect();
    
    // Calculer la somme des dépenses (nombres négatifs)
    let expenses_sum: i32 = transactions
        .iter()
        .filter(|&&x| x < 0)
        .sum();
    
    // Afficher la somme des dépenses
    println!("{}", expenses_sum.abs());
}
