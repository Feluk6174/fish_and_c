use std::{fs::File, io::Write};

use super::variables::{Size, Variables};

pub struct Register {
    pub name: String,
    letter: String,
    pub size: u64
}

impl Register {
    pub fn new(name: &str, letter:&str, size:u64) -> Self {
        Self {
            name: String::from(name),
            letter: String::from(letter),
            size: size
        }
    }
    pub fn new_gen(letter: &str, size:u64) -> Result<Self, String> {
        if letter == "a" || letter == "b" || letter == "c" || letter == "d" {
            match size {
                1 => Ok(Register::new(&format!("{}l", letter), letter, 1)),
                2 => Ok(Register::new(&format!("{}x", letter), letter, 2)),
                4 => Ok(Register::new(&format!("e{}x", letter), letter, 4)),
                8 => Ok(Register::new(&format!("r{}x", letter), letter, 8)),
                _ => Err(format!("Expected size 1, 2, 4, 8 got {}", size))
            }
        }
        else if letter == "si" || letter == "di" {
            match size {
                1 => Ok(Register::new(&format!("{}l", letter), letter, 1)),
                2 => Ok(Register::new(&format!("{}", letter), letter, 2)),
                4 => Ok(Register::new(&format!("e{}", letter), letter, 4)),
                8 => Ok(Register::new(&format!("r{}", letter), letter, 8)),
                _ => Err(format!("Expected size 1, 2, 4, 8 got {}", size))
            }
        }
        else {
            let num = match letter.parse::<u8>() {
                Ok(num) => num,
                Err(_) => return Err(String::from("letter has to be a, b, c, d or a number"))
            };
            if num < 8 || num > 15 {
                return Err(format!("{} register number have to be between 8 and 15", letter))
            }
            match size {
                1 => Ok(Register::new(&format!("r{}b", letter), letter, 1)),
                2 => Ok(Register::new(&format!("r{}w", letter), letter, 2)),
                4 => Ok(Register::new(&format!("r{}d", letter), letter, 4)),
                8 => Ok(Register::new(&format!("r{}", letter), letter, 8)),
                _ => Err(format!("Expected size 1, 2, 4, 8 got {}", size))
            }
        }
    }
    pub fn prefix(&self) -> String {
        match self.size {
            1 => String::from("BYTE"),
            2 => String::from("WORD"),
            4 => String::from("DWORD"),
            8 => String::from("QWORD"),
            _ => String::from("WTF")
        }
    }
    pub fn _get_in_size_adapted(&self, size:u64, file: &mut File) -> Result<Register, String> {
        let reg = self.get_in_size(size)?;
        file.write_all(format!("and {}, {}n", reg.name, 2_u64.pow(size as u32)-1).as_bytes()).expect("Failed to write to file");
        Ok(reg)
    }
    pub fn get_in_size(&self, size:u64) -> Result<Register, String> {
        Ok(Register::new_gen(&self.letter, size)?)
    }
}

pub fn store_reg_to_mem(vars: &mut Variables, name:String, reg: &Register, file: &mut File) ->Result<(), String> {
    
    let var = vars.get(&name)?;

    file.write_all(format!("lea rbx, [r15+{}]
mov {}[rbx], {}
", var.rel_pos, var.var_type.prefix()?, reg.name).as_bytes()).expect("Couldn't write to file!");

    Ok(())
}