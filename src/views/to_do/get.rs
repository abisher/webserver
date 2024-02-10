use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;
use crate::json_serialization::to_do_item::ToDoItems;
use crate::state::read_file;
use crate::to_do::{ItemTypes, to_do_factory, enums::TaskStatus};

pub async fn get() -> impl Responder {
    let state = read_file("./state.json");

    let mut array_buffer = Vec::new();

    for (key, value) in state {
        let status = TaskStatus::from_string(
            value.as_str().unwrap().to_string());

        let item = to_do_factory(&key, status);

        array_buffer.push(item);
    }

    let return_package = ToDoItems::new(array_buffer);

    web::Json(return_package)
}