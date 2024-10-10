use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut vote_count = HashMap::new();
    let mut total_votes = 0;

    while let Some(Ok(line)) = lines.next() {
        let candidate = line.trim();
        if candidate == "***" {
            break;
        }

        *vote_count.entry(candidate.to_string()).or_insert(0) += 1;
        total_votes += 1;
    }

    let mut max_votes = 0;
    let mut winner = String::new();
    let mut is_runoff = false;

    for (candidate, &count) in &vote_count {
        if count > max_votes {
            max_votes = count;
            winner = candidate.clone();
            is_runoff = false;
        } else if count == max_votes {
            is_runoff = true;
        }
    }

    if is_runoff {
        println!("Runoff!");
    } else {
        println!("{}", winner);
    }
}
