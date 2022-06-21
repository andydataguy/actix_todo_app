use actix_web::dev::ServiceRequest;

/// TODO: Setup proper authentication systems later. 
/// Checks to see if the password from the token matches. If so it returns `Ok` if not 
/// then there is an error message.
///
/// # Parameters
/// * password (String): password to be checked
///
/// # Returns
/// * (Result<String, &'templates str>): password if correct, error message if not

fn check_password(password: String) -> Result<String, &'static str> {
  if password == "token" {
      return Ok(password)
  }
  return Err("token not authorized")
}

/// This function extracts a password from the `ServiceRequest` header. It then checks to ensure 
/// that the key exists and that it is in a `String` format.
///
/// ### Arguments
/// * request(&ServiceRequest): The request passed through the view function
///
/// ### Returns
/// * Result<String, &'static str>): Returns processed token if successful, 
/// and an error message if not.

fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {

  match request.headers().get("user-token") {
      Some(token) => {
          match token.to_str() {
              Ok(processed_password) => Ok(String::from(processed_password)),
              Err(_processed_password) => Err("there was an error processing token")
          }
      },
      None => Err("there is no token")
  }
}

/// This function checks to see if the token password is correct. If so it returns `Ok` 
/// and if not it returns an error message.
///
/// ### Arguments
/// * request(&ServiceRequest): The request passed through the view function
///
/// ### Returns
/// * Result<String, &'static str>): Returns `Ok` if successful or an error message if not.

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
  match extract_header_token(request) {
      Ok(token) => {
          match check_password(token) {
              Ok(token) => Ok(token),
              Err(message) => Err(message)
          }
      },
      Err(message) => Err(message)
  }
}