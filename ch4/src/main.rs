fn main() {
    play();
}

fn play() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    println!("{v:?}");
    let mut s: String = v.remove(0);
    println!("{s}");
    println!("{v:?}"); 

    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);

}car