use actix_web::{ web, App, HttpServer };
use actix_web::dev::Service;
/*
 * @Author: yowayimono
 * @Date: 2024-10-14 14:24:07
 * @LastEditors: yowayimono
 * @LastEditTime: 2024-10-16 18:33:38
 * @Description: nothing
 */
use lazy_static::lazy_static;
mod utils;
mod config;
mod models;
mod pkg;
mod handler;
use log::{info};
use pkg::db::MongoConnection;
use handler::user_handler;
use actix_web::middleware::Logger;
use config::AppConf;

mod router;

use router::routes;

lazy_static! {
    pub static ref GLOBAL_CONFIG: AppConf = AppConf::new("./conf.toml");
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_connection = MongoConnection::new().await.expect("Failed to connect to MongoDB");

    env_logger::init();
    info!("Starting Http Server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(routes::user_scope(web::Data::new(mongo_connection.clone())))
    })
    .bind(
        format!(
            "{}:{}",
            GLOBAL_CONFIG.ip.as_deref().unwrap(),
            GLOBAL_CONFIG.port.unwrap_or(8081)
        )
    )?
    .run().await
}