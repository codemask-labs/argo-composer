pub struct Yaml<T> {
    inner: T,
}

// #![allow(dead_code)]

// use std::{
//     fs::{File, create_dir_all},
//     io::Read,
//     path::PathBuf,
// };

// use rust_yaml::{CommentedValue, Value};
// use serde::{Serialize, de::DeserializeOwned};

// #[derive(Debug)]
// pub enum YamlError {
//     FailedToOpenFile,
//     FailedToReadFile,
//     FailedToParseYaml,
//     FailedToDeserialize,
//     FailedToSerialize,
//     FailedToWriteFile,
//     FailedToUpdateComments,
// }

// #[derive(Debug, Clone)]
// pub struct Yaml<T> {
//     inner: T,
//     with_comments: CommentedValue,
// }

// impl<T> Yaml<T>
// where
//     T: DeserializeOwned + Serialize,
// {
//     pub fn from_str(contents: &str) -> Result<Yaml<T>, YamlError> {
//         let yaml_config = rust_yaml::YamlConfig {
//             preserve_comments: true,
//             loader_type: rust_yaml::LoaderType::RoundTrip,
//             ..Default::default()
//         };

//         let yaml = rust_yaml::Yaml::with_config(yaml_config);
//         let with_comments = match yaml.load_str_with_comments(contents) {
//             Ok(result) => result,
//             Err(error) => {
//                 eprintln!("{error}");

//                 return Err(YamlError::FailedToParseYaml);
//             }
//         };

//         let inner = match serde_yaml_ng::from_str::<T>(contents) {
//             Ok(result) => result,
//             Err(error) => {
//                 eprintln!("{error}");

//                 return Err(YamlError::FailedToDeserialize);
//             }
//         };

//         Ok(Yaml {
//             inner,
//             with_comments,
//         })
//     }

//     pub fn from_path(path: PathBuf) -> Result<Yaml<T>, YamlError> {
//         let mut file = match File::open(path) {
//             Ok(result) => result,
//             Err(error) => {
//                 eprintln!("{error}");

//                 return Err(YamlError::FailedToOpenFile);
//             }
//         };

//         let mut contents = String::new();

//         if let Err(error) = file.read_to_string(&mut contents) {
//             eprintln!("{error}");

//             return Err(YamlError::FailedToReadFile);
//         }

//         Self::from_str(&contents)
//     }

//     pub fn from_string(contents: String) -> Result<Yaml<T>, YamlError> {
//         Self::from_str(&contents)
//     }

//     pub fn serialize_to_string(&self) -> Result<String, YamlError> {
//         let value = serde_yaml_ng::to_value(&self.inner).map_err(|error| {
//             eprintln!("{error}");

//             YamlError::FailedToSerialize
//         })?;

//         let mut result = self.with_comments.clone();

//         result.value = Self::to_value(&value)?;

//         let yaml_config = rust_yaml::YamlConfig {
//             preserve_comments: true,
//             loader_type: rust_yaml::LoaderType::RoundTrip,
//             ..Default::default()
//         };

//         rust_yaml::Yaml::with_config(yaml_config)
//             .dump_str_with_comments(&result)
//             .map_err(|error| {
//                 eprintln!("{error}");

//                 YamlError::FailedToDeserialize
//             })
//     }

//     pub fn serialize_to_path(&self, path: PathBuf) -> Result<(), YamlError> {
//         // Create parent directories if they don't exist
//         if let Some(parent) = path.parent() {
//             if let Err(error) = create_dir_all(parent) {
//                 eprintln!("{error}");

//                 return Err(YamlError::FailedToWriteFile);
//             }
//         }

//         let yaml_string = self.serialize_to_string()?;

//         if let Err(error) = std::fs::write(&path, yaml_string) {
//             eprintln!("{error}");

//             return Err(YamlError::FailedToWriteFile);
//         }

//         Ok(())
//     }

