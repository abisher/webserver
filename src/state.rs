use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).
        expect(&format!("Can't open file {}", file_name));

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();
    let state = json.as_object().unwrap().clone();
    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);

    fs::write(
        file_name.to_string(),
        new_data.to_string()
    ).expect("Unable to write file");
}


#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_it() {
        use serde::Deserialize;

        #[derive(Deserialize, Debug)]
        struct User {
            fingerprint: String,
            location: String,
        }


            // The type of `j` is `&str`
            let j = "
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

            let u: User = serde_json::from_str(j).unwrap();
            println!("{:#?}", u);

        let val = json!([(1,2), (1,2)]);
        let obj = val.as_array().unwrap();
        println!("{:?}", obj);
    }
}