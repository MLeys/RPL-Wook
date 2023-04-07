// enum IpAddrKind {
//     V4,
//     V6,
// }
// fn route(ip_kind: IpAddrKind) {} // function that takes any kind of the specified enum types
// route(IpAddrKind::V4);
// route(IpAddrKind::V6); // how they would be called with either varient

// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;

// this is less efficient way using structs
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

// fn main() {    
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }
    
//     // let home = IpAddr::V4(String::from("127.0.0.1"));
//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));
// }

fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

// Option<T> 
// enum Option<T> { // T = any type
//     None,
//     Some(T),
// }
let some_number = Some(5); // Option<i32>
let some_char = Some('e');  // Option<char>

let absent_number: Option<i32> = None;

// Cannot add i8 to Option<i8> ***
// In order to have a value that can possibly be null, 
// you must explicitly opt in by making the type of that value Option<T>
// Then you MUST handle the case if null


// MAtch



#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        _ => (), // nothing happens if do not roll 3 or 7
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    // fn move_player(num_spaces: u8) {}
    // fn reroll() {}
}