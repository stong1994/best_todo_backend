mod api;
mod model;
mod repo;

#[macro_use]
extern crate rocket;

//add imports below
use api::task::{create_task, delete_task, get_all_tasks, get_task, update_task};
use repo::mongo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![create_task])
        .mount("/", routes![get_task])
        .mount("/", routes![update_task])
        .mount("/", routes![delete_task])
        .mount("/", routes![get_all_tasks])
}
