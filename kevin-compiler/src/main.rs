#![feature(io)]
#![feature(convert)]

use std::io::prelude::*;
use std::io;

mod parse_state;
use parse_state::*;

fn emitln(s: &str) {
    println!("\t{}", s);
}

fn condition() {
    emitln("<condition>");
}

fn do_if(parser: &mut ParseState) {
    parser.consume('i');
    let label = parser.new_label();
    condition();
    emitln(&format!("jz {}", label));
    block(parser);
    parser.consume('e');
    emitln(&format!("{}:", label));
}

fn other(parser: &mut ParseState) {
    emitln(&format!("{}", parser.get_name()));
}

fn block(parser: &mut ParseState) {
    while parser.token != 'e' {
        match parser.token {
            'i' => do_if(parser),
            _   => other(parser)
        }
    }
}

fn program(parser: &mut ParseState) {
    block(parser);
    if parser.token != 'e' {
        expected("End");
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

    let mut parser = &mut ParseState::new(io::stdin().chars());
    program(parser);

    wrapup();
}
