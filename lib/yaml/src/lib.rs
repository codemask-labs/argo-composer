use std::path::PathBuf;

pub mod deserializer;
pub mod serializer;
pub mod utils;

pub use deserializer::*;
pub use serializer::*;
pub use utils::*;

// Re-export derive macros
pub use yaml_derive::{Deserialize, Serialize};

use yaml_ast::YamlDocument;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::deserializer::YamlDeserializer;
    pub use crate::serializer::YamlSerializer;
    pub use crate::utils::{extract_object_pairs, find_field_value};
    pub use crate::{
        Yaml, YamlError, deserialize_document, get_discriminator_field, parse_documents_from_path,
        parse_documents_from_string,
    };
    pub use yaml_derive::{Deserialize, Serialize};
}

#[derive(Debug)]
pub enum YamlError {
    FailedToOpenFile,
    FailedToReadFile,
    FailedToParseYaml,
    FailedToDeserialize,
    FailedToSerialize,
    FailedToWriteFile,
    FailedToUpdateComments,
}

#[derive(Debug)]
pub struct Yaml<T: Default + YamlDeserializer + YamlSerializer> {
    inner: T,
}

impl<T> Yaml<T>
where
    T: Default + YamlDeserializer + YamlSerializer,
{
    pub fn from_path(yaml_path: PathBuf) -> Result<T, YamlError> {
        // Parse the YAML string to AST
        let documents = yaml_ast::YamlAbstractSyntaxTree::from_path(yaml_path).map_err(|e| {
            eprintln!("Failed to parse YAML: {:?}", e);
            YamlError::FailedToParseYaml
        })?;

        // Get the first document
        let document = documents.first().ok_or(YamlError::FailedToDeserialize)?;

        // Deserialize from nodes
        T::from_yaml_nodes(&document.nodes).map_err(|e| {
            eprintln!("Failed to deserialize: {}", e);
            YamlError::FailedToDeserialize
        })
    }

    pub fn from_string(yaml_string: &str) -> Result<T, YamlError> {
        // Parse the YAML string to AST
        let documents = yaml_ast::YamlAbstractSyntaxTree::parse_string(yaml_string.to_string())
            .map_err(|e| {
                eprintln!("Failed to parse YAML: {:?}", e);
                YamlError::FailedToParseYaml
            })?;

        // Get the first document
        let document = documents.first().ok_or(YamlError::FailedToDeserialize)?;

        // Deserialize from nodes
        T::from_yaml_nodes(&document.nodes).map_err(|e| {
            eprintln!("Failed to deserialize: {}", e);
            YamlError::FailedToDeserialize
        })
    }

    pub fn serialize_to_string(target: T) -> Result<String, YamlError>
    where
        T: YamlSerializer,
    {
        // Convert the value to YAML nodes
        let nodes = target.to_yaml_nodes();

        // Create a YAML document
        let document = YamlDocument { nodes };

        // Format the document to a string
        let formatter = YamlFormatter::new();
        let output = formatter.format_documents(&[document]);

        Ok(output)
    }
}

impl<T> Default for Yaml<T>
where
    T: Default + YamlDeserializer + YamlSerializer,
{
    fn default() -> Self {
        Yaml {
            inner: T::default(),
        }
    }
}

impl<T> std::ops::Deref for Yaml<T>
where
    T: Default + YamlDeserializer + YamlSerializer,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for Yaml<T>
where
    T: Default + YamlDeserializer + YamlSerializer,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Parse all documents from a YAML file and return them as raw AST documents
pub fn parse_documents_from_path(yaml_path: PathBuf) -> Result<Vec<YamlDocument>, YamlError> {
    yaml_ast::YamlAbstractSyntaxTree::from_path(yaml_path).map_err(|e| {
        eprintln!("Failed to parse YAML: {:?}", e);
        YamlError::FailedToParseYaml
    })
}

/// Parse all documents from a YAML string and return them as raw AST documents
pub fn parse_documents_from_string(yaml_string: &str) -> Result<Vec<YamlDocument>, YamlError> {
    yaml_ast::YamlAbstractSyntaxTree::parse_string(yaml_string.to_string()).map_err(|e| {
        eprintln!("Failed to parse YAML: {:?}", e);
        YamlError::FailedToParseYaml
    })
}

/// Deserialize a single YAML document to a specific type
pub fn deserialize_document<T: YamlDeserializer>(document: &YamlDocument) -> Result<T, YamlError> {
    T::from_yaml_nodes(&document.nodes).map_err(|e| {
        eprintln!("Failed to deserialize: {}", e);
        YamlError::FailedToDeserialize
    })
}

/// Get the value of a discriminator field (e.g., "kind") from a YAML document
/// This is useful for determining the type of a document before deserializing
pub fn get_discriminator_field(document: &YamlDocument, field_name: &str) -> Option<String> {
    use yaml_ast::YamlValue;

    for node in &document.nodes {
        if let YamlValue::Object(pairs) = &node.value {
            for (key, value_node) in pairs {
                if key == field_name {
                    if let YamlValue::String(value) = &value_node.value {
                        return Some(value.clone());
                    }
                }
            }
        }
    }
    None
}
