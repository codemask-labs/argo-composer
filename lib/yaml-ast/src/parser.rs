use crate::types::{Line, YamlAbstractSyntaxTreeError, YamlDocument, YamlNode, YamlValue};
use crate::utils;
use std::{fs::File, io::Read, path::PathBuf};

pub struct YamlAbstractSyntaxTree {
    lines: Vec<Line>,
    current_line_index: usize,
}

impl YamlAbstractSyntaxTree {
    pub fn from_path(path: PathBuf) -> Result<Vec<YamlDocument>, YamlAbstractSyntaxTreeError> {
        let mut content = String::new();
        let mut file =
            File::open(path).map_err(|_| YamlAbstractSyntaxTreeError::FailedToOpenFile)?;
        file.read_to_string(&mut content)
            .map_err(|_| YamlAbstractSyntaxTreeError::FailedToReadFile)?;

        Self::parse_string(content)
    }

    pub fn parse_string(input: String) -> Result<Vec<YamlDocument>, YamlAbstractSyntaxTreeError> {
        let chars: Vec<char> = input.chars().collect();

        let mut parser = Self {
            lines: Vec::new(),
            current_line_index: 0,
        };

        let mut documents = Vec::new();
        let mut current_document_nodes = Vec::new();
        let mut character_position = 0;

        // Parse loop: tokenize and parse nodes on-demand
        loop {
            // Tokenize next line if needed
            if parser.current_line_index >= parser.lines.len() && character_position < chars.len() {
                let (line, next_position) = utils::parse_single_line(&chars, character_position)?;
                character_position = next_position;
                parser.lines.push(line);
            }

            // If no more lines available, break
            if parser.current_line_index >= parser.lines.len() {
                break;
            }

            let line = &parser.lines[parser.current_line_index];

            // Handle document markers
            if !line.content.is_empty() && line.content.trim() == "---" {
                if !current_document_nodes.is_empty() {
                    documents.push(YamlDocument {
                        nodes: current_document_nodes,
                    });
                    current_document_nodes = Vec::new();
                }
                parser.current_line_index += 1;
                continue;
            }

            if !line.content.is_empty() && line.content.trim() == "..." {
                if !current_document_nodes.is_empty() {
                    documents.push(YamlDocument {
                        nodes: current_document_nodes,
                    });
                    current_document_nodes = Vec::new();
                }
                parser.current_line_index += 1;
                continue;
            }

            // Parse one node - it will consume all lines it needs
            if let Some(node) = parser.parse_node(0, &chars, &mut character_position)? {
                current_document_nodes.push(node);
            }
        }

        // Add final document if it has nodes
        if !current_document_nodes.is_empty() {
            documents.push(YamlDocument {
                nodes: current_document_nodes,
            });
        }

        Ok(documents)
    }

    // ============================================================================
    // NODE PARSING - Build YamlNode tree from lines
    // ============================================================================

    fn current_line(&self) -> &Line {
        &self.lines[self.current_line_index]
    }

    fn parse_node(
        &mut self,
        expected_indent: usize,
        chars: &[char],
        character_position: &mut usize,
    ) -> Result<Option<YamlNode>, YamlAbstractSyntaxTreeError> {
        // Tokenize next line if needed
        if self.current_line_index >= self.lines.len() && *character_position < chars.len() {
            let (line, next_position) = utils::parse_single_line(chars, *character_position)?;
            *character_position = next_position;
            self.lines.push(line);
        }

        // Guard: EOF
        if self.current_line_index >= self.lines.len() {
            return Ok(None);
        }

        let line = self.current_line();

        // Guard: standalone comment line - return as Comment value
        if line.content.is_empty() && line.leading_comment.is_some() {
            let comment = line.leading_comment.clone().unwrap();
            self.current_line_index += 1;
            return Ok(Some(YamlNode {
                value: YamlValue::Comment(comment),
                inline_comment: None,
                leading_comment: None,
            }));
        }

        // Guard: empty line (no content, no comment) - preserve for formatting
        if line.content.is_empty() && line.leading_comment.is_none() {
            self.current_line_index += 1;
            return Ok(Some(YamlNode {
                value: YamlValue::EmptyLine,
                inline_comment: None,
                leading_comment: None,
            }));
        }

        // Guard: dedent
        if line.indent < expected_indent {
            return Ok(None);
        }

        let inline_comment = line.inline_comment.clone();
        let content = line.content.trim();

        // Pattern matching for value types
        if content.starts_with('[') {
            let value = self.parse_inline_array_from_line(content)?;
            self.current_line_index += 1;
            return Ok(Some(YamlNode {
                value,
                inline_comment,
                leading_comment: None,
            }));
        }

        if content.starts_with('{') {
            let value = self.parse_inline_object_from_line(content)?;
            self.current_line_index += 1;
            return Ok(Some(YamlNode {
                value,
                inline_comment,
                leading_comment: None,
            }));
        }

        // Check for multiline string indicators (|, >, |2, >-3, etc.)
        if content.starts_with('|') || content.starts_with('>') {
            let (is_literal, strip_trailing, explicit_indent) =
                utils::parse_multiline_indicator(content);
            let base_indent = line.indent;
            self.current_line_index += 1;

            let multiline_value = self.parse_multiline_string(
                base_indent,
                is_literal,
                strip_trailing,
                explicit_indent,
                chars,
                character_position,
            )?;
            return Ok(Some(YamlNode {
                value: YamlValue::String(multiline_value),
                inline_comment,
                leading_comment: None,
            }));
        }

        if content.starts_with("- ") {
            return Ok(Some(YamlNode {
                value: self.parse_sequence(expected_indent, chars, character_position)?,
                inline_comment,
                leading_comment: None,
            }));
        }

        if content.contains(':') {
            return Ok(Some(YamlNode {
                value: self.parse_object(expected_indent, chars, character_position)?,
                inline_comment,
                leading_comment: None,
            }));
        }

        // Scalar value
        let value = self.parse_scalar_value(content);
        self.current_line_index += 1;
        Ok(Some(YamlNode {
            value,
            inline_comment,
            leading_comment: None,
        }))
    }

