use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dataset {
    file_system_id: String,
    rid: conjure_object::ResourceIdentifier,
}
impl Dataset {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(file_system_id: T, rid: conjure_object::ResourceIdentifier) -> Dataset
    where
        T: Into<String>,
    {
        Dataset {
            file_system_id: file_system_id.into(),
            rid: rid,
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
    ///Uniquely identifies this dataset.
    #[inline]
    pub fn rid(&self) -> &conjure_object::ResourceIdentifier {
        &self.rid
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<Dataset> for BuilderStage2 {
    #[inline]
    fn from(value: Dataset) -> Self {
        BuilderStage2 {
            file_system_id: value.file_system_id,
            rid: value.rid,
        }
    }
}
///The stage 0 builder for the [`Dataset`] type
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
///The stage 1 builder for the [`Dataset`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    file_system_id: String,
}
impl BuilderStage1 {
    ///Uniquely identifies this dataset.
    #[inline]
    pub fn rid(self, rid: conjure_object::ResourceIdentifier) -> BuilderStage2 {
        BuilderStage2 {
            file_system_id: self.file_system_id,
            rid: rid,
        }
    }
}
///The stage 2 builder for the [`Dataset`] type
#[derive(Debug, Clone)]
pub struct BuilderStage2 {
    file_system_id: String,
    rid: conjure_object::ResourceIdentifier,
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
    ///Uniquely identifies this dataset.
    #[inline]
    pub fn rid(mut self, rid: conjure_object::ResourceIdentifier) -> Self {
        self.rid = rid;
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> Dataset {
        Dataset {
            file_system_id: self.file_system_id,
            rid: self.rid,
        }
    }
}
impl ser::Serialize for Dataset {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 2usize;
        let mut s = s.serialize_struct("Dataset", size)?;
        s.serialize_field("fileSystemId", &self.file_system_id)?;
        s.serialize_field("rid", &self.rid)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for Dataset {
    fn deserialize<D>(d: D) -> Result<Dataset, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("Dataset", &["fileSystemId", "rid"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = Dataset;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<Dataset, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut file_system_id = None;
        let mut rid = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::FileSystemId => file_system_id = Some(map_.next_value()?),
                Field_::Rid => rid = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let file_system_id = match file_system_id {
            Some(v) => v,
            None => return Err(de::Error::missing_field("fileSystemId")),
        };
        let rid = match rid {
            Some(v) => v,
            None => return Err(de::Error::missing_field("rid")),
        };
        Ok(Dataset { file_system_id, rid })
    }
}
enum Field_ {
    FileSystemId,
    Rid,
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
            "rid" => Field_::Rid,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
