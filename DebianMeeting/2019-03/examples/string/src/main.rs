fn main() {
    let a = "Hello";
    let mut b = a.to_string();

    println!("a: {}", a);
    println!("b: {}", b);

    b.push_str(" World");
    println!("b: {}", b);
}
