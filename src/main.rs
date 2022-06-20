use actix_web::{App, HttpServer};
use actix_service::Service;

mod state;
mod to_do;
mod views;
mod json_serialization;
mod processes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

  // Creates a new instance of an HttpServer
  HttpServer::new(|| {

    // saves the new App instance as a variable for later use
    let app = App::new()

      // wraps token authorization middleware to each request to ensure users are logged in
      .wrap_fn(|req, srv| { 

        // limits token authorization for requests containing the `/item/` path
        if *&req.path().contains("/item/") {

          // calls the process_token function from our token file and returns either success or  error
          match views::token::process_token(&req) {
            Ok(_token) => println!("Success! The token is approved"),
            Err(message) => println!("Token error: {}", message),
          }
        }

        // processes the request and allows response to be returned asynchronously
        let fut = srv.call(req);
        async {
          
          let result = fut.await?;
          Ok(result)
        }

      // Connects the new HTTPserver with the views defined in this application
      }).configure(views::views_factory);
    return app
  })
    
    // Starts the HTTP server
    .bind("127.0.0.1:8000")?
    .run()
    .await
}