use crate::types::{Line, YamlAbstractSyntaxTreeError};

/// Parse a single line from the character array starting at the given position.
/// Returns the parsed Line and the new position after the line.
pub fn parse_single_line(
    chars: &[char],
    start_position: usize,
) -> Result<(Line, usize), YamlAbstractSyntaxTreeError> {
    let mut current_position = start_position;
    let mut line = Line {
        indent: 0,
        content: String::new(),
        leading_comment: None,
        inline_comment: None,
    };

    // Guard: EOF
    if current_position >= chars.len() {
        return Ok((line, current_position));
    }

    // Measure indentation
    while current_position < chars.len() {
        match chars[current_position] {
            ' ' => {
                line.indent += 1;
                current_position += 1;
            }
            '\t' => {
                line.indent += 4;
                current_position += 1;
            }
            _ => break,
        }
    }

    // Guard: empty line or newline
    if current_position >= chars.len()
        || chars[current_position] == '\n'
        || chars[current_position] == '\r'
    {
        current_position = skip_line_ending(chars, current_position);
        return Ok((line, current_position));
    }

    // Guard: comment-only line - preserve it
    if chars[current_position] == '#' {
        let comment_start = current_position + 1; // Skip the '#'
        current_position = skip_until_newline(chars, current_position);
        let comment_text: String = chars[comment_start..current_position].iter().collect();
        // Trim leading space after #
        line.leading_comment = Some(comment_text.trim_start().to_string());
        current_position = skip_line_ending(chars, current_position);
        return Ok((line, current_position));
    }

    // Collect content until comment or newline
    let mut is_inside_string = false;
    let mut string_delimiter = '\0';

    while current_position < chars.len() {
        let current_char = chars[current_position];

        // Guard: handle quoted strings
        if !is_inside_string && (current_char == '"' || current_char == '\'') {
            is_inside_string = true;
            string_delimiter = current_char;
            line.content.push(current_char);
            current_position += 1;
            continue;
        }

        if is_inside_string {
            line.content.push(current_char);
            current_position += 1;

            if current_char == string_delimiter {
                is_inside_string = false;
            }
            continue;
        }

        // Guard: inline comment (outside strings) - preserve it
        if current_char == '#' {
            let comment_start = current_position + 1; // Skip the '#'
            current_position = skip_until_newline(chars, current_position);
            let comment_text: String = chars[comment_start..current_position].iter().collect();
            // Trim leading space after #
            line.inline_comment = Some(comment_text.trim_start().to_string());
            break;
        }

        // Guard: end of line
        if current_char == '\n' || current_char == '\r' {
            break;
        }

        line.content.push(current_char);
        current_position += 1;
    }

    // Skip line ending
    current_position = skip_line_ending(chars, current_position);

    Ok((line, current_position))
}

/// Skip characters until a newline is found.
/// Returns the position of the newline (or EOF).
pub fn skip_until_newline(chars: &[char], start_position: usize) -> usize {
    let mut current_position = start_position;
    while current_position < chars.len() {
        if chars[current_position] == '\n' || chars[current_position] == '\r' {
            break;
        }
        current_position += 1;
    }
    current_position
}

/// Skip the line ending characters (\n, \r, or \r\n).
/// Returns the position after the line ending.
pub fn skip_line_ending(chars: &[char], current_position: usize) -> usize {
    let mut next_position = current_position;

    // Guard: EOF
    if next_position >= chars.len() {
        return next_position;
    }

    // Handle \r\n or \r or \n
    if chars[next_position] == '\r' {
        next_position += 1;
        if next_position < chars.len() && chars[next_position] == '\n' {
            next_position += 1;
        }
    } else if chars[next_position] == '\n' {
        next_position += 1;
    }

    next_position
}

/// Parse multiline string indicator like |, >, |2, >-3, etc.
/// Returns (is_literal, strip_trailing, explicit_indent)
pub fn parse_multiline_indicator(indicator: &str) -> (bool, bool, Option<usize>) {
    let is_literal = indicator.starts_with('|');
    let strip_trailing = indicator.contains('-');

    // Extract explicit indentation number (e.g., |2, >-3)
    let explicit_indent = indicator
        .chars()
        .skip(1) // Skip | or >
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .ok();

    (is_literal, strip_trailing, explicit_indent)
}
