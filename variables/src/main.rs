fn main() {
    let mut x: i8 = 125;
    println!("The value of x is: {}", x);
    x = 43 % 5;
    println!("The value of x is: {}", x);
    another_function(five());
}

fn another_function(x: i32) {
    println!("Another function. The value of x is: {}", x);
    if x < 5 {
        println!("small number");
    } else {
        println!("big number");
    }
}

fn five() -> i32 {
    5
}
