struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn x(&self)-> &T {
        return &self.x
    }
    fn y(&self)-> &U {
        return &self.y
    }
}

fn main() {
    let point = Point::new(5, 6.5);

    println!("x : {}",point.x());
    println!("y : {}",point.y());
}

