// Implementation 1
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("Area = {}", area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Implementation 2
// fn main() {
//     let rect1 = (30, 50);

//     println!("Area = {}", area(rect1));
// }

// fn area(rect: (u32, u32)) -> u32 {
//     rect.0 * rect.1
// }

// Implementation 3
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("Area = {}", area(&rect1));
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

// Implementation 5
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("Rect is {:#?}", rect1);
// }

// Implmentation 6
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;

//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// Implementation 7
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
