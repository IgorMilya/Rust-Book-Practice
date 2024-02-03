use ::std::thread;
use std::sync::mpsc;
use std::time::Duration;
use::std::sync::{Mutex, Arc};
use std::rc::Rc;

fn main() {
    // Using Threads to Run Code Simultaneously
    // let handle = thread::spawn(|| {
    //     for i in 0..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    //
    // for i in 0..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    // handle.join().unwrap();
    //
    // let v = vec![1, 2, 3];
    //
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });
    //
    // handle.join().unwrap();

    ////////////////////////////////////////////////////////////////////////////////////////////////

    // Using Message Passing to Transfer Data Between Threads

    // let (tx, rx) = mpsc::channel();
    // let tx2 = tx.clone();
    //
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    //
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];
    //
    //     for val in vals {
    //         tx2.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    //
    // for received in rx {
    //     println!("Got: {}", received);
    // }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////

    // Shared-State Concurrency

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());





}
