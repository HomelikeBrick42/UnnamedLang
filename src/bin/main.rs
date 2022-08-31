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
        "
proc factorial(n: u64) => u64 {
    if n <= 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

do_whatever(print_u64, 42)

proc loop() => void {
    print_char(65)
    print_char(10)
    loop()
}

// loop()

proc print_n(n: u64) => void {
    var i: u64 <- 1
    while i <= n {
        print_u64(i)
        // i <- i + 1
    }
}

print_n(10)

// while 0 == 0 {
//     print_char(input_char())
// }

proc print_s64(value: s64) => void #extern \"print_s64\"
proc print_u64(value: u64) => void #extern \"print_u64\"

proc do_whatever(print_proc: proc(u64) => void, value: s64) => void {
    print_proc(factorial(6))
    print_proc(cast(u64) value)
    print_char(69)
    print_char(10)
}

proc print_char(value: s32) => s32 #extern \"putchar\"
proc input_char() => s32 #extern \"getchar\"
",
    ));
    let program = Ast::File(file);
    let mut names = HashMap::from([
        ("type".into(), Declaration::Builtin(AstBuiltin::Type.into())),
        ("void".into(), Declaration::Builtin(AstBuiltin::Void.into())),
        ("bool".into(), Declaration::Builtin(AstBuiltin::Bool.into())),
        (
            "s8".into(),
            Declaration::Builtin(
                AstBuiltin::IntegerType {
                    size: 1,
                    signed: true,
                }
                .into(),
            ),
        ),
        (
            "s16".into(),
            Declaration::Builtin(
                AstBuiltin::IntegerType {
                    size: 2,
                    signed: true,
                }
                .into(),
            ),
        ),
        (
            "s32".into(),
            Declaration::Builtin(
                AstBuiltin::IntegerType {
                    size: 4,
                    signed: true,
                }
                .into(),
            ),
        ),
        (
            "s64".into(),
            Declaration::Builtin(
                AstBuiltin::IntegerType {
                    size: 8,
                    signed: true,
                }
                .into(),
            ),
        ),
        (
            "u8".into(),
            Declaration::Builtin(
                AstBuiltin::IntegerType {
                    size: 1,
                    signed: false,
                }
                .into(),
            ),
        ),
        (
            "u16".into(),
            Declaration::Builtin(
                AstBuiltin::IntegerType {
                    size: 2,
                    signed: false,
                }
                .into(),
            ),
        ),
        (
            "u32".into(),
            Declaration::Builtin(
                AstBuiltin::IntegerType {
                    size: 4,
                    signed: false,
                }
                .into(),
            ),
        ),
        (
            "u64".into(),
            Declaration::Builtin(
                AstBuiltin::IntegerType {
                    size: 8,
                    signed: false,
                }
                .into(),
            ),
        ),
    ]);
    unwrap_error(resolve_names(&program, &mut names));
    unwrap_error(resolve(&program, None, &mut vec![], &None));
    let mut string = Vec::new();
    emit(&program, &mut 1, &mut string).unwrap();
    std::fs::write("output.c", &string).unwrap();
}
