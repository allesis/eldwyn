pub(crate) enum Action {
    Accept,
    MatchSymbol { symbol: u8 },
    MatchAny,
}

impl Action {
    pub(crate) fn new(symbol: u8) -> Self {
        match symbol {
            b'.' => Action::MatchAny,
            _ => Action::MatchSymbol { symbol },
        }
    }
}
