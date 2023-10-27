use std::collections::HashMap;

use crate::lex::*;
use crate::parse::*;

#[derive(Debug)]
pub enum EvalError<'text> {
    LexError(LexError),
    ParseError(ParseError),
    VarNotFound(&'text str),
    InvalidFnCall(String),
    CannotChangeConstant(&'text str),
}

#[derive(Debug)]
pub struct State {
    constants: HashMap<&'static str, f64>,
    variables: HashMap<String, f64>,
}

impl State {
    pub fn new() -> Self {
        Self {
            constants: {
                use std::f64::consts::*;

                let mut map = HashMap::new();
                map.insert("PI", PI);
                map.insert("TAU", TAU);
                map.insert("E", E);
                map
            },
            variables: HashMap::new(),
        }
    }

    fn value_of(&self, var: &str) -> Option<f64> {
        self.constants.get(var).or(self.variables.get(var)).cloned()
    }

    fn set_var<'text>(&mut self, var: &'text str, val: f64) -> Result<(), EvalError<'text>> {
        match self.constants.contains_key(var) {
            true => Err(EvalError::CannotChangeConstant(var)),
            false => {
                self.variables.insert(var.to_string(), val);
                Ok(())
            }
        }
    }
}

pub trait Eval<'text> {
    fn eval(&self, state: &mut State) -> Result<f64, EvalError<'text>>;
}

pub fn eval<'text>(text: &'text str, state: &mut State) -> Result<f64, EvalError<'text>> {
    let tokens = lex(text)?;
    let expr = parse(&tokens)?;
    expr.eval(state)
}

impl<'text> Eval<'text> for AssignmentExpr<'text> {
    fn eval(&self, state: &mut State) -> Result<f64, EvalError<'text>> {
        match self {
            AssignmentExpr::Assign(lhs, rhs) => {
                let rhs = rhs.eval(state)?;
                state.set_var(lhs, rhs)?;
                Ok(rhs)
            }
            AssignmentExpr::MulAssign(lhs, rhs) => {
                let rhs =
                    state.value_of(lhs).ok_or(EvalError::VarNotFound(lhs))? * rhs.eval(state)?;
                state.set_var(lhs, rhs)?;
                Ok(rhs)
            }
            AssignmentExpr::DivAssign(lhs, rhs) => {
                let rhs =
                    state.value_of(lhs).ok_or(EvalError::VarNotFound(lhs))? / rhs.eval(state)?;
                state.set_var(lhs, rhs)?;
                Ok(rhs)
            }
            AssignmentExpr::ModAssign(lhs, rhs) => {
                let rhs =
                    state.value_of(lhs).ok_or(EvalError::VarNotFound(lhs))? % rhs.eval(state)?;
                state.set_var(lhs, rhs)?;
                Ok(rhs)
            }
            AssignmentExpr::AddAssign(lhs, rhs) => {
                let rhs =
                    state.value_of(lhs).ok_or(EvalError::VarNotFound(lhs))? + rhs.eval(state)?;
                state.set_var(lhs, rhs)?;
                Ok(rhs)
            }
            AssignmentExpr::SubAssign(lhs, rhs) => {
                let rhs =
                    state.value_of(lhs).ok_or(EvalError::VarNotFound(lhs))? - rhs.eval(state)?;
                state.set_var(lhs, rhs)?;
                Ok(rhs)
            }
            AssignmentExpr::AdditiveExpr(a) => a.eval(state),
        }
    }
}

impl<'text> Eval<'text> for AdditiveExpr<'text> {
    fn eval(&self, state: &mut State) -> Result<f64, EvalError<'text>> {
        match self {
            AdditiveExpr::Add(lhs, rhs) => Ok(lhs.eval(state)? + rhs.eval(state)?),
            AdditiveExpr::Sub(lhs, rhs) => Ok(lhs.eval(state)? - rhs.eval(state)?),
            AdditiveExpr::MultiplicativeExpr(expr) => expr.eval(state),
        }
    }
}

impl<'text> Eval<'text> for MultiplicativeExpr<'text> {
    fn eval(&self, state: &mut State) -> Result<f64, EvalError<'text>> {
        match self {
            MultiplicativeExpr::Mul(lhs, rhs) => Ok(lhs.eval(state)? * rhs.eval(state)?),
            MultiplicativeExpr::Div(lhs, rhs) => Ok(lhs.eval(state)? / rhs.eval(state)?),
            MultiplicativeExpr::Mod(lhs, rhs) => Ok(lhs.eval(state)? % rhs.eval(state)?),
            MultiplicativeExpr::ExponentialExpr(expr) => expr.eval(state),
        }
    }
}

impl<'text> Eval<'text> for ExponentialExpr<'text> {
    fn eval(&self, state: &mut State) -> Result<f64, EvalError<'text>> {
        match self {
            ExponentialExpr::Pow(base, exp) => Ok(base.eval(state)?.powf(exp.eval(state)?)),
            ExponentialExpr::UnaryExpr(expr) => expr.eval(state),
        }
    }
}

