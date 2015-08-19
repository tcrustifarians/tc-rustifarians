#![feature(io)]
#![feature(convert)]

use std::io::prelude::*;
use std::io;

#[macro_use]
mod parse_state;
use parse_state::*;

fn factor(parser: &mut ParseState) -> i64 {
    if parser.token == '(' {
        parser.consume('(');
        let value = expression(parser);
        parser.consume(')');
        value
    } else {
        parser.get_num()
    }
}

fn term(parser: &mut ParseState) -> i64 {
    let mut value: i64 = factor(parser);

    loop {
        match parser.token {
            '*' => {
                parser.consume('*');
                value *= factor(parser);
            },
            '/' => {
                parser.consume('/');
                value /= factor(parser);
            }
            _ => break
        }
    }
    value
}

fn expression(parser: &mut ParseState) -> i64 {
    let mut value: i64 = if is_add_op(parser.token) {
        0
    } else {
        term(parser)
    };

    loop {
        match parser.token {
            '+' => {
                parser.consume('+');
                value += term(parser);
            },
            '-' => {
                parser.consume('-');
                value -= term(parser)
            },
            _   => break
        }
    }

    value
}

fn main() {
    let mut parser = ParseState::new(io::stdin().chars());
    let value = expression(&mut parser);
    println!("{}", value);
}
