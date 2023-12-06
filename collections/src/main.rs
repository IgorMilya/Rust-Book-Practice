use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    // VECTORS

    // let a = [1, 2, 3];
    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    //
    // let mut v2 = vec![1, 2, 3, 4, 5];
    // let third = &v2[2];
    // v2.push(6);
    //
    // println!("{}", third);
    //
    // match v2.get(2) {
    //     Some(value) => println!("OK {}", value),
    //     None => println!("Error")
    // }

    ////////////////////////////////////////////////////////////////

    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // for i in &mut v {
    //     *i += 50;
    // }
    //
    // for i in &v {
    //     println!("{}", i);
    // }

    ////////////////////////////////////////////////////////////////

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f32),
    //     Text(String),
    // }
    //
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Float(10.12),
    //     SpreadsheetCell::Text(String::from("red")),
    // ];
    //
    // match &row[1] {
    //     SpreadsheetCell::Int(i) => println!("{}", i),
    //     _ => println!("Not Integer"),
    // };

    /////////////////////////////////////////////////////////////////

    // STRINGS

    // let mut s = String::from("foo");
    // s.push_str("bar");
    // s.push('!');
    //
    // println!("{}", s);
    //
    //
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3: String = s1 + &s2;
    // let s3: String = format!("{}{}", s1, s2);  // doesn't use the ownership
    //
    // println!("{}", s3);


    // let hello: String = String::from("Hello");
    // // let c: char = hello[1];
    //
    // println!("{}", c);


    // let hello2_0: String = String::from("China");

    // Bytes
    // for b in "Cgina".bytes() {
    //     println!("{}", b);
    // };

    // Scalar values
    // for c in "China2346ыфцв".chars() {
    //     println!("{}", c);
    // }

    // Grapheme clusters

    // for g in "China".graphemes(true) {
    //     println!("{}", g);
    // }

    /////////////////////////////////////////////////////////////////////////

    // HASHMAP

    // let blue = String::from("Blue");
    // let yellow = String::from("Yellow");
    //
    // let mut scores = HashMap::new();
    //
    // scores.insert(blue, 10);
    // scores.insert(yellow, 50);
    //
    // let team_name = String::from("Blue"); // we can do it via lifestyle or smth like that, however now we don't know how to implement that and this way is the most convenient for us
    // let score = scores.get(&team_name);
    //
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // };

    ////////////////////////////////////////////////////////////////////////

    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 20);
    //
    // scores.entry(String::from("Yellow")).or_insert( 30);
    // scores.entry(String::from("Yellow")).or_insert( 40);
    //
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // };

    ////////////////////////////////////////////////////////////////////////

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    };

    println!("{:?}", map);
}









