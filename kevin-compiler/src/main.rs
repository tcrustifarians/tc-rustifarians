#![feature(io)]
#![feature(convert)]

use std::io::prelude::*;
use std::io;

mod parse_state;
use parse_state::*;

fn factor(parser: &mut ParseState) -> i64 {
    if parser.token == '(' {
        parser.consume('(');
        let value = expression(parser);
        parser.consume(')');
        value
    } else if is_alpha(parser.token) {
        let name = parser.get_name();
        match parser.var_table.get(name.as_str()) {
            Some(v) => *v,
            None    => error(format!("Undefined variable {}", name).as_str())
        }
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

fn assignment(parser: &mut ParseState) {
    let name = parser.get_name();
    parser.consume('=');
    let value = expression(parser);
    parser.var_table.insert(name, value);
}

fn newline(parser: &mut ParseState) {
    if !is_newline(parser.token) {
        expected("Newline");
    }
    parser.skip_newline();
}

fn input(parser: &mut ParseState) {
    parser.consume('?');
    let name = parser.get_name();
    newline(parser);
    let value = parser.get_num(); // only values allowed so far are integers
    parser.var_table.insert(name, value);
}

fn main() {
    let mut parser = ParseState::new(io::stdin().chars());
    loop {
        match parser.token {
            '?' => input(&mut parser),
            '.' => break,
            _   => assignment(&mut parser)
        }
        newline(&mut parser);
    }
    println!("{:?}", parser.var_table);
}
