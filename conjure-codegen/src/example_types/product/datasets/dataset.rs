use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Dataset {
    file_system_id: String,
    rid: conjure_object::ResourceIdentifier,
}
impl Dataset {
    #[doc = r" Constructs a new instance of the type."]
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
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn file_system_id(&self) -> &str {
        &*self.file_system_id
    }
    #[doc = "Uniquely identifies this dataset."]
    #[inline]
    pub fn rid(&self) -> &conjure_object::ResourceIdentifier {
        &self.rid
    }
}
#[doc = "A builder for the `Dataset` type."]
#[derive(Debug, Clone, Default)]
pub struct Builder {
    file_system_id: Option<String>,
    rid: Option<conjure_object::ResourceIdentifier>,
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
    #[doc = "Uniquely identifies this dataset."]
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn rid(&mut self, rid: conjure_object::ResourceIdentifier) -> &mut Self {
        self.rid = Some(rid);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> Dataset {
        Dataset {
            file_system_id: self
                .file_system_id
                .clone()
                .expect("field file_system_id was not set"),
            rid: self.rid.clone().expect("field rid was not set"),
        }
    }
}
impl From<Dataset> for Builder {
    #[inline]
    fn from(_v: Dataset) -> Builder {
        Builder {
            file_system_id: Some(_v.file_system_id),
            rid: Some(_v.rid),
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
        Ok(Dataset {
            file_system_id,
            rid,
        })
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
