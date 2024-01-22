use ::std::thread;
use std::sync::mpsc;
// use std::time::Duration;

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


    fn main() {
        let (tx, rx) = mpsc::channel();
    }



















}
