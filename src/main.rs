use actix_web ::{get,web,App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex; //to interact with memory
mod todolist;
use  todolist::service;

struct AppState{
    todolist_entries: Mutex<Vec<TodolistEntry>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TodolistEntry{
    id: i32,
    date: i64,
    title: String,
    description: String,
}

#[get("/")]
async fn index() -> String{
    "Hello world! This is my rust server".to_string()
}

#[actix_web::main]
async fn mymain()->std::io::Result<()>{
    let app_data = web::Data::new(AppState{
        todolist_entries: Mutex::new(vec![])
    });
    
    HttpServer::new(move || {
        App::new()
        .app_data(app_data.clone())
        .service(index)
        .configure(service::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn main(){
    mymain().unwrap();
}