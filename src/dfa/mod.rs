mod action;
mod state;

use crate::{dfa::state::State, error::Error};

pub struct DFA {
    states: Vec<State>,
}

impl DFA {
    #[allow(unused)]
    pub(crate) fn new(pattern: &mut str) -> Result<Self, Error> {
        let chars = pattern.chars();
        chars.for_each(|c| println!("{}", c));
        Ok(Self { states: vec![] })
    }

    #[allow(unused)]
    pub(crate) fn run(&self, input: &str) -> bool {
        todo!()
    }

    fn validate_pattern(&self, pattern: &mut str) -> Result<(), Error> {
        fn capture_group(pattern: &mut str) -> Result<(), Error> {
            todo!()
        }

        fn match_group(pattern: &mut str) -> Result<(), Error> {
            todo!()
        }

        fn digit(pattern: &mut str) -> Result<(), Error> {
            todo!()
        }

        fn letter(pattern: &mut str) -> Result<(), Error> {
            todo!()
        }

        fn whitespace(pattern: &mut str) -> Result<(), Error> {
            todo!()
        }
        loop {
            if let Some((current, pattern)) = pattern.split_at_mut_checked(1) {
                todo!()
            }
        }
    }
}
