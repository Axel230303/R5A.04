use std::io::{self, Write};

fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for p in 2..=limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
    }
    is_prime
}

fn goldbach_pairs(x: usize, is_prime: &Vec<bool>) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for p1 in 2..=x/2 {
        if is_prime[p1] && is_prime[x - p1] {
            pairs.push((p1, x - p1));
        }
    }
    pairs
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid number");

    let mut test_cases = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let x: usize = input.trim().parse().expect("Invalid number");
        test_cases.push(x);
    }

    let limit = 32_000;
    let is_prime = sieve_of_eratosthenes(limit);

    for &x in &test_cases {
        let pairs = goldbach_pairs(x, &is_prime);
        println!("{} has {} representation(s)", x, pairs.len());
        for (p1, p2) in pairs {
            println!("{}+{}", p1, p2);
        }
        println!();
    }
}
