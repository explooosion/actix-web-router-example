extern crate serde_derive;
use actix_web::{HttpRequest, Json, Result};

#[allow(unused_imports)]
use crate::models::todo::Todo;

pub fn get_todo(_req: HttpRequest) -> Result<Json<Vec<Todo>>> {
    Ok(Json(vec![
        Todo {
            id: 0,
            item: "Study".to_owned(),
            done: true,
        },
        Todo {
            id: 1,
            item: "Work".to_owned(),
            done: false,
        },
    ]))
}

#[warn(dead_code)]
pub fn create_todo(todo: Json<Todo>) -> Result<Json<Todo>> {
    Ok(todo)
    // Ok(Json(Todo {
    //     id: 1,
    //     item: "Work".to_owned(),
    //     done: false,
    // }))
    // req.urlencoded::<Todo>() // <- get UrlEncoded future
    //     .from_err()
    //     .and_then(|data| {
    //         // <- deserialized instance
    //         println!("Item: {:?}", data.item);
    //         ok(HttpResponse::Ok().into())
    //     })
    //     .responder()
}

pub fn update_todo(req: HttpRequest) -> Result<Json<Todo>> {
    unimplemented!()
}

pub fn delete_todo(req: HttpRequest) -> Result<Json<Todo>> {
    unimplemented!()
}
