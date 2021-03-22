
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    TagOpening(String),
    TagClosing(String),
    TagValue(String),
    Attribute(String),
    AttributeValue(String),

    RightArrow,
}

pub struct Lexer {
    input: Vec<char>,
    index: usize
}

impl Lexer {
    pub fn new(mut input: String) -> Result<Lexer, ()> {
        if &input[0..15] != "<!DOCTYPE html>" {
            return Err(())
        }
        input.drain(0..15);

        Ok(Lexer {
            input: input.chars().collect(),
            index: 0
        })
    }

    pub fn next(&mut self) -> Option<char> {
        if self.index < self.input.len() {
            self.index += 1;
            return Some(self.input[self.index - 1])
        }
        else {
            return None
        }
    }

    pub fn get_word(&mut self) -> String {
        let mut word = String::new();

        loop {
            let c = match self.next() {
                Some(c) => c,
                None => break
            };

            if c == '>' {
                self.index -= 1;
                break
            }
            else if c == ' ' && word.len() > 0 {
                break
            }
            else {
                word.push(c);
            }
        }

        word
    }

    pub fn get_tag_val(&mut self) -> String {
        let mut tag_val = String::new();
        loop {
            let c = match self.next() {
                Some(c) => c,
                None => break
            };

            if c == '<' {
                self.index -= 1;
                break
            }
            else {
                tag_val.push(c);
            }
        }

        tag_val
    }

    pub fn get_attr_val(&mut self) -> String {
        let mut attr_val = String::new();
        loop {
            let c = match self.next() {
                Some(c) => c,
                None => break
            };

            if c == '\"' {
                break
            }
            else {
                attr_val.push(c);
            }
        }

        attr_val
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut output: Vec<Token> = Vec::new();

        loop {
            let c = match self.next() {
                Some(c) => c,
                None => break
            };

            if c == '<' {
                match self.next() {
                    Some(next_c) => {
                        if next_c != '/' {
                            self.index -= 1;
                            output.push(Token::TagOpening(self.get_word()));
                        }
                        else {
                            output.push(Token::TagClosing(self.get_word()));
                        }
                    }
                    None => break
                }
            }
            else if c == '>' {
                output.push(Token::RightArrow);
            }
            else if c == ' ' || c == '=' || c == '\n' || c == '\r' {continue}

            else {
                let last = match output.last() {
                    Some(l) => l,
                    None => break
                };
                if let Token::RightArrow = last {
                    self.index -= 1;
                    output.push(Token::TagValue(self.get_tag_val())) 
                }
                else if c == '\"' {
                    output.push(Token::AttributeValue(self.get_attr_val())) 
                }
                else {
                    self.index -= 1;
                    output.push(Token::Attribute(self.get_word())) 
                }
            }
        }

        output
    }

}