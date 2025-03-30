use crate::tokenizer::Token;
use std::collections::{HashMap, VecDeque};
use crate::ast::Node;

pub(crate) struct Evaluator {
    ast: Node,
    ident_bit_index: HashMap<char, usize>,
}

pub(crate) struct EvaluatorPassResult {
    pub(crate) result: bool,
    pub(crate) ident_states: Vec<(char, bool)>,
}

impl Evaluator {
    pub(crate) fn new(ast: Node) -> Self {
        let mut res = Evaluator {
            ast,
            ident_bit_index: HashMap::new(),
        };

        res.calc_ident_bit_index();

        res
    }

    pub(crate) fn get_identifiers(&self) -> impl Iterator<Item = char> + '_ {
        self.ident_bit_index.keys().cloned()
    }

    fn calc_ident_bit_index(&mut self) {
        let mut idents: Vec<char> = Vec::new();

        let mut to_visit: VecDeque<&Node> = VecDeque::new();
        to_visit.push_back(&self.ast);
        while let Some(node) = to_visit.pop_front() {
            match node {
                Node::Const(_) => {}
                Node::SingleOp { operand, .. } => {
                    to_visit.push_back(operand);
                }
                Node::DoubleOp { left, right, .. } => {
                    to_visit.push_back(left);
                    to_visit.push_back(right);
                }
                Node::Group(g) => {
                    to_visit.push_back(g);
                }
                Node::Identifier(c) => {
                    if !idents.contains(&c) {
                        idents.push(*c);
                    }
                }
            }
        }

        idents.sort();

        for (i, c) in idents.iter().enumerate() {
            self.ident_bit_index.insert(*c, i);
        }
    }

    pub(crate) fn evaluate(&self, pass: usize) -> bool {
        self.evaluate_node(&self.ast, pass)
    }

    pub(crate) fn evaluate_iter(&self) -> impl Iterator<Item = EvaluatorPassResult> + '_ {
        let ident_count = self.ident_bit_index.len();
        (0..(1 << ident_count)).map(
            move |pass| EvaluatorPassResult {
                result: self.evaluate(pass),
                ident_states: self
                    .ident_bit_index
                    .iter()
                    .map(|(c, i)| (*c, pass & ((1 << i) as usize) != 0))
                    .collect(),
            },
        )
    }

    pub(crate) fn get_ident_bit(&self, c: char, pass: usize) -> bool {
        let index = self.ident_bit_index.get(&c).unwrap();
        pass & ((1 << index) as usize) != 0
    }

    fn evaluate_node(&self, node: &Node, pass: usize) -> bool {
        match node {
            Node::Const(b) => *b,
            Node::SingleOp { op, operand } => match op {
                Token::Not => !self.evaluate_node(operand, pass),
                _ => {
                    panic!("Invalid operator, please report the expression that caused this error")
                }
            },
            Node::DoubleOp { op, left, right } => match op {
                Token::And => self.evaluate_node(left, pass) && self.evaluate_node(right, pass),
                Token::Or => self.evaluate_node(left, pass) || self.evaluate_node(right, pass),
                Token::Xor => self.evaluate_node(left, pass) ^ self.evaluate_node(right, pass),
                _ => {
                    panic!("Invalid operator, please report the expression that caused this error")
                }
            },
            Node::Group(g) => self.evaluate_node(g, pass),
            Node::Identifier(ident) => self.get_ident_bit(*ident, pass),
        }
    }
}
