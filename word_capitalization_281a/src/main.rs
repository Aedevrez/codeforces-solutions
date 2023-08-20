use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let input = take_input()?;
    let input: String = input.split(" ").map(|c| format!("{}{}", c.to_string().remove(0).to_uppercase(), &c[1..])).collect();
    println!("{input}");
    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}