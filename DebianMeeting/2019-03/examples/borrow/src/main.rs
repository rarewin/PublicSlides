fn consume(_s: &String) {}

fn catlength(a: &String, b: &String) -> usize {
    return a.len() + b.len();
}

fn main() {
    let s0: String = "hoge".to_string();
    let _s1 = &s0;
    let _s2 = &s0;

    let h: String = "Hello World".to_string();
    consume(&h);
    let _hh = h;

    let i0 = 5;
    let i1 = &i0;

    println!("i0: {}, i1: {}", i0, *i1);

    let s0 = "hoge".to_string();
    let s1 = "fuga".to_string();
    println!("{}", catlength(&s0, &s1));
}
