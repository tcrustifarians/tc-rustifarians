use std::io;
use std::process::exit;
use std::collections::hash_map::HashMap;

pub fn error(s: &str) -> ! {
    panic!("Error: {}.", s);
}

pub fn expected(s: &str) -> ! {
    error(format!("{} expected", s).as_str());
}

pub fn is_alpha(token: char) -> bool {
    match token.to_uppercase().next().unwrap() {
        'A' ... 'Z' => true,
        _           => false
    }
}

#[allow(dead_code)]
pub fn is_digit(token: char) -> bool {
    match token {
        '0' ... '9' => true,
        _           => false
    }
}

#[allow(dead_code)]
pub fn is_alnum(token: char) -> bool {
    is_alpha(token) || is_digit(token)
}

pub fn is_whitespace(token: char) -> bool {
    match token {
        ' ' | '\t' => true,
        _          => false
    }
}

#[allow(dead_code)]
pub fn is_add_op(token: char) -> bool {
    match token {
        '+' | '-' => true,
        _         => false
    }
}

pub struct ParseState {
    pub token: char,
    tokens: io::Chars<io::Stdin>,
    pub var_table: HashMap<String, i64>,
    label_count: i64
}

impl ParseState {
    pub fn new(char_stream: io::Chars<io::Stdin>) -> ParseState {
        let mut ps = ParseState {
            tokens: char_stream,
            token: '\0',
            var_table: HashMap::new(),
            label_count: 0
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

    #[allow(dead_code)]
    pub fn get_num(&mut self) -> String {
        if !is_digit(self.token) {
            expected("Integer");
        }
        let mut num = String::from("");
        while is_digit(self.token) {
            num = format!("{}{}", num, self.token);
            self.advance();
        }
        self.skip_whitespace();
        num
    }

    pub fn get_name(&mut self) -> String {
        if !is_alpha(self.token) {
            expected("Name");
        }
        let name: String = format!("{}", self.token);
        self.advance();
        self.skip_whitespace();
        name
    }

    pub fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_count);
        self.label_count += 1;
        label
    }
}
