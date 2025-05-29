use std::fmt;
use std::fmt::Debug;

pub struct Options<O, A> {
    pub orientation: Orientation,
    pub theme: Theme,
    pub node_label: Box<dyn Fn(&O) -> String>,
    pub edge_label: Box<dyn Fn(&A) -> String>,
}

impl<O: Debug, A: Debug> Default for Options<O, A> {
    fn default() -> Self {
        Self {
            orientation: Default::default(),
            theme: Default::default(),
            node_label: Box::new(|n| format!("{:?}", n)),
            edge_label: Box::new(|e| format!("{:?}", e)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Orientation

/// Graph orientation for visualization
#[derive(Debug, Clone, Copy, Default)]
pub enum Orientation {
    /// Left to right layout
    LR,
    /// Top to bottom layout
    #[default]
    TB,
}

// Used for dot output
impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Orientation::LR => write!(f, "LR"),
            Orientation::TB => write!(f, "TB"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Themes

/// Theme for graph visualization
pub struct Theme {
    pub bgcolor: String,
    pub fontcolor: String,
    pub color: String,
    pub orientation: Orientation,
}

pub fn light_theme() -> Theme {
    Theme {
        bgcolor: String::from("white"),
        fontcolor: String::from("black"),
        color: String::from("black"),
        orientation: Orientation::LR,
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

impl Default for Theme {
    fn default() -> Self {
        dark_theme()
    }
}
