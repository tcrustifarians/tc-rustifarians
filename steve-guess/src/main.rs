
extern crate rand;

use std::io::Write;

fn main() {
  let ans = answer();

  println!("{:?}", ans);

  let mut string = return_guess("Enter your guess: ");

  println!("You entered {}", string);

  if "abc".to_string() == "abc".to_string() {
    println!("Now you know your abc's");
  }

  // The following causes a borrow error
  //   string.remove(string.len() - 1);
  // But the following two lines fix it
  let idx = string.len() - 1;
  string.remove(idx);

  println!("len is {}", string.len());

  if ans.to_string() == string {
    println!("You are right");
  }
}

fn return_guess(prompt:&str) -> String
{
  print!("{}", prompt);
  let _ = std::io::stdout().flush();

  let mut string = String::new();

  std::io::stdin().read_line(&mut string)
           .ok()
           .expect("Failed to read line");

  string
}


fn answer() -> i32
{
  let num = rand::random::<f32>();
  let ans: i32 = (num * 100.0).floor() as i32;
  ans
}

