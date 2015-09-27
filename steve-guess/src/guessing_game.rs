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

    fn get_guess_from_user(&self, prompt:&str) -> Option<i32>;
    fn get_answer(&self) -> i32;

    fn after_guess(&mut self)
    {
    }
}

