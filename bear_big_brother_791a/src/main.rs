use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let bros = take_input()?;
    let mut bros = bros.split(" ").flat_map(|x| x.parse::<u32>());
    let mut limak = bros.next().unwrap();
    let mut bob = bros.next().unwrap();
    let mut counter = 0;
    while limak <= bob {
        counter += 1;
        limak *= 3;
        bob *= 2;
    }
    println!("{counter}");
    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}
