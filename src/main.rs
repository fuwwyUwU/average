mod lib;
use std::io;

fn main() {
    let mut input = String::new();

    println!("please enter your numbers with a whitespace between them (1 2 3)");
    io::stdin()
    .read_line(&mut input)
    .expect("failed to read input");
    let mut numbers = match lib::get_numbers(input) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("failed to read the numbers, {}", e);
            return
        }
    };

    if numbers.len() < 1 {
        eprintln!("you have to enter at least one number");
        return
    }

    println!("{:?}", numbers);

    let mut average_type = String::new();

    println!("what average would you like to recive? mean, median or mode?");

    io::stdin()
    .read_line(&mut average_type)
    .expect("failed to read input");
    
    loop {
        match average_type.trim().to_lowercase().as_str() {
            "mode" => println!("{}", lib::mode(&numbers)),
            "mean" => println!("{}", lib::mean(&numbers)),
            "median" => println!("{}", lib::median(&mut numbers)),
            _ => eprintln!("lean to speell"),
        }
    }
    
}
