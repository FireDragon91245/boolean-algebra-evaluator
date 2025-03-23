use crate::tokenizer::Token;
use std::cmp::max;

macro_rules! invalid_char_error {
    ($self:expr) => {
        Err(format!(
            "Invalid character '{}' at pos {}\n\n{}\n{}{}\n",
            get_char_at_index(&$self.original_src, $self.position).unwrap_or_else(|| ' '),
            $self.position,
            $self.original_src,
            " ".repeat(max(0, $self.position)),
            "^^^"
        ))
    };
}

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Const(bool),
    Identifier(char),
    SingleOp {
        op: Token,
        operand: Box<Node>,
    },
    DoubleOp {
        op: Token,
        left: Box<Node>,
        right: Box<Node>,
    },
    Group(Box<Node>),
}

fn get_char_at_index(s: &String, i: usize) -> Option<char> {
    if i < s.len() {
        s.chars().nth(i)
    } else {
        None
    }
}

pub(crate) struct Parser {
    tokens: Vec<Token>,
    position: usize,
    original_src: String,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>, original_src: &String) -> Self {
        Parser {
            tokens,
            position: 0,
            original_src: original_src.clone(),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn consume(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Node, String> {
        self.parse_eq()
    }

    fn parse_eq(&mut self) -> Result<Node, String> {
        let mut left = self.parse_xor()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Equal => {
                    let op = self.consume().unwrap();
                    let right = self.parse_xor()?;
                    left = Node::DoubleOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_xor(&mut self) -> Result<Node, String> {
        let mut left = self.parse_or()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Xor => {
                    let op = self.consume().unwrap();
                    let right = self.parse_or()?;
                    left = Node::DoubleOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_or(&mut self) -> Result<Node, String> {
        let mut left = self.parse_and()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Or => {
                    let op = self.consume().unwrap();
                    let right = self.parse_and()?;
                    left = Node::DoubleOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Node, String> {
        let mut left = self.parse_not()?;

        while let Some(token) = self.peek() {
            match token {
                Token::And => {
                    let op = self.consume().unwrap();
                    let right = self.parse_not()?;
                    left = Node::DoubleOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Node, String> {
        while let Some(token) = self.peek() {
            match token {
                Token::Not => {
                    let op = self.consume().unwrap();
                    let right = self.parse_factor()?;
                    return Ok(Node::SingleOp {
                        op,
                        operand: Box::new(right),
                    });
                }
                _ => break,
            }
        }
        self.parse_factor()
    }

    fn parse_factor(&mut self) -> Result<Node, String> {
        if let Some(token) = self.consume() {
            match token {
                Token::Identifier(ident) => Ok(Node::Identifier(ident)),
                Token::ConstTrue => Ok(Node::Const(true)),
                Token::ConstFalse => Ok(Node::Const(false)),
                Token::GroupOpen => {
                    let node = self.parse_eq()?;
                    if let Some(Token::GroupClose) = self.consume() {
                        Ok(Node::Group(Box::new(node)))
                    } else {
                        Err(format!(
                            "Invalid character '{}' at pos {}\n\n{}\n{}{}\n",
                            get_char_at_index(&self.original_src, self.position)
                                .unwrap_or_else(|| ' '),
                            self.position,
                            self.original_src,
                            " ".repeat(max(0, self.position - 1)),
                            "^^^"
                        ))
                    }
                }
                _ => invalid_char_error!(self),
            }
        } else {
            invalid_char_error!(self)
        }
    }
}
