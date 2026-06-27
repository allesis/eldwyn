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

fn main() {}
