use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let input = take_input()?;
    let mut input = input
                                                    .split(" ")
                                                    .map(|x| x.parse().unwrap());
    input.next().unwrap();
    let last: u32 = input.next().unwrap();

    let score_input = take_input()?;
    let scores: Vec<u32> = score_input
                                    .split(" ")
                                    .map(|x| x.parse::<u32>().unwrap())
                                    .collect();

    let last_score = scores[(last - 1) as usize];
    let total = scores
                            .into_iter()
                            .filter(|x| x >= &last_score && x > &0)
                            .count();

    println!("{total}");
    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}