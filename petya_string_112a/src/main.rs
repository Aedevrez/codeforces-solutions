use std::{io, error::Error, cmp::Ordering};

fn main() -> Result<(), Box<dyn Error>>{
    let first_input = take_input()?.to_lowercase();
    let second_input = take_input()?.to_lowercase();

    println!("{}", match first_input.cmp(&second_input) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    });
    
    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}