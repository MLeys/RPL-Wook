// structs do not auto Display with println!
// you must specify
// Add call to debug for ability to display debug of struct

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
    
    println!("rect1 is {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 { //immutable borrow of Rectangle
    rectangle.width * rectangle.height
}