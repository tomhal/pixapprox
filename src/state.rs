pub struct State {
    pub vars: Vec<f32>,
}

impl State {
    pub fn new(n: usize) -> Self {
        let mut vars = Vec::with_capacity(n);

        vars.resize(n, 0.0);

        Self { vars }
    }
}
