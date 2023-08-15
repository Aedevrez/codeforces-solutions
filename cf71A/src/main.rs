use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let times = take_input()?.parse()?;

    for _ in 0..times {
        let input = take_input()?;

        if input.len() > 10 {
            println!("{}{}{}", nth_char(&input, 0), (input.len() - 2), nth_char(&input, input.len()-1));
        } else {
            println!("{input}");
        }
    }
    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}

fn nth_char(str: &String, n: usize) -> char {
    str.chars().nth(n).unwrap()
}