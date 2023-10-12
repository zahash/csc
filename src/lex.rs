use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token<'text> {
    Symbol(&'static str),
    Ident(&'text str),
    Decimal(f64),
}

lazy_static! {
    static ref IDENT_REGEX: Regex = Regex::new(r#"^[A-Za-z_][A-Za-z0-9_]*"#).unwrap();
    static ref FLOAT_REGEX: Regex = Regex::new(r"^(\d+\.\d+|\d+\.|\.\d+|\d+)").unwrap();
}

#[derive(Debug)]
pub enum LexError {
    InvalidToken { pos: usize },
}

pub fn lex(text: &str) -> Result<Vec<Token>, LexError> {
    match text.is_empty() {
        true => Ok(vec![]),
        false => {
            let mut tokens = vec![];
            let mut pos = 0;

            loop {
                while let Some(" ") | Some("\n") = text.get(pos..pos + 1) {
                    pos += 1;
                }

                if pos >= text.len() {
                    break;
                }

                let (token, next_pos) = lex_token(text, pos)?;
                tokens.push(token);
                pos = next_pos;
            }

            Ok(tokens)
        }
    }
}

fn lex_token(text: &str, pos: usize) -> Result<(Token, usize), LexError> {
    lex_ident(text, pos)
        .or(lex_decimal(text, pos))
        .or(lex_symbol(text, pos, "{"))
        .or(lex_symbol(text, pos, "}"))
        .or(lex_symbol(text, pos, "["))
        .or(lex_symbol(text, pos, "]"))
        .or(lex_symbol(text, pos, "("))
        .or(lex_symbol(text, pos, ")"))
        .or(lex_symbol(text, pos, "..."))
        .or(lex_symbol(text, pos, "."))
        .or(lex_symbol(text, pos, ","))
        .or(lex_symbol(text, pos, ":"))
        .or(lex_symbol(text, pos, ";"))
        .or(lex_symbol(text, pos, "->"))
        .or(lex_symbol(text, pos, "++"))
        .or(lex_symbol(text, pos, "+="))
        .or(lex_symbol(text, pos, "+"))
        .or(lex_symbol(text, pos, "--"))
        .or(lex_symbol(text, pos, "-="))
        .or(lex_symbol(text, pos, "-"))
        .or(lex_symbol(text, pos, "*="))
        .or(lex_symbol(text, pos, "*"))
        .or(lex_symbol(text, pos, "/="))
        .or(lex_symbol(text, pos, "/"))
        .or(lex_symbol(text, pos, "%="))
        .or(lex_symbol(text, pos, "%"))
        .or(lex_symbol(text, pos, "^="))
        .or(lex_symbol(text, pos, "^"))
        .or(lex_symbol(text, pos, "=="))
        .or(lex_symbol(text, pos, "!="))
        .or(lex_symbol(text, pos, "="))
        .or(lex_symbol(text, pos, "&&"))
        .or(lex_symbol(text, pos, "&="))
        .or(lex_symbol(text, pos, "&"))
        .or(lex_symbol(text, pos, "||"))
        .or(lex_symbol(text, pos, "|="))
        .or(lex_symbol(text, pos, "|"))
        .or(lex_symbol(text, pos, "!"))
        .or(lex_symbol(text, pos, "?"))
        .or(lex_symbol(text, pos, "~"))
        .or(lex_symbol(text, pos, "<<="))
        .or(lex_symbol(text, pos, "<<"))
        .or(lex_symbol(text, pos, ">>="))
        .or(lex_symbol(text, pos, ">>"))
        .or(lex_symbol(text, pos, "<="))
        .or(lex_symbol(text, pos, ">="))
        .or(lex_symbol(text, pos, "<"))
        .or(lex_symbol(text, pos, ">"))
        .ok_or(LexError::InvalidToken { pos })
}

fn lex_ident(text: &str, pos: usize) -> Option<(Token, usize)> {
    let (token, pos) = lex_with_pattern(text, pos, &IDENT_REGEX)?;
    Some((Token::Ident(token), pos))
}

fn lex_decimal(text: &str, pos: usize) -> Option<(Token, usize)> {
    let (token, pos) = lex_with_pattern(text, pos, &FLOAT_REGEX)?;
    Some((Token::Decimal(token.parse().ok()?), pos))
}

fn lex_symbol(text: &str, pos: usize, symbol: &'static str) -> Option<(Token<'static>, usize)> {
    if let Some(substr) = text.get(pos..) {
        if substr.starts_with(symbol) {
            return Some((Token::Symbol(symbol), pos + symbol.len()));
        }
    }

    None
}

fn lex_with_pattern<'text>(
    text: &'text str,
    pos: usize,
    pat: &Regex,
) -> Option<(&'text str, usize)> {
    if let Some(slice) = text.get(pos..text.len()) {
        if let Some(m) = pat.find(slice) {
            assert!(
                m.start() == 0,
                "put caret ^ to match the text from the `pos` (text is sliced to start from pos)"
            );
            return Some((m.as_str(), pos + m.end()));
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_all() {
        let src = r#"

        idEnt_123 123 123. .123 123.123 {}[]()
        ....,:;+++--->-*/%^!===*=/=%=+=-=&=^=|==&&&|||!?~<<<<=<=<>>=>>>=>
        "#;

        use Token::*;

        match lex(src) {
            Ok(tokens) => assert_eq!(
                vec![
                    Ident("idEnt_123"),
                    Decimal(123.0),
                    Decimal(123.0),
                    Decimal(0.123),
                    Decimal(123.123),
                    Symbol("{"),
                    Symbol("}"),
                    Symbol("["),
                    Symbol("]"),
                    Symbol("("),
                    Symbol(")"),
                    Symbol("..."),
                    Symbol("."),
                    Symbol(","),
                    Symbol(":"),
                    Symbol(";"),
                    Symbol("++"),
                    Symbol("+"),
                    Symbol("--"),
                    Symbol("->"),
                    Symbol("-"),
                    Symbol("*"),
                    Symbol("/"),
                    Symbol("%"),
                    Symbol("^"),
                    Symbol("!="),
                    Symbol("=="),
                    Symbol("*="),
                    Symbol("/="),
                    Symbol("%="),
                    Symbol("+="),
                    Symbol("-="),
                    Symbol("&="),
                    Symbol("^="),
                    Symbol("|="),
                    Symbol("="),
                    Symbol("&&"),
                    Symbol("&"),
                    Symbol("||"),
                    Symbol("|"),
                    Symbol("!"),
                    Symbol("?"),
                    Symbol("~"),
                    Symbol("<<"),
                    Symbol("<<="),
                    Symbol("<="),
                    Symbol("<"),
                    Symbol(">>="),
                    Symbol(">>"),
                    Symbol(">="),
                    Symbol(">")
                ],
                tokens
            ),

            Err(LexError::InvalidToken { pos }) => assert!(false, "{}", &src[pos..]),
        }
    }
}
