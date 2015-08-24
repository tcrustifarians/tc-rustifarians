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

fn do_if(parser: &mut ParseState, label: &str) {
    parser.consume('i');
    let label1 = parser.new_label();
    let mut label2 = label1.clone();
    condition();
    emitln(&format!("jz {}", label1));
    block(parser, label);
    if parser.token == 'l' {
        parser.consume('l');
        label2 = parser.new_label();
        emitln(&format!("jmp {}", label2));
        emit_label(&label1);
        block(parser, label);
    }
    parser.consume('e');
    emit_label(&label2);
}

fn do_break(parser: &mut ParseState, label: &str) {
    parser.consume('b');
    if label == "" {
        error("No loop to break from");
    }
    emitln(&format!("jmp {}", label));
}

fn do_loop(parser: &mut ParseState) {
    parser.consume('p');
    let (label1, label2) = (parser.new_label(), parser.new_label());
    emit_label(&label1);
    block(parser, &label2);
    parser.consume('e');
    emitln(&format!("jmp {}", label1));
    emit_label(&label2);
}

fn do_do(parser: &mut ParseState) {
    parser.consume('d');
    let (label1, label2) = (parser.new_label(), parser.new_label());
    expression();
    emitln("mov %eax, %ecx");
    emitln("cmp %ecx, 0");
    emitln(&format!("je {}", label2));
    emit_label(&label1);
    block(parser, &label2);
    emitln(&format!("loop {}", label1));
    emit_label(&label2);
}

fn do_repeat(parser: &mut ParseState) {
    parser.consume('r');
    let (label1, label2) = (parser.new_label(), parser.new_label());
    emit_label(&label1);
    block(parser, &label2);
    parser.consume('u');
    condition();
    emitln(&format!("jz {}", label1));
    emit_label(&label2);
}

fn do_while(parser: &mut ParseState) {
    parser.consume('w');
    let label1 = parser.new_label();
    let label2 = parser.new_label();
    emit_label(&label1);
    condition();
    emitln(&format!("jz {}", label2));
    block(parser, &label2);
    parser.consume('e');
    emitln(&format!("jmp {}", label1));
    emit_label(&label2);
}

fn do_for(parser: &mut ParseState) {
    parser.consume('f');
    let (label1, label2) = (parser.new_label(), parser.new_label());
    let name = parser.get_name();
    parser.consume('=');
    expression(); // initial value
    emitln("subq $1, %rax");
    emitln(&format!("movq %rax, _{}@GOTPCREL(%rip)", name));
    expression(); // final value
    emitln("movq %rax, %rdx");
    emit_label(&label1);
    emitln(&format!("movq _{}@GOTPCREL(%rip), %rax", name));
    emitln("incq %rax");
    emitln(&format!("movq %rax, _{}@GOTPCREL(%rip)", name));
    emitln("cmpq %rdx, %rax");
    emitln(&format!("jle {}", label2));
    block(parser, &label2);
    emitln(&format!("jmp {}", label1));
    emit_label(&label2);
}

fn expression() {
    emitln("<expr>  ## leave result in %rax");
}

fn other(parser: &mut ParseState) {
    emitln(&format!("{}", parser.get_name()));
}

fn block(parser: &mut ParseState, label: &str) {
    while !['e', 'l', 'u'].contains(&parser.token) {
        match parser.token {
            'b' => do_break(parser, label),
            'd' => do_do(parser),
            'f' => do_for(parser),
            'i' => do_if(parser, label),
            'p' => do_loop(parser),
            'r' => do_repeat(parser),
            'w' => do_while(parser),
            _   => other(parser)
        }
    }
}

fn program(parser: &mut ParseState) {
    block(parser, "");
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
