use langite::syntax::parse_file;

fn main() {
    let filepath = "test.lang";
    let source = "
proc foo(a: int, b: int, c: int) => int {
    return a + b * c
}

foo(5, 6, 7)
";
    let parse_start_time = std::time::Instant::now();
    let result = parse_file(filepath, source);
    let time = parse_start_time.elapsed().as_secs_f64();
    match result {
        Ok(_asts) => {
            println!("Parsed successfully in {:.3}ms", time * 1000.0);
        }
        Err(error) => println!("{error}"),
    }
}
