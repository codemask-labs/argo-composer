use yaml::prelude::*;

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

#[test]
pub fn test_struct_level_comments() {
    // Test that struct-level doc comments appear at the beginning of serialized output
    let object = Example {
        hello: String::from("Test"),
        test: None,
    };

    let result = Yaml::serialize_to_string(object).expect("Failed to serialize");

    println!("\n=== STRUCT LEVEL COMMENTS TEST ===\n{}\n", result);

    // The struct doesn't have doc comments, but fields should have them
    assert!(result.contains("# The greeting message"));
}

/// Struct with doc comments for testing preservation
#[derive(Deserialize, Serialize, Default)]
struct DocumentedConfig {
    /// The service name
    pub name: String,

    /// The service version
    pub version: String,
}

#[test]
pub fn test_document_comments_vs_struct_comments() {
    // This test demonstrates the current behavior when deserializing a YAML document
    // that has its own comments, then re-serializing it with a struct that has doc comments.

    // Original YAML with document-level comments
    let yaml_with_comments = r#"
# This is the original document comment
# It describes the configuration file
# Multiple lines of important context

# Service name configuration
name: my-service

# Version information
version: 1.0.0
"#;

    // Deserialize the YAML (comments are lost during deserialization)
    let config: DocumentedConfig =
        Yaml::from_string(yaml_with_comments).expect("Failed to deserialize");

    assert_eq!(config.name, "my-service");
    assert_eq!(config.version, "1.0.0");

    // Re-serialize - this will generate new YAML with struct-level doc comments
    // NOTE: The original document comments are NOT preserved because:
    // 1. Deserialization only extracts field values, not comments
    // 2. Serialization generates fresh YAML from struct definition
    let re_serialized = Yaml::serialize_to_string(config).expect("Failed to serialize");

    println!("\n=== ORIGINAL YAML ===\n{}\n", yaml_with_comments);
    println!("=== RE-SERIALIZED YAML ===\n{}\n", re_serialized);

    // Verify that struct-level doc comments appear (if the struct had any at the top)
    // DocumentedConfig doesn't have struct-level comments, only field comments
    assert!(re_serialized.contains("# The service name"));
    assert!(re_serialized.contains("# The service version"));

    // Original document comments are NOT in the re-serialized output
    assert!(!re_serialized.contains("This is the original document comment"));
    assert!(!re_serialized.contains("It describes the configuration file"));

    // This is expected behavior - struct doc comments define the canonical documentation
    // Original document comments are considered transient parsing artifacts
}

/// Struct with both struct-level and field-level doc comments
/// This demonstrates how struct comments appear in serialization
#[derive(Deserialize, Serialize, Default)]
struct FullyDocumentedConfig {
    /// Application name
    pub app_name: String,

    /// Application port
    pub port: String,
}

#[test]
pub fn test_struct_level_comments_with_documented_struct() {
    // Test that struct-level doc comments appear at the beginning of serialized output
    let config = FullyDocumentedConfig {
        app_name: "test-app".to_string(),
        port: "8080".to_string(),
    };

    let result = Yaml::serialize_to_string(config).expect("Failed to serialize");

    println!(
        "\n=== FULLY DOCUMENTED STRUCT SERIALIZATION ===\n{}\n",
        result
    );

    // Verify struct-level comments appear
    assert!(result.contains("# Struct with both struct-level and field-level doc comments"));
    assert!(result.contains("# This demonstrates how struct comments appear in serialization"));

    // Verify field-level comments also appear
    assert!(result.contains("# Application name"));
    assert!(result.contains("# Application port"));

    // Verify the actual values
    assert!(result.contains("app_name: test-app"));
    assert!(result.contains("port: 8080"));
}

#[test]
pub fn test_round_trip_comment_behavior() {
    // This test explicitly documents the round-trip behavior:
    // - Parse YAML with custom comments
    // - Deserialize to struct
    // - Re-serialize to YAML
    // - Result: Struct doc comments appear, original comments don't

    let original_yaml = r#"
# WARNING: This is a production configuration
# Do not modify without approval

name: production-service
version: 2.5.3
"#;

    // Deserialize
    let config: DocumentedConfig = Yaml::from_string(original_yaml).expect("Failed to deserialize");

    // Modify a field
    let mut config = config;
    config.version = "2.5.4".to_string();

    // Re-serialize
    let updated_yaml = Yaml::serialize_to_string(config).expect("Failed to serialize");

    println!("\n=== ROUND TRIP TEST ===");
    println!("ORIGINAL:\n{}", original_yaml);
    println!("UPDATED:\n{}", updated_yaml);

    // The updated YAML will have:
    // - Struct doc comments (if any at struct level)
    // - Field doc comments
    // - Updated values
    // But NOT the original document comments
    assert!(updated_yaml.contains("# The service name"));
    assert!(updated_yaml.contains("# The service version"));
    assert!(updated_yaml.contains("version: 2.5.4"));
    assert!(!updated_yaml.contains("WARNING: This is a production configuration"));
}
