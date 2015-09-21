extern crate rand;

use guessing_game::GuessingGame;
use guessing_game::GameOption;

pub struct JumpingGuessingGame {
  answer: i32
}

impl JumpingGuessingGame {
    pub fn new() -> Self {
        JumpingGuessingGame
    }
}

impl GuessingGame for JumpingGuessingGame {

}
