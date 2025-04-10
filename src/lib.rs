use dot_structures::{Attribute, Edge, EdgeTy, Graph, Id, Node, NodeId, Port, Stmt, Vertex};
use open_hypergraphs::lax::OpenHypergraph;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::fmt;

/// Graph orientation for visualization
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    /// Left to right layout
    LR,
    /// Top to bottom layout
    TB,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Orientation::LR => write!(f, "LR"),
            Orientation::TB => write!(f, "TB"),
        }
    }
}

/// Theme for graph visualization
pub struct Theme {
    pub bgcolor: String,
    pub fontcolor: String,
    pub color: String,
    pub orientation: Orientation,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            bgcolor: String::from("white"),
            fontcolor: String::from("black"),
            color: String::from("black"),
            orientation: Orientation::LR,
        }
    }
}

/// A dark theme preset
pub fn dark_theme() -> Theme {
    Theme {
        bgcolor: String::from("#4a4a4a"),
        fontcolor: String::from("white"),
        color: String::from("white"),
        orientation: Orientation::LR,
    }
}

/// Generates a GraphViz DOT representation of a lax open hypergraph
pub fn generate_dot<O, A>(graph: &OpenHypergraph<O, A>, theme: &Theme) -> Graph
where
    O: Clone + Debug + PartialEq + Hash,
    A: Clone + Debug + PartialEq,
{
    // Create a directed graph
    let mut dot_graph = Graph::DiGraph {
        id: Id::Plain(String::from("G")),
        strict: false,
        stmts: Vec::new(),
    };

    // Set graph attributes
    dot_graph.add_stmt(Stmt::Attribute(Attribute(
        Id::Plain(String::from("rankdir")),
        Id::Plain(theme.orientation.to_string()),
    )));

    // Set background color
    dot_graph.add_stmt(Stmt::Attribute(Attribute(
        Id::Plain(String::from("bgcolor")),
        Id::Plain(format!("\"{}\"", theme.bgcolor.clone())),
    )));

    // Add default node attributes statement
    dot_graph.add_stmt(Stmt::Node(Node {
        id: NodeId(Id::Plain(String::from("node")), None),
        attributes: vec![
            Attribute(
                Id::Plain(String::from("shape")),
                Id::Plain(String::from("record")),
            ),
            Attribute(
                Id::Plain(String::from("style")),
                Id::Plain(String::from("rounded")),
            ),
            Attribute(
                Id::Plain(String::from("fontcolor")),
                Id::Plain(format!("\"{}\"", theme.fontcolor.clone())),
            ),
            Attribute(
                Id::Plain(String::from("color")),
                Id::Plain(format!("\"{}\"", theme.color.clone())),
            ),
        ],
    }));

    // Add default edge attributes statement
    dot_graph.add_stmt(Stmt::Node(Node {
        id: NodeId(Id::Plain(String::from("edge")), None),
        attributes: vec![
            Attribute(
                Id::Plain(String::from("fontcolor")),
                Id::Plain(format!("\"{}\"", theme.fontcolor.clone())),
            ),
            Attribute(
                Id::Plain(String::from("color")),
                Id::Plain(format!("\"{}\"", theme.color.clone())),
            ),
            Attribute(
                Id::Plain(String::from("arrowhead")),
                Id::Plain(String::from("none")),
            ),
        ],
    }));

    // Add nodes for each node in the hypergraph
    let node_stmts = generate_node_stmts(graph);
    for stmt in node_stmts {
        dot_graph.add_stmt(stmt);
    }

    // Add record nodes for each hyperedge
    let edge_stmts = generate_edge_stmts(graph);
    for stmt in edge_stmts {
        dot_graph.add_stmt(stmt);
    }

    // Add source and target interface nodes
    let interface_stmts = generate_interface_stmts(graph);
    for stmt in interface_stmts {
        dot_graph.add_stmt(stmt);
    }

    // Connect nodes to edges
    let connection_stmts = generate_connection_stmts(graph);
    for stmt in connection_stmts {
        dot_graph.add_stmt(stmt);
    }

    // Add quotient connections (dotted lines between unified nodes)
    let quotient_stmts = generate_quotient_stmts(graph);
    for stmt in quotient_stmts {
        dot_graph.add_stmt(stmt);
    }

    dot_graph
}

/// Generate node statements for each node in the hypergraph
fn generate_node_stmts<O, A>(graph: &OpenHypergraph<O, A>) -> Vec<Stmt>
where
    O: Clone + Debug + PartialEq,
    A: Clone + Debug + PartialEq,
{
    let mut stmts = Vec::new();

    for i in 0..graph.hypergraph.nodes.len() {
        let label = format!("{:?}", graph.hypergraph.nodes[i]);

        stmts.push(Stmt::Node(Node {
            id: NodeId(Id::Plain(format!("n_{}", i)), None),
            attributes: vec![
                Attribute(
                    Id::Plain(String::from("shape")),
                    Id::Plain(String::from("point")),
                ),
                Attribute(
                    Id::Plain(String::from("xlabel")),
                    Id::Plain(format!("\"{}\"", label)),
                ),
            ],
        }));
    }

    stmts
}

