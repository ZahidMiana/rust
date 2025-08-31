
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn calculate_area(&self) -> u32 {
        self.height * self.width
    }   

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width >= other.width && self.height >= other.height
    }
}   

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 60,
        height: 60,
    };

    println!("The area of Rectangle {:?} is {}", rect1, rect1.calculate_area());
    println!("Can rect 1 hold rect 2? {}", rect2.can_hold(&rect1));
}