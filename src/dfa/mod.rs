mod action;

pub(crate) type Pattern = Vec<u8>;
use std::rc::Rc;

use crate::{dfa::action::Action, error::Error};

pub struct Dfa {
    start: State,
}

impl Dfa {
    #[allow(unused)]
    pub(crate) fn new(pattern: &Pattern) -> Result<Self, Error> {
        let start = pattern
            .iter()
            .rev()
            .fold(State::new(Action::Accept, None), |state, symbol| {
                State::new(Action::new(*symbol), Some(Rc::new(state)))
            });

        Ok(Self { start })
    }

    #[allow(unused)]
    pub(crate) fn run(&self, input: &str) -> bool {
        self.start.run(input)
    }
}

pub(crate) struct State {
    pub(crate) action: Action,
    pub(crate) next: Option<Rc<State>>,
}

impl State {
    pub(crate) fn new(action: Action, next: Option<Rc<State>>) -> Self {
        Self { action, next }
    }

    pub(crate) fn run(&self, input: &str) -> bool {
        match self.action {
            Action::Accept => input.is_empty(),
            Action::MatchSymbol { symbol } => {
                if input.starts_with(char::from(symbol)) && self.next.is_some() {
                    self.success(input)
                } else {
                    false
                }
            }
            Action::MatchAny => {
                if input.is_empty() {
                    false
                } else {
                    self.success(input)
                }
            }
        }
    }

    fn success(&self, input: &str) -> bool {
        if let Some(next) = self.next.clone() {
            next.run(input.get(1..).unwrap_or(""))
        } else {
            false
        }
    }
}
