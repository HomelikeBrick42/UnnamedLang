use std::collections::HashMap;

use langite::{
    binding::{bind_file, extra_checks, BoundNode, BoundNodes, Type},
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

    let type_type = nodes.get_types_mut().get_builtin_type(Type::Type);

    let mut register_type = |name, typ| {
        let typ = nodes.get_types_mut().get_builtin_type(typ);
        let node = nodes.add_node(
            BoundNode::Type(typ),
            SourceSpan::builtin_location(),
            type_type,
        );
        assert!(names.insert(name, node).is_none());
    };
    register_type("void", Type::Void);
    register_type("type", Type::Type);
    register_type("never", Type::Never);
    register_type("int", Type::Int);
    register_type("char", Type::Char);

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

    let extra_checks_start_time = std::time::Instant::now();
    extra_checks(root, &nodes).unwrap_or_else(|error| {
        println!("{error}");
        std::process::exit(1)
    });
    let extra_checks_time = extra_checks_start_time.elapsed().as_secs_f64();
    println!(
        "Extra checks completed successfully in {:.3}ms",
        extra_checks_time * 1000.0
    );

    println!("\nTypes:\n{}", nodes.get_types());
    println!("\nNodes:\n{nodes}");
}
