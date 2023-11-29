fn five() -> i32 {
    5
}

fn main() {
    let tup = (500, 6.4, 1);
    let (_i, y, _z) = tup;
    println!("The value of y is: {y}");
/////////
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("The value of 0 is: {five_hundred}");
    println!("The value of 1 is: {six_point_four}");
    println!("The value of 2 is: {one}");
////////
    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let _a = [3; 5]; // let a = [3, 3, 3, 3, 3];
///////
    another_function(5);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");
///////
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
//////
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
//////
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}