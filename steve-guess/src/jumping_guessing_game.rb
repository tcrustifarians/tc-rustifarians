use std::io::prelude::*;

extern crate rand;

use guessing_game::GuessingGame;
use guessing_game::GameOption;

pub struct JumpingGuessingGame;

impl JumpingGuessingGame {
    pub fn new() -> Self {
        JumpingGuessingGame
    }
}

impl GuessingGame for JumpingGuessingGame {

}
