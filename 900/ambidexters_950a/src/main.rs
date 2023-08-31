use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let input = take_input()?;
    let mut input = input.split(" ").flat_map(|x| x.parse::<i32>());
    let left = input.next().unwrap();
    let right = input.next().unwrap();
    let amb = input.next().unwrap();
    let mut total = 0;
    let rest;

    match left < right {
        true => {total += left;
                rest = right - left;},
        false => {total += right;
                rest = left - right;},
    };

    match rest < amb {
        true => {total += rest + ((amb - rest)/2);},
        false => {total += amb;},
    };

    let total = total * 2;

    println!("{total}");
    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}