struct Rect<T> {
    x: T,
    y: T
}

impl<T: PartialOrd> Rect<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn can_hold(&self, other: &Rect<T>) -> bool {
        self.x >= other.x && self.y >= other.y
    }
}

#[cfg(test)]
mod un_tests {
    use super::*;

    #[test]
    fn it_works() {
        let smaller = Rect::new(4u32, 6u32);
        let bigger = Rect::new(6u32, 12u32);
        assert!(bigger.can_hold(&smaller));
    }
}
