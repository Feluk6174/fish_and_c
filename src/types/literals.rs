use std::fs::File;
use std::io::Write;
use crate::compiler::register::Register;

pub fn load_number_literal(value: &str, register: &Register, file:&mut File) {
    file.write_all(format!("mov {}, {}\n", register.name, value).as_bytes()).expect("Error writing to file");
}

pub fn load_register_result(start_reg: &str, dest_reg: &Register, file:&mut File) {
    if start_reg == dest_reg.name {
        return 
    }
    file.write_all(format!("mov {}, {}\n", dest_reg.name, start_reg).as_bytes()).expect("Error writing to file");
}