struct Span {
    col_start: usize,
    col_end: usize,
    line_start: usize,
    line_end: usize

}

struct Token {
    kind: TokenKind,
    position: Span,
    value: String
}


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
    Void,
}


impl KeywordToken {

    fn map_string_to_token(value: &str) -> Result<TokenKind, String> {
        return match(value) {
           
            "new" => TokenKind::Keyword(KeywordToken::New),
            "class" => TokenKind::Keyword(KeywordToken::Class),
            "char" => TokenKind::Keyword(KeywordToken::Char),
            "int" => TokenKind::Keyword(KeywordToken::Int),
            "print" => TokenKind::Keyword(KeywordToken::Print),
            "break" => TokenKind::Keyword(KeywordToken::Break),
            "continue" => TokenKind::Keyword(KeywordToken::Continue),
            "while" => TokenKind::Keyword(KeywordToken::While),
            "if" => TokenKind::Keyword(KeywordToken::If),
            "else" => TokenKind::Keyword(KeywordToken::Else),
            "void" => TokenKind::Keyword(KeywordToken::Void),
            _ => Err(format!("Unexpected keywordtoken:  {}", value)),
        }
    
    }

}




pub enum SpecialChar {
    Slash,

} 

pub enum TypeKind {
    Int,
    Char
}
