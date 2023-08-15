use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let times: u32 = take_input()?.parse()?;
    let mut solution: u32 = 0;
    for _ in 0..times {
        let current: u32 = take_input()?
                            .split(" ")
                            .map(|a| a.parse::<u32>().unwrap())
                            .sum();
        if current > 1 {
            solution += 1 ;
        }
    }
    println!("{solution}");
    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}