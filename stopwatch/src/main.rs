use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Number of button presses
    let button_pressed: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Reading all button press times
    let times: Vec<i32> = lines
        .map(|x| x.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate the time
    if button_pressed % 2 != 0{
        println!("still running");
    }else {
        let mut total_time = 0;
        for i in (0..button_pressed).step_by(2){
            total_time = total_time + times[i+1] - times[i];
        }
        println!("{}", total_time);
    }
    
}
