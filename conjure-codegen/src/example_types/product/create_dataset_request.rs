use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateDatasetRequest {
    file_system_id: String,
    path: String,
}
impl CreateDatasetRequest {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T, U>(file_system_id: T, path: U) -> CreateDatasetRequest
    where
        T: Into<String>,
        U: Into<String>,
    {
        CreateDatasetRequest {
            file_system_id: file_system_id.into(),
            path: path.into(),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn file_system_id(&self) -> &str {
        &*self.file_system_id
    }
    #[inline]
    pub fn path(&self) -> &str {
        &*self.path
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<CreateDatasetRequest> for BuilderStage2 {
    #[inline]
    fn from(value: CreateDatasetRequest) -> Self {
        BuilderStage2 {
            file_system_id: value.file_system_id,
            path: value.path,
        }
    }
}
///The stage 0 builder for the [`CreateDatasetRequest`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    #[inline]
    pub fn file_system_id<T>(self, file_system_id: T) -> BuilderStage1
    where
        T: Into<String>,
    {
        BuilderStage1 {
            file_system_id: file_system_id.into(),
        }
    }
}
///The stage 1 builder for the [`CreateDatasetRequest`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    file_system_id: String,
}
impl BuilderStage1 {
    #[inline]
    pub fn path<T>(self, path: T) -> BuilderStage2
    where
        T: Into<String>,
    {
        BuilderStage2 {
            file_system_id: self.file_system_id,
            path: path.into(),
        }
    }
}
///The stage 2 builder for the [`CreateDatasetRequest`] type
#[derive(Debug, Clone)]
pub struct BuilderStage2 {
    file_system_id: String,
    path: String,
}
impl BuilderStage2 {
    #[inline]
    pub fn file_system_id<T>(mut self, file_system_id: T) -> Self
    where
        T: Into<String>,
    {
        self.file_system_id = file_system_id.into();
        self
    }
    #[inline]
    pub fn path<T>(mut self, path: T) -> Self
    where
        T: Into<String>,
    {
        self.path = path.into();
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> CreateDatasetRequest {
        CreateDatasetRequest {
            file_system_id: self.file_system_id,
            path: self.path,
        }
    }
}
impl ser::Serialize for CreateDatasetRequest {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 2usize;
        let mut s = s.serialize_struct("CreateDatasetRequest", size)?;
        s.serialize_field("fileSystemId", &self.file_system_id)?;
        s.serialize_field("path", &self.path)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for CreateDatasetRequest {
    fn deserialize<D>(d: D) -> Result<CreateDatasetRequest, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("CreateDatasetRequest", &["fileSystemId", "path"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = CreateDatasetRequest;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<CreateDatasetRequest, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut file_system_id = None;
        let mut path = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::FileSystemId => file_system_id = Some(map_.next_value()?),
                Field_::Path => path = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let file_system_id = match file_system_id {
            Some(v) => v,
            None => return Err(de::Error::missing_field("fileSystemId")),
        };
        let path = match path {
            Some(v) => v,
            None => return Err(de::Error::missing_field("path")),
        };
        Ok(CreateDatasetRequest {
            file_system_id,
            path,
        })
    }
}
enum Field_ {
    FileSystemId,
    Path,
    Unknown_,
}
impl<'de> de::Deserialize<'de> for Field_ {
    fn deserialize<D>(d: D) -> Result<Field_, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(FieldVisitor_)
    }
}
struct FieldVisitor_;
impl<'de> de::Visitor<'de> for FieldVisitor_ {
    type Value = Field_;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E>(self, value: &str) -> Result<Field_, E>
    where
        E: de::Error,
    {
        let v = match value {
            "fileSystemId" => Field_::FileSystemId,
            "path" => Field_::Path,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
