// #![allow(warnings)]
extern crate actix_web;
extern crate env_logger;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[allow(unused_imports)]
use actix_web::middleware::Logger;
use actix_web::{server, App};

mod models;
mod routes;

use crate::routes::todo;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();

    let url = "localhost:3000";
    info!("Running server on {}", url);

    server::new(|| {
        App::new()
            // 啟用中介層日誌
            .middleware(Logger::default())
            .middleware(Logger::new("%a %{User-Agent}i"))
            // 回傳 JSON Todo 物件
            .resource("/todo", |r| {
                r.get().with(todo::get_todo);
                r.post().with(todo::create_todo);
                r.put().with(todo::update_todo);
                r.delete().with(todo::delete_todo);
            })
    })
    .bind(url)
    .unwrap()
    .run();
}
