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