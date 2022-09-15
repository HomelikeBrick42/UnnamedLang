use std::fmt::Display;

use langite::*;

fn unwrap_error<T, E: Display>(result: Result<T, E>) -> T {
    result.unwrap_or_else(|error| {
        eprintln!("{}", error);
        std::process::exit(1)
    })
}

fn main() {
    let filepath = "./test.lang";
    let file = unwrap_error(parse_file(
        filepath.into(),
        &std::fs::read_to_string(filepath).unwrap(),
        &mut Default::default(),
    ));
    let program = Ast::File(file);
    println!("{program:#?}");
}
