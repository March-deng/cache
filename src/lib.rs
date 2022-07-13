pub mod cache;
mod config;
pub mod storage;

use actix_web::{web, App, HttpResponse, HttpServer};
pub use cache::{Cache, CacheError, Stat};

// define an error kind

// 启动一个web服务接口
pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{kind}/stat", web::get().to(get_stat))
            .route("/{kind}/key/{key}", web::get().to(get))
            .route("/{kind}/key/{key}", web::put().to(put))
            .route("/{kind}/key/{key}", web::delete().to(delete))
    })
    .bind("")
    .unwrap()
    .run()
    .await
}

async fn set_cache() {}

// TODO: add tracing log
async fn get_stat(kind: web::Path<String>) -> HttpResponse {
    todo!()
}

async fn get(path: web::Path<(String, String)>) -> HttpResponse {
    todo!()
}

async fn delete(path: web::Path<(String, String)>) -> HttpResponse {
    todo!()
}

async fn put(path: web::Path<(String, String)>) -> HttpResponse {
    todo!()
}

async fn batch_get() -> HttpResponse {
    todo!()
}

async fn batch_put() -> HttpResponse {
    todo!()
}

async fn batch_delete() -> HttpResponse {
    todo!()
}
