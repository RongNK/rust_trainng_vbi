use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() -> io::Result<()>{
    // Read user input
    let mut user_input = String::new();
    println!("Enter your word :");
    std::io::stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input.lines()
        .next().expect("Could not read entry.");

    let file = File::open("1-s2.0-S0960982203005347-mmc6.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let str_tmp = line.unwrap();
        if str_tmp.contains(&user_input){
            count += 1;
        }
    }
    println!("The number of word '{}' is: {}",user_input,count);
    Ok(())
}