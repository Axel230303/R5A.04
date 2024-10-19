use std::io::{self, BufRead};

fn main() {
    // On lit l'entrée standard
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // On récupère le nombre d'estimations
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // On traite chaque estimation
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            // On calcule le nombre de chiffres dans l'estimation
            let digits = line.trim().len();
            println!("{}", digits);
        }
    }
}
