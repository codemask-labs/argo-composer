use yaml_ast::{YamlDocument, YamlNode, YamlValue};

/// Trait for types that can be serialized to YAML AST nodes
pub trait YamlSerializer {
    /// Convert this type into YAML AST nodes
    fn to_yaml_nodes(&self) -> Vec<YamlNode>;
}

/// Formatter for converting YAML AST to formatted strings
pub struct YamlFormatter {
    indent_size: usize,
}

impl YamlFormatter {
    pub fn new() -> Self {
        Self { indent_size: 2 }
    }

    pub fn with_indent(indent_size: usize) -> Self {
        Self { indent_size }
    }

    /// Format YAML documents to a string
    pub fn format_documents(&self, documents: &[YamlDocument]) -> String {
        let mut output = String::new();

        for (index, document) in documents.iter().enumerate() {
            if index > 0 {
                output.push_str("---\n");
            }

            for node in &document.nodes {
                self.format_node(&mut output, node, 0);
            }
        }

        output
    }

    /// Format a single YAML node
    fn format_node(&self, output: &mut String, node: &YamlNode, indent_level: usize) {
        match &node.value {
            YamlValue::Comment(comment) => {
                // Handle multi-line comments by splitting and prefixing each line
                for line in comment.lines() {
                    self.write_indent(output, indent_level);
                    output.push_str("# ");
                    output.push_str(line);
                    output.push('\n');
                }
            }
            YamlValue::EmptyLine => {
                output.push('\n');
            }
            YamlValue::Object(pairs) => {
                self.format_object(output, pairs, indent_level, &node.inline_comment);
            }
            YamlValue::Collection(items) => {
                self.format_sequence(output, items, indent_level);
            }
            YamlValue::String(value) => {
                self.write_indent(output, indent_level);
                self.write_string_value(output, value);
                self.write_inline_comment(output, &node.inline_comment);
                output.push('\n');
            }
            YamlValue::Number(num) => {
                self.write_indent(output, indent_level);
                output.push_str(&num.to_string());
                self.write_inline_comment(output, &node.inline_comment);
                output.push('\n');
            }
            YamlValue::Boolean(bool_value) => {
                self.write_indent(output, indent_level);
                output.push_str(if *bool_value { "true" } else { "false" });
                self.write_inline_comment(output, &node.inline_comment);
                output.push('\n');
            }
            YamlValue::Null => {
                self.write_indent(output, indent_level);
                output.push_str("null");
                self.write_inline_comment(output, &node.inline_comment);
                output.push('\n');
            }
        }
    }

    /// Format an object (key-value pairs)
    fn format_object(
        &self,
        output: &mut String,
        pairs: &[(String, YamlNode)],
        indent_level: usize,
        _inline_comment: &Option<String>,
    ) {
        for (index, (key, value_node)) in pairs.iter().enumerate() {
            // Write leading comment if present
            if let Some(leading_comment) = &value_node.leading_comment {
                for line in leading_comment.lines() {
                    self.write_indent(output, indent_level);
                    output.push_str("# ");
                    output.push_str(line);
                    output.push('\n');
                }
            }
            
            self.write_indent(output, indent_level);
            output.push_str(key);
            output.push(':');

            match &value_node.value {
                YamlValue::Object(nested_pairs) => {
                    self.write_inline_comment(output, &value_node.inline_comment);
                    output.push('\n');
                    self.format_object(output, nested_pairs, indent_level + 1, &None);
                }
                YamlValue::Collection(items) => {
                    self.write_inline_comment(output, &value_node.inline_comment);
                    output.push('\n');
                    self.format_sequence(output, items, indent_level + 1);
                }
                YamlValue::String(string_value) => {
                    output.push(' ');
                    self.write_string_value(output, string_value);
                    self.write_inline_comment(output, &value_node.inline_comment);
                    output.push('\n');
                }
                YamlValue::Number(num) => {
                    output.push(' ');
                    output.push_str(&num.to_string());
                    self.write_inline_comment(output, &value_node.inline_comment);
                    output.push('\n');
                }
                YamlValue::Boolean(bool_value) => {
                    output.push(' ');
                    output.push_str(if *bool_value { "true" } else { "false" });
                    self.write_inline_comment(output, &value_node.inline_comment);
                    output.push('\n');
                }
                YamlValue::Null => {
                    output.push('\n');
                    // Null value on next line is implied by empty value
                }
                YamlValue::Comment(_) | YamlValue::EmptyLine => {
                    output.push('\n');
                    self.format_node(output, value_node, indent_level + 1);
                }
            }

            // Add empty line between top-level keys for readability
            if indent_level == 0 && index < pairs.len() - 1 {
                if let Some((_, next_value)) = pairs.get(index + 1) {
                    if !matches!(next_value.value, YamlValue::EmptyLine) {
                        output.push('\n');
                    }
                }
            }
        }
    }

