#[macro_use]
extern crate bson;
#[macro_use]
extern crate anyhow;

// use crate::task::task;
use crate::{common::*, task::Task};
use actix_web::{web, App, FromRequest, HttpServer};

mod task;
mod common;
mod middleware;

const DEFAULT_CONFIG_FILE: &str = "config.yml";
const CONFIG_FILE_ENV: &str = "MYBLOG_CONFIG";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    common::init_logger();

    let config = task_config();
    log::info!("[load_config] {:?}", config);

    actix_web::web::block(|| Result::<(), ()>::Ok(autowired::setup_submitted_beans())).await?;
    log::info!("[beans] loaded: {:?}", autowired::list_bean_names());

    let binding_address = format!("{}:{}", config.host, config.port);
    HttpServer::new(|| {
        App::new()
            .app_data(web::Json::<Task>::configure(|cfg| {
                cfg.error_handler(|err, req| {
                    log::error!("json extractor error, path={}, {}", req.uri(), err);
                    BusinessError::ArgumentError.into()
                })
            }))
            .service(
                web::scope("/tasks")
                    .route("", web::get().to(task::list_task))
                    .route("", web::post().to(task::save_task))
                    .route("{id}", web::put().to(task::update_task))
                    .route("{id}", web::delete().to(task::remove_task)),
            )
    })
    .bind(&binding_address)
    .expect(&format!("Can not bind to {}", binding_address))
    .run()
    .await?;
    Ok(())
}
