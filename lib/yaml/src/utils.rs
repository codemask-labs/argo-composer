use yaml_ast::{YamlNode, YamlValue};

/// Helper to find a field value in YAML object pairs
pub fn find_field_value<'a>(
    pairs: &'a [(String, YamlNode)],
    field_name: &str,
) -> Option<&'a YamlValue> {
    pairs
        .iter()
        .find(|(key, _)| key == field_name)
        .map(|(_, node)| &node.value)
}

/// Helper to extract Object pairs from nodes (skipping comments)
/// Merges all Object nodes into a single collection of pairs
pub fn extract_object_pairs(nodes: &[YamlNode]) -> Result<Vec<(String, YamlNode)>, String> {
    let mut all_pairs = Vec::new();

    for node in nodes {
        if let YamlValue::Object(pairs) = &node.value {
            all_pairs.extend(pairs.iter().cloned());
        }
    }

    if all_pairs.is_empty() {
        Err("No Object nodes found in YAML nodes".to_string())
    } else {
        Ok(all_pairs)
    }
}
