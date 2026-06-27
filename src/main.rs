#[test]
fn doesnt_halt_and_catch_fire() {
    use eldwyn::Regex;

    let re = Regex::new("").expect("Failed to construct regex from pattern");
    assert!(re.run(""));
}

#[test]
fn can_match_simple_patterns() {
    use eldwyn::Regex;

    let re = Regex::new("hello").expect("Failed to construct regex from pattern");
    assert!(re.run("hello"));
}

#[test]
fn doesnt_accept_early() {
    use eldwyn::Regex;

    let re = Regex::new("hello").expect("Failed to construct regex from pattern");
    assert!(!re.run("hell"));
}

#[test]
fn can_match_on_wildcard() {
    use eldwyn::Regex;

    let re = Regex::new(".").expect("Failed to construct regex from pattern");
    assert!(re.run("a"));
}

#[test]
fn can_match_longer_wildcards() {
    use eldwyn::Regex;

    let re = Regex::new(".....").expect("Failed to construct regex from pattern");
    assert!(re.run("hello"));
    assert!(re.run("cruel"));
    assert!(re.run("world"));
    assert!(!re.run("goodbye"));
}

#[test]
fn wildcard_positions_is_honored() {
    use eldwyn::Regex;

    let re = Regex::new("I .... cheese!").expect("Failed to construct regex from pattern");
    assert!(re.run("I hate cheese!"));
    assert!(re.run("I like cheese!"));
    assert!(re.run("I love cheese!"));
    assert!(!re.run("I despise cheese!"));
}

fn main() {}
