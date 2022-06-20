use actix_web::web;
use super::path::Path;
mod items;
mod content_loader;

/// This factory function is our main URL get function for app users.
/// This function adds our views to the web server serving HTML
///
/// ### Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// ### Returns
/// None

pub fn app_factory(app: &mut web::ServiceConfig) {
  
    // define the path struct
    let base_path: Path = Path{prefix: String::from("/")};
    // define the routes for the app
    app.route(&base_path.define(String::from("")),
              web::get().to(items::items));
}
