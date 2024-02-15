use std::fs;

pub fn read_file(file_path: &str) -> String {
    let data = fs::read_to_string(file_path)
        .expect("Unable to read file");
    data
}

pub fn add_component(component_tag: String, html_data: String) -> String {
    let css_tag = component_tag.to_uppercase() + "_CSS";
    let html_tag = component_tag.to_uppercase() + "_HTML";
}