#[derive(Debug, PartialEq)]
pub enum Element {
    Inner(String, usize),       //(value, parent)
    UserDefined{tag_name: String, attributes: Vec<Attribute>, parent: usize, children: Vec<usize>, self_closing: bool},
}

impl Element {
    pub fn new_populated(tag_name: String, attributes: Vec<Attribute>, parent: usize, children: Vec<usize>, self_closing: bool) -> Element {
        Element::UserDefined {
            tag_name,
            attributes,
            parent,
            children,
            self_closing
        }
    }

    pub fn new_empty(tag_name: String, parent: usize, self_closing: bool) -> Element {
        Element::UserDefined {
            tag_name,
            attributes: Vec::new(),
            parent,
            children: Vec::new(),
            self_closing
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    name: String,
    value: String
}

impl Attribute {
    pub fn new(name: String, value: String) -> Attribute {
        Attribute {
            name,
            value
        }
    }

    pub fn as_str(&self) -> String {
        format!("{} = \"{}\"", self.name, self.value)
    }
}