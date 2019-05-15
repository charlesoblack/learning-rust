#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
                    width: 30,
                    height: 50,
                };

    println!("The are of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:#?}",rect1);

    // the following is allowed because we used an immutable reference above
    // println!("{}",rect1.width)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
