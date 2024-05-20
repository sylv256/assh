#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Word,
    String,
    Comment,
}

#[derive(Debug)]
pub struct Token {
    pub(crate) lexeme: String,
    pub(crate) kind: TokenKind,
}

pub(crate) fn lex(input: &str) -> Vec<Token> {
    let mut input_chars = input.chars().peekable();
    let mut tokens = vec![];

    loop {
        let mut lexeme = String::new();
        let mut kind: Option<TokenKind> = None;

        loop {
            let character = input_chars.next();
            if character.is_none() {
                break
            }
            let character = character.unwrap();

            // check newlines separately to ensure single-line comments work
            match character {
                '\r' => continue, // just in case
                '\n' => break, // at the end of a line
                _ => {},
            }

            if kind == Some(TokenKind::Comment) {
                lexeme.push(character);
                continue
            }

            match character {
                '#' => {
                    if let Some(kind) = kind {
                        tokens.push(Token {
                            lexeme,
                            kind,
                        });
                    }

                    lexeme = String::new();
                    kind = Some(TokenKind::Comment);
                },
                '"' => {
                    if kind == Some(TokenKind::String) {
                        break
                    }
                    kind = Some(TokenKind::String);
                },
                ' ' => {
                    match kind {
                        Some(TokenKind::Word) => break,
                        Some(TokenKind::String) | Some(TokenKind::Comment) => {
                            lexeme.push(character);
                        },
                        None => break,
                    }
                },
                _ => {
                    lexeme.push(character);

                    if kind.is_none() {
                        kind = Some(TokenKind::Word);
                    }
                },
            }
        }

        if !lexeme.is_empty() {
            tokens.push(Token {
                lexeme,
                kind: kind.expect("TokenKind should never be empty"),
            });
        }

        if input_chars.peek().is_none() {
            break
        }
    }

    tokens
}
