
mod simple_guessing_game;
mod guessing_game;

use simple_guessing_game::SimpleGuessingGame;
use jumping_guessing_game::JumpingGuessingGame;
use guessing_game::GameOption;
use guessing_game::GuessingGame;


fn main() {

    let response = SimpleGuessingGame::display_prompt_and_get_response("Select game type, 1) Simple, 2) Jumping: ");
    if response == "1" {
        let simple_game = SimpleGuessingGame::new();
    }
    else {
        let jumping_game = JumpingGuessingGame::new();
    }

    let game = SimpleGuessingGame::new();

    println!("Answer: {}", game.get_answer());

    loop {
        match game.return_guess("Enter your guess: ") {
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


