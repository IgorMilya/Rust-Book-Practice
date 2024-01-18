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

fn main() {

}