/// Generate record node statements for each hyperedge
fn generate_edge_stmts<O, A>(graph: &OpenHypergraph<O, A>) -> Vec<Stmt>
where
    O: Clone + Debug + PartialEq,
    A: Clone + Debug + PartialEq,
{
    let mut stmts = Vec::new();

    for i in 0..graph.hypergraph.edges.len() {
        let hyperedge = &graph.hypergraph.adjacency[i];
        let label = format!("{:?}", graph.hypergraph.edges[i]);

        // Create port sections for sources
        let mut source_ports = String::new();
        for j in 0..hyperedge.sources.len() {
            source_ports.push_str(&format!("<s_{j}> | "));
        }
        if !source_ports.is_empty() {
            source_ports.truncate(source_ports.len() - 3); // Remove last " | "
        }

        // Create port sections for targets
        let mut target_ports = String::new();
        for j in 0..hyperedge.targets.len() {
            target_ports.push_str(&format!("<t_{j}> | "));
        }
        if !target_ports.is_empty() {
            target_ports.truncate(target_ports.len() - 3); // Remove last " | "
        }

        // Create full record label with proper quoting for GraphViz DOT format
        let record_label = if source_ports.is_empty() && target_ports.is_empty() {
            format!("\"{}\"", label)
        } else if source_ports.is_empty() {
            format!("\"{{ {} | {{ {} }} }}\"", label, target_ports)
        } else if target_ports.is_empty() {
            format!("\"{{ {{ {} }} | {} }}\"", source_ports, label)
        } else {
            format!(
                "\"{{ {{ {} }} | {} | {{ {} }} }}\"",
                source_ports, label, target_ports
            )
        };

        stmts.push(Stmt::Node(Node {
            id: NodeId(Id::Plain(format!("e_{}", i)), None),
            attributes: vec![
                Attribute(Id::Plain(String::from("label")), Id::Plain(record_label)),
                Attribute(
                    Id::Plain(String::from("shape")),
                    Id::Plain(String::from("record")),
                ),
            ],
        }));
    }

    stmts
}

/// Generate statements connecting nodes to edges
fn generate_connection_stmts<O, A>(graph: &OpenHypergraph<O, A>) -> Vec<Stmt>
where
    O: Clone + Debug + PartialEq,
    A: Clone + Debug + PartialEq,
{
    let mut stmts = Vec::new();

    // Connect source nodes to edge ports
    for (i, hyperedge) in graph.hypergraph.adjacency.iter().enumerate() {
        for (_j, &node_id) in hyperedge.sources.iter().enumerate() {
            let node_idx = node_id.0; // Convert NodeId to usize

            let edge = Edge {
                ty: EdgeTy::Pair(
                    Vertex::N(NodeId(Id::Plain(format!("n_{}", node_idx)), None)),
                    Vertex::N(NodeId(Id::Plain(format!("e_{}", i)), None)),
                ),
                attributes: vec![],
            };
            stmts.push(Stmt::Edge(edge));
        }

        // Connect edge target ports to target nodes
        for (j, &node_id) in hyperedge.targets.iter().enumerate() {
            let node_idx = node_id.0; // Convert NodeId to usize

            // Create a port with the correct format
            let port = Some(Port(None, Some(format!("t_{}", j))));

            let edge = Edge {
                ty: EdgeTy::Pair(
                    Vertex::N(NodeId(Id::Plain(format!("e_{}", i)), port)),
                    Vertex::N(NodeId(Id::Plain(format!("n_{}", node_idx)), None)),
                ),
                attributes: vec![],
            };
            stmts.push(Stmt::Edge(edge));
        }
    }

    stmts
}

