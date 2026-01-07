#[derive(Debug, Clone, PartialEq)]
pub enum YamlValue {
    Null,
    Number(f64),
    Boolean(bool),
    String(String),
    Collection(Vec<YamlNode>),
    Object(Vec<(String, YamlNode)>),
    Comment(String), // Standalone comment line
    EmptyLine,       // Empty line for formatting preservation
}

// AST Node with inline comment support
#[derive(Debug, Clone, PartialEq)]
pub struct YamlNode {
    pub value: YamlValue,
    pub inline_comment: Option<String>,
    pub leading_comment: Option<String>, // Comment that appears before this node
}

#[derive(Debug)]
pub enum YamlAbstractSyntaxTreeError {
    FailedToOpenFile,
    FailedToReadFile,
    UnexpectedEof,
    InvalidSyntax { line: u64, message: String },
}

pub struct YamlDocument {
    pub nodes: Vec<YamlNode>, // Can include Comment nodes and content nodes
}

// Represents a parsed line
#[derive(Debug, Clone)]
pub(crate) struct Line {
    pub(crate) indent: usize,
    pub(crate) content: String,
    pub(crate) leading_comment: Option<String>,
    pub(crate) inline_comment: Option<String>,
}
