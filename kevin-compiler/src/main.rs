#![feature(io)]
#![feature(convert)]

use std::io::prelude::*;
use std::io;

mod parse_state;
use parse_state::*;

fn emitln(s: &str) {
    println!("\t{}", s);
}

fn identifier(parser: &mut ParseState) {
    let name = parser.get_name();
    if parser.token == '(' {
        parser.consume('(');
        parser.consume(')');
        emitln("pushq %rbp");
        emitln("movq %rsp, %rbp");
        emitln(format!("callq _{}", name).as_str());
        return
    }
    emitln(format!("movq _{}@GOTPCREL(%rip), %rax", name).as_str());
}

fn factor(parser: &mut ParseState) {
    if parser.token == '(' {
        parser.consume('(');
        expression(parser);
        parser.consume(')');
        return
    } else if is_alpha(parser.token) {
        return identifier(parser)
    }

    let num = parser.get_num();
    emitln(format!("movq ${}, %rax", num).as_str());
}

fn multiply(parser: &mut ParseState) {
    parser.consume('*');
    factor(parser);
    emitln("popq %rbx");
    emitln("imulq %rbx");
}

fn divide(parser: &mut ParseState) {
    parser.consume('/');
    factor(parser);
    emitln("popq %rbx");
    emitln("idivq %rbx");
}

fn term(parser: &mut ParseState) {
    factor(parser);
    loop {
        match parser.token {
            '*' => {
                emitln("pushq %rax");
                multiply(parser);
            },
            '/' => {
                emitln("pushq %rax");
                divide(parser);
            }
            _ => break
        }
    }
}

fn add(parser: &mut ParseState) {
    parser.consume('+');
    term(parser);
    emitln("popq %rbx");
    emitln("addq %rbx, %rax");
}

fn subtract(parser: &mut ParseState) {
    parser.consume('-');
    term(parser);
    emitln("popq %rbx");
    emitln("subq %rbx, %rax");
    emitln("negq %rax");
}

fn expression(parser: &mut ParseState) {
    if is_add_op(parser.token) {
        emitln("xorq %rax, %rax");
    }
    else {
        term(parser);
    }

    while is_add_op(parser.token) {
        emitln("pushq %rax");
        match parser.token {
            '+' => add(parser),
            '-' => subtract(parser),
            _   => expected("add operation")
        }
    }
}

fn assignment(parser: &mut ParseState) {
    let name = parser.get_name();
    parser.consume('=');
    expression(parser);
    emitln(format!("movq %rax, _{}@GOTPCREL(%rip)", name).as_str());
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

    let mut parser = &mut ParseState::new(io::stdin().chars());
    assignment(parser);

    wrapup();
}
