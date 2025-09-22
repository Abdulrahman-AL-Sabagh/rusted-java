pub enum TokenKind {
    Identifier,
    Keyword(KeywordToken)
    Type(TypeKind),
    SpecialCharacter(SpecialChar)
    None
}


pub enum KeywordToken {
    New,
    Class,
    Char,
    Int,
    Print,
    Break,
    Continue,
    While,
    IF,
    Else,
}

pub enum SpecialChar {
    Slash,
    
} 

pub enum TypeKind {
    Int,
    Char
}
