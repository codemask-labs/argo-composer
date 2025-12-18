use std::path::PathBuf;

pub mod deserializer;
pub mod serializer;

pub use deserializer::*;
pub use serializer::*;

// Re-export derive macros
pub use yaml_derive::{Deserialize, Serialize};

use yaml_ast::YamlDocument;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::deserializer::{
        FromYamlValue, YamlDeserializer, extract_object_pairs, find_field_value,
    };
    pub use crate::serializer::YamlSerializer;
    pub use crate::{Yaml, YamlError};
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
pub struct Yaml<T: Default> {
    inner: T,
}

impl<T> Yaml<T>
where
    T: Default,
{
    pub fn from_path(_path: PathBuf) -> Result<T, YamlError> {
        todo!()
    }

    pub fn from_string(yaml_string: &str) -> Result<T, YamlError>
    where
        T: YamlDeserializer,
    {
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
    T: Default,
{
    fn default() -> Self {
        Yaml {
            inner: T::default(),
        }
    }
}

impl<T> std::ops::Deref for Yaml<T>
where
    T: Default,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for Yaml<T>
where
    T: Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
