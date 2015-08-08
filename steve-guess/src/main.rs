
extern crate rand;

use std::io::Write;

fn main() {
    let ans = answer();

    println!("Answer: {}", ans);

    loop {
        match return_guess("Enter your guess: ") {
            Some(guess) => {
                if guess == ans {
                    println!("You are right");
                    break;
                }
            },
            None        => {
                println!("Invalid guess. It is not a number");
            }
        }
    }
}

fn return_guess(prompt:&str) -> Option<i32>
{
  print!("{}", prompt);
  let _ = std::io::stdout().flush();

  let mut string = String::new();

  std::io::stdin().read_line(&mut string)
           .ok()
           .expect("Failed to read line");

  string.trim().parse::<i32>().ok()
}


fn answer() -> i32
{
  let num = rand::random::<f32>();
  let ans: i32 = (num * 10.0).floor() as i32;
  ans
}

