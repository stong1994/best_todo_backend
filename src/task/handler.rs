use actix_web::{web, HttpRequest};
use autowired::Autowired;
use bson::oid::ObjectId;
use bson::Document;

use crate::task::service::TaskService;
use crate::common::*;

use super::{Task, TaskQuery};
use crate::common::service::MongodbCrudService;

const TASK_SERVICE: Autowired<TaskService> = Autowired::new(); // todo Autowired

pub async fn save_task(task: web::Json<Task>) -> RespResult {
    let task: Task = task.into_inner();
    let id = TASK_SERVICE.save(&task).await?;
    log::info!("save_task, id={}", id);
    Resp::ok(id).to_json_result()
}

pub async fn list_task() -> RespResult {
    // let query = query.into_inner();

    // // 构造查询参数
    // let mut filter: Document = doc! {};
    // if query._id.is_some() {
    //     filter.insert("_id", query._id.unwrap());
    // }

    // // 关键字模糊查询
    // if !query.keyword.is_empty() {
    //     filter.insert(
    //         "$or",
    //         bson::Bson::Array(vec![
    //             doc! {"title": {"$regex": & query.keyword, "$options": "i"}}.into(),
    //         ]),
    //     );
    // }

    // let list = TASK_SERVICE.list_with_filter(filter).await?;
    let list = TASK_SERVICE.list_all().await?;
    Resp::ok(list).to_json_result()
}

pub async fn update_task(req: HttpRequest, task: web::Json<Task>) -> RespResult {
    let id = req.match_info().get("id").unwrap_or("");

    let oid = ObjectId::with_string(id).map_err(|e| {
        log::error!("update_task, can't parse id to ObjectId, {:?}", e);
        BusinessError::ValidationError("id".to_owned())
    })?;

    let effect = TASK_SERVICE
        .update_by_oid(oid, &task.into_inner())
        .await?;
    log::info!("update task, id={}, effect={}", id, effect);

    Resp::ok(effect).to_json_result()
}

pub async fn remove_task(req: HttpRequest) -> RespResult {
    let id = req.match_info().get("id").unwrap_or("");
    if id.is_empty() {
        return Err(BusinessError::ValidationError("id".to_owned()));
    }

    let oid = ObjectId::with_string(id).map_err(|e| {
        log::error!("remove_task, can't parse id to ObjectId, {:?}", e);
        BusinessError::ValidationError("id".to_owned())
    })?;

    let deleted = TASK_SERVICE.remove_by_oid(oid).await?;
    log::info!("delete task, id={}, effect={}", id, deleted);

    Resp::ok(deleted).to_json_result()
}
