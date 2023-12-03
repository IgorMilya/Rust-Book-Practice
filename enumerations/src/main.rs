#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr1 {
    v4: (u8, u8, u8, u8),
    v6: String
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    println!("{:?}", home);

    ////////////////////////////////////////////////////////////////

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);


    let ok_result: Result<i32, &str> = Ok(42);
    let result = ok_result.unwrap_or_else(|err| {
        println!("Error: {}", err);
        -1
    });

    println!("Result: {}", result);  // Output: Result: 42

}
