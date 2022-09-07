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
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
