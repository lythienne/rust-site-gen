pub enum Token {
    Text(String),
    SmallBreak,             // space-space-newline
    BigBreak,               // newline-newline
    ItalicStart,            // *text
    ItalicEnd,              // text*
    BoldStart,              // **text
    BoldEnd,                // text**
    BoldItalicStart,        // ***text
    BoldItalicEnd,          // text***
    /* will implement later??
    strikebound,            // ~~text
    quote,                  // > text
    header(u8),             // # text
    unorderedlist,          // * text or - text
    orderedlist(u32),       // number. text
    codebound,              // `
    blockcodebound,         // ```
    sitelenponabound,       // $text
    blockspbound,           // $$$
    */
    EOF                     //end of file
}

struct Lexer {
    curr: Option<char>,
    chars: &mut impl Iterator<char>,
    is_start: bool,
    maybe_next_token: Option<Token>,
}

impl Lexer {
    fn eat(c: char) {
        if (self.curr != Some(c)) {
            panic!("Lexing error: expected {c}, got {self.curr}");
        }
        self.curr = self.chars.next();
    }

    pub fn new(chars: &mut impl Iterator<char>) -> Lexer {
        let curr = chars.next();
        while curr != None && c.is_whitespace() {
            curr = chars.next();
        }
        Lexer { curr: curr, is_start: true, chars: chars }
    }

    pub fn next_token() -> Token {
        if let Some(token) = self.maybe_next_token {
            self.maybe_next_token = None;
            return token;
        }

        match self.curr {
            Some('*') => lex_stars(),
            Some(c) => lex_text(),
            None => Token::EOF
        }
    }

    fn lex_text() -> Token {
        let text = self.curr.toString();
        eat(self.curr);

        while let Some(c) = self.curr {
            match self.curr {
            }
            
        }
    }

    fn lex_stars() -> Token {
        let mut star_count = 1;
        eat('*');
        while self.curr == '*' {
            star_count += 1; 
            eat('*');
        } 
        if self.is_start {
            match star_count {
                1 => return Token::ItalicStart,
                2 => return Token::BoldStart,
                _ => return Token::BoldItalicStart,
            }
        }
        else {
            match star_count {
                1 => return Token::ItalicEnd,
                2 => return Token::BoldEnd,
                _ => return Token::BoldItalicEnd,
            }
        }
    } 
}
