use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BackingFileSystem {
    file_system_id: String,
    base_uri: String,
    configuration: std::collections::BTreeMap<String, String>,
}
impl BackingFileSystem {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new<T, U, V>(file_system_id: T, base_uri: U, configuration: V) -> BackingFileSystem
    where
        T: Into<String>,
        U: Into<String>,
        V: IntoIterator<Item = (String, String)>,
    {
        BackingFileSystem {
            file_system_id: file_system_id.into(),
            base_uri: base_uri.into(),
            configuration: configuration.into_iter().collect(),
        }
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[doc = "The name by which this file system is identified."]
    #[inline]
    pub fn file_system_id(&self) -> &str {
        &*self.file_system_id
    }
    #[inline]
    pub fn base_uri(&self) -> &str {
        &*self.base_uri
    }
    #[inline]
    pub fn configuration(&self) -> &std::collections::BTreeMap<String, String> {
        &self.configuration
    }
}
#[doc = "A builder for the `BackingFileSystem` type."]
#[derive(Debug, Clone, Default)]
pub struct Builder {
    file_system_id: Option<String>,
    base_uri: Option<String>,
    configuration: std::collections::BTreeMap<String, String>,
}
impl Builder {
    #[doc = "The name by which this file system is identified."]
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn file_system_id<T>(&mut self, file_system_id: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.file_system_id = Some(file_system_id.into());
        self
    }
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn base_uri<T>(&mut self, base_uri: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.base_uri = Some(base_uri.into());
        self
    }
    #[inline]
    pub fn configuration<T>(&mut self, configuration: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.configuration = configuration.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_configuration<T>(&mut self, configuration: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.configuration.extend(configuration);
        self
    }
    #[inline]
    pub fn insert_configuration<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.configuration.insert(key.into(), value.into());
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> BackingFileSystem {
        BackingFileSystem {
            file_system_id: self
                .file_system_id
                .clone()
                .expect("field file_system_id was not set"),
            base_uri: self.base_uri.clone().expect("field base_uri was not set"),
            configuration: self.configuration.clone(),
        }
    }
}
impl From<BackingFileSystem> for Builder {
    #[inline]
    fn from(_v: BackingFileSystem) -> Builder {
        Builder {
            file_system_id: Some(_v.file_system_id),
            base_uri: Some(_v.base_uri),
            configuration: _v.configuration,
        }
    }
}
impl ser::Serialize for BackingFileSystem {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 2usize;
        let skip_configuration = self.configuration.is_empty();
        if !skip_configuration {
            size += 1;
        }
        let mut s = s.serialize_struct("BackingFileSystem", size)?;
        s.serialize_field("fileSystemId", &self.file_system_id)?;
        s.serialize_field("baseUri", &self.base_uri)?;
        if skip_configuration {
            s.skip_field("configuration")?;
        } else {
            s.serialize_field("configuration", &self.configuration)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for BackingFileSystem {
    fn deserialize<D>(d: D) -> Result<BackingFileSystem, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "BackingFileSystem",
            &["fileSystemId", "baseUri", "configuration"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = BackingFileSystem;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<BackingFileSystem, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut file_system_id = None;
        let mut base_uri = None;
        let mut configuration = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::FileSystemId => file_system_id = Some(map_.next_value()?),
                Field_::BaseUri => base_uri = Some(map_.next_value()?),
                Field_::Configuration => configuration = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let file_system_id = match file_system_id {
            Some(v) => v,
            None => return Err(de::Error::missing_field("fileSystemId")),
        };
        let base_uri = match base_uri {
            Some(v) => v,
            None => return Err(de::Error::missing_field("baseUri")),
        };
        let configuration = match configuration {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(BackingFileSystem {
            file_system_id,
            base_uri,
            configuration,
        })
    }
}
enum Field_ {
    FileSystemId,
    BaseUri,
    Configuration,
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
            "baseUri" => Field_::BaseUri,
            "configuration" => Field_::Configuration,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
