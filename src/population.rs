use rand::rngs::StdRng;

use crate::expr::{Expr, Program};

#[derive(Debug, Clone)]
pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            individuals: Vec::with_capacity(size),
        }
    }

    /// Generates a population with random simple individuals
    pub fn random(rng: &StdRng, size: usize) -> Self {
        let mut pop = Population::with_capacity(size);

        for i in 0..size {
            let ind = Individual::random(rng);
            pop.individuals.push(ind);
        }

        pop
    }

    pub fn size(&self) -> usize {
        self.individuals.len()
    }
}

#[derive(Debug, Clone)]
pub struct Individual {
    pub prg: Program,
    pub error: Option<f32>,
}

impl Individual {
    /// Generates a random simple individual
    pub fn random(rng: &StdRng) -> Self {
        Individual {
            error: None,
            prg: Program {
                code: vec![Expr::Const(1.0)],
            },
        }
    }
}
