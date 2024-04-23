use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct Dataset {
    #[builder(into)]
    file_system_id: String,
    #[builder()]
    rid: conjure_object::ResourceIdentifier,
}
impl Dataset {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        file_system_id: impl Into<String>,
        rid: conjure_object::ResourceIdentifier,
    ) -> Self {
        Self::builder().file_system_id(file_system_id).rid(rid).build()
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
