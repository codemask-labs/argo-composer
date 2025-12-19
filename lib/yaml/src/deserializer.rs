use yaml_ast::{YamlNode, YamlValue};

/// Trait for types that can be deserialized from YAML AST nodes and values
pub trait YamlDeserializer: Sized {
    /// Create an instance from YAML nodes
    fn from_yaml_nodes(nodes: &[YamlNode]) -> Result<Self, String>;

    /// Create an instance from a YAML value (for primitive types)
    /// Complex types like Vec and BTreeMap should not be allowed to return a Self but error instead
    fn from_yaml_value(value: &YamlValue) -> Result<Self, String>;
}

// Implement for String
impl YamlDeserializer for String {
    fn from_yaml_nodes(nodes: &[YamlNode]) -> Result<Self, String> {
        for node in nodes {
            return Self::from_yaml_value(&node.value);
        }
        Err("No nodes to deserialize String from".to_string())
    }

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
impl YamlDeserializer for bool {
    fn from_yaml_nodes(nodes: &[YamlNode]) -> Result<Self, String> {
        for node in nodes {
            return Self::from_yaml_value(&node.value);
        }
        Err("No nodes to deserialize bool from".to_string())
    }

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
impl YamlDeserializer for i64 {
    fn from_yaml_nodes(nodes: &[YamlNode]) -> Result<Self, String> {
        for node in nodes {
            return Self::from_yaml_value(&node.value);
        }
        Err("No nodes to deserialize i64 from".to_string())
    }

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

// Implement for Option<T>
impl<T: YamlDeserializer> YamlDeserializer for Option<T> {
    fn from_yaml_nodes(nodes: &[YamlNode]) -> Result<Self, String> {
        for node in nodes {
            return Self::from_yaml_value(&node.value);
        }
        Ok(None)
    }

    fn from_yaml_value(value: &YamlValue) -> Result<Self, String> {
        match value {
            YamlValue::Null => Ok(None),
            // For complex types (objects), we need to wrap in a node and use from_yaml_nodes
            YamlValue::Object(_) | YamlValue::Collection(_) => {
                let node = YamlNode {
                    value: value.clone(),
                    inline_comment: None,
                    leading_comment: None,
                };
                T::from_yaml_nodes(&[node]).map(Some)
            }
            // For primitive types, try from_yaml_value
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

    fn from_yaml_value(value: &YamlValue) -> Result<Self, String> {
        Err(format!(
            "Cannot deserialize Vec from scalar value {:?}. Expected a Collection node.",
            value
        ))
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

    fn from_yaml_value(value: &YamlValue) -> Result<Self, String> {
        Err(format!(
            "Cannot deserialize BTreeMap from scalar value {:?}. Expected an Object node.",
            value
        ))
    }
}
