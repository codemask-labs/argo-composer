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

#[cfg(test)]
mod unit_test {
    use crate::*;

    #[derive(Deserialize, Serialize, Default)]
    struct Example {
        /// The greeting message
        pub hello: String,
        /// An optional test value
        pub test: Option<String>,
    }

    #[test]
    pub fn serializing_to_string() {
        let object = Example {
            hello: String::from("Unit test example"),
            test: None,
        };

        let result = Yaml::serialize_to_string(object).expect("Failed to serialize");

        println!("Serialized YAML:\n{}", result);

        // Check that the output contains our field
        assert!(result.contains("hello: Unit test example"));
        // Check that optional field with None is not included
        assert!(!result.contains("test:"));
        // Check that doc comment is included
        assert!(result.contains("# The greeting message"));
    }

    #[test]
    pub fn serializing_with_optional_value() {
        let object = Example {
            hello: String::from("Hello World"),
            test: Some(String::from("Optional value")),
        };

        let result = Yaml::serialize_to_string(object).expect("Failed to serialize");

        println!("Serialized YAML with optional:\n{}", result);

        assert!(result.contains("hello: Hello World"));
        assert!(result.contains("test: Optional value"));
        assert!(result.contains("# The greeting message"));
        assert!(result.contains("# An optional test value"));
    }

    #[derive(Deserialize, Serialize, Default)]
    struct ComplexExample {
        /// This is the application name
        /// It should be unique across your cluster
        pub name: String,

        /// The namespace where the app will be deployed
        pub namespace: String,

        /// Enable high availability mode
        /// This will create multiple replicas
        pub ha_enabled: Option<String>,
    }

    #[test]
    pub fn serializing_with_multiline_comments() {
        let object = ComplexExample {
            name: String::from("my-app"),
            namespace: String::from("production"),
            ha_enabled: Some(String::from("true")),
        };

        let result = Yaml::serialize_to_string(object).expect("Failed to serialize");

        println!("Serialized YAML with multi-line comments:\n{}", result);

        assert!(result.contains("# This is the application name"));
        assert!(result.contains("# It should be unique across your cluster"));
        assert!(result.contains("# Enable high availability mode"));
        assert!(result.contains("# This will create multiple replicas"));
        assert!(result.contains("name: my-app"));
        assert!(result.contains("namespace: production"));
        assert!(result.contains("ha_enabled:"));
    }

    #[derive(Deserialize, Serialize, Default)]
    struct ComprehensiveExample {
        /// Application configuration
        pub app_name: String,

        /// Port number for the service
        pub port: String,

        /// Optional description field
        pub description: Option<String>,

        /// Environment (dev, staging, prod)
        pub environment: String,

        /// Optional feature flag
        pub experimental_feature: Option<String>,
    }

    #[test]
    pub fn comprehensive_serialization_test() {
        let object = ComprehensiveExample {
            app_name: String::from("argo-composer"),
            port: String::from("8080"),
            description: Some(String::from("A tool for composing Argo CD applications")),
            environment: String::from("production"),
            experimental_feature: None,
        };

        let result = Yaml::serialize_to_string(object).expect("Failed to serialize");

        println!("\n=== COMPREHENSIVE SERIALIZATION TEST ===\n{}\n", result);

        // Verify required fields are present with comments
        assert!(result.contains("# Application configuration"));
        assert!(result.contains("app_name: argo-composer"));

        assert!(result.contains("# Port number for the service"));
        assert!(result.contains("port: 8080"));

        // Verify optional field with Some value has comment
        assert!(result.contains("# Optional description field"));
        assert!(result.contains("description: A tool for composing Argo CD applications"));

        assert!(result.contains("# Environment (dev, staging, prod)"));
        assert!(result.contains("environment: production"));

        // Verify optional field with None does NOT have comment or field
        assert!(!result.contains("# Optional feature flag"));
        assert!(!result.contains("experimental_feature:"));
    }

    #[derive(Deserialize, Serialize, Default)]
    struct DatabaseConfig {
        /// Database host
        pub host: String,

        /// Database port
        pub port: String,

        /// Database name
        pub database: String,
    }

    #[derive(Deserialize, Serialize, Default)]
    struct ServerConfig {
        /// Server name
        pub name: String,

        /// Database configuration
        pub db_config: DatabaseConfig,

        /// Optional backup database
        pub backup_db: Option<DatabaseConfig>,
    }

    #[test]
    pub fn test_nested_object_serialization() {
        let object = ServerConfig {
            name: String::from("api-server"),
            db_config: DatabaseConfig {
                host: String::from("localhost"),
                port: String::from("5432"),
                database: String::from("production"),
            },
            backup_db: Some(DatabaseConfig {
                host: String::from("backup-host"),
                port: String::from("5432"),
                database: String::from("backup"),
            }),
        };

        let result = Yaml::serialize_to_string(object).expect("Failed to serialize");

        println!("\n=== NESTED OBJECT SERIALIZATION TEST ===\n{}\n", result);

        // Verify basic structure
        assert!(result.contains("name: api-server"));
        assert!(result.contains("db_config:"));
        assert!(result.contains("backup_db:"));

        // Verify nested db_config fields
        assert!(result.contains("host: localhost"));
        assert!(result.contains("port: 5432"));
        assert!(result.contains("database: production"));

        // Verify nested backup_db fields
        assert!(result.contains("host: backup-host"));
        assert!(result.contains("database: backup"));
    }

    #[test]
    pub fn test_deserialization_simple() {
        let yaml_str = r#"
# This is a comment in the YAML
hello: World
# Another comment
test: Some value
"#;

        let result: Example = Yaml::from_string(yaml_str).expect("Failed to deserialize");

        assert_eq!(result.hello, "World");
        assert_eq!(result.test, Some("Some value".to_string()));
    }

    #[test]
    pub fn test_deserialization_with_missing_optional() {
        let yaml_str = r#"
hello: Just hello
"#;

        let result: Example = Yaml::from_string(yaml_str).expect("Failed to deserialize");

        assert_eq!(result.hello, "Just hello");
        assert_eq!(result.test, None);
    }

    #[test]
    pub fn test_round_trip_serialization() {
        let original = Example {
            hello: String::from("Round trip test"),
            test: Some(String::from("Optional data")),
        };

        // Serialize
        let yaml_string = Yaml::serialize_to_string(original).expect("Failed to serialize");
        println!("\nSerialized:\n{}", yaml_string);

        // Deserialize
        let deserialized: Example = Yaml::from_string(&yaml_string).expect("Failed to deserialize");

        // Verify
        assert_eq!(deserialized.hello, "Round trip test");
        assert_eq!(deserialized.test, Some("Optional data".to_string()));
    }

    #[test]
    pub fn test_nested_deserialization() {
        let yaml_str = r#"
name: test-server
db_config:
  host: db.example.com
  port: 3306
  database: mydb
backup_db:
  host: backup.example.com
  port: 3306
  database: backup_db
"#;

        let result: ServerConfig = Yaml::from_string(yaml_str).expect("Failed to deserialize");

        assert_eq!(result.name, "test-server");
        assert_eq!(result.db_config.host, "db.example.com");
        assert_eq!(result.db_config.port, "3306");
        assert_eq!(result.db_config.database, "mydb");

        let backup = result.backup_db.expect("backup_db should be Some");
        assert_eq!(backup.host, "backup.example.com");
        assert_eq!(backup.database, "backup_db");
    }
}
