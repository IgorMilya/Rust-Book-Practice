use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ToDo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}
#[derive(Serialize, Deserialize)]

struct Address {
    street: String,
    city: String,
}
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Receive type-checked JSON

    let todos: Vec<ToDo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", todos);

    // Send and receive type-checked JSON

    let new_todo = ToDo {
        user_id: 1,
        id: None,
        title: "Hi".to_owned(),
        completed: false
    };

    let new_todo: ToDo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);

    // Send and receive arbitrary JSON

    let new_todo: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&serde_json::json!({
            "userId": 1,
            "title": "Subscribe to Let's Get Rusty".to_owned(),
            "completed": false
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&new_todo).unwrap();
    println!("{}", j);

    Ok(())
}