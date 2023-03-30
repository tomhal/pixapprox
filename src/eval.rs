use std::f32::consts::TAU;

use crate::{
    expr::{Expr, Program},
    stack::Stack2,
    state::State,
};

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
}
