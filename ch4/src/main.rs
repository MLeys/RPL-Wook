fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("{word}");
    let my_string_literal = "hello world";
    println!("{word}");
    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    println!("{word}");
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{word}");
}

// fn play() {
//     let mut v: Vec<String> = vec![String::from("Hello world")];
//     println!("{v:?}");
//     let mut s: String = v.remove(0);
//     println!("{s}");
//     println!("{v:?}"); 

//     s.push('!');
//     println!("{s}");
//     assert!(v.len() == 0);

// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
    
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     println!{"{s} s"};
//     s.len()
    
// }
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
