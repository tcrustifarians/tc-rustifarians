#![feature(io)]

use std::io::prelude::*;
use std::io;

mod parse_state;
use parse_state::*;

fn emitln(s: &str) {
    println!("\t{}", s);
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

fn relation(parser: &mut ParseState) {
    emitln("<relation>");
    parser.advance();
}

fn bool_factor(parser: &mut ParseState) {
    if !is_boolean(parser.token) {
        relation(parser);
        return;
    }

    if parser.get_boolean() {
        emitln("movq $-1, %rax");
    } else {
        emitln("xorq %rax, %rax");
    }
}

fn not_factor(parser: &mut ParseState) {
    if !is_not_op(parser.token) {
        bool_factor(parser);
        return;
    }
    parser.consume('!');
    bool_factor(parser);
    emitln("not %eax");
}

fn bool_term(parser: &mut ParseState) {
    not_factor(parser);
    while is_and_op(parser.token) {
        emitln("pushq %rax");
        parser.consume('&');
        not_factor(parser);
        emitln("andq (%rsp), %rax");
        emitln("addq $8, %rsp");
    }
}

fn bool_or(parser: &mut ParseState) {
    parser.consume('|');
    bool_term(parser);
    emitln("or (%rsp), %rax");
}

fn bool_xor(parser: &mut ParseState) {
    parser.consume('^');
    bool_term(parser);
    emitln("xor (%rsp), %rax");
}

fn bool_expression(parser: &mut ParseState) {
    bool_term(parser);
    while is_or_op(parser.token) {
        emitln("pushq %rax");
        match parser.token {
            '|' => bool_or(parser),
            '^' => bool_xor(parser),
            _   => break
        }
    }
}

fn main() {
    preamble();

    let mut parser = &mut ParseState::new(io::stdin().chars());
    bool_expression(parser);

    wrapup();
}
