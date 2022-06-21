use super::content_loader::read_file;
use actix_web::HttpResponse;
use super::content_loader::add_component;

/// Renders the main app view showing all items saved in the state. 
/// 
/// ### Arguments
/// * 
/// 
/// ### Returns
/// * (HttpResponse) with HTML content from file

pub async fn items() -> HttpResponse {

  let mut html_data = read_file(
    String::from("./templates/main.html"));
  let javascript_data: String = read_file(
    String::from("./javascript/main.js"));
  let css_data: String = read_file(
    String::from("./css/main.css"));
  let base_css_data: String = read_file(
    String::from("./css/base.css"));

// Loads a javascript file into the {{JAVASCRIPT}} variable in the HTML template
html_data = html_data.replace("{{JAVASCRIPT}}", &javascript_data);

// Loads a CSS file into the {{CSS}} variable in the HTML template
html_data = html_data.replace("{{CSS}}", &css_data);

// Loads a CSS file into the {{BASE_CSS}} variable in the HTML template
html_data = html_data.replace("{{BASE_CSS}}", &base_css_data);

// Loads inherited CSS styling through the HEADER_CSS section of the HTML file
html_data = add_component(String::from("header"), html_data);

HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html_data)
}