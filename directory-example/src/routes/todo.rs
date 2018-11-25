use actix_web::{HttpRequest, Json, Result};

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
