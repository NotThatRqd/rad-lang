use semver::{VersionReq};
use crate::tokenizer::Token;

mod tokenizer;

fn main() {
    let contents =
        std::fs::read_to_string("resources/test/test_1.rad_lang").expect("Failed to read file");

    let tokens = tokenizer::tokenize(&contents);

    println!("{:?}", tokens);

    if let Token::VersionSpecifier(version) = &tokens[0] {
        let req = VersionReq::parse(">= 0.1.0, <1.0.0").unwrap();
        if !req.matches(version) {
            panic!("Version mismatch");
        }
    } else {
        panic!("Version specifier not found at the beginning of the file");
    }
}
