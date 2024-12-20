#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 70,
        height: 100,
    };
    let area = rect1.area();

    println!("rect1 is {rect1:?}, has an area of {area:?}");
}
