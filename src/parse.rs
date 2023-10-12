use std::fmt::{self, Display, Formatter};

use crate::lex::Token;

#[derive(Debug)]
pub enum ParseError {
    SyntaxError(usize, &'static str),
    Expected(Token<'static>, usize),
    IncompleteParse(usize),
}

pub fn parse<'text>(tokens: &[Token<'text>]) -> Result<Expr<'text>, ParseError> {
    let (expr, pos) = parse_expr(&tokens, 0)?;
    match pos < tokens.len() {
        true => Err(ParseError::IncompleteParse(pos).into()),
        false => Ok(expr),
    }
}

pub type Expr<'text> = AssignmentExpr<'text>;

fn parse_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
) -> Result<(Expr<'text>, usize), ParseError> {
    parse_assignment_expr(tokens, pos)
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentExpr<'text> {
    AdditiveExpr(AdditiveExpr<'text>),
    Assign(&'text str, Box<AssignmentExpr<'text>>),
    MulAssign(&'text str, Box<AssignmentExpr<'text>>),
    DivAssign(&'text str, Box<AssignmentExpr<'text>>),
    ModAssign(&'text str, Box<AssignmentExpr<'text>>),
    AddAssign(&'text str, Box<AssignmentExpr<'text>>),
    SubAssign(&'text str, Box<AssignmentExpr<'text>>),
}

fn parse_assignment_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
) -> Result<(AssignmentExpr<'text>, usize), ParseError> {
    if let Some(Token::Ident(ident)) = tokens.get(pos) {
        if let Some(op) = tokens.get(pos + 1) {
            if op == &Token::Symbol("=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 2)?;
                return Ok((AssignmentExpr::Assign(ident, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("*=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 2)?;
                return Ok((AssignmentExpr::MulAssign(ident, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("/=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 2)?;
                return Ok((AssignmentExpr::DivAssign(ident, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("%=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 2)?;
                return Ok((AssignmentExpr::ModAssign(ident, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("+=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 2)?;
                return Ok((AssignmentExpr::AddAssign(ident, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("-=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 2)?;
                return Ok((AssignmentExpr::SubAssign(ident, Box::new(rhs)), pos));
            }
        }
    }

    let (expr, pos) = parse_additive_expr(tokens, pos)?;
    Ok((expr.into(), pos))
}

#[derive(Debug, PartialEq, Clone)]
pub enum AdditiveExpr<'text> {
    MultiplicativeExpr(MultiplicativeExpr<'text>),
    Add(Box<AdditiveExpr<'text>>, MultiplicativeExpr<'text>),
    Sub(Box<AdditiveExpr<'text>>, MultiplicativeExpr<'text>),
}

fn parse_additive_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
) -> Result<(AdditiveExpr<'text>, usize), ParseError> {
    let (lhs, mut pos) = parse_multiplicative_expr(tokens, pos)?;
    let mut lhs = lhs.into();
    while let Some(token) = tokens.get(pos) {
        match token {
            Token::Symbol("+") => {
                let (rhs, next_pos) = parse_multiplicative_expr(tokens, pos + 1)?;
                pos = next_pos;
                lhs = AdditiveExpr::Add(Box::new(lhs), rhs);
            }
            Token::Symbol("-") => {
                let (rhs, next_pos) = parse_multiplicative_expr(tokens, pos + 1)?;
                pos = next_pos;
                lhs = AdditiveExpr::Sub(Box::new(lhs), rhs);
            }
            _ => break,
        }
    }
    Ok((lhs, pos))
}

#[derive(Debug, PartialEq, Clone)]
pub enum MultiplicativeExpr<'text> {
    ExponentialExpr(ExponentialExpr<'text>),
    Mul(Box<MultiplicativeExpr<'text>>, ExponentialExpr<'text>),
    Div(Box<MultiplicativeExpr<'text>>, ExponentialExpr<'text>),
    Mod(Box<MultiplicativeExpr<'text>>, ExponentialExpr<'text>),
}

fn parse_multiplicative_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
) -> Result<(MultiplicativeExpr<'text>, usize), ParseError> {
    let (lhs, mut pos) = parse_exponential_expr(tokens, pos)?;
    let mut lhs = lhs.into();
    while let Some(token) = tokens.get(pos) {
        match token {
            Token::Symbol("*") => {
                let (rhs, next_pos) = parse_exponential_expr(tokens, pos + 1)?;
                pos = next_pos;
                lhs = MultiplicativeExpr::Mul(Box::new(lhs), rhs);
            }
            Token::Symbol("/") => {
                let (rhs, next_pos) = parse_exponential_expr(tokens, pos + 1)?;
                pos = next_pos;
                lhs = MultiplicativeExpr::Div(Box::new(lhs), rhs);
            }
            Token::Symbol("%") => {
                let (rhs, next_pos) = parse_exponential_expr(tokens, pos + 1)?;
                pos = next_pos;
                lhs = MultiplicativeExpr::Mod(Box::new(lhs), rhs);
            }
            _ => break,
        }
    }
    Ok((lhs, pos))
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExponentialExpr<'ident> {
    Pow(UnaryExpr<'ident>, Box<ExponentialExpr<'ident>>),
    UnaryExpr(UnaryExpr<'ident>),
}

fn parse_exponential_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
) -> Result<(ExponentialExpr<'text>, usize), ParseError> {
    let (lhs, pos) = parse_unary_expr(tokens, pos)?;
    if let Some(token) = tokens.get(pos) {
        if token == &Token::Symbol("^") {
            let (rhs, pos) = parse_exponential_expr(tokens, pos + 1)?;
            return Ok((ExponentialExpr::Pow(lhs, Box::new(rhs)), pos));
        }
    }
    Ok((lhs.into(), pos))
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryExpr<'text> {
    PostfixExpr(PostfixExpr<'text>),
    UnaryAdd(Box<UnaryExpr<'text>>),
    UnarySub(Box<UnaryExpr<'text>>),
}

fn parse_unary_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
) -> Result<(UnaryExpr<'text>, usize), ParseError> {
    match tokens.get(pos) {
        Some(Token::Symbol("+")) => {
            let (expr, pos) = parse_unary_expr(tokens, pos + 1)?;
            Ok((UnaryExpr::UnaryAdd(Box::new(expr)), pos))
        }
        Some(Token::Symbol("-")) => {
            let (expr, pos) = parse_unary_expr(tokens, pos + 1)?;
            Ok((UnaryExpr::UnarySub(Box::new(expr)), pos))
        }
        _ => {
            let (expr, pos) = parse_postfix_expr(tokens, pos)?;
            Ok((expr.into(), pos))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixExpr<'text> {
    Primary(Primary<'text>),
    FunctionCall(&'text str, Vec<AssignmentExpr<'text>>),
}

fn parse_postfix_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
) -> Result<(PostfixExpr<'text>, usize), ParseError> {
    if let Some(Token::Ident(name)) = tokens.get(pos) {
        if let Some(Token::Symbol("(")) = tokens.get(pos + 1) {
            let (args, pos) = many(
                tokens,
                pos + 2,
                parse_assignment_expr,
                Some(Token::Symbol(",")),
            );

            let Some(Token::Symbol(")")) = tokens.get(pos) else {
                return Err(ParseError::Expected(Token::Symbol(")"), pos));
            };

            return Ok((PostfixExpr::FunctionCall(name, args), pos + 1));
        }
    }

    let (expr, pos) = parse_primary_expr(tokens, pos)?;
    Ok((expr.into(), pos))
}

#[derive(Debug, PartialEq, Clone)]
pub enum Primary<'text> {
    Ident(&'text str),
    Float(f64),
    Parens(Box<Expr<'text>>),
}

fn parse_primary_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
) -> Result<(Primary<'text>, usize), ParseError> {
    match tokens.get(pos) {
        Some(Token::Ident(ident)) => Ok((Primary::Ident(ident), pos + 1)),
        Some(Token::Decimal(n)) => Ok((Primary::Float(*n), pos + 1)),
        Some(Token::Symbol("(")) => {
            let (expr, pos) = parse_expr(tokens, pos + 1, )?;
            match tokens.get(pos) {
                Some(Token::Symbol(")")) => Ok((Primary::Parens(Box::new(expr)), pos + 1)),
                _ => Err(ParseError::Expected(Token::Symbol(")"), pos)),
            }
        }
        _ => Err(ParseError::SyntaxError(
            pos,
            "parse_primary_expr: expected <identifier> or `int` or `char` or `float` or `string` or ( <expression> ) ",
        )),
    }
}

fn many<'text, Ast>(
    tokens: &[Token<'text>],
    mut pos: usize,
    parser: impl Fn(&[Token<'text>], usize) -> Result<(Ast, usize), ParseError>,
    delimiter: Option<Token>,
) -> (Vec<Ast>, usize) {
    let mut list = vec![];

    while let Ok((ast, next_pos)) = parser(tokens, pos) {
        list.push(ast);
        pos = next_pos;

        if let Some(delimiter) = &delimiter {
            match tokens.get(pos) {
                Some(token) if token == delimiter => {
                    pos += 1;
                }
                _ => break,
            };
        }
    }

    (list, pos)
}

impl<'text> Display for AssignmentExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AssignmentExpr::AdditiveExpr(expr) => write!(f, "{}", expr),
            AssignmentExpr::Assign(lhs, rhs) => write!(f, "({} = {})", lhs, rhs),
            AssignmentExpr::MulAssign(lhs, rhs) => write!(f, "({} *= {})", lhs, rhs),
            AssignmentExpr::DivAssign(lhs, rhs) => write!(f, "({} /= {})", lhs, rhs),
            AssignmentExpr::ModAssign(lhs, rhs) => write!(f, "({} %= {})", lhs, rhs),
            AssignmentExpr::AddAssign(lhs, rhs) => write!(f, "({} += {})", lhs, rhs),
            AssignmentExpr::SubAssign(lhs, rhs) => write!(f, "({} -= {})", lhs, rhs),
        }
    }
}

impl<'text> Display for AdditiveExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AdditiveExpr::MultiplicativeExpr(expr) => write!(f, "{}", expr),
            AdditiveExpr::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
            AdditiveExpr::Sub(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
        }
    }
}

impl<'text> Display for MultiplicativeExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MultiplicativeExpr::ExponentialExpr(expr) => write!(f, "{}", expr),
            MultiplicativeExpr::Mul(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
            MultiplicativeExpr::Div(lhs, rhs) => write!(f, "({} / {})", lhs, rhs),
            MultiplicativeExpr::Mod(lhs, rhs) => write!(f, "({} % {})", lhs, rhs),
        }
    }
}

impl<'text> Display for ExponentialExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExponentialExpr::Pow(base, exp) => write!(f, "({} ^ {})", base, exp),
            ExponentialExpr::UnaryExpr(expr) => write!(f, "{}", expr),
        }
    }
}

impl<'text> Display for UnaryExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UnaryExpr::PostfixExpr(expr) => write!(f, "{}", expr),
            UnaryExpr::UnaryAdd(expr) => write!(f, "{}", expr),
            UnaryExpr::UnarySub(expr) => write!(f, "-{}", expr),
        }
    }
}

impl<'text> Display for PostfixExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PostfixExpr::Primary(expr) => write!(f, "{}", expr),
            PostfixExpr::FunctionCall(expr, args) => {
                write!(f, "{}", expr)?;
                write!(f, "(")?;
                write_arr(f, args, ", ")?;
                write!(f, ")")
            }
        }
    }
}

impl<'text> Display for Primary<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Primary::Ident(ident) => write!(f, "{}", ident),
            Primary::Float(n) => write!(f, "{}", n),
            Primary::Parens(expr) => write!(f, "({})", expr),
        }
    }
}

fn write_arr<T>(f: &mut Formatter<'_>, arr: &[T], sep: &str) -> fmt::Result
where
    T: Display,
{
    if let Some(item) = arr.get(0) {
        write!(f, "{}", item)?;
        for item in &arr[1..] {
            write!(f, "{}{}", sep, item)?;
        }
    }

    Ok(())
}

impl<'text> From<AdditiveExpr<'text>> for AssignmentExpr<'text> {
    fn from(value: AdditiveExpr<'text>) -> Self {
        AssignmentExpr::AdditiveExpr(value)
    }
}

impl<'text> From<MultiplicativeExpr<'text>> for AdditiveExpr<'text> {
    fn from(value: MultiplicativeExpr<'text>) -> Self {
        AdditiveExpr::MultiplicativeExpr(value)
    }
}

impl<'text> From<ExponentialExpr<'text>> for MultiplicativeExpr<'text> {
    fn from(value: ExponentialExpr<'text>) -> Self {
        MultiplicativeExpr::ExponentialExpr(value)
    }
}

impl<'text> From<UnaryExpr<'text>> for ExponentialExpr<'text> {
    fn from(value: UnaryExpr<'text>) -> Self {
        ExponentialExpr::UnaryExpr(value)
    }
}

impl<'text> From<PostfixExpr<'text>> for UnaryExpr<'text> {
    fn from(value: PostfixExpr<'text>) -> Self {
        UnaryExpr::PostfixExpr(value)
    }
}

impl<'text> From<Primary<'text>> for PostfixExpr<'text> {
    fn from(value: Primary<'text>) -> Self {
        PostfixExpr::Primary(value)
    }
}

