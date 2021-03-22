use crate::element::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Document {
    pub inner: HashMap<usize, Element>
}

impl Document {
    pub fn new() -> Document {
        Document { inner: HashMap::new() }
    }

    pub fn as_str(&self) -> String {
        let mut output = String::new();
        let element = self.inner.get(&0).unwrap();

        match element {
            Element::UserDefined{tag_name, attributes, children, ..} => {
                output = format!("<!DOCTYPE html>\n{}", self.element_as_str(&0, 0).unwrap());
            }    
            Element::Inner(val, ..) => output = format!("<!DOCTYPE html>\n{}",val)
        }
        output
    }

    fn element_as_str(&self, element: &usize, depth: usize) -> Result<String, String> {
        let element = self.inner.get(element).unwrap();

        match element {
            Element::Inner(val, ..) => Ok(val.clone()),
            Element::UserDefined{tag_name, attributes, children, self_closing, ..} => {
                let self_spacing = std::iter::repeat("    ").take(depth).collect::<String>();
                let child_spacing = std::iter::repeat("    ").take(depth + 1).collect::<String>();

                let mut attributes: Vec<String> = attributes.iter().map(|a| a.as_str()).collect();
                let mut output = attributes.drain(0..).fold(String::from("<") + tag_name, |out, attr| out + " " + &attr);

                if *self_closing {
                    return Ok(format!("{}/>", output))
                }

                let mut children: Vec<String> = children.iter().map(|c| self.element_as_str(c, depth + 1).unwrap()).collect();
                

                output = children.drain(0..).fold(output + ">", |out, child| out + "\n" + &child_spacing + &child);
                Ok(format!("{}\n{}</{}>", output, self_spacing, tag_name))
            }
        }
    }
}