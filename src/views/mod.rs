use actix_web::web;
mod path; 
mod auth;
mod to_do;
mod app;
pub mod token;

/// This function acts as an entry point to connect the HTTP server with the 
/// views defined in this application. It calls a series of functions that adds 
/// the auth views and the to do items views.
///
/// ### Arguments
/// * app(&mut web::ServiceConfig): an exclusive reference to the HTTPserver for configuration
///
/// ### Returns
/// None

pub fn views_factory(app: &mut web::ServiceConfig) {
  auth::auth_factory(app);
  to_do::item_factory(app);
  app::app_factory(app);
}