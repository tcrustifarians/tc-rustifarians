use std::io::prelude::*;

extern crate rand;

use guessing_game::GuessingGame;
//use guessing_game::GameOption;

pub struct SimpleGuessingGame {
    answer: i32
}

impl SimpleGuessingGame {

    pub fn new() -> Self {
        SimpleGuessingGame {
            answer: SimpleGuessingGame::pick_answer()
        }
    }

    fn pick_answer() -> i32
    {
      let num = rand::random::<f32>();
      let ans = (num * 10.0).floor() as i32;
      ans
    }
}

impl GuessingGame for SimpleGuessingGame {

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


    fn get_answer(&self) -> i32
    {
        self.answer
    }

}

