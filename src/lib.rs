use crate::{
    dfa::{DFA, Pattern},
    error::Error,
};

mod dfa;
mod error;

pub struct Regex {
    dfa: DFA,
}

pub trait IntoPattern {
    fn into(self) -> Pattern;
}

impl IntoPattern for Vec<u8> {
    fn into(self) -> Pattern {
        self
    }
}

impl IntoPattern for &'static str {
    fn into(self) -> Pattern {
        self.as_bytes().to_vec()
    }
}

impl Regex {
    pub fn new<T>(p: T) -> Result<Self, Error>
    where
        T: IntoPattern,
    {
        let pattern: Pattern = p.into();
        match DFA::new(&pattern) {
            Ok(dfa) => Ok(Self { dfa }),
            Err(err) => Err(err),
        }
    }

    pub fn run(&self, s: &str) -> bool {
        self.dfa.run(s)
    }
}
