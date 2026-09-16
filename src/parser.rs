use lexer::Lexer;
use lexer::Token;

#[derive(Debug, PartialEq)]
enum Block {
    Text(Vec<Line>),
}

#[derive(Debug, PartialEq)]
struct Line {
    line: Vec<StyledText>
}

#[derive(Debug, PartialEq)]
struct StyledText {
    text: String,
    style: TextStyle
}

#[derive(Debug, PartialEq)]
enum TextStyle { Plain, Bold, Italic, BoldItalic }

pub struct Parser {
    lex: &mut Lexer,
    curr_token: Token;
}

pub fn parse_md(&self) -> Vec<Block> {
    let mut ir = Vec::new();
    self.curr_token = self.lex.next_token();

    while self.curr_token != Token::EOF {
        ir.push(self.parse_block());
    }

    ir
}

fn parse_block(&self) -> Block {
    let mut lines = Vec::new();

    while self.curr_token 

    Text(lines)
}
