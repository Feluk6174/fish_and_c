
use std::fs::File;
use std::io::Write;

pub fn add_reg_lit(size: u64, reg:&str, num: &str, file:File) -> Result<(), String> {
    file.write_all(format!("add rax, [{}]{}
", num));
    Ok(())
}