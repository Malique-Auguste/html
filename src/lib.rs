pub mod parser;
pub mod element;
pub mod lexer;
pub mod document;

use lexer::*;
use element::*;
use document::*;
use parser::*;

#[cfg(test)]
mod element_test {
    use super::*;

    #[test]
    fn as_str() {
        let mut doc = Document::new();
        doc.inner.insert(0, Element::new_populated("html".into(), vec![Attribute::new("color".into(), "green".into())], 0, vec![1, 2, 3], false));

        doc.inner.insert(1, Element::Inner("Hello World".into(), 0));
        doc.inner.insert(2, Element::new_populated("h1".into(), vec![], 0, vec![4], false));
        doc.inner.insert(3, Element::new_populated("span".into(), vec![], 0, vec![5], false));

        doc.inner.insert(4, Element::Inner("This is a Heading".into(), 2));
        doc.inner.insert(5, Element::new_populated("a".into(), vec![], 3, vec![6], false));

        doc.inner.insert(6, Element::Inner("This is a link within a span".into(), 5));
        

        let as_str = String::from(
"<!DOCTYPE html>
<html color = \"green\">
    Hello World
    <h1>
        This is a Heading
    </h1>
    <span>
        <a>
            This is a link within a span
        </a>
    </span>
</html>"
        );

        println!("{}\n\n{}\n", as_str, doc.as_str());
        assert_eq!(as_str, doc.as_str());
    }
}


#[cfg(test)]
mod lexer_test {
    use super::*;

    #[test]
    fn lex() {
        let str = String::from(
"<!DOCTYPE html>
<html>
    <h1 no_val_attribute color = \"green\">This is a header</h1>
    \"Not an Attribute Value\" 
    <span>This is a span.</span>
    Text after child tag
</html>"
        );

        let out = vec![
            Token::TagOpening("html".into()),
            Token::RightArrow,
            Token::TagOpening("h1".into()),
            Token::Attribute("no_val_attribute".into()),
            Token::Attribute("color".into()),
            Token::AttributeValue("green".into()),
            Token::RightArrow,
            Token::TagValue("This is a header".into()),
            Token::TagClosing("h1".into()),
            Token::RightArrow,
            Token::TagValue("\"Not an Attribute Value\" \n    ".into()),
            Token::TagOpening("span".into()),
            Token::RightArrow,
            Token::TagValue("This is a span.".into()),
            Token::TagClosing("span".into()),
            Token::RightArrow,
            Token::TagValue("Text after child tag\n".into()),
            Token::TagClosing("html".into()),
            Token::RightArrow,
        ];

        let mut lexer = Lexer::new(str.clone()).unwrap();
        assert_eq!(out, lexer.lex());
    }
}

#[cfg(test)]
mod parse_lex_test {
    use super::*;


    #[test]
    fn from_str_to_str() {
        let input = String::from(
"<!DOCTYPE html>
<html>
    <h1 no_val_attribute color = \"green\">This is a header</h1>
    \"Not an Attribute Value\"
    <span \"unused attribute val\">This is a span.</span>
    Text after child tag
</html>"
        );

        let mut lexer = Lexer::new(input.clone()).unwrap();
        let tokens = lexer.lex();

        let mut parser = Parser::new(tokens);

        let doc = parser.parse().unwrap();
        let doc = doc.as_str();
        println!("{}", doc);

        let output = String::from(
"<!DOCTYPE html>
<html>
    <h1 color = \"green\">
        This is a header
    </h1>
    \"Not an Attribute Value\"
    
    <span>
        This is a span.
    </span>
    Text after child tag

</html>"
        );

        assert_eq!(output, doc);
    }

    #[test]
    fn from_str_to_str2() {
        let input = std::fs::read_to_string("test.html").unwrap();

        let mut lexer = Lexer::new(input.clone()).unwrap();
        let tokens = lexer.lex();

        let mut parser = Parser::new(tokens);

        let doc = parser.parse().unwrap();
        let doc = doc.as_str();
        println!("{}", doc);
        panic!()
    }
}