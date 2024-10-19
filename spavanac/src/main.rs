fn main() {
    // On lit l'entrée standard
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Impossible de lire le fichier");

    // On récupère l'heure du fichier
    let parts: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().expect("Ce n'est pas un nombre acceptable")).collect();

    //on récupère les heures et les minutes
    let mut hours = parts[0];
    let mut minutes = parts[1];

    //On enlève 45 minutes de l'heure et/ou minutes
    if minutes > 45{
        minutes -= 45;
    } else{
        minutes += 15;
        if hours ==0{
            hours =23;
        }else {
            hours -=1;
        }
    }
    //Affichage de l'heure du réveil
    println!("{} {}",hours, minutes);
}
