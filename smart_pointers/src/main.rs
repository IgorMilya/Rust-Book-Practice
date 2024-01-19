// Box
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
// use List::{Cons, Nil};
//
// fn main() {
//     let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

/////////////////////////////////////////////////////////////////////////////////////
// The Deref Trait

// use std::ops::Deref;
//
// struct MyBox<T>(T);
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
//
// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);
//
//     assert_eq!(5, x);
//     assert_eq!(5, *(y.deref()));
//
//     let m = MyBox::new(String::from("Rust"));
//     hello(&m);
// }
//
// fn hello(name: &str) {
//     println!("Hello, {}!", name);
// }

///////////////////////////////////////////////////////////////////////////////////////////

// The Drop Trait
// struct CustomSmartPointer {
//     data: String,
// }
//
// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }
//
// fn main() {
//     let c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };
//     let d = CustomSmartPointer {
//         data: String::from("other stuff"),
//     };
//     println!("CustomSmartPointers created.");
// }

///////////////////////////////////////////////////////////////////////////////////////////

// Reference Counting only in single thread
// use std::rc::Rc;
//
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
//
// fn main() {
//     // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     // let b = Cons(3, Rc::clone(&a));
//     // let c = Cons(4, Rc::clone(&a));
//
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

////////////////////////////////////////////////////////////////////////////////////////////////

// RefCell<T> and the Interior Mutability Pattern

// pub trait Messenger {
//     fn send(&self, msg: &str);
// }
//
// pub struct LimitTracker<'a, T: Messenger> {
//     messenger: &'a T,
//     value: usize,
//     max: usize,
// }
//
// impl<'a, T> LimitTracker<'a, T>
//     where
//         T: Messenger,
// {
//     pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }
//
//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;
//
//         let percentage_of_max = self.value as f64 / self.max as f64;
//
//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: You are over your quota!");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger
//                 .send("Urgent warning: You've used up over 90% of your quota!");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger
//                 .send("Warning: You've used up over 75% of your quota!");
//         }
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::cell::RefCell;
//
//     struct MockMessenger {
//         sent_messages: RefCell<Vec<String>>,
//     }
//
//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: RefCell::new(vec![]),
//             }
//         }
//     }
//
//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             // self.sent_messages.borrow_mut().push(String::from(message));
//             let mut one_borrow = self.sent_messages.borrow_mut();
//             let mut two_borrow = self.sent_messages.borrow_mut();
//
//             one_borrow.push(String::from(message));
//             two_borrow.push(String::from(message));
//         }
//     }
//
//
//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
//
//         limit_tracker.set_value(80);
//
//         assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
//     }
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}