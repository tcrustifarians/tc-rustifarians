
mod simple_guessing_game;

use simple_guessing_game::SimpleGuessingGame;
use simple_guessing_game::GameOption;

use simple_guessing_game::GuessingGame;


fn main() {
    let game = SimpleGuessingGame::new();

    let ans = game.answer();

    println!("Answer: {}", ans);

    loop {
        match game.return_guess("Enter your guess: ") {
            Some(guess) => {
                match game.evaluate_guess(guess, ans) { 
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
    }
}


