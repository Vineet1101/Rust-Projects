
pub enum TokenType{
    // Single-character tokens.
LeftParen, RightParen, LeftBrace, RightBrace,
COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
// One or two character tokens.
BANG, BangEqual,
EQUAL, EqualEqual,
GREATER, GreaterEqual,
LESS, LessEqual,
// Literals.
IDENTIFIER, STRING, NUMBER,
// Keywords.
AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
EOF
}

pub struct Token{
    token_type:TokenType,
    lexeme:String,
    line:u32

}

impl Token{
    pub fn Token(&mut self,token_type:TokenType,lexeme:String,line:u32){
        self.token_type=token_type;
        self.lexeme=lexeme;
        self.line=line
    }

    // pub fn to_string(&mut self)->String{
    //     // format!({self.token_type},)
    // }
}