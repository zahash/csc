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
                ("ln", [x]) => Ok(x.eval(state)?.ln()),
                ("log2", [x]) => Ok(x.eval(state)?.log2()),
                ("log10", [x]) => Ok(x.eval(state)?.log10()),
                ("log", [x, b]) => Ok(x.eval(state)?.log(b.eval(state)?)),
                ("sin", [rad]) => Ok(rad.eval(state)?.sin()),
                ("asin" | "arcsin", [rad]) => Ok(rad.eval(state)?.asin()),
                ("sinh", [rad]) => Ok(rad.eval(state)?.sinh()),
                ("cos", [rad]) => Ok(rad.eval(state)?.cos()),
                ("acos" | "arccos", [rad]) => Ok(rad.eval(state)?.acos()),
                ("cosh", [rad]) => Ok(rad.eval(state)?.cosh()),
                ("tan", [rad]) => Ok(rad.eval(state)?.tan()),
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

    #[test]
    fn test_eval() {
        let mut state = State::new();

        let result = eval("a = 2^(1/2)", &mut state).unwrap();
        println!("{}", result);
        println!("{:?}", state);
    }
}
