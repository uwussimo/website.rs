use proc_macro::{Delimiter, TokenStream, TokenTree};

#[proc_macro]
pub fn jsx(input: TokenStream) -> TokenStream {
    let tokens = input.into_iter().collect::<Vec<_>>();
    let mut parser = Parser::new(tokens);
    let nodes = parser.parse_nodes(None);
    format!("::dioxus::prelude::rsx! {{ {} }}", nodes)
        .parse()
        .unwrap()
}

struct Parser {
    tokens: Vec<TokenTree>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<TokenTree>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse_nodes(&mut self, closing: Option<&str>) -> String {
        let mut out = String::new();

        while !self.is_eof() {
            if self.starts_closing_tag() {
                let name = self.consume_closing_tag();
                if closing.is_none() || closing == Some(name.as_str()) {
                    break;
                }
                continue;
            }

            if self.peek_punct('<') {
                out.push_str(&self.parse_element());
            } else if let Some(TokenTree::Literal(lit)) = self.peek() {
                out.push_str(&lit.to_string());
                out.push(' ');
                self.pos += 1;
            } else if let Some(TokenTree::Group(group)) = self.peek() {
                if group.delimiter() == Delimiter::Brace {
                    out.push('{');
                    out.push_str(&group.stream().to_string());
                    out.push_str("} ");
                    self.pos += 1;
                } else {
                    self.pos += 1;
                }
            } else {
                self.pos += 1;
            }
        }

        out
    }

    fn parse_element(&mut self) -> String {
        self.expect_punct('<');
        let tag = self.parse_path();
        let attrs = self.parse_attrs();

        if self.peek_punct('/') {
            self.expect_punct('/');
            self.expect_punct('>');
            return format!("{tag} {{ {attrs} }} ");
        }

        self.expect_punct('>');
        let children = self.parse_nodes(Some(&tag));
        format!("{tag} {{ {attrs} {children} }} ")
    }

    fn parse_attrs(&mut self) -> String {
        let mut out = String::new();

        while !self.is_eof() && !self.peek_punct('>') && !self.peek_punct('/') {
            let Some(name) = self.consume_ident() else {
                self.pos += 1;
                continue;
            };

            let name = if name == "className" {
                "class".to_string()
            } else {
                name
            };

            if self.peek_punct('=') {
                self.expect_punct('=');
                let value = self.parse_attr_value();
                out.push_str(&format!("{name}: {value}, "));
            } else {
                out.push_str(&format!("{name}: true, "));
            }
        }

        out
    }

    fn parse_attr_value(&mut self) -> String {
        match self.next() {
            Some(TokenTree::Literal(lit)) => lit.to_string(),
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
                group.stream().to_string()
            }
            Some(token) => token.to_string(),
            None => String::new(),
        }
    }

    fn parse_path(&mut self) -> String {
        let mut out = self.consume_ident().unwrap_or_default();

        while self.peek_double_colon() {
            self.expect_punct(':');
            self.expect_punct(':');
            if let Some(segment) = self.consume_ident() {
                out.push_str("::");
                out.push_str(&segment);
            }
        }

        out
    }

    fn consume_closing_tag(&mut self) -> String {
        self.expect_punct('<');
        self.expect_punct('/');
        let name = self.parse_path();
        self.expect_punct('>');
        name
    }

    fn starts_closing_tag(&self) -> bool {
        self.peek_punct_at(self.pos, '<') && self.peek_punct_at(self.pos + 1, '/')
    }

    fn peek_double_colon(&self) -> bool {
        self.peek_punct_at(self.pos, ':') && self.peek_punct_at(self.pos + 1, ':')
    }

    fn consume_ident(&mut self) -> Option<String> {
        match self.next() {
            Some(TokenTree::Ident(ident)) => Some(ident.to_string()),
            Some(other) => {
                self.pos -= 1;
                let _ = other;
                None
            }
            None => None,
        }
    }

    fn expect_punct(&mut self, ch: char) {
        if !self.peek_punct(ch) {
            return;
        }
        self.pos += 1;
    }

    fn peek(&self) -> Option<&TokenTree> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<TokenTree> {
        let token = self.tokens.get(self.pos).cloned();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn peek_punct(&self, ch: char) -> bool {
        self.peek_punct_at(self.pos, ch)
    }

    fn peek_punct_at(&self, pos: usize, ch: char) -> bool {
        matches!(self.tokens.get(pos), Some(TokenTree::Punct(punct)) if punct.as_char() == ch)
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
