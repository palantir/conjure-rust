use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CreateDatasetRequest {
    file_system_id: String,
    path: String,
}
impl CreateDatasetRequest {
    #[doc = r" Constructs a new instance of the type."]
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
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
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
#[doc = "A builder for the `CreateDatasetRequest` type."]
#[derive(Debug, Clone, Default)]
pub struct Builder {
    file_system_id: Option<String>,
    path: Option<String>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    pub fn file_system_id<T>(&mut self, file_system_id: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.file_system_id = Some(file_system_id.into());
        self
    }
    #[doc = r""]
    #[doc = r" Required."]
    pub fn path<T>(&mut self, path: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.path = Some(path.into());
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> CreateDatasetRequest {
        CreateDatasetRequest {
            file_system_id: self
                .file_system_id
                .clone()
                .expect("field file_system_id was not set"),
            path: self.path.clone().expect("field path was not set"),
        }
    }
}
impl From<CreateDatasetRequest> for Builder {
    #[inline]
    fn from(_v: CreateDatasetRequest) -> Builder {
        Builder {
            file_system_id: Some(_v.file_system_id),
            path: Some(_v.path),
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
