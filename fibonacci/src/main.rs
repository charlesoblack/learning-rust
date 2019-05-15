use std::io;

fn main() {
    println!("Which position Fibonacci number do you want?");

    let mut position = String::new();

    io::stdin().read_line(&mut position)
        .expect("Could not read line!");
    
    let position: u128 = match position.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input was not a number!")
    };

    println!("Selected position: {}",position);

    let fib_number = fibonacci(position);

    println!("The {}'th Fibonacci number is: {}",position,fib_number);
}

fn fibonacci(pos: u128) -> u128 {
    match pos {
        1 => 1,
        2 => 1,
        x => fibonacci(x-1) + fibonacci(x-2),
    }
}