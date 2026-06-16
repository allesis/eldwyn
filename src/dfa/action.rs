pub(crate) enum Action {
    MatchSymbol { symbol: char },
    MatchDigit,
    MatchLetter,
    MatchWhitespace,
    MatchStartOfString,
    MatchEndOfString,
    MatchSet,
    CaptureGroup,
}
