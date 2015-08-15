use std::io::prelude::*;

extern crate rand;

use guessing_game::GuessingGame;
use guessing_game::GameOption;

pub struct SimpleGuessingGame;

impl SimpleGuessingGame {
    pub fn new() -> Self {
        SimpleGuessingGame
    }
}

impl GuessingGame for SimpleGuessingGame {

    fn evaluate_guess(&self, guess: i32, ans: i32) -> GameOption
    {
        match (guess - ans).abs() {
            1...2 => {
                println!("you are hot");
            },
            0 => {
                println!("You are right");
                return GameOption::GameDone;
            },
            _ => {
                println!("you are cold");
            },
        }
        
        GameOption::GameContinue
    }

    fn return_guess(&self, prompt:&str) -> Option<i32>
    {
      print!("{}", prompt);
      let _ = ::std::io::stdout().flush();

      let mut string = String::new();

      ::std::io::stdin().read_line(&mut string)
               .ok()
               .expect("Failed to read line");

      string.trim().parse::<i32>().ok()
    }


    fn answer(&self) -> i32
    {
      let num = rand::random::<f32>();
      let ans: i32 = (num * 10.0).floor() as i32;
      ans
    }
}

