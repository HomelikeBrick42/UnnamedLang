use std::{collections::HashMap, fmt::Display};

use langite::*;

fn unwrap_error<T, E: Display>(result: Result<T, E>) -> T {
    result.unwrap_or_else(|error| {
        eprintln!("{}", error);
        std::process::exit(1)
    })
}

fn main() {
    let file = unwrap_error(parse_file(
        "test.lang".into(),
        "do_whatever(print_int)

proc print_int(value: s64): void #extern \"print_int\"

proc do_whatever(print_proc: proc(s64): void): void {
    print_proc(42)
}
",
    ));
    let program = Ast::File(file);
    let mut names = HashMap::from([
        ("type".into(), Declaration::Builtin(AstBuiltin::Type.into())),
        ("void".into(), Declaration::Builtin(AstBuiltin::Void.into())),
        ("s64".into(), Declaration::Builtin(AstBuiltin::S64.into())),
    ]);
    unwrap_error(resolve_names(&program, &mut names));
    unwrap_error(resolve(&program, None, &mut vec![]));
    let mut string = Vec::new();
    emit(&program, &mut 1, &mut string).unwrap();
    std::fs::write("output.c", &string).unwrap();
}
