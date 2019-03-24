struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct GenPoint<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
struct GenPoint2<T, U> {
    x: T,
    y: U,
}

impl Point {
    fn abs(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let p0 = Point { x: 1.0, y: 1.0 };
    println!("{}", p0.abs());

    let _p1 = GenPoint::<i32> { x: 1, y: 1 };
    let _p2 = GenPoint { x: 1, y: 1 };
    let _p3 = GenPoint { x: 1.5, y: 2.0 };

    // mismatched types
    // let _p4 = GenPoint { x: 1.5, y: 2 };

    let _p5 = GenPoint2 { x: 1.5, y: 2 };

    let x = 0.0;
    let y = 0.0;

    let _p6 = GenPoint { x, y };
}
