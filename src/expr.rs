use core::panic;
use std::f32::consts::TAU;
use std::fmt::Display;

use crate::stack::Stack2;
use crate::state::State;

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

pub fn eval(prg: &Program, state: &State) -> f32 {
    let mut stack = Stack2::new();

    for expr in prg.code.iter() {
        match *expr {
            Expr::Const(x) => stack.push(x),
            Expr::Var(i) => stack.push(state.vars[i]),
            Expr::Add => {
                let a = stack.pop();
                let b = stack.pop();
                stack.push(a + b)
            }
            Expr::Sub => {
                let a = stack.pop();
                let b = stack.pop();
                stack.push(b - a)
            }
            Expr::Mul => {
                let a = stack.pop();
                let b = stack.pop();
                stack.push(a * b)
            }
            Expr::Cos => {
                let a = stack.pop() * TAU;
                stack.push(a.cos())
            }
            Expr::Sin => {
                let a = stack.pop() * TAU;
                stack.push(a.sin())
            }
            Expr::Atan => {
                let a = stack.pop();
                stack.push(a.atan())
            }
        }
    }

    stack.result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn add_consts() {
        let state = State { vars: vec![] };
        let prg = Program {
            code: vec![Expr::Const(2.0), Expr::Const(3.0), Expr::Add],
        };

        let result = eval(&prg, &state);
        assert_eq!(5.0, result);
    }

    #[test]
    pub fn add_vars() {
        let state = State {
            vars: vec![2.0, 3.0],
        };
        let prg = Program {
            code: vec![Expr::Var(0), Expr::Var(1), Expr::Add],
        };

        let result = eval(&prg, &state);
        assert_eq!(5.0, result);
    }

    #[test]
    pub fn sub_consts() {
        let state = State { vars: vec![] };
        let prg = Program {
            code: vec![Expr::Const(2.0), Expr::Const(3.0), Expr::Sub],
        };

        let result = eval(&prg, &state);
        assert_eq!(-1.0, result);
    }

    #[test]
    pub fn sub_vars() {
        let state = State {
            vars: vec![2.0, 3.0],
        };
        let prg = Program {
            code: vec![Expr::Var(0), Expr::Var(1), Expr::Sub],
        };

        let result = eval(&prg, &state);
        assert_eq!(-1.0, result);
    }

    #[test]
    pub fn mul_consts() {
        let state = State { vars: vec![] };
        let prg = Program {
            code: vec![Expr::Const(2.0), Expr::Const(3.0), Expr::Mul],
        };

        let result = eval(&prg, &state);
        assert_eq!(6.0, result);
    }

    #[test]
    pub fn mul_vars() {
        let state = State {
            vars: vec![2.0, 3.0],
        };
        let prg = Program {
            code: vec![Expr::Var(0), Expr::Var(1), Expr::Mul],
        };

        let result = eval(&prg, &state);
        assert_eq!(6.0, result);
    }

    #[test]
    pub fn cos_const() {
        let state = State { vars: vec![] };
        let prg = Program {
            code: vec![Expr::Const(0.0), Expr::Cos],
        };

        let result = eval(&prg, &state);
        assert_eq!(1.0, result);
    }

    #[test]
    pub fn cos_var() {
        let state = State { vars: vec![0.0] };
        let prg = Program {
            code: vec![Expr::Var(0), Expr::Cos],
        };

        let result = eval(&prg, &state);
        assert_eq!(1.0, result);
    }

    #[test]
    pub fn atan_const() {
        let state = State { vars: vec![] };
        let prg = Program {
            code: vec![Expr::Const(0.0), Expr::Atan],
        };

        let result = eval(&prg, &state);
        assert_eq!(0.0, result);
    }

    #[test]
    pub fn atan_var() {
        let state = State { vars: vec![0.0] };
        let prg = Program {
            code: vec![Expr::Var(0), Expr::Atan],
        };

        let result = eval(&prg, &state);
        assert_eq!(0.0, result);
    }

    #[test]
    #[should_panic]
    pub fn underflow_op_1() {
        let state = State { vars: vec![] };
        let prg = Program {
            code: vec![Expr::Const(1.0), Expr::Add],
        };

        let result = eval(&prg, &state);
        assert_eq!(0.0, result);
    }

    #[test]
    #[should_panic]
    pub fn overflow_op_1() {
        let state = State { vars: vec![] };
        let prg = Program {
            code: vec![
                Expr::Const(1.0),
                Expr::Const(1.0),
                Expr::Const(1.0),
                Expr::Add,
            ],
        };

        let result = eval(&prg, &state);
        assert_eq!(0.0, result);
    }

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
