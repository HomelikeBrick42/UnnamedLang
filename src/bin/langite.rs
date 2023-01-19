use std::collections::HashMap;

use langite::{
    binding::{bind_file, BoundNode, BoundNodes, Type},
    syntax::{parse_file, SourceSpan},
};

fn main() {
    let filepath = "test.lang";
    let source = std::fs::read_to_string(filepath).unwrap();
    let parse_start_time = std::time::Instant::now();
    let expressions = parse_file(filepath, &source).unwrap_or_else(|error| {
        println!("{error}");
        std::process::exit(1)
    });
    let parse_time = parse_start_time.elapsed().as_secs_f64();
    println!("Parsed successfully in {:.3}ms", parse_time * 1000.0);

    let mut nodes = BoundNodes::new();
    let mut names = HashMap::new();

    let int_type = nodes.get_types_mut().get_builtin_type(Type::Int);
    let int = nodes.add_node(
        BoundNode::Type(int_type),
        SourceSpan::builtin_location(),
        int_type,
    );
    names.insert("int", int);

    let binding_start_time = std::time::Instant::now();
    let root = bind_file(filepath, &mut nodes, &expressions, &names).unwrap_or_else(|error| {
        println!("{error}");
        std::process::exit(1)
    });
    let binding_time = binding_start_time.elapsed().as_secs_f64();
    println!(
        "Type checked successfully in {:.3}ms",
        binding_time * 1000.0
    );

    _ = root;
    println!("\nTypes:\n{}", nodes.get_types());
    println!("\nNodes:\n{nodes}");
}
