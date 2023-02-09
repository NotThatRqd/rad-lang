mod tokenizer;

fn main() {
    let contents =
        std::fs::read_to_string("resources/test/test_1.rad_lang").expect("Failed to read file");

    let tokens = tokenizer::tokenize(&contents);

    println!("{:?}", tokens);
}
