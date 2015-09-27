
mod jumping_guessing_game;
mod simple_guessing_game;
mod guessing_game;
mod user_io;

use simple_guessing_game::SimpleGuessingGame;
use jumping_guessing_game::JumpingGuessingGame;
use guessing_game::GameOption;
use guessing_game::GuessingGame;
use user_io::display_prompt_and_get_response;

fn main() {

    let mut game: Box<GuessingGame>;

    let response = display_prompt_and_get_response("Select game type, 1) Simple, 2) Jumping: ");
    if response == "1" {
        game = Box::new(SimpleGuessingGame::new());
    }
    else {
        game = Box::new(JumpingGuessingGame::new());
    }

    println!("Answer: {}", game.get_answer());

    loop {
        match game.get_guess_from_user("Enter your guess: ") {
            Some(guess) => {
                match game.evaluate_guess(guess, game.get_answer()) { 
                    GameOption::GameDone => {
                        break;
                    },
                    GameOption::GameContinue => {}
                }
            },
            None        => {
                println!("Invalid guess. It is not a number");
            }
        }
        game.after_guess()
    }
}


