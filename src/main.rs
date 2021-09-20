#![feature(exclusive_range_pattern)]
#[allow(dead_code)]

fn is_prime(n: u64) -> bool {
    // Note: there are nicer *and* faster ways to do a primality test. But that
    // was not necessarily the point of this task.

    // 1 is special and yes: 1 is not a prime!
    if n <= 1 {
        return false;
    }

    // Iterator through all possible divisors
    for d in 2..n {
        if n % d == 0 {
            return false;
        }
    }

    true
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    fn create(a: f32, b: f32) -> Self {
        Self {
            x: a,
            y: b,
        }
    }

    fn move_point(&mut self, a: f32, b: f32) {
        self.x = a;
        self.y = b;
    }
}

fn main() {
    match 100 {
        n @ 0..=100 if is_prime(n) => {},
        _ => println!("not a prime below 101"),
    }

    match Point::origin() {
        Point { x: 0.0, y} => println!("on y-axis: {}", y),
        Point { x: 3.0, y: name} => println!("{}", name),
        p => println!("somwhere else: {:?}", p),
    }

    let mut point = Point::create(3.0, 4.0);
    match point {
        Point { x: 0.0, y} => println!("on y-axis: {}", y),
        Point { x: 3.0, y: name} => println!("{}", name),
        p => println!("somwhere else: {:?}", p),
    }
    point.move_point(1.5, 2.0);
    println!("moved point: {:?}", point);

    // let point = Point::create(2.0, 1.8);
    match Point::create(1.8, 2.7) {
        Point { x: 0.0, y} => println!("on y-axis: {}", y),
        Point { x: 3.0, y: name} => println!("{}", name),
        p => println!("somwhere else: {:?}", p),
    }
}
