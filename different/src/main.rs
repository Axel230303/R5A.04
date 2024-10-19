use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    //Gérer si plusieurs lignes dans le fichier
    for line in stdin.lock().lines() {
        //unwrap pour gérer les erreurs 
        let line = line.unwrap();
        //Utilisation d'un vecteur u128 car a une plus grande limite de valeurs
        let numbers: Vec<u128> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        //Calcul de la différence entre les deux nombres avec let diff contenant le if/else (possible)
        let diff = if numbers[0] > numbers[1] {
            numbers[0] - numbers[1]
        } else {
            numbers[1] - numbers[0]
        };
        println!("{}", diff);
    }
}
