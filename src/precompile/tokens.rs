#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TTS {
    NumberLiteral,
    StringLiteral,
    VarType,
    Name,
    Function,
    Parenthesis,
    SquareKeys,
    Keys,
    NewCommand,
    Assignation,
    Comparison,
    ArithmeticOperation,
    IfKeyword,
    ElseKeyword,
    WhileKeyword,
    ForKeyword,
    ReturnKeyword,
    BreakKeyword,
    ContinueKeyword,
    Address,
    Pointer,
    DeclarationArgs,
    Assembly,
    AssemblyCode,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TTS,
    pub text: String,
}

impl Token {
    pub fn number_literal(text: &str) -> Self {
        Self {
            token_type: TTS::NumberLiteral,
            text: String::from(text),
        }
    }
    pub fn string_literal(text: &str) -> Self {
        Self {
            token_type: TTS::StringLiteral,
            text: String::from(text),
        }
    }
    pub fn var_type(text: &str) -> Self {
        Self {
            token_type: TTS::VarType,
            text: String::from(text),
        }
    }
    pub fn name(text: &str) -> Self {
        Self {
            token_type: TTS::Name,
            text: String::from(text),
        }
    }
    pub fn function(text: &str) -> Self {
        Self {
            token_type: TTS::Function,
            text: String::from(text),
        }
    }
    pub fn parenthesis(text: &str) -> Self {
        Self {
            token_type: TTS::Parenthesis,
            text: String::from(text),
        }
    }
    pub fn square_keys(text: &str) -> Self {
        Self {
            token_type: TTS::SquareKeys,
            text: String::from(text),
        }
    }
    pub fn keys(text: &str) -> Self {
        Self {
            token_type: TTS::Keys,
            text: String::from(text),
        }
    }
    pub fn new_command(text: &str) -> Self {
        Self {
            token_type: TTS::NewCommand,
            text: String::from(text),
        }
    }
    pub fn assignation(text: &str) -> Self {
        Self {
            token_type: TTS::Assignation,
            text: String::from(text),
        }
    }
    pub fn arithmetic_operation(text: &str) -> Self {
        Self {
            token_type: TTS::ArithmeticOperation,
            text: String::from(text),
        }
    }
    pub fn comparison(text: &str) -> Self {
        Self {
            token_type: TTS::Comparison,
            text: String::from(text),
        }
    }
    pub fn if_keyword(text: &str) -> Self {
        Self {
            token_type: TTS::IfKeyword,
            text: String::from(text),
        }
    }
    pub fn else_keyword(text: &str) -> Self {
        Self {
            token_type: TTS::ElseKeyword,
            text: String::from(text),
        }
    }
    pub fn while_keyword(text: &str) -> Self {
        Self {
            token_type: TTS::WhileKeyword,
            text: String::from(text),
        }
    }
    pub fn for_keyword(text: &str) -> Self {
        Self {
            token_type: TTS::ForKeyword,
            text: String::from(text),
        }
    }
    pub fn return_keyword(text: &str) -> Self {
        Self {
            token_type: TTS::ReturnKeyword,
            text: String::from(text),
        }
    }
    pub fn break_keyword(text: &str) -> Self {
        Self {
            token_type: TTS::BreakKeyword,
            text: String::from(text),
        }
    }
    pub fn continue_keyword(text: &str) -> Self {
        Self {
            token_type: TTS::ContinueKeyword,
            text: String::from(text),
        }
    }
    pub fn address(text: &str) -> Self {
        Self {
            token_type: TTS::Address,
            text: String::from(text),
        }
    }
    pub fn pointer(text: &str) -> Self {
        Self {
            token_type: TTS::Pointer,
            text: String::from(text),
        }
    }
    pub fn declaration_args() -> Self {
        Self {
            token_type: TTS::DeclarationArgs,
            text: String::new(),
        }
    }
    pub fn assembly() -> Self {
        Self {
            token_type: TTS::Assembly,
            text: String::new(),
        }
    }pub fn assembly_code(text: &str) -> Self {
        Self {
            token_type: TTS::AssemblyCode,
            text: String::from(text),
        }
    }
}
