use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Pas réussi à lire la ligne");
    let n: usize = input.trim().parse().expect("Erreur de conversion");

    let mut total_qaly = 0.0;

    for _ in 0..n{
        input.clear();
        
        io::stdin().read_line(&mut input).expect("Pas réussi à lire la ligne");
        let values: Vec<f64> = input.trim().split_whitespace().map(|x| x.parse().expect("Erreur Conversion")).collect();

        let quality_year = values[0];
        let years = values[1];

        total_qaly += quality_year * years;
    }

    println!("{:.3}",total_qaly); 
}