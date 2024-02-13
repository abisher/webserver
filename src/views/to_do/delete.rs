use actix_web::{HttpResponse, HttpRequest};
use serde_json::value::Value;
use serde_json::Map;


use crate::json_serialization::{to_do_item::ToDoItem,
                                to_do_items::ToDoItems};
use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::state::read_file;
use crate::processes::process_input;
use crate::jwt::JwToken;



async pub fn delete()