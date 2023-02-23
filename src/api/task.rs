use std::str::FromStr;

use crate::{model::task::Task, repo::mongo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/task", data = "<new_task>")]
pub fn create_task(
    db: &State<MongoRepo>,
    new_task: Json<Task>,
) -> Result<Json<Task>, Status> {
    let data = Task {
        id: None,
        title: new_task.title.to_owned(),
        is_done: new_task.is_done.to_owned(),
        is_urgent: new_task.is_urgent.to_owned(),
        is_important: new_task.is_important.to_owned(),
    };
    let task_detail = db.create_task(data);
    match task_detail {
        Ok(id) => {
            let task_detail = db.get_task_by_objid(id.inserted_id.as_object_id().unwrap().clone());
            match task_detail {
                Ok(task) => Ok(Json(task)),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/task/<id>")]
pub fn get_task(db: &State<MongoRepo>, id: String) -> Result<Json<Task>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let task_detail = db.get_task(&id);
    match task_detail {
        Ok(task) => Ok(Json(task)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/task/<id>", data = "<new_task>")]
pub fn update_task(
    db: &State<MongoRepo>,
    id: String,
    new_task: Json<Task>,
) -> Result<Json<Task>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Task {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        title: new_task.title.to_owned(),
        is_done: new_task.is_done.to_owned(),
        is_important: new_task.is_important.to_owned(),
        is_urgent: new_task.is_urgent.to_owned(),
    };
    let update_result = db.update_task(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_task_info = db.get_task(&id);
                return match updated_task_info {
                    Ok(task) => Ok(Json(task)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/task/<id>")]
pub fn delete_task(db: &State<MongoRepo>, id: String) -> Result<Json<&str>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_task(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("task successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/tasks")]
pub fn get_all_tasks(db: &State<MongoRepo>) -> Result<Json<Vec<Task>>, Status> {
    let tasks = db.get_all_tasks();
    match tasks {
        Ok(tasks) => Ok(Json(tasks)),
        Err(_) => Err(Status::InternalServerError),
    }
}
