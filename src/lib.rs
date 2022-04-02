pub mod ast;
pub mod error;
pub mod lexer;
pub mod parsing;
pub mod source_location;
pub mod token;

#[cfg(test)]
mod lexer_tests {
    use crate::{
        lexer::Lexer,
        source_location::SourceLocation,
        token::{Token, TokenKind},
    };

    #[test]
    fn empty_file() {
        let filepath = "empty.lang";
        let source = "";
        let mut lexer = Lexer::new(filepath, source);
        assert_eq!(
            lexer.next_token(),
            Ok(Token {
                kind: TokenKind::EndOfFile,
                location: SourceLocation {
                    filepath: filepath.to_string(),
                    position: 0,
                    line: 1,
                    column: 1
                },
                length: 0,
            })
        );
    }
}

#[cfg(test)]
mod parsing_tests {
    use std::rc::Rc;

    use crate::{
        ast::Ast,
        parsing::parse_file,
        source_location::SourceLocation,
        token::{Token, TokenKind},
    };

    #[test]
    fn empty_file() {
        let filepath = "empty.lang";
        let source = "";
        assert_eq!(
            parse_file(filepath, source),
            Ok(Rc::new(Ast::File {
                expressions: vec![],
                end_of_file_token: Token {
                    kind: TokenKind::EndOfFile,
                    location: SourceLocation {
                        filepath: filepath.to_string(),
                        position: 0,
                        line: 1,
                        column: 1
                    },
                    length: 0,
                }
            }))
        );
    }

    #[test]
    fn random() {
        let filepath = "random.lang";
        let source = "
			const foo = 5

			const do_something = func(a: int, b: int): int {
				return a + b
			}

			const greet_user = proc(): void {
				print(\"What is your name: \")
				name: string <- read_line_from_console(stdin)
				print(\"Hello, %\\n\", name)
			}

			const int_or_bool = func(condition: bool): type {
				if condition {
					return int
				} else {
					return bool
				}
			}

			const identity[T: type] = func(value: T): T {
				return value
			}

			bar: int <- identity[int](1 + 2 * 3)

			some_variable: int_or_bool(true)
		";
        parse_file(filepath, source).unwrap();
    }
}