    fn parse_scalar_value(&self, text: &str) -> YamlValue {
        match text {
            "true" | "True" | "TRUE" | "yes" | "Yes" | "YES" => YamlValue::Boolean(true),
            "false" | "False" | "FALSE" | "no" | "No" | "NO" => YamlValue::Boolean(false),
            "null" | "Null" | "NULL" | "~" | "" => YamlValue::Null,
            _ => {
                if let Ok(num) = text.parse::<f64>() {
                    YamlValue::Number(num)
                } else {
                    YamlValue::String(text.to_string())
                }
            }
        }
    }

    fn parse_multiline_string(
        &mut self,
        base_indent: usize,
        is_literal: bool,
        strip_trailing: bool,
        explicit_indent: Option<usize>,
        chars: &[char],
        character_position: &mut usize,
    ) -> Result<String, YamlAbstractSyntaxTreeError> {
        let mut lines = Vec::new();

        // Determine the expected indentation for content
        let content_indent = if let Some(explicit) = explicit_indent {
            base_indent + explicit
        } else {
            // Auto-detect from first non-empty line
            base_indent + 2 // default assumption
        };

        // Collect all lines that belong to this multiline string
        loop {
            // Tokenize next line if needed
            if self.current_line_index >= self.lines.len() && *character_position < chars.len() {
                let (line, next_position) = utils::parse_single_line(chars, *character_position)?;
                *character_position = next_position;
                self.lines.push(line);
            }

            // Check if we have more lines
            if self.current_line_index >= self.lines.len() {
                break;
            }

            let line = &self.lines[self.current_line_index];

            // Guard: stop at dedent or same indent (next key)
            if line.indent <= base_indent && !line.content.is_empty() {
                break;
            }

            // Guard: skip comment-only lines in multiline strings
            if line.content.is_empty() && line.leading_comment.is_some() {
                self.current_line_index += 1;
                continue;
            }

            // Guard: check minimum indentation for explicit indicators
            if explicit_indent.is_some() && line.indent < content_indent && !line.content.is_empty()
            {
                break;
            }

            // Add content (already stripped of its indentation by tokenizer)
            if !line.content.is_empty() {
                lines.push(line.content.clone());
            } else if !lines.is_empty() {
                // Empty line in the middle
                lines.push(String::new());
            }
            self.current_line_index += 1;
        }

        if is_literal {
            // Literal (|) - preserve line breaks
            let mut result = lines.join("\n");
            if !strip_trailing {
                result.push('\n');
            }
            Ok(result)
        } else {
            // Folded (>) - fold lines into single line
            let result = lines.join(" ");
            Ok(result.trim().to_string())
        }
    }

