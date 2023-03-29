use rand::{rngs::StdRng, Rng};

use smallvec::SmallVec;

use crate::expr::{Expr, Program};

pub fn mutate(rng: &mut StdRng, prg: &mut Program, nvars: usize) {
    let nth = rng.gen_range(0..prg.code.len());
    let expr = prg.code[nth];

    let new_code: SmallVec<[_; 3]> = match expr {
        Expr::Const(x) => mutated_constant(rng, x, nvars),
        Expr::Var(i) => mutated_var(rng, i, nvars),
        Expr::Add | Expr::Sub | Expr::Mul => mutated_binary_op(rng, nvars),
        Expr::Cos | Expr::Sin | Expr::Atan => mutated_unary_op(rng, nvars),
    };

    if !new_code.is_empty() {
        prg.code[nth] = new_code[0];
        for i in 1..new_code.len() {
            prg.code.insert(nth + i, new_code[i])
        }
    }
}

pub fn mutated_constant(rng: &mut StdRng, x: f32, nvars: usize) -> SmallVec<[Expr; 3]> {
    let c = make_const(rng);
    let choice = rng.gen_range(0..10);
    match choice {
        0 => smallvec![c],

        1 => smallvec![Expr::Var(rng.gen_range(0..nvars))],

        2 => smallvec![Expr::Const(x), c, Expr::Add],
        3 => smallvec![c, Expr::Const(x), Expr::Add],

        4 => smallvec![Expr::Const(x), c, Expr::Sub],
        5 => smallvec![c, Expr::Const(x), Expr::Sub],

        6 => smallvec![Expr::Const(x), c, Expr::Mul],
        7 => smallvec![c, Expr::Const(x), Expr::Mul],

        8 => smallvec![Expr::Const(x), Expr::Cos],
        9 => smallvec![Expr::Const(x), Expr::Sin],
        // 10 => smallvec![Expr::Const(x), Expr::Atan],
        _ => panic!("mutated_constant: choice {} not in match", choice),
    }
}

pub fn mutated_var(rng: &mut StdRng, i: usize, nvars: usize) -> SmallVec<[Expr; 3]> {
    let c = make_const(rng);
    let choice = rng.gen_range(0..10);
    match choice {
        0 => smallvec![make_const(rng)],

        1 => smallvec![Expr::Var(rng.gen_range(0..nvars))],

        2 => smallvec![Expr::Var(i), c, Expr::Add],
        3 => smallvec![c, Expr::Var(i), Expr::Add],

        4 => smallvec![Expr::Var(i), c, Expr::Sub],
        5 => smallvec![c, Expr::Var(i), Expr::Sub],

        6 => smallvec![Expr::Var(i), c, Expr::Mul],
        7 => smallvec![c, Expr::Var(i), Expr::Mul],

        8 => smallvec![Expr::Var(i), Expr::Cos],
        9 => smallvec![Expr::Var(i), Expr::Sin],
        // 10 => smallvec![Expr::Var(i), Expr::Atan],
        _ => panic!("mutated_var: choice {} not in match", choice),
    }
}

pub fn mutated_binary_op(rng: &mut StdRng, nvars: usize) -> SmallVec<[Expr; 3]> {
    let choice = rng.gen_range(0..3);
    match choice {
        0 => smallvec![Expr::Add],
        1 => smallvec![Expr::Sub],
        2 => smallvec![Expr::Mul],
        _ => panic!("mutated_binary_op: choice {} not in match", choice),
    }
}

pub fn mutated_unary_op(rng: &mut StdRng, nvars: usize) -> SmallVec<[Expr; 3]> {
    let c = make_const(rng);
    let choice = rng.gen_range(0..5);
    match choice {
        0 => smallvec![Expr::Cos],
        1 => smallvec![Expr::Sin],
        2 => smallvec![c, Expr::Add],
        3 => smallvec![c, Expr::Mul],
        // 2 => smallvec![Expr::Atan],

        // Removes the unary operator instead of replacing it
        4 => smallvec![],
        _ => panic!("mutated_unary_op: choice {} not in match", choice),
    }
}

pub fn make_const(rng: &mut StdRng) -> Expr {
    Expr::Const(rng.gen::<f32>() * 2.0 - 1.0)
}

// #[cfg(test)]
// mod tests {
//     use crate::state::State;

//     use super::*;
// }