    /// Format a sequence (array)
    fn format_sequence(&self, output: &mut String, items: &[YamlNode], indent_level: usize) {
        for item in items {
            match &item.value {
                YamlValue::EmptyLine => {
                    output.push('\n');
                }
                YamlValue::Comment(comment) => {
                    self.write_indent(output, indent_level);
                    output.push_str("# ");
                    output.push_str(comment);
                    output.push('\n');
                }
                YamlValue::Object(pairs) => {
                    self.write_indent(output, indent_level);
                    output.push_str("- ");
                    if let Some((first_key, first_value)) = pairs.first() {
                        output.push_str(first_key);
                        output.push(':');

                        match &first_value.value {
                            YamlValue::String(s) => {
                                output.push(' ');
                                self.write_string_value(output, s);
                                self.write_inline_comment(output, &first_value.inline_comment);
                                output.push('\n');
                            }
                            YamlValue::Number(num) => {
                                output.push(' ');
                                output.push_str(&num.to_string());
                                self.write_inline_comment(output, &first_value.inline_comment);
                                output.push('\n');
                            }
                            YamlValue::Boolean(bool_value) => {
                                output.push(' ');
                                output.push_str(if *bool_value { "true" } else { "false" });
                                self.write_inline_comment(output, &first_value.inline_comment);
                                output.push('\n');
                            }
                            _ => {
                                output.push('\n');
                                self.format_node(output, first_value, indent_level + 1);
                            }
                        }

                        // Format remaining pairs with proper indentation
                        for (key, value) in pairs.iter().skip(1) {
                            self.write_indent(output, indent_level + 1);
                            output.push_str(key);
                            output.push(':');

                            match &value.value {
                                YamlValue::String(s) => {
                                    output.push(' ');
                                    self.write_string_value(output, s);
                                    self.write_inline_comment(output, &value.inline_comment);
                                    output.push('\n');
                                }
                                YamlValue::Number(num) => {
                                    output.push(' ');
                                    output.push_str(&num.to_string());
                                    self.write_inline_comment(output, &value.inline_comment);
                                    output.push('\n');
                                }
                                YamlValue::Boolean(bool_value) => {
                                    output.push(' ');
                                    output.push_str(if *bool_value { "true" } else { "false" });
                                    self.write_inline_comment(output, &value.inline_comment);
                                    output.push('\n');
                                }
                                _ => {
                                    output.push('\n');
                                    self.format_node(output, value, indent_level + 2);
                                }
                            }
                        }
                    }
                }
                _ => {
                    self.write_indent(output, indent_level);
                    output.push_str("- ");

                    match &item.value {
                        YamlValue::String(s) => {
                            self.write_string_value(output, s);
                            self.write_inline_comment(output, &item.inline_comment);
                            output.push('\n');
                        }
                        YamlValue::Number(num) => {
                            output.push_str(&num.to_string());
                            self.write_inline_comment(output, &item.inline_comment);
                            output.push('\n');
                        }
                        YamlValue::Boolean(bool_value) => {
                            output.push_str(if *bool_value { "true" } else { "false" });
                            self.write_inline_comment(output, &item.inline_comment);
                            output.push('\n');
                        }
                        YamlValue::Null => {
                            output.push('\n');
                        }
                        YamlValue::Collection(nested_items) => {
                            output.push('\n');
                            self.format_sequence(output, nested_items, indent_level + 1);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Write indentation
    fn write_indent(&self, output: &mut String, level: usize) {
        output.push_str(&" ".repeat(level * self.indent_size));
    }

    /// Write a string value, quoting if necessary
    fn write_string_value(&self, output: &mut String, value: &str) {
        // Check if string needs quoting
        if self.needs_quoting(value) {
            output.push('"');
            // Escape special characters
            for character in value.chars() {
                match character {
                    '"' => output.push_str("\\\""),
                    '\\' => output.push_str("\\\\"),
                    '\n' => output.push_str("\\n"),
                    '\r' => output.push_str("\\r"),
                    '\t' => output.push_str("\\t"),
                    _ => output.push(character),
                }
            }
            output.push('"');
        } else {
            output.push_str(value);
        }
    }

    /// Check if a string needs quoting
    fn needs_quoting(&self, value: &str) -> bool {
        if value.is_empty() {
            return true;
        }

        // Check for special YAML values that would be interpreted differently
        matches!(value, "true" | "false" | "yes" | "no" | "null" | "~")
            || value.starts_with('#')
            || value.starts_with('-')
            || value.starts_with('[')
            || value.starts_with('{')
            || value.starts_with('&')
            || value.starts_with('*')
            || value.starts_with('!')
            || value.contains(':')
            || value.contains('\n')
            || value.contains('\r')
            || value.contains('"')
            || value.starts_with(' ')
            || value.ends_with(' ')
    }

    /// Write inline comment if present
    fn write_inline_comment(&self, output: &mut String, comment: &Option<String>) {
        if let Some(comment_text) = comment {
            output.push_str(" # ");
            output.push_str(comment_text);
        }
    }
}

impl Default for YamlFormatter {
    fn default() -> Self {
        Self::new()
    }
}

// Implement YamlSerializer for Vec<T> where T implements YamlSerializer
impl<T: YamlSerializer> YamlSerializer for Vec<T> {
    fn to_yaml_nodes(&self) -> Vec<YamlNode> {
        if self.is_empty() {
            // Empty array
            vec![YamlNode {
                value: YamlValue::Collection(vec![]),
                inline_comment: None,
                leading_comment: None,
            }]
        } else {
            // Convert each item to its nodes and wrap in Collection
            let items: Vec<YamlNode> = self.iter()
                .flat_map(|item| item.to_yaml_nodes())
                .collect();
            
            vec![YamlNode {
                value: YamlValue::Collection(items),
                inline_comment: None,
                leading_comment: None,
            }]
        }
    }
}

// Implement YamlSerializer for String
impl YamlSerializer for String {
    fn to_yaml_nodes(&self) -> Vec<YamlNode> {
        vec![YamlNode {
            value: YamlValue::String(self.clone()),
            inline_comment: None,
                leading_comment: None,
        }]
    }
}

// Implement YamlSerializer for bool
impl YamlSerializer for bool {
    fn to_yaml_nodes(&self) -> Vec<YamlNode> {
        vec![YamlNode {
            value: YamlValue::Boolean(*self),
            inline_comment: None,
                leading_comment: None,
        }]
    }
}

// Implement YamlSerializer for i64
impl YamlSerializer for i64 {
    fn to_yaml_nodes(&self) -> Vec<YamlNode> {
        vec![YamlNode {
            value: YamlValue::Number(*self as f64),
            inline_comment: None,
                leading_comment: None,
        }]
    }
}

// Implement YamlSerializer for BTreeMap<String, String>
impl YamlSerializer for std::collections::BTreeMap<String, String> {
    fn to_yaml_nodes(&self) -> Vec<YamlNode> {
        let pairs: Vec<(String, YamlNode)> = self.iter()
            .map(|(k, v)| {
                (k.clone(), YamlNode {
                    value: YamlValue::String(v.clone()),
                    inline_comment: None,
                leading_comment: None,
                })
            })
            .collect();
        
        vec![YamlNode {
            value: YamlValue::Object(pairs),
            inline_comment: None,
                leading_comment: None,
        }]
    }
}
