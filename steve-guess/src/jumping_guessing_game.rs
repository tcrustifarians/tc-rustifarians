extern crate rand;

use guessing_game::GuessingGame;
use user_io::display_prompt_and_get_response;

pub struct JumpingGuessingGame {
    answer: i32
}

impl JumpingGuessingGame {
    pub fn new() -> Self {
        JumpingGuessingGame {
            answer: 0
        }
    }
}

impl GuessingGame for JumpingGuessingGame {
    fn get_guess_from_user(&self, prompt: &str) -> Option<i32> {
        let response = display_prompt_and_get_response(prompt); 
        response.trim().parse::<i32>().ok()
    }

    fn get_answer(&self) -> i32
    {
        self.answer
    }

    fn after_guess(&mut self)
    {
        self.answer = self.answer + 1
    }
}
