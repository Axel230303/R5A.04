use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        let mut cities = HashSet::new();
        
        for _ in 0..n {
            let city = lines.next().unwrap().unwrap();
            cities.insert(city);
        }
        
        println!("{}", cities.len());
    }
}