    fn parse_object(
        &mut self,
        base_indent: usize,
        chars: &[char],
        character_position: &mut usize,
    ) -> Result<YamlValue, YamlAbstractSyntaxTreeError> {
        let mut pairs = Vec::new();
        let _is_root_level = base_indent == 0;

        loop {
            // Tokenize next line if needed (parse_object may need multiple lines)
            if self.current_line_index >= self.lines.len() && *character_position < chars.len() {
                let (line, next_position) = utils::parse_single_line(chars, *character_position)?;
                *character_position = next_position;
                self.lines.push(line);
            }

            // Check if we have more lines to parse
            if self.current_line_index >= self.lines.len() {
                break;
            }

            let line = self.lines[self.current_line_index].clone();

            // Guard: dedent
            if line.indent < base_indent {
                break;
            }

            // Guard: empty lines - skip them
            if line.content.is_empty() {
                // At root level, empty lines break the object (for formatting preservation)
                if base_indent == 0 {
                    break;
                }
                self.current_line_index += 1;
                continue;
            }

            let content = line.content.trim();
            let inline_comment = line.inline_comment.clone();

            // Guard: parse key-value pair
            if let Some(colon_position) = content.find(':') {
                let key = content[..colon_position].trim().to_string();
                let value_string = content[colon_position + 1..].trim();

                self.current_line_index += 1;

                let value_node = if value_string.is_empty() {
                    // Value on next line(s) - assume 2-space indent
                    // Skip any leading Comment nodes and get the first non-comment node
                    let mut node = self.parse_node(base_indent + 2, chars, character_position)?;
                    while let Some(n) = &node {
                        if matches!(n.value, YamlValue::Comment(_))
                            || matches!(n.value, YamlValue::EmptyLine)
                        {
                            node = self.parse_node(base_indent + 2, chars, character_position)?;
                        } else {
                            break;
                        }
                    }
                    node.unwrap_or(YamlNode {
                        value: YamlValue::Null,
                        inline_comment: None,
                        leading_comment: None,
                    })
                } else if value_string.starts_with('|') || value_string.starts_with('>') {
                    // Multiline string indicator on same line as key (|, >, |2, >-3, etc.)
                    let (is_literal, strip_trailing, explicit_indent) =
                        utils::parse_multiline_indicator(value_string);

                    let multiline_value = self.parse_multiline_string(
                        base_indent,
                        is_literal,
                        strip_trailing,
                        explicit_indent,
                        chars,
                        character_position,
                    )?;
                    YamlNode {
                        value: YamlValue::String(multiline_value),
                        inline_comment,
                        leading_comment: None,
                    }
                } else {
                    // Inline value
                    YamlNode {
                        value: self.parse_scalar_value(value_string),
                        inline_comment,
                        leading_comment: None,
                    }
                };

                pairs.push((key, value_node));
            } else {
                break;
            }
        }

        Ok(YamlValue::Object(pairs))
    }

    fn parse_sequence(
        &mut self,
        base_indent: usize,
        chars: &[char],
        character_position: &mut usize,
    ) -> Result<YamlValue, YamlAbstractSyntaxTreeError> {
        let mut items = Vec::new();

        loop {
            // Tokenize next line if needed (parse_sequence may need multiple lines)
            if self.current_line_index >= self.lines.len() && *character_position < chars.len() {
                let (line, next_position) = utils::parse_single_line(chars, *character_position)?;
                *character_position = next_position;
                self.lines.push(line);
            }

            // Check if we have more lines to parse
            if self.current_line_index >= self.lines.len() {
                break;
            }

            let line = self.lines[self.current_line_index].clone();

            // Guard: dedent (but allow empty lines within sequence)
            if line.indent < base_indent && !line.content.is_empty() {
                break;
            }

            // Handle empty lines within sequence
            if line.content.is_empty() && line.leading_comment.is_none() {
                items.push(YamlNode {
                    value: YamlValue::EmptyLine,
                    inline_comment: None,
                    leading_comment: None,
                });
                self.current_line_index += 1;
                continue;
            }

            // Handle comment lines within sequence
            if line.content.is_empty() && line.leading_comment.is_some() {
                items.push(YamlNode {
                    value: YamlValue::Comment(line.leading_comment.unwrap()),
                    inline_comment: None,
                    leading_comment: None,
                });
                self.current_line_index += 1;
                continue;
            }

            let content = line.content.trim();
            let inline_comment = line.inline_comment.clone();

            // Guard: sequence item
            if !content.starts_with("- ") {
                break;
            }

            let value_string = content[2..].trim();
            self.current_line_index += 1;

            let value_node = if value_string.is_empty() {
                // Value on next line(s) - assume 2-space indent
                self.parse_node(base_indent + 2, chars, character_position)?
                    .unwrap_or(YamlNode {
                        value: YamlValue::Null,
                        inline_comment: None,
                        leading_comment: None,
                    })
            } else {
                YamlNode {
                    value: self.parse_scalar_value(value_string),
                    inline_comment,
                    leading_comment: None,
                }
            };

            items.push(value_node);
        }

        Ok(YamlValue::Collection(items))
    }

    fn parse_inline_array_from_line(
        &self,
        _content: &str,
    ) -> Result<YamlValue, YamlAbstractSyntaxTreeError> {
        // Simplified for now
        Ok(YamlValue::Collection(vec![]))
    }

    fn parse_inline_object_from_line(
        &self,
        _content: &str,
    ) -> Result<YamlValue, YamlAbstractSyntaxTreeError> {
        // Simplified for now
        Ok(YamlValue::Object(vec![]))
    }
}
