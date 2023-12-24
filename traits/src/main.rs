// use std::fmt::{Debug, Display};
//
// pub struct NewsArticle {
//     pub headline: String,
//     pub author: String,
//     pub content: String,
// }
//
// impl Summary for NewsArticle {
//     fn summarize_author(&self) -> String {
//         format!("{}", self.author)
//     }
// }
//
// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }
//
// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }
//
// pub trait Summary {
//     fn summarize_author(&self) -> String;
//
//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }

// for simple cases
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

////////////////////////////////

// Diff types of Summary for item1 and item2
// pub fn notify(item1: &impl Summary + Display, item2: &impl Summary) {
//
// }

// Same types of Summary for item1 and item2
// pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
//
// }

////////////////////////////////

// it is not readable
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     return 5
// }

// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where
//         T: Display + Clone,
//         U: Clone + Debug,
// {
//     5
// }
////////////////////////////////

// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     }
// }

// fn main() {
// let tweet = Tweet {
//     username: String::from("horse_ebooks"),
//     content: String::from(
//         "of course, as you probably already know, people",
//     ),
//     reply: false,
//     retweet: false,
// };
//
// let article = NewsArticle {
//     author: String::from("Ihor"),
//     headline: String::from("The Sky is Falling"),
//     content: String::from("of course, as you probably already know, people"),
// };

// notify(&article);
// println!("Tweet: {}", tweet.summarize());
// println!("Article: {}", article.summarize());


//     println!("{}", returns_summarizable().summarize())
//
// }

////////////////////////////////////////////////////////////////////////////////////////////////
// Using Trait Bounds to Conditionally Implement Methods

// use std::fmt::Display;
//
// struct Pair<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }
//
// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }
//
// fn main() {
//     let i = Pair::new(4, 5);
//     i.cmp_display();
// }


////////////////////////////////////////////////////////////////////////////////////////////////

//// Define a trait named Printable
// trait Printable {
//     fn print(&self);
// }
//
// // Implement the Printable trait for the i32 type
// impl Printable for i32 {
//     fn print(&self) {
//         println!("Printing i32: {}", self);
//     }
// }
//
// // Implement the Printable trait for the String type
// impl Printable for String {
//     fn print(&self) {
//         println!("Printing String: {}", self);
//     }
// }
//
// // A generic function that prints any type implementing the Printable trait
// fn print_value<T: Printable>(value: T) {
//     value.print();
// }
//
// fn main() {
//     let num = 42;
//     let text = String::from("Hello, Rust!");
//
//     print_value(num);   // Prints: Printing i32: 42
//     print_value(text);  // Prints: Printing String: Hello, Rust!
// }