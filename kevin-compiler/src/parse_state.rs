use std::io;
use std::process::exit;
use std::collections::hash_map::HashMap;

pub fn error(s: &str) -> ! {
    panic!("Error: {}.", s);
}

pub fn expected(s: &str) -> ! {
    error(&format!("{} expected", s));
}

pub fn is_whitespace(token: char) -> bool {
    match token {
        ' ' | '\t' => true,
        _          => false
    }
}

pub fn is_not_op(token: char) -> bool {
    token == '!'
}

pub fn is_or_op(token: char) -> bool {
    match token {
        '|' | '~' => true,
        _         => false
    }
}

pub fn is_and_op(token: char) -> bool {
    token == '&'
}

pub fn is_boolean(token: char) -> bool {
    match token {
        'T' | 'F' | 't' | 'f' => true,
        _                     => false
    }
}

pub struct ParseState {
    pub token: char,
    tokens: io::Chars<io::Stdin>,
    pub var_table: HashMap<String, i64>
}

impl ParseState {
    pub fn new(char_stream: io::Chars<io::Stdin>) -> ParseState {
        let mut ps = ParseState {
            tokens: char_stream,
            token: '\0',
            var_table: HashMap::new()
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
            expected(&format!("'{}'", target));
        }
        self.advance();
        self.skip_whitespace();
    }

    pub fn get_boolean(&mut self) -> bool {
        if !is_boolean(self.token) {
            expected("Boolean Literal");
        }

        let ch = self.token.to_uppercase().next().expect(
            "boolean literal was not 'T' or 'F'!");
        let value = ch == 'T';
        self.advance();
        self.skip_whitespace();
        value
    }
}
