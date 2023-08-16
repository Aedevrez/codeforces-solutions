use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let mut x_index: i32 = 0;
    let y_index: i32; 
    loop {
        let seq = take_input()?;
        let list: Vec<u32> = seq.split(" ").flat_map(|x| x.parse::<u32>()).collect();
        
        if list.contains(&1) {
            y_index = (list.into_iter().position(|x| x == 1).unwrap() + 1) as i32;  
            x_index += 1; 
            break;
        } else {
            x_index += 1; 
        }
    }

    let total_moves: i32 = (x_index - 3).abs() + (y_index - 3).abs();

    println!("{total_moves}");
    Ok(())
}

fn take_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}
