use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let times = take_input()?.parse::<u32>()?;
    let mut x = 0;

    for _ in 0..times {
        let input = take_input()?;
        let mut input = input.chars();
        input.next();
        match input.next().unwrap() {
            '+' => {x += 1},
            '-' => {x -= 1},
            _ => {println!("bu hatalı")},
        }
    }

    println!("{x}");

    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}