impl<'text> From<Primary<'text>> for Expr<'text> {
    fn from(value: Primary<'text>) -> Self {
        Expr::AdditiveExpr(AdditiveExpr::MultiplicativeExpr(
            MultiplicativeExpr::ExponentialExpr(ExponentialExpr::UnaryExpr(
                UnaryExpr::PostfixExpr(PostfixExpr::Primary(value)),
            )),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::*;

    use pretty_assertions::assert_eq;

    macro_rules! check {
        ($f:ident, $src:expr, $expected:expr) => {
            let tokens = lex($src).expect("** LEX ERROR");
            let (stmt, pos) = $f(&tokens, 0).expect("** Unable to parse statement");
            assert_eq!(pos, tokens.len(), "** Unable to parse all Tokens\n{}", stmt);
            let stmt = format!("{}", stmt);
            assert_eq!($expected, stmt);
        };
        ($f:ident, $src:expr) => {
            check!($f, $src, $src)
        };
    }

    // macro_rules! check_ast {
    //     ($f:ident, $src:expr, $expected:expr) => {
    //         let tokens = lex($src).expect("** LEX ERROR");
    //         let (stmt, pos) = $f(&tokens, 0).expect("** Unable to parse statement");
    //         assert_eq!(pos, tokens.len());
    //         assert_eq!($expected, stmt);
    //     };
    // }

    // macro_rules! ast {
    //     ($f:ident, $src:expr) => {{
    //         let tokens = lex($src).expect("** LEX ERROR");
    //         let (stmt, pos) = $f(&tokens, 0).expect("** Unable to parse statement");
    //         assert_eq!(pos, tokens.len());
    //         stmt
    //     }};
    // }

    #[test]
    fn test_primary() {
        check!(parse_expr, "ident");
        check!(parse_expr, "123");
        check!(parse_expr, "123.123");
        check!(parse_expr, "(a)");
        check!(parse_expr, "(log(15, 2))");
    }

    #[test]
    fn test_postfix_expr() {
        check!(parse_expr, "add(a, b)");
    }

    #[test]
    fn test_unary_expr() {
        check!(parse_expr, "+a", "a");
        check!(parse_expr, "-a");
    }

    #[test]
    fn test_exponential_expr() {
        check!(parse_expr, "a^b", "(a ^ b)");
        check!(parse_expr, "-a^b", "(-a ^ b)");
        check!(parse_expr, "a^b^c", "(a ^ (b ^ c))");
        check!(parse_expr, "-a^-b^+c", "(-a ^ (-b ^ c))");
    }

    #[test]
    fn test_multiplicative_expr() {
        check!(parse_expr, "a * b", "(a * b)");
        check!(parse_expr, "a / b", "(a / b)");
        check!(parse_expr, "a % b", "(a % b)");
        check!(parse_expr, "a * b / c % d", "(((a * b) / c) % d)");
    }

    #[test]
    fn test_additive_expr() {
        check!(parse_expr, "a + b", "(a + b)");
        check!(parse_expr, "a - b", "(a - b)");
        check!(parse_expr, "a + b - c", "((a + b) - c)");
    }

    #[test]
    fn test_assignment_expr() {
        check!(parse_expr, "a = b", "(a = b)");
        check!(parse_expr, "a *= b", "(a *= b)");
        check!(parse_expr, "a /= b", "(a /= b)");
        check!(parse_expr, "a %= b", "(a %= b)");
        check!(parse_expr, "a += b", "(a += b)");
        check!(parse_expr, "a -= b", "(a -= b)");

        check!(parse_expr, "a -= b /= c", "(a -= (b /= c))");
    }
}
