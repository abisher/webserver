use std::fs;

pub fn read_file(file_path: &str) -> String {
    let data = fs::read_to_string(file_path)
        .expect("Unable to read file");
    data
}

pub fn add_component(component_tag: String, html_data: String) -> String {
    let css_tag = component_tag.to_uppercase() + "_CSS";
    let html_tag = component_tag.to_uppercase() + "_HTML";
    let ccs_path = String::from("./templates/components/") +
        &component_tag.to_lowercase() + ".css";
    let css_loaded = read_file(&ccs_path);
    let html_path = String::from("./templates/components/") +
        &component_tag.to_lowercase() + ".html";
    let html_loaded = read_file(&html_path);

    let html_data = html_data.replace(html_tag.as_str(), &html_loaded)
        .replace(css_tag.as_str(), &css_loaded);

    html_data
}