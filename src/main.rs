#![feature(test)]
mod joinoperator;

fn main() {
    let words = vec!["Rust", "World"];
    assert_eq!(words.connect("-"), "Rust-World");
    let words = vec!["Rust", "World"];
    assert_eq!(words.join("-"), "Rust-World");
}
