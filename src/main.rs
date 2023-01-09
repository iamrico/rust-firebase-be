#[macro_use] extern crate rocket;
use firebase_rs::*;
use rocket::{
    serde::{json::Json, Deserialize, Serialize},
};
use rocket::form::Form;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, FromForm, Clone)]
struct Task {
    title: String,
    description: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
struct DeleteRequest {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct GetTodoResponse {
    todos: Vec<Task>
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

#[delete("/todo", data="<todo_form>")]
async fn deleteTask(todo_form: Form<DeleteRequest>) -> Json<Response> {
    let todo = todo_form.into_inner();
    let firebase_client = Firebase::new("https://todo-58ce6-default-rtdb.asia-southeast1.firebasedatabase.app/").unwrap();
    let name = todo.name.to_string();

    delete_task(&firebase_client, &name).await;

    Json(Response { name: "successful deletion of todo".to_string() })
}

#[get("/todo")]
async fn listTodo() -> Json<GetTodoResponse> {
    let firebase_client = Firebase::new("https://todo-58ce6-default-rtdb.asia-southeast1.firebasedatabase.app/").unwrap();

    let tasks = get_tasks(&firebase_client).await;

    let newArray = tasks.values().cloned().collect();
    Json(GetTodoResponse{todos: newArray})
}
#[launch]
fn rocket() -> _ { 
    rocket::build().mount("/", routes![createTodo, deleteTask, listTodo])
}


async fn set_task(firebase_client: &Firebase, task: &Task) -> Response {
    let firebase = firebase_client.at("tasks");
    let _tasks = firebase.set::<Task>(&task).await;
    
    return string_to_response(&_tasks.unwrap().data);
}

async fn delete_task(firebase_client: &Firebase, name: &String) {
    let firebase = firebase_client.at("tasks").at(&name);
    let _result = firebase.delete().await;

}

async fn get_tasks(firebase_client: &Firebase) -> HashMap<String, Task>{
    let firebase = firebase_client.at("tasks");
    let tasks = firebase.get::<HashMap<String, Task>>().await;
    return tasks.unwrap();
}

fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}