
struct Rectangle {
    height: f64,
    width: f64
}

impl Rectangle {
    fn new(height: f64, width: f64) -> Rectangle {
        Rectangle { height, width }
    }

    fn cal_area(&self)-> f64{
        self.height * self.width
    }
}

fn main() {
    let rect = Rectangle::new(5.3, 6.3);
    println!("rect {:#?}",rect);
    println!("The area of the rect is {}", rect.cal_area());
}
