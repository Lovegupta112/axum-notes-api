use axum::{
    Router,
    extract::{Json, Path, State},
    response,
    routing::{delete, get, post, put},
};
use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use core::panic;
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    id: Option<usize>,
    title: String,
    author: String,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    notes: Vec<Note>,
}
#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    let notes = Arc::new(RwLock::new(AppState { notes: Vec::new() }));

    let app = Router::new()
        .route("/", get(server_health))
        .route("/notes", get(fetch_notes))
        .with_state(Arc::clone(&notes))
        .route("/add-note", post(add_note))
        .with_state(Arc::clone(&notes))
        .route("/update-note", put(update_note))
        .with_state(Arc::clone(&notes))
        .route("/delete-note/{noteId}", delete(delete_note))
        .with_state(Arc::clone(&notes));

    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listner, app).await.unwrap();
}

async fn server_health() -> &'static str {
    println!("Server is healthy..");
    "Server is Healthy.."
}
async fn fetch_notes(State(state): State<Arc<RwLock<AppState>>>) -> Json<Value> {
    println!("fetch notes: {:?}", state);
    let data = &*state.read().unwrap();
    response::Json(json!({"data":data,"message":"all notes fetched Successfull."}))
}
async fn add_note(State(state): State<Arc<RwLock<AppState>>>, Json(payload): Json<Note>) -> String {
    let mut app_state = state.write().unwrap();
    let mut new_note = payload;
    let new_note_id: usize = app_state.notes.len();
    new_note.id = Some(new_note_id);
    new_note.created_at = Some(Utc::now().to_string());
    app_state.notes.push(new_note);
    "Successfully added !".to_string()
}
async fn update_note(State(state): State<Arc<RwLock<AppState>>>, Json(data): Json<Note>) -> String {
    let mut app_state = state.write().unwrap();

    if data.id.is_none() {
        panic!("Note id is required..");
    }
    let updated_note_id: usize = data.id.unwrap();
    
    if data.author.ne(&app_state.notes[updated_note_id].author) {
       panic!("Author should be same..");
    }

    app_state.notes[updated_note_id].title = data.title;
    app_state.notes[updated_note_id].updated_at = Some(Utc::now().to_string());

    "Note updated successfully.".to_string()
}
async fn delete_note(State(state): State<Arc<RwLock<AppState>>>, Path(id): Path<usize>) -> String {
    let mut app_state = state.write().unwrap();

    app_state.notes.retain(|obj| obj.id.unwrap() != id);
    format!("Note {id} deleted successfully.")
}
