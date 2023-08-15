use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let input = take_input()?;
    let input = input.split(" ").flat_map(|x| x.parse::<u32>()).reduce(|acc, e| acc * e / 2).unwrap();
    println!("{input}");

    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}