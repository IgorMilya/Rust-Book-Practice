// fn main() {
//     let mut s = String::from("hello");
//
//     s.push_str(", world!");
//
//     println!("{}", s);
//     ////////////////////////////////
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//
//     println!("s1 = {s1}, s2 = {s2}");
// }




////////////////////////////////
// fn main() {
//     let s = String::from("hello");
//
//     takes_ownership(s);
//     println!("{}", s);
//
//     let x = 5;
//
//     makes_copy(x);
//
//
// }
//
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }
//
// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }



////////////////////////////////
// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM
//
//     println!("{}, {}, and {}", r1, r2, r3);
//
// }

////////////////////////////////////////////////////////////////

// fn main() {
    // let s = String::from("Asw 123");
    //
    // let results = first_word(&s);
    // println!("{}", results);

    // let my_string = String::from("hello world");
    //
    // // `first_word` works on slices of `String`s, whether partial or whole
    // let word = first_word(&my_string[0..6]);
    // let word = first_word(&my_string[..]);
    // // `first_word` also works on references to `String`s, which are equivalent
    // // to whole slices of `String`s
    // let word = first_word(&my_string);
    //
    // let my_string_literal = "hello world";
    //
    // // `first_word` works on slices of string literals, whether partial or whole
    // let word = first_word(&my_string_literal[0..6]);
    // let word = first_word(&my_string_literal[..]);
    //
    // // Because string literals *are* string slices already,
    // // this works too, without the slice syntax!
    // let word = first_word(my_string_literal);

// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         println!("i {i}");
//         println!("item {item}");
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//
//     &s[..]
// }


////////////////////////////////////////////////////////////////
// fn main() {
//     let s = String::from("hello world");
//
//     let hello = &s[0..5];
//     // let hello = &s[..5]; // same
//     let world = &s[6..11];
//     // let world = &s[6..]; // same
//
//     let len = s.len();
//
//     // let slice = &s[0..len]; // whole string
//     // let slice = &s[..]; // same
//
//     println!("{}", hello);
//     println!("{}", world);
// }

