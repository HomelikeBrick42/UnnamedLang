use std::io::Write;

use langite::*;

fn usage(f: &mut dyn Write) -> Result<(), std::io::Error> {
    let program_name = std::env::args().next().unwrap();
    writeln!(f, "Usage: {} <command> [arguments]", program_name)?;
    writeln!(f, "Commands:")?;
    writeln!(f, "    help                   - displays this message")?;
    writeln!(
        f,
        "    dump_tokens <filepath> - prints all the tokens in the file"
    )?;
    Ok(())
}

fn main() {
    let stdout = &mut std::io::stdout();
    let stderr = &mut std::io::stderr();

    let mut args = std::env::args();
    let _program_name = args.next().unwrap();
    let command = args.next().unwrap_or_else(|| {
        writeln!(stderr, "Expected a command").unwrap();
        usage(stderr).unwrap();
        std::process::exit(1)
    });

    match &command as &str {
        "help" => {
            usage(stdout).unwrap();
        }

        "dump_tokens" => {
            let filepath = args.next().unwrap_or_else(|| {
                writeln!(stderr, "Expected a filepath").unwrap();
                usage(stderr).unwrap();
                std::process::exit(1)
            });
            let source = std::fs::read_to_string(filepath.clone()).unwrap_or_else(|e| {
                writeln!(stderr, "Unable to read file '{}': {}", filepath, e).unwrap();
                usage(stderr).unwrap();
                std::process::exit(1)
            });
            let mut lexer = Lexer::new(filepath, &source);
            'print_loop: loop {
                match lexer.next_token() {
                    Ok(token) => {
                        write!(stdout, "{}: {}", token.location, token.kind).unwrap();
                        if !token.data.is_none() {
                            write!(stdout, ", {}", token.data).unwrap();
                        }
                        writeln!(stdout).unwrap();
                        if token.kind.is_end_of_file() {
                            break 'print_loop;
                        }
                    }
                    Err(error) => {
                        stdout.flush().unwrap();
                        writeln!(stderr, "{}", error).unwrap();
                        std::process::exit(1)
                    }
                }
            }
        }

        _ => {
            writeln!(stderr, "Unknown command '{}'", command).unwrap();
            usage(stderr).unwrap();
            std::process::exit(1)
        }
    }
}
