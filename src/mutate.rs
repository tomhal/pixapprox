use rand::{rngs::StdRng, Rng};

use smallvec::SmallVec;

use crate::expr::{Expr, Program};

/// The maximum number of items returned by the mutate sub-functions.
/// Keep this value as low as possible to ensure efficient
/// transfer to called in registers instead of memcopy.
const MAX_MUTATION_SIZE: usize = 3;

pub fn mutate(rng: &mut StdRng, prg: &mut Program, nvars: usize) {
    let nth = rng.gen_range(0..prg.code.len());
    let expr = prg.code[nth];

    let new_code: SmallVec<[_; MAX_MUTATION_SIZE]> = match expr {
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

pub fn mutated_constant(
    rng: &mut StdRng,
    x: f32,
    nvars: usize,
) -> SmallVec<[Expr; MAX_MUTATION_SIZE]> {
    let choice = rng.gen_range(0..11);
    match choice {
        0 => smallvec![make_const(rng)],

        1 => smallvec![Expr::Var(rng.gen_range(0..nvars))],

        2 => smallvec![Expr::Const(x), make_const(rng), Expr::Add],
        3 => smallvec![make_const(rng), Expr::Const(x), Expr::Add],

        4 => smallvec![Expr::Const(x), make_const(rng), Expr::Sub],
        5 => smallvec![make_const(rng), Expr::Const(x), Expr::Sub],

        6 => smallvec![Expr::Const(x), make_const(rng), Expr::Mul],
        7 => smallvec![make_const(rng), Expr::Const(x), Expr::Mul],

        8 => smallvec![Expr::Const(x), Expr::Cos],
        9 => smallvec![Expr::Const(x), Expr::Sin],
        10 => smallvec![Expr::Const(x), Expr::Atan],
        _ => panic!("mutated_constant: choice {} not in match", choice),
    }
}

pub fn mutated_var(
    rng: &mut StdRng,
    i: usize,
    nvars: usize,
) -> SmallVec<[Expr; MAX_MUTATION_SIZE]> {
    let choice = rng.gen_range(0..11);
    match choice {
        0 => smallvec![make_const(rng)],

        1 => smallvec![Expr::Var(rng.gen_range(0..nvars))],

        2 => smallvec![Expr::Var(i), make_const(rng), Expr::Add],
        3 => smallvec![make_const(rng), Expr::Var(i), Expr::Add],

        4 => smallvec![Expr::Var(i), make_const(rng), Expr::Sub],
        5 => smallvec![make_const(rng), Expr::Var(i), Expr::Sub],

        6 => smallvec![Expr::Var(i), make_const(rng), Expr::Mul],
        7 => smallvec![make_const(rng), Expr::Var(i), Expr::Mul],

        8 => smallvec![Expr::Var(i), Expr::Cos],
        9 => smallvec![Expr::Var(i), Expr::Sin],
        10 => smallvec![Expr::Var(i), Expr::Atan],
        _ => panic!("mutated_var: choice {} not in match", choice),
    }
}

pub fn mutated_binary_op(rng: &mut StdRng, nvars: usize) -> SmallVec<[Expr; MAX_MUTATION_SIZE]> {
    let choice = rng.gen_range(0..3);
    match choice {
        0 => smallvec![Expr::Add],
        1 => smallvec![Expr::Sub],
        2 => smallvec![Expr::Mul],
        _ => panic!("mutated_binary_op: choice {} not in match", choice),
    }
}

pub fn mutated_unary_op(rng: &mut StdRng, nvars: usize) -> SmallVec<[Expr; MAX_MUTATION_SIZE]> {
    let choice = rng.gen_range(0..6);
    match choice {
        0 => smallvec![Expr::Cos],
        1 => smallvec![Expr::Sin],
        2 => smallvec![make_const(rng), Expr::Add],
        3 => smallvec![make_const(rng), Expr::Mul],
        4 => smallvec![Expr::Atan],

        // Removes the unary operator instead of replacing it
        5 => smallvec![],
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
