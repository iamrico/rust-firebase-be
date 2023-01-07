#[macro_use] extern crate rocket;
use firebase_rs::*;
use rocket::{
    serde::{json::Json, Deserialize, Serialize},
};
use rocket::form::Form;

#[derive(Serialize, Deserialize, Debug, FromForm)]
struct Task {
    title: String,
    description: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,

}

#[post("/todo", data="<todo_form>")]
async fn createTodo(todo_form: Form<Task>) -> Json<Response> {
    let todo = todo_form.into_inner();
    let task = Task {
        title: todo.title.to_string(),
        description: todo.description.to_string()
    };
    let firebase_client = Firebase::new("https://todo-58ce6-default-rtdb.asia-southeast1.firebasedatabase.app/").unwrap();
    
    let response = set_task(&firebase_client, &task).await;
    let name = &response.name;

    Json(Response { name: name.to_string() })
}

#[launch]
fn rocket() -> _ { 
    rocket::build().mount("/todo", routes![createTodo])
}


async fn set_task(firebase_client: &Firebase, task: &Task) -> Response {
    let firebase = firebase_client.at("tasks");
    let _tasks = firebase.set::<Task>(&task).await;
    
    return string_to_response(&_tasks.unwrap().data);
}

fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}