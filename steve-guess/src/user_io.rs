use std::io::prelude::*;

pub fn display_prompt_and_get_response(prompt:&str) -> String
{
  print!("{}", prompt);
  let _ = ::std::io::stdout().flush();

  let mut string = String::new();

  ::std::io::stdin().read_line(&mut string)
           .ok()
           .expect("Failed to read line");

  string
}
