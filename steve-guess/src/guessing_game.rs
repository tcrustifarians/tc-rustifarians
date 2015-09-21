use std::io::prelude::*;

pub enum GameOption {
    GameDone,
    GameContinue,
}

pub trait GuessingGame {

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

    fn display_prompt_and_get_response(prompt:&str) -> String
    {
      print!("{}", prompt);
      let _ = ::std::io::stdout().flush();

      let mut string = String::new();

      ::std::io::stdin().read_line(&mut string)
               .ok()
               .expect("Failed to read line");

      string
    }

    fn return_guess(&self, prompt:&str) -> Option<i32>;
    fn get_answer(&self) -> i32;

    fn after_guess(&self)
    {
    }
}

