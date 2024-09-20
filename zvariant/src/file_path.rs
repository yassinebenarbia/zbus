use std::{ffi::{OsStr, OsString}, path::{Path, PathBuf}};

use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};
use crate::de::Deserializer as ZbusDeserializer;

use crate::Type;

/// A file name represented as a nul-terminated byte array.
#[derive(Type, Debug, Default, PartialEq)]
#[zvariant(signature = "ay")]
pub struct FilePath(OsString);

impl From<&Path> for FilePath {
    fn from(value: &Path) -> Self {
        Self(value.as_os_str().to_os_string())
    }
}

impl From<PathBuf> for FilePath {
    fn from(value: PathBuf) -> Self {
        Self(value.as_os_str().to_os_string())
    }
}

impl<'de> Deserialize<'de> for FilePath {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct FilePathVisitor;
        impl<'de> Visitor<'de> for FilePathVisitor {
            type Value = FilePath;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("unable to deserialize FilePath")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                unsafe {
                    Ok(FilePath(
                            OsStr::from_encoded_bytes_unchecked(v).to_os_string()
                    ))
                }
            }
        }
        let visitor = FilePathVisitor;
        deserializer.deserialize_bytes(visitor)
    }
}

impl Serialize for FilePath {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_bytes(&self.0.as_encoded_bytes())
    }
}


impl AsRef<FilePath> for FilePath {
    fn as_ref(&self) -> &FilePath {
        &self
    }
}

impl Into<PathBuf> for FilePath {
    fn into(self) -> PathBuf {
        PathBuf::from(self.0)
    }
}

#[cfg(test)]
mod file_path {
    use crate::zvariant::Signature;
    use super::*;

    #[test]
    fn filepath_from() {
        let path = Path::new("/hello/world");
        let _ = FilePath::from(path);
        let path_buf = PathBuf::from("/hello/world");
        let _ = FilePath::from(path_buf);
    }

    #[test]
    fn filepath_signature() {
        assert_eq!(
            &Signature::Array(zvariant_utils::signature::Child::Static {
                child: &Signature::U8
            }),
            FilePath::SIGNATURE
        );
    }

    #[test]
    fn into_test() {
        let first = PathBuf::from("/hello/world");
        let p = FilePath::from(first.clone());
        let second: PathBuf = p.into();
        assert_eq!(first, second);
    }
}
