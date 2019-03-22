fn main() {
    let mut a = "abcd".to_string(); // String::from("abcd"); も可
    let b = &a[1..];
    let /* mut */ c = "zyxwv";

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);

    a.push_str("efg");
    println!("a: {}", a);

    // no method named `push_str` found for type `&str` in the current scope
    // c.push_str("ut");
}
