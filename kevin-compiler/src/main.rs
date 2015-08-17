#![feature(io)]
#![feature(convert)]

use std::io::prelude::*;
use std::io;
use std::process::exit;

macro_rules! expected {
    ($fmt:expr) => (panic!("{} expected", $fmt));
}

fn advance(tokens: &mut io::Chars<io::Stdin>) -> char {
    match tokens.next() {
        None         => exit(0),
        Some(Err(_)) => panic!("uh-oh"),
        Some(Ok(c))  => c
    }
}

fn consume(target: char, mut token: char, tokens: &mut io::Chars<io::Stdin>) -> char {
    if token != target {
        expected!(format!("'{}'", target).as_str());
    }
    token = advance(tokens);
    skip_whitespace(token, tokens)
}

#[allow(dead_code)]
fn is_alpha(token: char) -> bool {
    match token.to_uppercase().next().unwrap() {
        'A' ... 'Z' => true,
        _           => false
    }
}

fn is_digit(token: char) -> bool {
    match token {
        '0' ... '9' => true,
        _           => false
    }
}

fn is_alnum(token: char) -> bool {
    is_alpha(token) || is_digit(token)
}

fn is_whitespace(token: char) -> bool {
    match token {
        ' ' | '\t' => true,
        _          => false
    }
}

fn skip_whitespace(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> char {
    while is_whitespace(token) {
        token = advance(tokens);
    }
    token
}

fn is_add_op(token: char) -> bool {
    match token {
        '+' | '-' => true,
        _         => false
    }
}

#[allow(dead_code)]
fn get_name(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> (String, char) {
    if !is_alpha(token) {
        expected!("Name");
    }
    let mut name: String = "".to_string();
    while is_alnum(token) {
        name = format!("{}{}", name, token);
        token = advance(tokens);
    }
    token = skip_whitespace(token, tokens);
    (name, token)
}

fn get_num(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> (i64, char) {
    if !is_digit(token) {
        expected!("Integer");
    }
    let mut num: i64 = 0;
    while is_digit(token) {
        num = match token.to_digit(10) {
            Some(i) => 10 * num + (i as i64),
            None    => expected!("Digit")
        };
        token = advance(tokens);
    }
    token = skip_whitespace(token, tokens);
    (num, token)
}

fn factor(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> (i64, char) {
    if token == '(' {
        token = consume('(', token, tokens);
        let value = match expression(token, tokens) {
            (v, t) => {
                token = t;
                v
            }
        };
        token = consume(')', token, tokens);
        (value, token)
    } else {
        get_num(token, tokens)
    }
}

fn term(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> (i64, char) {
    let mut value: i64 = match factor(token, tokens) {
        (v, t) => {
            token = t;
            v
        }
    };

    loop {
        match token {
            '*' => {
                token = consume('*', token, tokens);
                value = match factor(token, tokens) {
                    (v, t) => {
                        token = t;
                        value * v
                    }
                }
            },
            '/' => {
                token = consume('/', token, tokens);
                value = match factor(token, tokens) {
                    (v, t) => {
                        token = t;
                        value / v
                    }
                }
            }
            _ => break
        }
    }
    (value, token)
}

fn expression(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> (i64, char) {
    let mut value: i64 = if is_add_op(token) {
        0
    } else {
        let (v, t) = term(token, tokens);
        token = t;
        v
    };

    loop {
        match token {
            '+' => {
                token = consume('+', token, tokens);
                value = match term(token, tokens) {
                    (v, t) => {
                        token = t;
                        value + v
                    }
                }
            },
            '-' => {
                token = consume('-', token, tokens);
                value = match term(token, tokens) {
                    (v, t) => {
                        token = t;
                        value - v
                    }
                }
            },
            _   => break
        }
    }

    (value, token)
}

fn main() {
    let mut tokens = io::stdin().chars();
    let mut token = advance(&mut tokens);
    token = skip_whitespace(token, &mut tokens);
    let (value, _) = expression(token, &mut tokens);
    println!("{}", value);
}
