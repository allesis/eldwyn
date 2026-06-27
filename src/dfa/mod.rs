mod action;

pub(crate) type Pattern = Vec<u8>;

use std::collections::binary_heap::Iter;

use crate::{dfa::action::Action, error::Error};

pub struct Dfa {
    states: Vec<State>,
}

impl Dfa {
    #[allow(unused)]
    pub(crate) fn new(pattern: &Pattern) -> Result<Self, Error> {
        let states = pattern
            .iter()
            .map(|p| State::new(Action::new(*p)))
            .collect();
        Ok(Self { states })
    }

    #[allow(unused)]
    pub(crate) fn run(&self, input: &str) -> bool {
        let z = self.states.iter().zip(input.into());
    }
}

pub(crate) struct State {
    pub(crate) action: Action,
}

impl State {
    pub(crate) fn new(action: Action) -> Self {
        Self { action }
    }

    pub(crate) fn run(&self, input: &str) -> bool {
        match self.action {
            Action::Accept => input.is_empty(),
            Action::MatchSymbol { symbol } => input.starts_with(char::from(symbol)),
            Action::MatchAny => !input.is_empty(),
            Action::Noop => true,
        }
    }
}
