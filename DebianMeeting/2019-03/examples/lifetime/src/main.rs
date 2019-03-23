// error version
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

#[allow(dead_code)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    {
        let _a = 0;
    }
    // cannot find value `a` in this scope
    // println!("{}", a);

    let _r;
    {
        let b = 0;
        _r = &b;
    }
    // `b` does not live long enough
    // println!("{}", r);
}
