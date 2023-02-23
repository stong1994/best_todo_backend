use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::model::task::Task;
use crate::model::error::MyError;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Task>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        // let uri = match env::var("MONGOURI") {
        //     Ok(v) => v.to_string(),
        //     Err(_) => format!("Error loading env variable"),
        // };
        let uri = "mongodb://127.0.0.1";
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("best_todo");
        let col: Collection<Task> = db.collection("task");
        MongoRepo { col }
    }

    pub fn create_task(&self, new_task: Task) -> Result<InsertOneResult, Error> {
        let new_doc = Task {
            id: None,
            title: new_task.title,
            is_done: new_task.is_done.to_owned(),
            is_important: new_task.is_important,
            is_urgent: new_task.is_urgent,
        };
        let task = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating task");
        Ok(task)
    }

    pub fn get_task(&self, id: &String) -> Result<Task, MyError> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let task_detail = self
            .col
            .find_one(filter, None)
            .map_err(MyError::MongoDBError)?.ok_or(MyError::NotFound)?;
            // .ok()
            // .expect("Error getting task's detail");
        Ok(task_detail)
    }

    pub fn get_task_by_objid(&self, obj_id: ObjectId) -> Result<Task, Error> {
        let filter = doc! {"_id": obj_id};
        let task_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting task's detail");
        Ok(task_detail.unwrap())
    }

    pub fn update_task(&self, id: &String, new_task: Task) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_task.id,
                    "is_done": new_task.is_done,
                    "is_urgent": new_task.is_urgent,
                    "is_important": new_task.is_important,
                    "title": new_task.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating task");
        Ok(updated_doc)
    }

    pub fn delete_task(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let task_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting task");
        Ok(task_detail)
    }

    pub fn get_all_tasks(&self) -> Result<Vec<Task>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of tasks");
        let tasks = cursors.map(|doc| doc.unwrap()).collect();
        Ok(tasks)
    }
}
