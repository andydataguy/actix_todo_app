use std::fs;

/// This function is a basic content loader that will use the standard filesystem library
/// to pass content from the given file into the response (in this case an HTML file)
///
/// ### Arguments
/// * file_path(String): Path to the desired file to be loaded
///
/// ### Returns: Return a string of the file contents

pub fn read_file(file_path: String) -> String {

  // function that calls the filesystem library to read the input file
  let data: String = fs::read_to_string(
      file_path).expect("Unable to read file");
  return data
}

/// This function takes the name of the component, creates tags from the component name, 
/// and loads the HTML and CSS based on the components name
///
/// ### Arguments
/// * component_tag(String): CSS component tag to be added to the HTML response
/// * html_data(String): The HTML file to be modified
///
/// ### Returns: Return a string of the file contents

pub fn add_component(component_tag: String, html_data: String) -> String {

  // Save component name from files into a variable of type String
  let css_tag: String = component_tag.to_uppercase() + &String::from("_CSS");
  let html_tag: String = component_tag.to_uppercase() + &String::from("_HTML");

  // Loads the CSS file from our library and saves it into a variable for later use
  let css_path = String::from("./templates/components/") + &component_tag.to_lowercase() + &String::from(".css");
  let css_loaded = read_file(css_path);

  // Loads a HTML file from our library and saves it into a variable for later use
  let html_path = String::from("./templates/components/") + &component_tag.to_lowercase() + &String::from(".html");
  let html_loaded = read_file(html_path);

  // replaces the css tags with the previously loaded CSS and HTML files
  let html_data = html_data.replace(html_tag.as_str(), &html_loaded);
  let html_data = html_data.replace(css_tag.as_str(), &css_loaded);
  return html_data

}