use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input: u32 = input.trim().parse()?;

    for num in 1..input {
        if divisible_by_even(num, input) {
            println!("YES");
            return Ok(());
        }
    }

    println!("NO");
    Ok(())
}

fn divisible_by_even(num: u32, input: u32) -> bool {
    num % 2 == 0 && (input - num) % 2 == 0
}