impl<'text> Eval<'text> for UnaryExpr<'text> {
    fn eval(&self, state: &mut State) -> Result<f64, EvalError<'text>> {
        match self {
            UnaryExpr::PostfixExpr(expr) => expr.eval(state),
            UnaryExpr::UnaryAdd(expr) => expr.eval(state),
            UnaryExpr::UnarySub(expr) => Ok(-expr.eval(state)?),
        }
    }
}

impl<'text> Eval<'text> for PostfixExpr<'text> {
    fn eval(&self, state: &mut State) -> Result<f64, EvalError<'text>> {
        match self {
            PostfixExpr::Primary(expr) => expr.eval(state),
            PostfixExpr::FunctionCall(name, args) => match (*name, args.as_slice()) {
                ("exp", [x]) => Ok(x.eval(state)?.exp()),
                ("sqrt", [x]) => Ok(x.eval(state)?.sqrt()),
                ("cbrt", [x]) => Ok(x.eval(state)?.cbrt()),
                ("abs", [x]) => Ok(x.eval(state)?.abs()),
                ("floor", [x]) => Ok(x.eval(state)?.floor()),
                ("ceil", [x]) => Ok(x.eval(state)?.ceil()),
                ("round", [x]) => Ok(x.eval(state)?.round()),
                ("ln", [x]) => Ok(x.eval(state)?.ln()),
                ("log2", [x]) => Ok(x.eval(state)?.log2()),
                ("log10", [x]) => Ok(x.eval(state)?.log10()),
                ("log", [x, b]) => Ok(x.eval(state)?.log(b.eval(state)?)),
                ("sin", [rad]) => Ok(rad.eval(state)?.sin()),
                ("sinh", [rad]) => Ok(rad.eval(state)?.sinh()),
                ("asin", [rad]) => Ok(rad.eval(state)?.asin()),
                ("asinh", [rad]) => Ok(rad.eval(state)?.asinh()),
                ("cos", [rad]) => Ok(rad.eval(state)?.cos()),
                ("cosh", [rad]) => Ok(rad.eval(state)?.cosh()),
                ("acos", [rad]) => Ok(rad.eval(state)?.acos()),
                ("acosh", [rad]) => Ok(rad.eval(state)?.acosh()),
                ("tan", [rad]) => Ok(rad.eval(state)?.tan()),
                ("tanh", [rad]) => Ok(rad.eval(state)?.tanh()),
                ("atan", [rad]) => Ok(rad.eval(state)?.atan()),
                ("atanh", [rad]) => Ok(rad.eval(state)?.atanh()),
                ("cot", [rad]) => Ok(rad.eval(state)?.tan().recip()),
                ("coth", [rad]) => Ok(rad.eval(state)?.tanh().recip()),
                ("acot", [rad]) => Ok(std::f64::consts::FRAC_PI_2 - rad.eval(state)?.atan()),
                ("acoth", [rad]) => Ok(0.5 * (2.0 / (rad.eval(state)? - 1.0)).ln_1p()),
                ("sec", [rad]) => Ok(rad.eval(state)?.cos().recip()),
                ("sech", [rad]) => Ok(rad.eval(state)?.cosh().recip()),
                ("asec", [rad]) => Ok((rad.eval(state)?.recip()).acos()),
                ("asech", [rad]) => {
                    Ok((rad.eval(state)?.recip() + (rad.eval(state)?.powi(-2) - 1.0).sqrt()).ln())
                }
                ("csc", [rad]) => Ok(rad.eval(state)?.sin().recip()),
                ("csch", [rad]) => Ok(rad.eval(state)?.sinh().recip()),
                ("acsc", [rad]) => Ok((rad.eval(state)?.recip()).asin()),
                ("acsch", [rad]) => {
                    Ok((rad.eval(state)?.recip() + (rad.eval(state)?.powi(-2) + 1.0).sqrt()).ln())
                }
                _ => Err(EvalError::InvalidFnCall(format!("{}", self))),
            },
        }
    }
}

impl<'text> Eval<'text> for Primary<'text> {
    fn eval(&self, state: &mut State) -> Result<f64, EvalError<'text>> {
        match self {
            Primary::Parens(expr) => expr.eval(state),
            Primary::Ident(ident) => state.value_of(ident).ok_or(EvalError::VarNotFound(ident)),
            Primary::Float(n) => Ok(*n),
        }
    }
}

impl<'text> From<LexError> for EvalError<'text> {
    fn from(value: LexError) -> Self {
        EvalError::LexError(value)
    }
}

impl<'text> From<ParseError> for EvalError<'text> {
    fn from(value: ParseError) -> Self {
        EvalError::ParseError(value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! check {
        ($state:expr, $src:expr, $expected:expr) => {
            let res = eval($src, &mut $state).expect(&format!("unable to eval {}", $src));
            assert_eq!(res, $expected);
        };
    }

    #[test]
    fn test_eval() {
        let mut state = State::new();

        check!(&mut state, "a = 2 + 3", 5.);
        check!(&mut state, "a", 5.);
    }
}
