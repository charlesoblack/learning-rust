use std::io;

fn main() {
    println!("Fahrenheit to Celsius or Celsius to Fahrenheit?");
    println!("(1) F -> C");
    println!("(2) C -> F");

    let mut direction = String::new();

    io::stdin().read_line(&mut direction)
        .expect("Could not read line!");

    let direction = match direction.trim().parse() {
        Ok(1) => true,
        Ok(2) => false,
        _ => panic!("Could not understand direction!")
    };

    println!("How many degrees?");

    let mut value = String::new();

    io::stdin().read_line(&mut value)
        .expect("Could not read value!");

    let value = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not parse value!")
    };

    let converted_value = convert(direction,value);

    println!("Converted value is: {}",converted_value);
}

fn convert(dir: bool, value: i32) -> i32 {
    if dir {
        (value-32)*5/9
    } else {
        (value*9/5)+32
    }
}