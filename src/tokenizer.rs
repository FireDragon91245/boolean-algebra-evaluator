use std::cmp::max;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    And,
    Or,
    Not,
    Xor,
    Equal,
    GroupOpen,
    GroupClose,
    ConstTrue,
    ConstFalse,
    Identifier(char),
}

const VALID_IDENTIFIERS: &str = "abcdefghijklmnopqrstuvwxyz";

fn get_char_slice(s: &str, char_start: usize, char_len: usize) -> Option<&str> {
    let chars: Vec<_> = s.char_indices().collect();
    let start_byte: usize = chars.get(char_start)?.0;
    let end_byte: usize = chars.get(char_start + char_len - 1)?.0;
    Some(&s[start_byte..=end_byte])
}

pub(crate) fn tokenize(str: &String, allow_identifiers: bool) -> Result<Vec<Token>, String> {
    let mut result: Vec<Token> = Vec::new();
    let mut i = 0;
    while i < str.chars().count() {
        let c = str.chars().nth(i).unwrap();
        i += 1;
        match c {
            ' ' => continue,
            '(' => result.push(Token::GroupOpen),
            ')' => result.push(Token::GroupClose),
            '&' => result.push(Token::And),
            '|' => result.push(Token::Or),
            '^' => result.push(Token::Xor),
            '!' => result.push(Token::Not),
            '=' => result.push(Token::Equal),
            '1' => result.push(Token::ConstTrue),
            '0' => result.push(Token::ConstFalse),
            _ => {
                if let Some(peak) = get_char_slice(&str, i - 1, str.chars().count() - i + 1) {
                    if peak.starts_with("true") {
                        result.push(Token::ConstTrue);
                        i += "true".len() - 1;
                    } else if peak.starts_with("false") {
                        result.push(Token::ConstFalse);
                        i += "false".len() - 1;
                    } else if VALID_IDENTIFIERS.contains(c) && allow_identifiers {
                        result.push(Token::Identifier(c));
                    } else {
                        return Err(format!(
                            "Invalid character '{}' at pos {}\n\n{}\n{}{}\n",
                            c,
                            i + 1,
                            str,
                            " ".repeat(max(0, i - 1)),
                            "^^^"
                        ));
                    }
                } else if VALID_IDENTIFIERS.contains(c) && allow_identifiers {
                    result.push(Token::Identifier(c));
                } else {
                    return Err(format!(
                        "Invalid character '{}' at pos {}\n\n{}\n{}{}\n",
                        c,
                        i + 1,
                        str,
                        " ".repeat(max(0, i - 1)),
                        "^^^"
                    ));
                }
            }
        }
    }
    Ok(result)
}
