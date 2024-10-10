fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut result = Vec::new();

    for ch in input.trim().chars() {
        if ch == '<' {
            result.pop();
        } else {
            result.push(ch);
        }
    }

    let final_string: String = result.into_iter().collect();
    println!("{}", final_string);
}