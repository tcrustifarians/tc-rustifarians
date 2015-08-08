#![feature(io)]
#![feature(convert)]

use std::io::prelude::*;
use std::io;
use std::process::exit;

fn expected(s: &str) {
    panic!("{} expected", s);
}

fn advance(tokens: &mut io::Chars<io::Stdin>) -> char {
    match tokens.next() {
        None         => exit(0),
        Some(Err(_)) => panic!("uh-oh"),
        Some(Ok(c))  => c
    }
}

fn consume(target: char, token: char, tokens: &mut io::Chars<io::Stdin>) -> char {
    if token != target {
        expected(format!("'{}'", target).as_str());
    }
    advance(tokens)
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

fn is_add_op(token: char) -> bool {
    match token {
        '+' | '-' => true,
        _         => false
    }
}

#[allow(dead_code)]
fn get_name(token: char, tokens: &mut io::Chars<io::Stdin>) -> (char, char) {
    if !is_alpha(token) {
        expected("Name");
    }
    let next_token = advance(tokens);
    (token, next_token)
}

fn get_num(token: char, tokens: &mut io::Chars<io::Stdin>) -> (char, char) {
    if !is_digit(token) {
        expected("Integer");
    }
    let next_token = advance(tokens);
    (token, next_token)
}

fn emitln(s: &str) {
    println!("\t{}", s);
}

fn factor(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> char {
    if token == '(' {
        token = consume('(', token, tokens);
        expression();
        token = consume(')', token, tokens);
        return token
    }

    let (num, token) = get_num(token, tokens);
    emitln(format!("movq ${}, %rax", num).as_str());
    token
}

fn multiply(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> char {
    token = consume('*', token, tokens);
    token = factor(token, tokens);
    emitln("popq %rbx");
    emitln("imulq %rbx");
    token
}

fn divide(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> char {
    token = consume('/', token, tokens);
    token = factor(token, tokens);
    emitln("popq %rbx");
    emitln("idivq %rbx");
    token
}

fn term(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> char {
    token = factor(token, tokens);
    loop {
        match token {
            '*' => {
                emitln("pushq %rax");
                token = multiply(token, tokens);
            },
            '/' => {
                emitln("pushq %rax");
                token = divide(token, tokens);
            }
            _ => break
        }
    }
    token
}

fn add(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> char {
    token = consume('+', token, tokens);
    token = term(token, tokens);
    emitln("popq %rbx");
    emitln("addq %rbx, %rax");
    token
}

fn subtract(mut token: char, tokens: &mut io::Chars<io::Stdin>) -> char {
    token = consume('-', token, tokens);
    token = term(token, tokens);
    emitln("popq %rbx");
    emitln("subq %rbx, %rax");
    emitln("negq %rax");
    token
}

fn expression() {
    let mut tokens = io::stdin().chars();
    let mut token = advance(&mut tokens);
    if is_add_op(token) {
        emitln("xorq %rax, %rax");
    }
    else {
        token = term(token, &mut tokens);
    }

    while is_add_op(token) {
        emitln("pushq %rax");
        match token {
            '+' => token = add(token, &mut tokens),
            '-' => token = subtract(token, &mut tokens),
            _   => expected("add operation")
        }
    }
}

fn preamble() {
    println!(".text");
    println!(".globl _main");
    println!("_main:");
    emitln("pushq %rbp");
    emitln("movq %rsp, %rbp");
}

fn wrapup() {
    emitln("popq %rbp");
    emitln("movq %rax, %rdi");
    emitln("movq $0x2000001, %rax");
    emitln("syscall");
}

fn main() {
    preamble();
    expression();
    wrapup();
}
