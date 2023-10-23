use dom;
use std::collections::HashMap;

struct Parser{
    pos: usize,
    input: String
}

impl Parser {
    // read current character without consuming it
    fn peek(&self) -> char {
        self.input.chars().nth(self.pos).unwrap()
    }
    // check if next characters starts with the given string
    fn starts_with(&self, needle: &str) -> bool {
        return self.input[self.pos..].starts_with(needle);
    }
    // return true if all input has been consumed
    fn is_at_end_of_file(&self) -> bool {
        return self.pos >= self.input.len();
    }
    // return the current character and advance position
    fn next_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, '\0'));
        self.pos += next_pos;
        return cur_char;
    }
    // consume the characters until returns false
    fn consume_while<F>(&mut self, test: F) -> String where F:Fn(char) -> bool {
        let mut result = String::new();
        while !self.is_at_end_of_file() && test(self.next_char()) {
            result.push(self.next_char());
        }
        result
    }
    // consume and discard zero or more whitespace characters
    fn consume_whitespace(&mut self) {
        let whitespace = " \t\r\n";
        self.consume_while(|c| whitespace.contains(c));
    }
    // parse a tag or attribute name
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false
        })
    }
    // Parse a quoted value.
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.next_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.next_char() == open_quote);
        return value;
    }
    // Parse a single name="value" pair.
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.next_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }
    // Parse a list of name="value" pairs, separated by whitespace.
    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }
    // Parse a single element, including its open tag, contents, and closing tag.
    fn parse_element(&mut self) -> Node {
        // Opening tag.
        assert!(self.next_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.next_char() == '>');

        // Contents.
        let children = self.parse_nodes();

        // Closing tag.
        assert!(self.next_char() == '<');
        assert!(self.next_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.next_char() == '>');

        return elem(tag_name, attrs, children);
    }
    // Parse a text node.
    fn parse_text(&mut self) -> dom::Node {
       dom::text(self.consume_while(|c| c != '<'))
    }
    // Parse a single node.
    fn parse_node(&mut self) -> Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _   => self.parse_text()
        }
    }

    // Parse a sequence of sibling nodes.
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.is_at_end_of_file() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }
}

// Parse an HTML document and return the root element.
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser { pos: 0, input: source }.parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one.
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}


