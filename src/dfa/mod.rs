mod action;
mod state;

pub(crate) type Pattern = Vec<u8>;
use crate::{dfa::state::State, error::Error};

pub struct DFA {
    states: Vec<State>,
}

impl DFA {
    #[allow(unused)]
    pub(crate) fn new(pattern: &Pattern) -> Result<Self, Error> {
        let mut p = pattern.clone();
        let mut pattern_iter = p.iter_mut();
        let states = vec![];

        loop {
            let c = pattern_iter.next();
            match c {
                Some(c) => print!("{}", char::from_u32(*c as u32).unwrap_or('?')),
                None => break,
            }
        }
        println!("");

        Ok(Self { states })
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
