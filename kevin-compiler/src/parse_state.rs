use std::io;
use std::process::exit;

pub fn expected(s: &str) -> ! {
    panic!("{} expected", s);
}

pub fn is_alpha(token: char) -> bool {
    match token.to_uppercase().next().unwrap() {
        'A' ... 'Z' => true,
        _           => false
    }
}

pub fn is_digit(token: char) -> bool {
    match token {
        '0' ... '9' => true,
        _           => false
    }
}

pub fn is_alnum(token: char) -> bool {
    is_alpha(token) || is_digit(token)
}

pub fn is_whitespace(token: char) -> bool {
    match token {
        ' ' | '\t' => true,
        _          => false
    }
}

pub fn is_add_op(token: char) -> bool {
    match token {
        '+' | '-' => true,
        _         => false
    }
}

pub struct ParseState {
    pub token: char,
    tokens: io::Chars<io::Stdin>
}

impl ParseState {
    pub fn new(char_stream: io::Chars<io::Stdin>) -> ParseState {
        let mut ps = ParseState {
            tokens: char_stream,
            token: '\0'
        };
        ps.advance();
        ps.skip_whitespace();
        ps
    }

    pub fn advance(&mut self) {
        self.token = match self.tokens.next() {
            None         => exit(0),
            Some(Err(_)) => panic!("uh-oh"),
            Some(Ok(c))  => c
        }
    }

    pub fn skip_whitespace(&mut self) {
        while is_whitespace(self.token) {
            self.advance();
        }
    }

    pub fn consume(&mut self, target: char) {
        if self.token != target {
            expected(format!("'{}'", target).as_str());
        }
        self.advance();
        self.skip_whitespace();
    }

    pub fn get_num(&mut self) -> i64 {
        if !is_digit(self.token) {
            expected("Integer");
        }
        let mut num: i64 = 0;
        while is_digit(self.token) {
            num = match self.token.to_digit(10) {
                Some(i) => 10 * num + (i as i64),
                None    => expected("Digit")
            };
            self.advance();
        }
        self.skip_whitespace();
        num
    }

    #[allow(dead_code)]
    fn get_name(&mut self) -> String {
        if !is_alpha(self.token) {
            expected("Name");
        }
        let mut name: String = "".to_string();
        while is_alnum(self.token) {
            name = format!("{}{}", name, self.token);
            self.advance();
        }
        self.skip_whitespace();
        name
    }
}
