use crate::lexer::Token;
use crate::document::Document;
use crate::element::*;


pub struct Parser {
    input: Vec<Token>,
    index: usize
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Parser {
        Parser {
            input,
            index: 0
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.index < self.input.len() {
            return Some(&self.input[self.index])
        }
        else {
            return None
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.index < self.input.len() {
            return Some(self.input.remove(self.index))
        }
        else {
            return None
        }
    }

    
    pub fn parse(&mut self) -> Result<Document, String> {
        let mut doc = Document::new();
        let mut current_element = 0;

        loop {
            let token = match self.next() {
                Some(t) => t,
                None => break
            };

            match token {
                Token::TagOpening(tag_name) => {
                    doc.inner.insert(doc.inner.keys().len(), Element::new_empty(tag_name, current_element));

                    let num = doc.inner.keys().len() - 1;

                    if num != 0 {
                        let mut current = doc.inner.get_mut(&current_element).unwrap();
                        
                        match current {
                            Element::UserDefined{children, ..} => {
                                children.push(num);
                            }
                            _ => unreachable!()
                        }
                    }

                    current_element = doc.inner.keys().len() - 1;
                },
                Token::Attribute(attr_name) => {
                    if let Token::AttributeValue(_) = self.peek().unwrap() {
                        if let Token::AttributeValue(attr_val) = self.next().unwrap() {
                            let mut current = doc.inner.get_mut(&current_element).unwrap();
                            match current {
                                Element::UserDefined{attributes, ..} => {
                                    attributes.push(Attribute::new(attr_name, attr_val))
                                }
                                _ => unreachable!()
                            }
                        }
                    }
                },

                Token::AttributeValue(_) | Token::RightArrow => {},

                Token::TagValue(tag_val) => {
                    doc.inner.insert(doc.inner.keys().len(), Element::Inner(tag_val, current_element));

                    let num = doc.inner.keys().len() - 1;
                    let mut current = doc.inner.get_mut(&current_element).unwrap();

                    match current {
                        Element::UserDefined{children, ..} => {
                            children.push(num);
                        }
                        _ => unreachable!()
                    }
                },

                Token::TagClosing(tag_name_close) => {
                    let current = doc.inner.get(&current_element).unwrap();

                    match current {
                        Element::UserDefined{tag_name, parent, ..} => {
                            if *tag_name == tag_name_close {
                                current_element = *parent;
                            }
                            else {
                                return Err("A closing tag doesn't have a matching opening tag".into())
                            }
                        }
                        _ => unreachable!()
                    }
                }
            }


        }

        Ok(doc)
    }
    

}