//     fn to_value(value: &serde_yaml_ng::Value) -> Result<Value, YamlError> {
//         match value {
//             serde_yaml_ng::Value::Null => Ok(Value::Null),
//             serde_yaml_ng::Value::Bool(value) => Ok(Value::Bool(*value)),
//             serde_yaml_ng::Value::Number(value) => {
//                 if let Some(i) = value.as_i64() {
//                     Ok(Value::from(i))
//                 } else if let Some(u) = value.as_u64() {
//                     Ok(Value::from(u as i64))
//                 } else if let Some(f) = value.as_f64() {
//                     Ok(Value::from(f))
//                 } else {
//                     Err(YamlError::FailedToUpdateComments)
//                 }
//             }
//             serde_yaml_ng::Value::String(value) => Ok(Value::String(value.clone())),
//             serde_yaml_ng::Value::Sequence(value) => Ok(Value::Sequence(
//                 value
//                     .iter()
//                     .map(Self::to_value)
//                     .collect::<Result<_, YamlError>>()?,
//             )),
//             serde_yaml_ng::Value::Mapping(value) => Ok(Value::Mapping(
//                 value
//                     .iter()
//                     .map(|(k, v)| Ok((Self::to_value(k)?, Self::to_value(v)?)))
//                     .collect::<Result<_, YamlError>>()?,
//             )),
//             serde_yaml_ng::Value::Tagged(tagged) => Self::to_value(&tagged.value),
//         }
//     }
// }

// impl<T> std::ops::Deref for Yaml<T>
// where
//     T: DeserializeOwned + Serialize,
// {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// impl<T> std::ops::DerefMut for Yaml<T>
// where
//     T: DeserializeOwned + Serialize,
// {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }

// impl<T> Default for Yaml<T>
// where
//     T: DeserializeOwned + Serialize + Default,
// {
//     fn default() -> Self {
//         Yaml {
//             inner: T::default(),
//             with_comments: CommentedValue::new(Value::mapping()),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde::{Deserialize, Serialize};

//     #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
//     struct TestConfig {
//         name: String,
//         version: String,
//         #[serde(default)]
//         features: Vec<String>,
//     }

//     #[test]
//     fn test_from_str_basic() {
//         let yaml_content = r#"
// name: test-app
// version: 1.0.0
// features:
//   - feature1
//   - feature2
// "#;

//         let result: Result<Yaml<TestConfig>, _> = Yaml::from_str(yaml_content);
//         assert!(result.is_ok());

//         let yaml = result.unwrap();
//         assert_eq!(yaml.name, "test-app");
//         assert_eq!(yaml.version, "1.0.0");
//         assert_eq!(yaml.features, vec!["feature1", "feature2"]);
//     }

//     #[test]
//     fn test_from_string() {
//         let yaml_content = String::from(
//             r#"
// name: my-service
// version: 2.0.0
// features: []
// "#,
//         );

//         let result: Result<Yaml<TestConfig>, _> = Yaml::from_string(yaml_content);
//         assert!(result.is_ok());

//         let yaml = result.unwrap();
//         assert_eq!(yaml.name, "my-service");
//         assert_eq!(yaml.version, "2.0.0");
//         assert_eq!(yaml.features.len(), 0);
//     }

//     #[test]
//     fn test_to_string_basic() {
//         let yaml: Yaml<TestConfig> = Yaml::from_str(
//             r#"
// name: test-app
// version: 1.0.0
// features:
//   - feature1
// "#,
//         )
//         .unwrap();

//         let result = yaml.serialize_to_string();
//         assert!(result.is_ok());

//         let output = result.unwrap();
//         assert!(output.contains("name"));
//         assert!(output.contains("test-app"));
//         assert!(output.contains("version"));
//         assert!(output.contains("1.0.0"));
//     }

//     #[test]
//     fn test_comments_preservation() {
//         let yaml_with_comments = r#"# This is a header comment
// name: my-app  # inline comment
// # Comment above version
// version: 1.0.0
// features:
//   # First feature
//   - feature1
//   - feature2  # Second feature
// "#;

//         let result: Result<Yaml<TestConfig>, _> = Yaml::from_str(yaml_with_comments);
//         assert!(result.is_ok());

//         let yaml = result.unwrap();

//         // Verify data is correctly deserialized
//         assert_eq!(yaml.name, "my-app");
//         assert_eq!(yaml.version, "1.0.0");
//         assert_eq!(yaml.features, vec!["feature1", "feature2"]);

//         // Serialize back
//         let serialized = yaml.serialize_to_string().unwrap();

//         println!("{serialized}");

//         // Verify the data is correctly serialized
//         assert!(serialized.contains("my-app"));
//         assert!(serialized.contains("1.0.0"));
//         assert!(serialized.contains("feature1"));
//         assert!(serialized.contains("feature2"));
//     }

//     #[test]
//     fn test_read_only_comment_preservation() {
//         let yaml_with_comments = r#"# Header comment
// name: test-app
// version: 1.0.0
// features: []
// "#;

