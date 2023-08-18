use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let input = take_input()?;
    let mut input: Vec<u32> = input.split("+").flat_map(|x| x.parse()).collect();
    input.sort();
    let input: String = input.into_iter().map(|c| c.to_string()).reduce(|a, b| format!("{a}+{b}")).unwrap();
    println!("{input}");
    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}
