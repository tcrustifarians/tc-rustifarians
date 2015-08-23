#![feature(io)]

use std::io::prelude::*;
use std::io;

mod parse_state;
use parse_state::*;

fn emitln(s: &str) {
    println!("\t{}", s);
}

fn emit_label(label: &str) {
    println!("{}:", label);
}

fn condition() {
    emitln("<condition>");
}

fn do_if(parser: &mut ParseState) {
    parser.consume('i');
    let label1 = parser.new_label();
    let mut label2 = label1.clone();
    condition();
    emitln(&format!("jz {}", label1));
    block(parser);
    if parser.token == 'l' {
        parser.consume('l');
        label2 = parser.new_label();
        emitln(&format!("jmp {}", label2));
        emit_label(&label1);
        block(parser);
    }
    parser.consume('e');
    emit_label(&label2);
}

fn do_loop(parser: &mut ParseState) {
    parser.consume('p');
    let label = parser.new_label();
    emit_label(&label);
    block(parser);
    parser.consume('e');
    emitln(&format!("jmp {}", label));
}

fn do_while(parser: &mut ParseState) {
    parser.consume('w');
    let label1 = parser.new_label();
    let label2 = parser.new_label();
    emit_label(&label1);
    condition();
    emitln(&format!("jz {}", label2));
    block(parser);
    parser.consume('e');
    emitln(&format!("jmp {}", label1));
    emit_label(&label2);
}

fn other(parser: &mut ParseState) {
    emitln(&format!("{}", parser.get_name()));
}

fn block(parser: &mut ParseState) {
    while !['e', 'l'].contains(&parser.token) {
        match parser.token {
            'i' => do_if(parser),
            'p' => do_loop(parser),
            'w' => do_while(parser),
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
