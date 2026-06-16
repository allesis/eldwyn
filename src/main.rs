use eldwyn::Regex;

fn main() {
    let mut pattern = Vec::from("Hello, Cruel World!");
    let _ = Regex::new(&mut pattern);
}
