pub struct Register {
    pub name: String,
    pub size: u64
}

impl Register {
    pub fn new(name: String, size:u64) -> Self {
        Self {
            name: String::from(name),
            size: size
        }
    }
    pub fn new_gen(letter: &str, size:u64) -> Result<Self, String> {
        if letter == "a" || letter == "b" || letter == "c" || letter == "d" {
            match size {
                1 => Ok(Register::new(format!("{}l", letter), 1)),
                2 => Ok(Register::new(format!("{}x", letter), 2)),
                4 => Ok(Register::new(format!("e{}x", letter), 4)),
                8 => Ok(Register::new(format!("r{}x", letter), 8)),
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
                1 => Ok(Register::new(format!("r{}b", letter), 1)),
                2 => Ok(Register::new(format!("r{}w", letter), 2)),
                4 => Ok(Register::new(format!("r{}d", letter), 4)),
                8 => Ok(Register::new(format!("r{}", letter), 8)),
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
}