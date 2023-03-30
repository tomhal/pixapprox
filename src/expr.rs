use core::panic;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Program {
    pub code: Vec<Expr>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut b = false;
        for inst in self.code.iter() {
            if b {
                write!(f, " {}", inst)?
            } else {
                b = true;
                write!(f, "{}", inst)?
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Expr {
    Const(f32),
    Var(usize),
    Add,
    Sub,
    Mul,
    Cos,
    Sin,
    Atan,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Const(x) => write!(f, "{}", x),
            Expr::Var(i) => write!(f, "{}", gen_var_str(*i)),
            Expr::Add => write!(f, "+"),
            Expr::Sub => write!(f, "-"),
            Expr::Mul => write!(f, "*"),
            Expr::Cos => write!(f, "cos"),
            Expr::Sin => write!(f, "sin"),
            Expr::Atan => write!(f, "atan"),
        }
    }
}

/// Converts 0 to "x", 1 to "y", static str are returned
pub fn gen_var_str(n: usize) -> &'static str {
    match n {
        0 => "x",
        1 => "y",
        _ => panic!("gen_var_str: only 0 and 1 are handled"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn display_prg() {
        let state = State { vars: vec![42.0] };
        let prg = Program {
            code: vec![
                Expr::Const(1.0),
                Expr::Var(0),
                Expr::Const(2.0),
                Expr::Add,
                Expr::Mul,
            ],
        };

        let result = format!("{}", prg);
        assert_eq!("1 x 2 + *", result);
    }
}
