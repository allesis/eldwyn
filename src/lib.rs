use crate::{dfa::DFA, error::Error};

mod dfa;
mod error;

pub struct Regex {
    dfa: DFA,
}

impl Regex {
    pub fn new(pattern: &mut str) -> Result<Self, Error> {
        match DFA::new(pattern) {
            Ok(dfa) => Ok(Self { dfa }),
            Err(err) => Err(err),
        }
    }

    pub fn run(&self, s: &str) -> bool {
        self.dfa.run(s)
    }
}
