// #![allow(warnings)]

extern crate actix_web;
extern crate env_logger;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[allow(unused_imports)]
use actix_web::middleware::Logger;

use actix_web::{http::Method, server, App, HttpResponse};

mod models;
mod routes;

use crate::routes::todo;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();

    let url = "localhost:3000";
    // 展示不同狀態的 log
    info!("Running server on {}", url);
    error!("Running server on {}", url);
    warn!("Running server on {}", url);
    // debug!("Running server on {}", url);
    // trace!("Running server on {}", url);

    server::new(|| {
        App::new()
            // 啟用中介層日誌
            // .middleware(Logger::default())
            // .middleware(Logger::new("%a %{User-Agent}i"))
            // 使用前綴，後續網址接作用於該底下，同 .prefix("api")
            .prefix("/api")
            // 展示前綴
            .resource("", |r| r.get().f(|_| format!("Prefix With /api")))
            // 傳送 http 狀態
            .resource("ok", |r| r.get().f(|_| HttpResponse::Ok()))
            // 使用 Method::GET 判斷方法
            .resource("method", |r| {
                r.method(Method::GET).f(|_| HttpResponse::Ok())
            })
            // 使用 body 回傳純文字
            .resource("body", |r| r.get().f(|_| HttpResponse::Ok().body("body")))
            // 回傳 JSON Todo 物件
            .resource("/json", |r| r.get().with(todo::get_todo))
            // 展示不同方法請求、純文字回傳
            .resource("/text", |r| {
                r.get().f(|_| format!("GET"));
                r.post().f(|_| format!("POST"));
                r.put().f(|_| format!("PUT"));
                r.delete().f(|_| format!("DELETE"));
            })
            // 展示作用域 s1 底下的不同路由與不方法
            .scope("/s1", |route| {
                route
                    .resource("/v1", |r| r.get().f(|_| format!("s1 v1 get")))
                    .resource("/v2", |r| r.post().f(|_| format!("s1 v2 post")))
                    .resource("/v3", |r| r.put().f(|_| format!("s1 v3 put")))
                    .resource("/v4", |r| r.delete().f(|_| format!("s1 v4 delete")))
            })
            // 展示作用域 s2 底下同一個路由的不同方法
            .scope("/s2", |route| {
                route
                    .resource("/v1", |r| {
                        r.get().f(|_| format!("s2 v1 get"));
                        r.post().f(|_| format!("s2 v1 post"));
                    })
                    .resource("/v2", |r| {
                        r.put().f(|_| format!("s2 v2 put"));
                        r.delete().f(|_| format!("s2 v2 delete"));
                    })
            })
    })
    .bind(url)
    .unwrap()
    .run();
}
