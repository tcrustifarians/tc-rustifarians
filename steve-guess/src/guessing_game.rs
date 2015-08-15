pub enum GameOption {
    GameDone,
    GameContinue,
}

pub trait GuessingGame {
    fn evaluate_guess(&self, guess: i32, ans: i32) -> GameOption;
    fn return_guess(&self, prompt:&str) -> Option<i32>;
    fn answer(&self) -> i32;
}

