extern crate rand;

use guessing_game::GuessingGame;
use user_io::display_prompt_and_get_response;


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

    fn get_guess_from_user(&self, prompt:&str) -> Option<i32>
    {
        let response = display_prompt_and_get_response(prompt); 
        response.trim().parse::<i32>().ok()
    }


    fn get_answer(&self) -> i32
    {
        self.answer
    }

}

