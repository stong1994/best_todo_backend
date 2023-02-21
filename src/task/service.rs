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
        self.mongodb
            .database(DB_NAME)
            .collection(Task::TABLE_NAME)
    }

}
