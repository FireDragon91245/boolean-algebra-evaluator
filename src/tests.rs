#[cfg(test)]
mod test
{
    use crate::ast::Node;
    use crate::tokenizer::{tokenize, Token};

    #[test]
    fn test_tokens_spaces_ignored() {
        let tokens =tokenize(&"a & b | c".to_string(), true).unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Identifier('a'));
        assert_eq!(tokens[1], Token::And);
        assert_eq!(tokens[2], Token::Identifier('b'));
        assert_eq!(tokens[3], Token::Or);
        assert_eq!(tokens[4], Token::Identifier('c'));
    }

    #[test]
    fn test_tokens_error_invalid_token() {
        let tokens = tokenize(&"a|?".to_string(), true);
        assert!(tokens.is_err());
    }

    #[test]
    fn test_tokens_idents_not_allowed()
    {
        let tokens = tokenize(&"a & b | c".to_string(), false);
        assert!(tokens.is_err());
    }

    #[test]
    fn test_ast_smal_valid() {
        let tokens = [Token::Identifier('a'), Token::And, Token::Identifier('b')];
        let ast = crate::ast::Parser::new(tokens.into(), &"a & b".to_string()).parse().unwrap();
        assert_eq!(ast, Node::DoubleOp {
            op: Token::And,
            left: Box::new(Node::Identifier('a')),
            right: Box::new(Node::Identifier('b')),
        });
    }

    #[test]
    fn test_ast_error_missing_operand() {
        let tokens = [Token::Identifier('a'), Token::And];
        let ast = crate::ast::Parser::new(tokens.into(), &"a &".to_string()).parse();
        assert!(ast.is_err());
    }

    #[test]
    fn test_ast_unfinished_group() {
        let tokens = [Token::GroupOpen, Token::Identifier('a'), Token::And, Token::Identifier('b')];
        let ast = crate::ast::Parser::new(tokens.into(), &"(a & b".to_string()).parse();
        assert!(ast.is_err());
    }

    #[test]
    fn test_ast_invalid_double_op() {
        let tokens = [Token::Identifier('a'), Token::And, Token::And, Token::Identifier('b')];
        let ast = crate::ast::Parser::new(tokens.into(), &"a & & b".to_string()).parse();
        assert!(ast.is_err());
    }

    #[test]
    fn test_evaluator_and() {
        let ast = Node::DoubleOp {
            op: Token::And,
            left: Box::new(Node::Identifier('a')),
            right: Box::new(Node::Identifier('b')),
        };
        let evaluator = crate::evaluator::Evaluator::new(ast);
        assert_eq!(evaluator.evaluate(0), false);
        assert_eq!(evaluator.evaluate(1), false);
        assert_eq!(evaluator.evaluate(2), false);
        assert_eq!(evaluator.evaluate(3), true);
    }

    #[test]
    fn test_evaluator_or() {
        let ast = Node::DoubleOp {
            op: Token::Or,
            left: Box::new(Node::Identifier('a')),
            right: Box::new(Node::Identifier('b')),
        };
        let evaluator = crate::evaluator::Evaluator::new(ast);
        assert_eq!(evaluator.evaluate(0), false);
        assert_eq!(evaluator.evaluate(1), true);
        assert_eq!(evaluator.evaluate(2), true);
        assert_eq!(evaluator.evaluate(3), true);
    }

    #[test]
    fn test_evaluator_xor() {
        let ast = Node::DoubleOp {
            op: Token::Xor,
            left: Box::new(Node::Identifier('a')),
            right: Box::new(Node::Identifier('b')),
        };
        let evaluator = crate::evaluator::Evaluator::new(ast);
        assert_eq!(evaluator.evaluate(0), false);
        assert_eq!(evaluator.evaluate(1), true);
        assert_eq!(evaluator.evaluate(2), true);
        assert_eq!(evaluator.evaluate(3), false);
    }

    #[test]
    fn test_evaluator_not() {
        let ast = Node::SingleOp {
            op: Token::Not,
            operand: Box::new(Node::Identifier('a')),
        };
        let evaluator = crate::evaluator::Evaluator::new(ast);
        assert_eq!(evaluator.evaluate(0), true);
        assert_eq!(evaluator.evaluate(1), false);
    }

    #[test]
    fn test_evaluator_equals() {
        let ast = Node::DoubleOp {
            op: Token::Equal,
            left: Box::new(Node::Identifier('a')),
            right: Box::new(Node::Identifier('b')),
        };
        let evaluator = crate::evaluator::Evaluator::new(ast);
        assert_eq!(evaluator.evaluate(0), true);
        assert_eq!(evaluator.evaluate(1), false);
        assert_eq!(evaluator.evaluate(2), false);
        assert_eq!(evaluator.evaluate(3), true);
    }
}