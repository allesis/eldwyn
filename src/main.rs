use eldwyn::Regex;

fn main() {
    let mut pattern = String::from("Hello, Cruel World!");
    let _ = Regex::new(&mut pattern);
}
