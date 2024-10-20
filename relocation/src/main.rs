use std::io;

fn main() {
    let mut input = String::new();

    // Lire le numéro du joueur qui commence avec la boîte
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture");
    let mut player: usize = input.trim().parse().expect("Erreur lors de l'analyse");

    // Lire le nombre de questions posées
    input.clear();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture");
    let m: usize = input.trim().parse().expect("Erreur lors de l'analyse");

    // Constante de durée avant l'explosion
    let total_time: i32 = 210; // 3 minutes et 30 secondes

    let mut elapsed_time = 0;

    for _ in 0..m {
        // Lire les informations de chaque question
        input.clear();
        io::stdin().read_line(&mut input).expect("Erreur lors de la lecture");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let time: i32 = parts[0].parse().expect("Erreur lors de l'analyse du temps");
        let result = parts[1];

        // Ajouter le temps écoulé depuis la dernière question
        elapsed_time += time;

        // Si le temps écoulé dépasse le temps total, sortir de la boucle
        if elapsed_time >= total_time {
            break;
        }

        // Si la réponse est correcte, passer la boîte au joueur suivant
        if result == "T" {
            player = if player == 8 { 1 } else { player + 1 };
        }
    }

    // Afficher le joueur qui tient la boîte lorsque le temps est écoulé
    println!("{}", player);
}
