fn consume(_s: String) {}

fn generate(i: i32) -> Vec<i32> {
    let mut v = Vec::new();
    v.push(i);
    return v;
}

fn main() {
    let i0 = 1;
    let _i1 = i0;
    let _i2 = i0;

    let s0: String = "hoge".to_string();
    let _s1 = s0;
    // let _s2 = s0;

    let h: String = "Hello World".to_string();
    consume(h);
    // let hh = h;

    let v1 = generate(10);
    let mut v2 = generate(100);

    v2.push(101);

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
}