//         let yaml: Yaml<TestConfig> = Yaml::from_str(yaml_with_comments).unwrap();
//         let serialized = yaml.serialize_to_string().unwrap();

//         println!("Serialized (read-only):\n{}", serialized);

//         assert!(serialized.contains("name"));
//         assert!(serialized.contains("test-app"));
//         assert!(serialized.contains("version"));
//         assert!(serialized.contains("1.0.0"));
//         assert!(serialized.eq(yaml_with_comments));
//     }

//     #[test]
//     fn test_comment_preservation_without_value_changes() {
//         let yaml_str = r#"# Top level comment
// # Another comment
// name: my-service
// # Version comment
// version: 2.0.0
// features:
//   # Feature 1
//   - auth
//   # Feature 2
//   - logging
// "#;

//         let yaml: Yaml<TestConfig> = Yaml::from_str(yaml_str).unwrap();

//         assert_eq!(yaml.name, "my-service");
//         assert_eq!(yaml.version, "2.0.0");
//         assert_eq!(yaml.features, vec!["auth", "logging"]);

//         let serialized = yaml.serialize_to_string().unwrap();

//         println!("Original:\n{}", yaml_str);
//         println!("Serialized:\n{}", serialized);

//         assert!(serialized.contains("my-service"));
//         assert!(serialized.contains("2.0.0"));
//         assert!(serialized.contains("auth"));
//         assert!(serialized.contains("logging"));
//     }
//     #[test]
//     fn test_deref_access() {
//         let yaml: Yaml<TestConfig> = Yaml::from_str(
//             r#"
// name: deref-test
// version: 3.0.0
// features:
//   - test
// "#,
//         )
//         .unwrap();

//         // Test that we can access fields directly via Deref
//         assert_eq!(yaml.name, "deref-test");
//         assert_eq!(yaml.version, "3.0.0");
//         assert_eq!(yaml.features[0], "test");
//     }

//     #[test]
//     fn test_deref_mut_modification() {
//         let mut yaml: Yaml<TestConfig> = Yaml::from_str(
//             r#"
// name: original
// version: 1.0.0
// features: []
// "#,
//         )
//         .unwrap();

//         // Modify via DerefMut
//         yaml.name = "modified".to_string();
//         yaml.version = "2.0.0".to_string();
//         yaml.features.push("new-feature".to_string());

//         assert_eq!(yaml.name, "modified");
//         assert_eq!(yaml.version, "2.0.0");
//         assert_eq!(yaml.features, vec!["new-feature"]);

//         // Verify serialization includes modifications
//         let serialized = yaml.serialize_to_string().unwrap();
//         assert!(serialized.contains("modified"));
//         assert!(serialized.contains("2.0.0"));
//         assert!(serialized.contains("new-feature"));
//     }

//     #[test]
//     fn test_roundtrip_with_comments() {
//         let original = r#"# Application configuration
// name: roundtrip-test
// # Version information
// version: 1.0.0
// features:
//   - alpha
//   - beta
// "#;

//         // Parse
//         let mut yaml: Yaml<TestConfig> = Yaml::from_str(original).unwrap();

//         // Modify
//         yaml.version = "1.1.0".to_string();
//         yaml.features.push("gamma".to_string());

//         // Serialize
//         let serialized = yaml.serialize_to_string().unwrap();

//         assert!(serialized.contains("1.1.0"));
//         assert!(serialized.contains("gamma"));
//         assert!(serialized.contains("roundtrip-test"));
//         assert!(serialized.contains("alpha"));
//         assert!(serialized.contains("beta"));
//     }

//     #[test]
//     fn test_invalid_yaml() {
//         let invalid_yaml = r#"
// name: test
// version: [invalid
// "#;

//         let result: Result<Yaml<TestConfig>, _> = Yaml::from_str(invalid_yaml);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_deserialization_error() {
//         let yaml_missing_field = r#"
// name: test
// # version field is missing and required
// "#;

//         let result: Result<Yaml<TestConfig>, _> = Yaml::from_str(yaml_missing_field);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_default_impl() {
//         let yaml: Yaml<TestConfig> = Yaml::default();

//         assert_eq!(yaml.name, "");
//         assert_eq!(yaml.version, "");
//         assert_eq!(yaml.features.len(), 0);
//     }

//     impl Default for TestConfig {
//         fn default() -> Self {
//             TestConfig {
//                 name: String::new(),
//                 version: String::new(),
//                 features: Vec::new(),
//             }
//         }
//     }
// }
