use crate::precompile::{branch::Branch, tokens::TTS};

pub trait Size {
    fn size(self) -> Result<u64, String>;
    fn pure_size(self) -> Result<u64, String>;
}

#[derive(Debug, Clone, Copy)]
enum SimpleType {
    U8,
    U16,
    U32,
    U64
}

impl SimpleType {
    pub fn new(var_type: &str) -> Result<Self, String> {
        match var_type {
            "u8" => Ok(SimpleType::U8),
            "u16" => Ok(SimpleType::U16),
            "u32" => Ok(SimpleType::U32),
            "u64" => Ok(SimpleType::U64),
            _ => Err(format!("{} is not a recognised type", var_type))
        }
    }
}

impl Size for SimpleType {
    fn size(self) -> Result<u64, String> {
        match self {
            SimpleType::U8 => Ok(1),
            SimpleType::U16 => Ok(2),
            SimpleType::U32 => Ok(4),
            SimpleType::U64 => Ok(8),
            _ => Err(format!("{:?} is not a recognised type", self))
        }
    }
    fn pure_size(self) -> Result<u64, String> {
        self.size()
    }

}

#[derive(Debug, Clone, Copy)]
pub enum PointerType {
    U8,
    U16,
    U32,
    U64
}

impl PointerType {
    pub fn new(branch: &Branch) -> Result<Self, String> {
        match branch.branches[0].token.text.as_str() {
            "u8" => Ok(PointerType::U8),
            "u16" => Ok(PointerType::U16),
            "u32" => Ok(PointerType::U32),
            "u64" => Ok(PointerType::U64),
            _ => Err(format!("{} is not a recognised type", branch.branches[0].token.text))
        }
    }
}

impl Size for PointerType {
    fn pure_size(self) -> Result<u64, String> {
        return Ok(8);
    }
    fn size(self) -> Result<u64, String> {
        match self {
            PointerType::U8 => Ok(1),
            PointerType::U16 => Ok(2),
            PointerType::U32 => Ok(4),
            PointerType::U64 => Ok(8),
            _ => Err(format!("{:?} is not a recognised type", self))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Simple(SimpleType),
    Pointer(PointerType)
}


impl Type {
    pub fn new(branch: &Branch) -> Result<Self, String> {
        match branch.token.token_type {
            TTS::Pointer => {
                Ok(Type::Pointer(PointerType::new(branch)?))
            }
            TTS::VarType => {
                Ok(Type::Simple(SimpleType::new(&branch.token.text)?))
            }
            _ => Err(format!("{} is not a recgnised type", branch.token.text))
        }
    }
}

impl Size for Type {
    fn size(self) -> Result<u64, String> {
        match self {
            Type::Pointer(t) => t.size(),
            Type::Simple(t) => t.size()
        }
    }
    fn pure_size(self) -> Result<u64, String> {
        match self {
            Type::Pointer(t) => t.size(),
            Type::Simple(t) => t.pure_size()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    var_type: Type,
    size: u64,
    pure_size: u64
}

impl Variable {
    pub fn new(branch: &Branch) -> Result<Self, String> {
        let t = Type::new(branch)?;
        let name = branch.branches[1].token.text.clone();
        Ok(Self {
            name: name,
            var_type: t,
            size: t.size()?,
            pure_size: t.pure_size()?,
        })
        
    }
    pub fn new_from_arg(name: String, branch: &Branch) -> Result<Self, String>{
        let t = Type::new(branch)?;
        Ok(Self {
            name: name,
            var_type: t,
            size: t.size()?,
            pure_size: t.pure_size()?,
        })  
    }
    pub fn new_simple(name: &str, var_type: &str) -> Result<Self, String> {
        let t = Type::Simple(SimpleType::new(var_type)?);
        Ok(Self {
            name: String::from(name),
            var_type: t,
            size: t.size()?,
            pure_size: t.pure_size()?,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct Variables {
    vars: Vec<Variable>,
    pub rel_pos: u64
}

impl Variables {
    pub fn new() -> Self {
        Self {
            vars: Vec::new(),
            rel_pos: 0
        }
    }
    pub fn push(&mut self, var: Variable) {
        self.rel_pos += var.pure_size;
        self.vars.push(var);
    }
    pub fn push_arg(&mut self, name: String, t: &Branch) -> Result<(), String> {
        let var = Variable::new_from_arg(name, t)?;
        self.rel_pos += var.pure_size;
        self.vars.push(var);
        Ok(())
    }

    pub fn names(&self) -> Vec<&String> {
        let mut vars = Vec::new();
        for var in &self.vars {
            vars.push(&var.name)
        }
        vars
    }
}

