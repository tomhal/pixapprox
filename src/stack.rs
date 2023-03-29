use smallvec::SmallVec;
use std::fmt::Display;

pub struct Stack {
    stack: SmallVec<[f32; 64]>,
}

//  A simple stack that panics if stack underflow occurs
impl Stack {
    pub fn new() -> Self {
        Self { stack: smallvec![] }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> f32 {
        self.stack.pop().expect("Stack underflow")
    }

    #[inline(always)]
    pub fn push(&mut self, value: f32) {
        self.stack.push(value);
    }

    pub fn result(&mut self) -> f32 {
        assert!(
            self.stack.len() == 1,
            "Stack should contain exactly 1 item but was {}",
            self
        );
        self.pop()
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {:?}", self.stack)
    }
}

pub struct Stack2 {
    stack: [f32; 64],
    i: usize,
}

//  A simple stack that panics if stack underflow occurs
impl Stack2 {
    pub fn new() -> Self {
        Self {
            stack: [0.0; 64],
            i: 0,
        }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> f32 {
        if self.i > 0 {
            self.i -= 1;
            return self.stack[self.i];
        }
        panic!("Stack underflow")
    }

    #[inline(always)]
    pub fn push(&mut self, value: f32) {
        if self.i < 63 {
            self.stack[self.i] = value;
            self.i += 1;
            return;
        }
        panic!("Stack overflow")
    }

    pub fn result(&mut self) -> f32 {
        assert!(
            self.i == 1,
            "Stack should contain exactly 1 item but had {} items",
            self.i
        );
        self.pop()
    }
}

// impl Display for Stack2 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, " {:?}", self.stack)
//     }
// }
