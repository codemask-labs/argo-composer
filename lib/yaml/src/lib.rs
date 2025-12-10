use std::path::PathBuf;

mod deserializer;
mod serializer;

pub use deserializer::*;
pub use serializer::*;

// Re-export derive macros
pub use yaml_derive::{Deserialize, Serialize};

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
    pub fn from_path(path: PathBuf) -> Result<T, YamlError> {
        todo!()
    }

    pub fn serialize_to_string(target: T) -> Result<String, YamlError>
    where
        T: YamlSerializer,
    {
        todo!()
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

    #[derive(Deserialize, Serialize)]
    struct Example {
        pub hello: String,
        pub test: Option<String>,
    }

    #[test]
    pub fn serializing_to_string() {
        let object = Example {
            hello: String::from("Unit test example"),
            test: None,
        };

        let result = Yaml::serialize_to_string(object);

        // todo: check if `result` is what we need.

        println!("HELLO FROM TEST");
    }
}
