// use std::fmt;
//
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
//
// impl fmt::Display for User {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "User(active: {}, username: {}, email: {}, sign_in_count: {})",
//             self.active, self.username, self.email, self.sign_in_count
//         )
//     }
// }
//
// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//
//    let user2 = User {
//        email: String::from("another@example.com"),
//        ..user1
//    };
//
//     println!("{}", user2);
//
//
//     let black = Color(0,0,0);
//     let origin = Point(0,0,0);
//
// }
//
//
// fn build_user(username: String, email: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }
////////////////////////////////////////////////////////////////////////////////////////////////////

// struct Rectangle {
//     width: u32,
//     height: u32,
// }


// fn main() {

// WITH SIMPLE WAY
// let width = 30;
// let height = 50;

// WITH TUPLES
// let rect = (30, 50);

// WITH STRUCTURES
// let rect = Rectangle {
//     width: 30,
//     height: 50,
// };
//
// println!(
//     "The area of the rectangle is {} square pixels.",
//     area(&rect));
// }

// WITH SIMPLE WAY
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// WITH TUPLES
// fn area((width, height): (u32, u32)) -> u32 { // first way
//     width * height
// }
// fn area(dimensions: (u32, u32)) -> u32 { // second way
//     dimensions.0 * dimensions.1
// }

// WITH STRUCTURES
// fn area(rectangle: &Rectangle) -> u32 { // first way
//     rectangle.width * rectangle.height
// }
// fn area(Rectangle { width, height }: &Rectangle) -> u32 { // second way
//     width * height
// }


////////////////////////////////////////////////////////////////////////////////////////////////////
// Adding Useful Functionality with Derived Traits
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!("rect1 is {:#?}", rect1);
// }

// Another way to debug
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };
//
//     dbg!(&rect1);
// }
////////////////////////////////////////////////////////////////////////

//Method Syntax
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
    fn square(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
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
    let sq = Rectangle::square(3,4);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect3? {:#?}", sq);
}
































