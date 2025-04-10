use open_hypergraphs::lax::OpenHypergraph;
use open_hypergraphs_dot::generate_dot;

use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::process::Command;

use graphviz_rust::dot_structures::Graph;
use graphviz_rust::printer::{DotPrinter, PrinterContext};

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

/// Render a graph to DOT format string
pub fn render_dot(graph: &Graph) -> String {
    let mut ctx = PrinterContext::default();
    graph.print(&mut ctx)
}

fn main() -> std::io::Result<()> {
    // Create a simple lax hypergraph: Copy operation connected to Multiply
    let mut graph = OpenHypergraph::<NodeType, Operation>::empty();

    // Add a Copy operation (1 → 2)
    let (_, (input_nodes, x)) = graph.new_operation(
        Operation::Copy,
        vec![NodeType::A],
        vec![NodeType::B, NodeType::B],
    );

    // Add a Mul operation (2 → 1)
    let (_, (y, output_nodes)) = graph.new_operation(
        Operation::Mul,
        vec![NodeType::B, NodeType::B],
        vec![NodeType::A],
    );

    // Connect copy outputs to multiply inputs
    graph.unify(x[0], y[0]);
    graph.unify(x[1], y[1]);

    // Specify sources and targets for the interface
    graph.sources = input_nodes;
    graph.targets = output_nodes;

    // Generate GraphViz DOT representation
    let dot_graph = generate_dot(&graph);
    let dot_string = render_dot(&dot_graph);

    // Print DOT string
    println!("Generated DOT representation:");
    println!("{}", dot_string);

    // Save DOT to file
    let output_path = "examples/copy_multiply.dot";
    let mut file = File::create(output_path)?;
    file.write_all(dot_string.as_bytes())?;
    println!("DOT file saved to {}", output_path);

    // Try to render with GraphViz if available
    let output_png = "examples/copy_multiply.png";
    match Command::new("dot")
        .args(["-Tpng", output_path, "-o", output_png])
        .status()
    {
        Ok(status) if status.success() => println!("PNG image rendered to {}", output_png),
        _ => println!("Note: Install GraphViz to render the DOT file as an image."),
    }

    Ok(())
}
