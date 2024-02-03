use tokio::time::sleep;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]

async fn main() {
    // println!("Hello, world!");
    // let f = my_function();
    // println!("Let me know");
    // f.await;

    let mut handles = Vec::new();

    for i in 0..2 {
        let handle = tokio::spawn(async move {
           my_function(i).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function(i: i32) {
    println!("[{i}] I'm an async function!");
    let s1 = read_from_database().await;
    println!("[{i}] First func {}", s1);
    let s2 = read_from_database().await;
    println!("[{i}] Second func {}", s2);

}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}