#[macro_use] extern crate rocket;
use firebase_rs::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    title: String,
    description: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,

}

#[get("/")]
async fn index() -> &'static str {

    let task = Task {
        title: "LOL".to_string(),
        description: "WTF".to_string()
    };
    let firebase_client = Firebase::new("https://todo-58ce6-default-rtdb.asia-southeast1.firebasedatabase.app/").unwrap();
    
    let response = set_task(&firebase_client, &task).await;
    println!("{:?}", &response.name);

    "Hello World"
}

#[launch]
fn rocket() -> _ { 
    rocket::build().mount("/", routes![index])
}


async fn set_task(firebase_client: &Firebase, task: &Task) -> Response {
    let firebase = firebase_client.at("tasks");
    let _tasks = firebase.set::<Task>(&task).await;
    
    return string_to_response(&_tasks.unwrap().data);
}

fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}