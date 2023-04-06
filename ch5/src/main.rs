// structs do not auto Display with println!
// you must specify
// Add call to debug for ability to display debug of struct

// #[derive(Debug)]
#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:?}", rect1);
//     // can also use {:#?} which will ad diff style to output (new lines for info)
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }



// fn area(rectangle: &Rectangle) -> u32 { //immutable borrow of Rectangle
//     rectangle.width * rectangle.height
// }

// 5.3 methods tied to struct


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self { // assiciated function, NOT a method
        Self {
            width: size,
            height: size,
        }
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle { 
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect = Rectangle {
        width: 0,
        height: 0
    };
    println!("{}", rect.area());
    
    rect.set_width(1);     // this is now ok

    let rect_ref = &rect;





    let other_rect = Rectangle { width: 1, height: 1 };
    let max_rect = rect.max(other_rect);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}