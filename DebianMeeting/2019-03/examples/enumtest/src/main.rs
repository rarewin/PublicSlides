enum Fruits {
    Apple,
    Banana,
    Orange,
    Peach,
}

enum IpAddr {
    V4((u8, u8, u8, u8)),
    V6(String),
}

enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

#[derive(Debug)]
struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}

enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

fn extract_addr(addr: &IpAddr) {
    match addr {
        IpAddr::V4(a) => {
            println!("{}.{}.{}.{}", a.0, a.1, a.2, a.3);
        }
        IpAddr::V6(a) => {
            println!("{}", a);
        }
    }
}

fn main() {
    let _hoge = Fruits::Apple;

    let v4_addr = IpAddr::V4((127, 0, 0, 1));
    let v6_addr = IpAddr::V6("::1".into());

    extract_addr(&v4_addr);
    extract_addr(&v6_addr);

    let t = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    match &t {
        RoughTime::InTheFuture(_, t) => {
            println!("{}", t);
        }
        _ => {}
    }

    let u = Shape::Cuboid {
        corner1: Point3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        corner2: Point3d {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    };

    match &u {
        Shape::Cuboid {
            corner1: c1,
            corner2: c2,
        } => {
            println!("{:?}", c1);
        }
        _ => {}
    }
}
