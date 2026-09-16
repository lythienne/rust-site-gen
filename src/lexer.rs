use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),           // string of non special chars
    Stars(u8),              // has number of stars
    Space,
    BigSpace,               // 2 or more
    NewLine,
    EOF                     // end of file
}

pub struct Lexer<'a> {
    curr: Option<char>,
    chars: &'a mut Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn eat(&mut self, c: char) {
        if self.curr != Some(c) {
            panic!("Lexing error: expected {}, got {:?}", c, self.curr);
        }
        self.curr = self.chars.next();
    }

    pub fn new(chars: &'a mut Chars<'a>) -> Lexer<'a> {
        
        Lexer { curr: curr, chars: chars }
    }

    fn eat_whitespace(&mut self) {
        let mut curr = self.chars.next();
        while let Some(c) = curr {
            if !c.is_whitespace() {
                break;
            }
            curr = self.chars.next();
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.curr {
            Some('*') => self.lex_stars(),
            Some('\n') => {self.eat('\n'); Token::NewLine},
            Some(c) => {
                if c.is_whitespace() { self.lex_whitespace() }
                else { self.lex_word() }
            },
            None => Token::EOF
        }
    }

    fn is_special_char(c: char) -> bool {
        match c {
            '*' => true,
            _ => false
        }
    }

    fn lex_newline(&mut self) -> Token {
        self.eat('\n');
    }

    fn lex_whitespace(&mut self) -> Token {
        let mut space_count = 0; 

        while let Some(c) = self.curr {
            match c {
                '\n' => break,
                ' ' => space_count += 1,
                '\t' => space_count += 2,
                a if !a.is_whitespace() => break,
                _ => panic!("come write code to handle other whitespaces")
            }
            self.eat(c);
        }
        if space_count > 1 { Token::BigSpace } else { Token::Space }
    }

    fn lex_word(&mut self) -> Token {
        let mut word = String::new();

        while let Some(c) = self.curr {
            if c.is_whitespace() || Self::is_special_char(c) {
                break;
            }
            word.push(c);
            self.eat(c);
        }
        Token::Word(word)
    }

    fn lex_stars(&mut self) -> Token {
        let mut star_count = 1;
        self.eat('*');
        while let Some(c) = self.curr && c == '*' {
            star_count += 1; 
            self.eat('*');
        } 
        Token::Stars(star_count)
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Token::*;
    
    #[test]
    fn empty() {
        let empty = String::from("");
        let mut chars = empty.chars();
        let mut lex = Lexer::new(&mut chars);

        assert_eq!(EOF, lex.next_token());
        assert_eq!(EOF, lex.next_token());
    }

    #[test]
    fn one_word() {
        let text = String::from("words");
        let mut chars = text.chars();
        let mut lex = Lexer::new(&mut chars);

        assert_eq!(Word(text.clone()), lex.next_token());
        assert_eq!(EOF, lex.next_token());
    }

    #[test]
    fn many_words() {
        let text = String::from("hi foo bar");
        let mut chars = text.chars();
        let mut lex = Lexer::new(&mut chars);

        assert_eq!(Word(String::from("hi")), lex.next_token());
        assert_eq!(Space, lex.next_token());
        assert_eq!(Word(String::from("foo")), lex.next_token());
        assert_eq!(Space, lex.next_token());
        assert_eq!(Word(String::from("bar")), lex.next_token());
        assert_eq!(EOF, lex.next_token());
    }

    #[test]
    fn big_spaces_and_newlines() {
        let text = String::from("\n   \t    tab:\t\ntwo_spaces:  \nextra_spaces: \t \t\n\n");
        let mut chars = text.chars();
        let mut lex = Lexer::new(&mut chars);

        let tokenized = vec![Word(String::from("tab:")), BigSpace, NewLine,
            Word(String::from("two_spaces:")), BigSpace, NewLine,
            Word(String::from("extra_spaces:")), BigSpace, NewLine, NewLine, EOF];

        for token in tokenized {
            assert_eq!(token, lex.next_token());
        }
    }

    #[test]
    fn text_and_stars() {
        let text = String::from("* plain *italics* **bold** ***bold-italics***");
        let mut chars = text.chars();
        let mut lex = Lexer::new(&mut chars);

        let tokenized = vec![Stars(1), Space, Word(String::from("plain")), Space,
            Stars(1), Word(String::from("italics")), Stars(1), Space,
            Stars(2), Word(String::from("bold")), Stars(2), Space,
            Stars(3), Word(String::from("bold-italics")), Stars(3), EOF];

        for token in tokenized {
            assert_eq!(token, lex.next_token());
        }
    }
}
