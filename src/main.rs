use axum::{routing::{delete, get, post, put}, Router,response::{Json},extract::State};
use serde::{Serialize,Deserialize};
use serde_json::{json, Value};
use std::sync::{Arc};

#[derive(Serialize,Deserialize,Debug)]
struct Note{
    title:String,
    author:String,
    created_at:String
}

#[derive(Debug,Serialize,Deserialize)]
struct AppState{
    notes:Vec<Note>
}
#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    let notes=Arc::new(AppState{
        notes:Vec::new()
    });

    let app=Router::new()
    .route("/", get(hello_world))
    .route("/notes",get(fetch_notes)).with_state(Arc::clone(&notes))
    .route("/add-note",post(add_note)).with_state(Arc::clone(&notes))
    .route("/update-note",put(update_note))
    .route("/delete-note",delete(delete_note));


    let listner=tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    
    axum::serve(listner,app).await.unwrap();
    
}


async fn hello_world()->&'static str{
println!("hello world");
"hello world"
}
async fn fetch_notes(State(state):State<Arc<AppState>>)->Json<Value>{
println!("42...fetch notes: {:?}",state);
let data=&*state;
Json(json!({"data":data,"message":"all notes fetched Successfull."}))
}
async fn add_note(){
    
}
async fn update_note(){
    
}
async fn delete_note(){
    
}