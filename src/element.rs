#[derive(Debug, PartialEq)]
pub enum Element {
    Inner(String, usize),       //(value, parent)
    UserDefined{tag_name: String, attributes: Vec<Attribute>, parent: usize, children: Vec<usize>},
}

impl Element {
    pub fn new_populated(tag_name: String, attributes: Vec<Attribute>, parent: usize, children: Vec<usize>) -> Element {
        Element::UserDefined {
            tag_name,
            attributes,
            parent,
            children,
        }
    }

    pub fn new_empty(tag_name: String, parent: usize) -> Element {
        Element::UserDefined {
            tag_name,
            attributes: Vec::new(),
            parent,
            children: Vec::new(),
        }
    }

    /*
    pub fn as_str(&self, depth: usize) -> Result<String, String> {
        match self {
            Element::Inner(val) => Ok(val.clone()),
            Element::UserDefined{tag_name, attributes, children, is_root} => {
                if *is_root {
                    if depth != 0 {
                        return Err("A root element cannot be a child".into());
                    }

                    let child_spacing = std::iter::repeat("    ").take(depth + 1).collect::<String>();

                    let mut attributes: Vec<String> = attributes.iter().map(|a| a.as_str()).collect();
                    let mut children: Vec<String> = children.iter().map(|c| c.as_str(depth + 1).unwrap()).collect();

                    let mut output = attributes.drain(0..).fold(String::from("<!DOCTYPE html>\n") + "<" + tag_name, |out, attr| out + " " + &attr) + ">";
                    

                    output = children.drain(0..).fold(output, |out, child| out + "\n" + &child_spacing + &child);
                    Ok(format!("{}\n</{}>", output, tag_name))
                }
                else {
                    let self_spacing = std::iter::repeat("    ").take(depth).collect::<String>();
                    let child_spacing = std::iter::repeat("    ").take(depth + 1).collect::<String>();

                    let mut attributes: Vec<String> = attributes.iter().map(|a| a.as_str()).collect();
                    let mut children: Vec<String> = children.iter().map(|c| c.as_str(depth + 1).unwrap()).collect();

                    let mut output = attributes.drain(0..).fold(String::from("<") + tag_name, |out, attr| out + " " + &attr) + ">";
                    

                    output = children.drain(0..).fold(output, |out, child| out + "\n" + &child_spacing + &child);
                    Ok(format!("{}\n{}</{}>", output, self_spacing, tag_name))
                }
            }
        }
    }
    */
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