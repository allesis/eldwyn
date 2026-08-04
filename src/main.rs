use eldwyn::Regex;

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
