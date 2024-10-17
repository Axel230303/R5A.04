use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Lire la longueur du programme (pas vraiment utilisé dans ce cas)
    let _l: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Lire la ligne contenant les délimiteurs
    let line = lines.next().unwrap().unwrap();
    
    // Stack pour suivre les délimiteurs ouverts
    let mut stack = Vec::new();
    
    // Itérer sur chaque caractère avec son index
    for (i, ch) in line.chars().enumerate() {
        match ch {
            '(' | '[' | '{' => stack.push((ch, i)), // Ajouter les délimiteurs ouverts à la pile
            ')' | ']' | '}' => {
                if let Some((open, _)) = stack.pop() {
                    if !matches_pair(open, ch) {
                        println!("{} {}", ch, i);
                        return;
                    }
                } else {
                    // Cas où un délimiteur fermant n'a pas de correspondant
                    println!("{} {}", ch, i);
                    return;
                }
            },
            _ => {}, // Ignorer les espaces
        }
    }
    
    // Si à la fin il reste des délimiteurs ouverts non fermés
    if !stack.is_empty() {
        println!("ok so far");
    } else {
        println!("ok so far");
    }
}

// Fonction pour vérifier si les délimiteurs correspondent
fn matches_pair(open: char, close: char) -> bool {
    match (open, close) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        _ => false,
    }
}
