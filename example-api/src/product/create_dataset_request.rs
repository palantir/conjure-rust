use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct CreateDatasetRequest {
    #[builder(into)]
    file_system_id: String,
    #[builder(into)]
    path: String,
}
impl CreateDatasetRequest {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(file_system_id: impl Into<String>, path: impl Into<String>) -> Self {
        Self::builder().file_system_id(file_system_id).path(path).build()
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
