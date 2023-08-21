use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let size = take_input()?.parse::<u32>()?;
    let input: Vec<_> = take_input()?.chars().collect();
    let length = input.windows(2).filter(|a| a[0] != a[1]).collect::<Vec<_>>().len();
    println!("{}", (size - (length as u32) - 1));
    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}