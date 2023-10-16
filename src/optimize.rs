use std::{collections::VecDeque, f32::consts::TAU};

use crate::{
    expr::{Expr, Program},
    stack::Stack2,
    state::State,
};

fn top_is_two_constants(prg: &VecDeque<Expr>) -> bool {
    let len = prg.len();

    if len < 2 {
        return false;
    }

    let Some(Expr::Const(_)) = prg.get(len - 1) else {
        return false;
    };
    let Some(Expr::Const(_)) = prg.get(len - 2) else {
        return false;
    };

    true
}

fn top_is_one_constant(prg: &VecDeque<Expr>) -> bool {
    if prg.len() < 1 {
        return false;
    }

    let Some(Expr::Const(_)) = prg.back() else {
        return false;
    };

    true
}

fn pop_const(prg: &mut VecDeque<Expr>) -> f32 {
    let Some(Expr::Const(x)) = prg.pop_back() else {
        panic!("pop_const: Not a const on top");
    };

    x
}

pub fn optimize(prg: &Program) -> Program {
    let mut new_code: VecDeque<Expr> = VecDeque::new();

    for expr in prg.code.iter() {
        match *expr {
            Expr::Const(_) => new_code.push_back(*expr),
            Expr::Var(_) => new_code.push_back(*expr),
            Expr::Add => {
                if top_is_two_constants(&new_code) {
                    let a = pop_const(&mut new_code);
                    let b = pop_const(&mut new_code);
                    new_code.push_back(Expr::Const(a + b));
                } else {
                    new_code.push_back(*expr);
                }
            }
            Expr::Sub => {
                if top_is_two_constants(&new_code) {
                    let a = pop_const(&mut new_code);
                    let b = pop_const(&mut new_code);
                    new_code.push_back(Expr::Const(b - a));
                } else {
                    new_code.push_back(*expr);
                }
            }
            Expr::Mul => {
                if top_is_two_constants(&new_code) {
                    let a = pop_const(&mut new_code);
                    let b = pop_const(&mut new_code);
                    new_code.push_back(Expr::Const(b * a));
                } else {
                    new_code.push_back(*expr);
                }
            }
            Expr::Max => {
                if top_is_two_constants(&new_code) {
                    let a = pop_const(&mut new_code);
                    let b = pop_const(&mut new_code);
                    new_code.push_back(Expr::Const(a.max(b)));
                } else {
                    new_code.push_back(*expr);
                }
            }
            Expr::Min => {
                if top_is_two_constants(&new_code) {
                    let a = pop_const(&mut new_code);
                    let b = pop_const(&mut new_code);
                    new_code.push_back(Expr::Const(a.min(b)));
                } else {
                    new_code.push_back(*expr);
                }
            }
            Expr::Cos => {
                if top_is_one_constant(&new_code) {
                    let a = pop_const(&mut new_code);
                    new_code.push_back(Expr::Const((a * TAU).cos()));
                } else {
                    new_code.push_back(*expr);
                }
            }
            Expr::Sin => {
                if top_is_one_constant(&new_code) {
                    let a = pop_const(&mut new_code);
                    new_code.push_back(Expr::Const((a * TAU).sin()));
                } else {
                    new_code.push_back(*expr);
                }
            }
            Expr::Atan => {
                if top_is_one_constant(&new_code) {
                    let a = pop_const(&mut new_code);
                    new_code.push_back(Expr::Const(a.atan()));
                } else {
                    new_code.push_back(*expr);
                }
            }
            Expr::Drop => {
                panic!("Drop not done");
            }
            Expr::Dup => {
                if top_is_one_constant(&new_code) {
                    let a = pop_const(&mut new_code);
                    new_code.push_back(Expr::Const(a));
                    new_code.push_back(Expr::Const(a));
                } else {
                    new_code.push_back(*expr);
                }
            }
        }
    }

    Program {
        code: new_code.into_iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn optimize_add_1() {
        // 0.46546388 1 +
        let prg = Program {
            code: vec![Expr::Const(0.46546388), Expr::Const(1.0), Expr::Add],
        };

        let actual = optimize(&prg).code;
        let expected = vec![Expr::Const(1.46546388)];

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn top_is_one_constant_test_0() {
        let prg = VecDeque::from(vec![]);
        assert_eq!(false, top_is_one_constant(&prg));
    }

    #[test]
    pub fn top_is_one_constant_test_1() {
        let prg = VecDeque::from(vec![Expr::Const(0.46546388)]);
        assert_eq!(true, top_is_one_constant(&prg));
    }

    #[test]
    pub fn top_is_one_constant_test_2() {
        let prg = VecDeque::from(vec![Expr::Const(0.46546388), Expr::Const(1.0)]);
        assert_eq!(true, top_is_one_constant(&prg));
    }

    #[test]
    pub fn top_is_two_constants_test_0() {
        let prg = VecDeque::from(vec![]);
        assert_eq!(false, top_is_two_constants(&prg));
    }

    #[test]
    pub fn top_is_two_constants_test_1() {
        let prg = VecDeque::from(vec![Expr::Const(1.0)]);
        assert_eq!(false, top_is_two_constants(&prg));
    }

    #[test]
    pub fn top_is_two_constants_test_2() {
        let prg = VecDeque::from(vec![Expr::Const(1.0), Expr::Const(2.0)]);
        assert_eq!(true, top_is_two_constants(&prg));
    }

    #[test]
    pub fn top_is_two_constants_test_3() {
        let prg = VecDeque::from(vec![Expr::Const(1.0), Expr::Const(2.0), Expr::Const(3.0)]);
        assert_eq!(true, top_is_two_constants(&prg));
    }
}
