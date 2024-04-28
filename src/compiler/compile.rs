use std::{fs::File, io::Write};
use crate::precompile::branch::Branch;
use super::functions::{Function, build_functions, process_functions};

pub fn compile(tree: Vec<Branch>, file_name: &str) -> Result<(), String> {
    let (mut functions, signatures) = build_functions(&tree)?;
    let mut file = File::create(file_name).expect("Error writing to file");
    println!("{}", functions.len());
    add_base(&mut file, 10000, 100);
    process_functions(functions, signatures,&mut file)?;
    Ok(())
}


fn add_base(file:&mut File, mem_size:u64, p_buf_size:u64) {
    file.write_all(format!("; Coded in Fish&C
; https://github.com/Feluk6174/yeso-compiler
global _start

section .data
    mem: times {} db 0
    alloc: times 100 dd 0
    mem_fi dd 0
    p_buf_ptr: dq 0
    p_buf: times {} db 0

section .text

panic_program:
    mov rbx, QWORD[p_buf_ptr]
    call print_buffer

    mov rax, 60
    mov rdi, 120
    syscall

_start:
    call main
    ;end execution
    
    mov rbx, QWORD[p_buf_ptr]
    call print_buffer

    mov rax, 60
    mov rdi, 0
    syscall

", mem_size, p_buf_size).as_bytes()).expect("Error writing to file");
}