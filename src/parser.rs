use crate::lexer::Token;
use crate::document::Document;
use crate::element::*;

const SELF_CLOSING_TAGS: [&str; 14] = ["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"];

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
        let mut self_closing = false;

        loop {
            let token = match self.next() {
                Some(t) => t,
                None => break
            };
            //println!("{:?}", token);
            //println!("{:?}", self_closing);


            match token {
                Token::TagOpening(tag_name) => {
                    if SELF_CLOSING_TAGS.iter().any(|x| **x == tag_name || (String::from(*x) + "/") == tag_name) {
                        self_closing = true;
                        doc.inner.insert(doc.inner.keys().len(), Element::new_empty(tag_name.clone(), current_element, true));

                    }
                    else {
                        doc.inner.insert(doc.inner.keys().len(), Element::new_empty(tag_name.clone(), current_element, false));
                    }
                    let num = doc.inner.keys().len() - 1;

                    if num != 0 {
                        let current = doc.inner.get_mut(&current_element).unwrap();
                        
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
                            let current = doc.inner.get_mut(&current_element).unwrap();
                            match current {
                                Element::UserDefined{attributes, ..} => {
                                    attributes.push(Attribute::new(attr_name, attr_val))
                                }
                                _ => unreachable!()
                            }
                        }
                    }
                },

                Token::RightArrow => {
                    if self_closing {
                        let current = doc.inner.get(&current_element).unwrap();

                        match current {
                            Element::UserDefined{parent, ..} => {
                                current_element = *parent;
                                self_closing = false;
                            }
                            _ => unreachable!()
                        }
                    }
                }

                Token::AttributeValue(_) => {},

                Token::TagValue(tag_val) => {
                    doc.inner.insert(doc.inner.keys().len(), Element::Inner(tag_val, current_element));

                    let num = doc.inner.keys().len() - 1;
                    let current = doc.inner.get_mut(&current_element).unwrap();

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

            //println!("{:#?}", doc);
        }

        Ok(doc)
    }
    

}

