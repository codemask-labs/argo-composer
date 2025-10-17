use std::{fs::File, io::Read, path::PathBuf};

use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum YamlError {
    FailedToOpenFile,
    FailedToReadFile,
    FailedToParseYaml,
    FailedToDeserialize,
}

pub struct Yaml<T>
where
    T: DeserializeOwned + Serialize,
{
    inner: T,
}

impl<T> Yaml<T>
where
    T: DeserializeOwned + Serialize,
{
    pub fn from_path(path: PathBuf) -> Result<Yaml<T>, YamlError> {
        let mut file = match File::open(path) {
            Ok(result) => result,
            Err(_) => {
                return Err(YamlError::FailedToOpenFile);
            }
        };

        let mut contents = String::new();

        if let Err(_) = file.read_to_string(&mut contents) {
            return Err(YamlError::FailedToReadFile);
        }

        // Parse YAML with comments preservation for potential future use
        let _yaml = match rust_yaml::Yaml::new().load_str_with_comments(&contents) {
            Ok(result) => result,
            Err(_) => {
                return Err(YamlError::FailedToParseYaml);
            }
        };

        // Deserialize the YAML content directly into type T using serde_yaml_ng
        let inner = match serde_yaml_ng::from_str::<T>(&contents) {
            Ok(result) => result,
            Err(error) => {
                println!("error :: {:?}", error);
                return Err(YamlError::FailedToDeserialize);
            }
        };

        Ok(Yaml { inner })
    }
}

// Implement Deref to allow direct field access like `config.field_name`
impl<T> std::ops::Deref for Yaml<T>
where
    T: DeserializeOwned + Serialize,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Default for Yaml<T>
where
    T: DeserializeOwned + Serialize + Default,
{
    fn default() -> Self {
        Self {
            inner: T::default(),
        }
    }
}
