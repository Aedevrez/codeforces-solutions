use std::{io, error::Error, collections::HashSet};

fn main() -> Result<(), Box<dyn Error>>{
    let input = take_input()?;
    let mut set = HashSet::new();
    input.chars().for_each(|c| {set.insert(c);});
    match set.len() % 2 == 0 {
        true => {println!("CHAT WITH HER!")},
        false => {println!("IGNORE HIM!")},
    };

    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}