use super::content_loader::read_file;
use actix_web::HttpResponse;

/// Renders the main app view showing all items saved in the state. 
/// 
/// ### Arguments
/// * 
/// 
/// ### Returns
/// * (HttpResponse) with HTML content from file

pub async fn items() -> HttpResponse {

  // Loads a javascript file into the {{JAVASCRIPT}} variable in the HTML template
  // Loads the javascript and HTML files into a variable to be manipulated in the function
  let mut html_data = read_file(String::from("./templates/main.html"));
  let javascript_data = read_file(String::from("./javascript/main.js"));

  // Replaces the {{JAVASCRIPT}} variable in the HTML template with the text from our javascripot/main.rs file
  html_data = html_data.replace("{{JAVASCRIPT}}", &javascript_data);

  HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(html_data)
}
