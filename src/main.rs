use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: String,
    title: String,
    is_done: bool,
    is_important: bool,
    is_urgent: bool,
}

impl Task {
    fn new(title: String, is_important: bool, is_urgent: bool) -> Task {
        Task {
            id: Uuid::new_v4().to_string(),
            title,
            is_done: false,
            is_important,
            is_urgent,
        }
    }
}

#[derive(Debug)]
struct TaskData {
    tasks: Vec<Task>,
}

impl TaskData {
    fn new() -> TaskData {
        TaskData { tasks: vec![] }
    }

    fn get_tasks(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    fn add_task(&mut self, title: String, is_important: bool, is_urgent: bool) -> Task {
        let task = Task::new(title, is_important, is_urgent);
        self.tasks.push(task.clone());
        task
    }

    fn update_task(&mut self, task: &Task) -> Option<Task> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == task.id) {
            self.tasks[index] = task.clone();
            Some(task.clone())
        } else {
            None
        }
    }

    fn delete_task(&mut self, id: &str) -> Option<Task> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct AppState {
    task_data: Arc<Mutex<TaskData>>,
}

#[get("/tasks")]
async fn get_tasks(state: web::Data<AppState>) -> impl Responder {
    let task_data = state.task_data.lock().unwrap();
    let tasks = task_data.get_tasks();
    HttpResponse::Ok().json(tasks)
}

#[post("/tasks")]
async fn add_task(state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut task_data = state.task_data.lock().unwrap();
    let new_task = task_data.add_task(task.title.clone(), task.is_important, task.is_urgent);
    HttpResponse::Ok().json(new_task)
}

#[post("/tasks/{id}")]
async fn update_task(state: web::Data<AppState>, task: web::Json<Task>, id: web::Path<String>) -> impl Responder {
    let mut task_data = state.task_data.lock().unwrap();
    let mut task = task.into_inner();
    task.id = id.into_inner();
    if let Some(updated_task) = task_data.update_task(&task) {
        HttpResponse::Ok().json(updated_task)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("/tasks/{id}/done")]
async fn complete_task(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let mut task_data = state.task_data.lock().unwrap();
    if let Some(task) = task_data.tasks.iter_mut().find(|t| t.id == id.to_string()) {
        task.is_done = true;
        HttpResponse::Ok().json(task)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let task_data = Arc::new(Mutex::new(TaskData::new()));
    println!("Server listening on {}", addr);
    HttpServer::new(move || App::new()
    .data(AppState{
        task_data: task_data.clone(),
    })
    .service(get_tasks).service(add_task).service(update_task).service(complete_task))
        .bind(addr)?
        .run()
        .await
}