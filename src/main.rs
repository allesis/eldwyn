use eldwyn::Regex;

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

enum InputOption {
    Match,
    Pattern,
    Quit,
    Help,
}

fn main() {
    let mut re: Option<Regex> = None;
    usage();
    loop {
        if let Some(raw_input) = get_input() {
            let parsed_input = parse_input(raw_input.trim());

            if let Some(input) = parsed_input {
                match input {
                    InputOption::Match => {
                        if let Some(user_input) = get_input() {
                            match re {
                                Some(ref re) => println!("{}", re.run(&user_input)),
                                None => println!(
                                    "You must create a pattern before matching against it."
                                ),
                            }
                        } else {
                            panic!("Failed to get input from stdin");
                        }
                    }
                    InputOption::Pattern => {
                        if let Some(user_input) = get_input() {
                            match Regex::new(user_input.clone()) {
                                Ok(new_re) => re = Some(new_re),
                                _ => {
                                    println!(
                                        "Failed to construct regex from input!\nInvalid input was read as:\n\t\t{}",
                                        user_input
                                    );
                                    re = None;
                                }
                            }
                        } else {
                            panic!("Failed to get input from stdin");
                        }
                    }
                    InputOption::Quit => return,
                    InputOption::Help => usage(),
                }
            } else {
                println!("Invalid option selected\nPlease try again");
                usage();
            }
        }
    }
}

fn usage() {
    println!("USAGE TODO!");
}

fn get_input() -> Option<String> {
    let stdin = std::io::stdin();
    let input = &mut String::new();
    let res = stdin.read_line(input);
    match res {
        Err(_) => None,
        _ => Some(input.to_string()),
    }
}

fn parse_input(input: &str) -> Option<InputOption> {
    // Quit || quit || q || Q
    // Match || match || m || M
    // Pattern || pattern || p || P (any caps works)
    match input.to_lowercase().as_str() {
        "q" | "quit" => Some(InputOption::Quit),
        "m" | "match" => Some(InputOption::Match),
        "p" | "pattern" => Some(InputOption::Pattern),
        "h" | "help" | "?" => Some(InputOption::Help),
        _ => None,
    }
}
