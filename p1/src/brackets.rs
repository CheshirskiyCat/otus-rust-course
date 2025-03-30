#![allow(dead_code)]
/*
    Дана строка, состоящая только из символов '{', '}', '(', ')', '[', ']'.
    Такая строка является корректной, если:
    - каждой открывающей скобке соответствует закрывающая того же типа
    - соблюдается порядок закрытия скобок
    - для каждой закрывающей скобки есть соответствующая открывающая пара

    Написать функцию, которая проверит корректность данной строки.
*/
use crate::brackets::BracketType::{Closing, Opening};

struct Bracket {
    b_style: BracketStyle,
    b_type: BracketType,
}

#[derive(PartialEq, Eq)]
enum BracketStyle {
    Round,
    Curly,
    Box,
}

#[derive(PartialEq, Eq)]
enum BracketType {
    Opening,
    Closing,
}

fn map_to_bracket(c: char) -> Bracket {
    use BracketStyle::*;
    use BracketType::*;
    match c {
        '(' => Bracket { b_style : Round, b_type : Opening },
        ')' => Bracket { b_style : Round, b_type : Closing },
        '{' => Bracket { b_style : Curly, b_type : Opening },
        '}' => Bracket { b_style : Curly, b_type : Closing },
        '[' => Bracket { b_style : Box, b_type : Opening },
        ']' => Bracket { b_style : Box, b_type : Closing },
        _ => panic!("wrong bracket character"),
    }
}

fn validate_paren(s: &str) -> bool {
    let mut brackets: Vec<Bracket> = vec![];
    for c in s.chars() {
        let bracket = map_to_bracket(c);
        if let Some(last) = brackets.last() {
            if last.b_type == Opening {
                if last.b_style == bracket.b_style && bracket.b_type == Closing {
                    brackets.pop();
                } else if bracket.b_type == Opening {
                    brackets.push(bracket)
                } else {
                    break;
                }
            };
        } else {
            let is_closing_bracket = bracket.b_type == Closing;
            brackets.push(bracket);
            if is_closing_bracket { break }
        };
    }
    brackets.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(validate_paren("()"), true);
        assert_eq!(validate_paren("()[]{}"), true);
        assert_eq!(validate_paren("({[]()})"), true);
        assert_eq!(validate_paren("(}"), false);
        assert_eq!(validate_paren("()]"), false);
        assert_eq!(validate_paren("(){"), false);
        assert_eq!(validate_paren("(})"), false);
    }
}
