use yaml_ast::{YamlNode, YamlValue};

/// Trait for types that can be deserialized from YAML AST nodes
pub trait YamlDeserializer: Sized {
    /// Create an instance from YAML nodes
    fn from_yaml_nodes(nodes: &[YamlNode]) -> Result<Self, String>;
}

/// Helper to extract a value from a YAML node
pub trait FromYamlValue: Sized {
    fn from_yaml_value(value: &YamlValue) -> Result<Self, String>;
}

// Implement for String
impl FromYamlValue for String {
    fn from_yaml_value(value: &YamlValue) -> Result<Self, String> {
        match value {
            YamlValue::String(s) => Ok(s.clone()),
            YamlValue::Number(n) => Ok(n.to_string()),
            YamlValue::Boolean(b) => Ok(b.to_string()),
            YamlValue::Null => Ok(String::new()),
            _ => Err(format!("Cannot convert {:?} to String", value)),
        }
    }
}

// Implement for bool
impl FromYamlValue for bool {
    fn from_yaml_value(value: &YamlValue) -> Result<Self, String> {
        match value {
            YamlValue::Boolean(b) => Ok(*b),
            YamlValue::String(s) => match s.to_lowercase().as_str() {
                "true" | "yes" | "on" | "1" => Ok(true),
                "false" | "no" | "off" | "0" => Ok(false),
                _ => Err(format!("Cannot convert string '{}' to bool", s)),
            },
            _ => Err(format!("Cannot convert {:?} to bool", value)),
        }
    }
}

// Implement for i64
impl FromYamlValue for i64 {
    fn from_yaml_value(value: &YamlValue) -> Result<Self, String> {
        match value {
            YamlValue::Number(n) => Ok(*n as i64),
            YamlValue::String(s) => s
                .parse::<i64>()
                .map_err(|e| format!("Cannot convert string '{}' to i64: {}", s, e)),
            _ => Err(format!("Cannot convert {:?} to i64", value)),
        }
    }
}

// Implement YamlDeserializer for String
impl YamlDeserializer for String {
    fn from_yaml_nodes(nodes: &[YamlNode]) -> Result<Self, String> {
        for node in nodes {
            return Self::from_yaml_value(&node.value);
        }
        Err("No nodes to deserialize String from".to_string())
    }
}

// Implement for Option<T>
impl<T: FromYamlValue> FromYamlValue for Option<T> {
    fn from_yaml_value(value: &YamlValue) -> Result<Self, String> {
        match value {
            YamlValue::Null => Ok(None),
            _ => T::from_yaml_value(value).map(Some),
        }
    }
}

// Implement YamlDeserializer for Vec<T> where T implements YamlDeserializer
impl<T: YamlDeserializer> YamlDeserializer for Vec<T> {
    fn from_yaml_nodes(nodes: &[YamlNode]) -> Result<Self, String> {
        // Find the Collection node
        for node in nodes {
            if let YamlValue::Collection(items) = &node.value {
                // Deserialize each item in the collection
                let mut result = Vec::new();
                for item in items {
                    let deserialized = T::from_yaml_nodes(&[item.clone()])?;
                    result.push(deserialized);
                }
                return Ok(result);
            }
        }
        // If no collection found, return empty vec
        Ok(Vec::new())
    }
}

// Implement YamlDeserializer for BTreeMap<String, String>
impl YamlDeserializer for std::collections::BTreeMap<String, String> {
    fn from_yaml_nodes(nodes: &[YamlNode]) -> Result<Self, String> {
        let mut map = std::collections::BTreeMap::new();

        for node in nodes {
            if let YamlValue::Object(pairs) = &node.value {
                for (key, value_node) in pairs {
                    if let YamlValue::String(value) = &value_node.value {
                        map.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        Ok(map)
    }
}

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
