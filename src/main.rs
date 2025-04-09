use open_hypergraphs::lax::OpenHypergraph;
use open_hypergraphs_dot::{generate_dot, render_dot};
use std::fs::File;
use std::hash::Hash;
use std::io::Write;

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum NodeType {
    A,
    B,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Operation {
    Copy,
    Mul,
}

fn main() -> std::io::Result<()> {
    // Create a simple lax hypergraph: Copy operation connected to Multiply
    let mut graph = OpenHypergraph::<NodeType, Operation>::empty();

    // Add a Copy operation (1 → 2)
    let (_, (_, x)) = graph.new_operation(
        Operation::Copy,
        vec![NodeType::A],
        vec![NodeType::B, NodeType::B],
    );

    // Add a Mul operation (2 → 1)
    let (_, (y, _)) = graph.new_operation(
        Operation::Mul,
        vec![NodeType::B, NodeType::B],
        vec![NodeType::A],
    );

    // Connect copy outputs to multiply inputs
    graph.unify(x[0], y[0]);
    graph.unify(x[1], y[1]);

    // Generate GraphViz DOT representation
    let dot_graph = generate_dot(&graph);
    let dot_string = render_dot(&dot_graph);

    // Print DOT string
    println!("Generated DOT representation:");
    println!("{}", dot_string);

    // Save DOT to file
    let mut file = File::create("output.dot")?;
    file.write_all(dot_string.as_bytes())?;
    println!("DOT file saved to output.dot");

    Ok(())
}
