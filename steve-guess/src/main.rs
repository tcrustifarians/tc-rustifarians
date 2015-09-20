
mod simple_guessing_game;
mod guessing_game;

use simple_guessing_game::SimpleGuessingGame;
use guessing_game::GameOption;
use guessing_game::GuessingGame;


fn main() {
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


