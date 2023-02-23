use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Error getting task's detail: {0}")]
    MongoDBError(#[from] mongodb::error::Error),

    #[error("Task not found")]
    NotFound,
}