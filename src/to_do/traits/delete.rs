use serde_json::Map;
use serde_json::value::Value;
use std::env;

use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &String,
              state: &mut Map<String, Value>) {
        dotenv::dotenv().ok();
        let json_file = env::var("JSON_FILENAME")
            .expect("JSON filename not provided");
        state.remove(title);
        write_to_file(&json_file, state);
        println!("\n\n{} is being deleted", title);
    }
}