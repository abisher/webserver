use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;
use crate::state::read_file;
use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::json_serialization::{to_do_item::ToDoItem,
                                to_do_items::ToDoItems};

use crate::processes::process_input;


pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state = read_file("./state.json");

    let status: TaskStatus;

    match state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::new(result.as_str().unwrap());
        }

        None => {
            HttpResponse::NotFound().json(
                format!("{} is not in state", &to_do_item.title)
            )
        }
    }
}