use crate::task::Task;
use crate::common::service::MongodbCrudService;
use crate::middleware::mongodb::DB_NAME;
use autowired::Autowired;
use mongodb::Collection;

#[derive(Default, autowired::Component)]
pub struct TaskService {
    mongodb: Autowired<mongodb::Client>,
}

impl MongodbCrudService<Task> for TaskService {

    fn table(&self) -> Collection {
        let col = self.mongodb
            .database(DB_NAME)
            .collection(Task::TABLE_NAME);
        let name = col.name();
        println!("{}", name);
        col

        // let col = self.mongodb
        //     .database(DB_NAME)
        //     .collection(Task::TABLE_NAME);
        // // let db = client.database("best_todo");

        // // Get a handle to the "task" collection.
        // // let collection = db.collection("task");

        // // Insert a document into the "task" collection.
        // add_task_to_collection(col).await?;
        // col
    }
}

// async fn add_task_to_collection(collection: &Collection) -> Result<()> {
//     let document = doc! { "title": "Learn Rust", "is_done": false, "is_important": true, "is_urgent": true };
//     collection.insert_one(document, None).await?;
//     Ok(())
// }