/// Generate interface nodes for sources and targets of the hypergraph
fn generate_interface_stmts<O, A>(graph: &OpenHypergraph<O, A>) -> Vec<Stmt>
where
    O: Clone + Debug + PartialEq,
    A: Clone + Debug + PartialEq,
{
    let mut stmts = Vec::new();

    // Create source interface record node
    if !graph.sources.is_empty() {
        // Create port sections for sources
        let mut source_ports = String::new();
        for i in 0..graph.sources.len() {
            source_ports.push_str(&format!("<p_{i}> | "));
        }
        // Remove last " | "
        if !source_ports.is_empty() {
            source_ports.truncate(source_ports.len() - 3);
        }

        // Create the source interface node
        stmts.push(Stmt::Node(Node {
            id: NodeId(Id::Plain(String::from("sources")), None),
            attributes: vec![
                Attribute(
                    Id::Plain(String::from("label")),
                    Id::Plain(format!("\"{{ {{}} | {{ {} }} }}\"", source_ports)),
                ),
                Attribute(
                    Id::Plain(String::from("shape")),
                    Id::Plain(String::from("record")),
                ),
                Attribute(
                    Id::Plain(String::from("style")),
                    Id::Plain(String::from("invisible")),
                ),
                Attribute(
                    Id::Plain(String::from("rank")),
                    Id::Plain(String::from("source")),
                ),
            ],
        }));

        // Connect source interface ports to the source nodes
        for (i, &source_node_id) in graph.sources.iter().enumerate() {
            let edge = Edge {
                ty: EdgeTy::Pair(
                    Vertex::N(NodeId(
                        Id::Plain(String::from("sources")),
                        Some(Port(None, Some(format!("p_{}", i)))),
                    )),
                    Vertex::N(NodeId(Id::Plain(format!("n_{}", source_node_id.0)), None)),
                ),
                attributes: vec![Attribute(
                    Id::Plain(String::from("style")),
                    Id::Plain(String::from("dashed")),
                )],
            };
            stmts.push(Stmt::Edge(edge));
        }
    }

    // Create target interface record node
    if !graph.targets.is_empty() {
        // Create port sections for targets
        let mut target_ports = String::new();
        for i in 0..graph.targets.len() {
            target_ports.push_str(&format!("<p_{i}> | "));
        }
        // Remove last " | "
        if !target_ports.is_empty() {
            target_ports.truncate(target_ports.len() - 3);
        }

        // Create the target interface node
        stmts.push(Stmt::Node(Node {
            id: NodeId(Id::Plain(String::from("targets")), None),
            attributes: vec![
                Attribute(
                    Id::Plain(String::from("label")),
                    Id::Plain(format!("\"{{ {{ {} }} | {{}} }}\"", target_ports)),
                ),
                Attribute(
                    Id::Plain(String::from("shape")),
                    Id::Plain(String::from("record")),
                ),
                Attribute(
                    Id::Plain(String::from("style")),
                    Id::Plain(String::from("invisible")),
                ),
                Attribute(
                    Id::Plain(String::from("rank")),
                    Id::Plain(String::from("sink")),
                ),
            ],
        }));

        // Connect target nodes to target interface ports
        for (i, &target_node_id) in graph.targets.iter().enumerate() {
            let edge = Edge {
                ty: EdgeTy::Pair(
                    Vertex::N(NodeId(Id::Plain(format!("n_{}", target_node_id.0)), None)),
                    Vertex::N(NodeId(
                        Id::Plain(String::from("targets")),
                        Some(Port(None, Some(format!("p_{}", i)))),
                    )),
                ),
                attributes: vec![Attribute(
                    Id::Plain(String::from("style")),
                    Id::Plain(String::from("dashed")),
                )],
            };
            stmts.push(Stmt::Edge(edge));
        }
    }

    stmts
}

/// Generate statements for quotient connections (dotted lines between unified nodes)
fn generate_quotient_stmts<O, A>(graph: &OpenHypergraph<O, A>) -> Vec<Stmt>
where
    O: Clone + Debug + PartialEq,
    A: Clone + Debug + PartialEq,
{
    let mut stmts = Vec::new();

    // Extract unified node pairs from the quotient
    let (lefts, rights) = &graph.hypergraph.quotient;

    // Create a map to track which nodes are unified
    let mut unified_nodes = HashMap::new();

    for (left, right) in lefts.iter().zip(rights.iter()) {
        let left_idx = left.0; // Access the internal usize
        let right_idx = right.0;

        // Check if we've already seen this pair (in any order)
        let pair_key = if left_idx < right_idx {
            (left_idx, right_idx)
        } else {
            (right_idx, left_idx)
        };

        if unified_nodes.insert(pair_key, true).is_none() {
            // Create a dashed edge between unified nodes
            let edge = Edge {
                ty: EdgeTy::Pair(
                    Vertex::N(NodeId(Id::Plain(format!("n_{}", left_idx)), None)),
                    Vertex::N(NodeId(Id::Plain(format!("n_{}", right_idx)), None)),
                ),
                attributes: vec![
                    Attribute(
                        Id::Plain(String::from("style")),
                        Id::Plain(String::from("dotted")),
                    ),
                    Attribute(
                        Id::Plain(String::from("dir")),
                        Id::Plain(String::from("none")),
                    ),
                ],
            };
            stmts.push(Stmt::Edge(edge));
        }
    }

    stmts
}
