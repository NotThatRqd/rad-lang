use semver::Version;
use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    VersionSpecifier(Version),
    NameSpace(String),
}

pub fn tokenize(contents: &str) -> Vec<Token> {

    let mut tokens = Vec::new();

    for tok in contents.lines() {
        if tok.is_empty() {
            continue;
        }

        match Token::try_from(tok) {
            Ok(token) => tokens.push(token),
            Err(_) => (),
        }
    }

    tokens
}

pub struct ParseTokenError;

impl TryFrom<&str> for Token {
    type Error = ParseTokenError;

    fn try_from(value: &str) -> Result<Token, ParseTokenError> {
        // if the first character is a '[' and the second is 'v' then it is a version specifier
        if value.chars().nth(0).unwrap() == '[' && value.chars().nth(1).unwrap() == 'v' {
            let version = Version::from_str(remove_version_specifier_whitespace(value))
                .expect("parse semver version");
            return Ok(Token::VersionSpecifier(version));
        }

        Err(ParseTokenError)
    }
}

const VERSION_WORD_LENGTH: usize = "version".len();

fn remove_version_specifier_whitespace(value: &str) -> &str {
    for pair in value.char_indices() {
        let (index, _character) = pair;
        // skip the first 9 characters because they are "[version "
        if index < VERSION_WORD_LENGTH + 2 {
            continue;
        }

        // skip the last character because it is ']'
        return &value[index..value.len() - 1];
    }

    panic!("Failed to parse version specifier");
}


// This version is more robust but for simplicity I am going to use the one above for now

// Returns a version specifier token from string
// Panics if the syntax is invalid
// fn version_specifier_from_str(value: &str) -> Token {
//     for pair in value.char_indices() {
//         let (index, character) = pair;
//
//         println!("index: {}, character: {}", index, character);
//
//         if index == 0 {
//             if character != '[' {
//                 panic!("Expected '[' at the beginning of the version declaration");
//             }
//         } else if index == value.len() - 1 {
//             if character != ']' {
//                 panic!("Expected ']' at the end of the version declaration");
//             }
//         } else {
//             // skip the first 7 characters because they are "version"
//             if index < VERSION_WORD_LENGTH {
//                 // minus one because index is basically 1 based now because we skipped the '['
//                 // however, we don't want to skip the first character in the word 'version'
//                 if character != "version".chars().nth(index - 1).unwrap() {
//                     panic!("Expected 'version' at the beginning of the version declaration");
//                 }
//             } else {
//                 // we are past the word "version" so we should be looking for a space
//                 if index == VERSION_WORD_LENGTH + 1 && character != ' ' {
//                     panic!("Expected a space after the word 'version'");
//                 } else if index > VERSION_WORD_LENGTH + 1 {
//                     // we are past the space so we should be looking for a version
//                     // we give it the current index, where the version starts,
//                     // and the length of the string minus 2 because we don't want to include the ']' at the end
//                     dbg!("&value[index..value.len() - 2] = {}", &value[index..value.len() - 1]);
//                     let version = Version::from_str(&value[index..value.len() - 2])
//                         .expect("parse semver version");
//                     return Token::VersionSpecifier(version);
//                 }
//             }
//         }
//     }
//
//     panic!("Failed to parse version specifier");
// }
