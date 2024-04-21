use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BackingFileSystem {
    file_system_id: String,
    base_uri: String,
    configuration: std::collections::BTreeMap<String, String>,
}
impl BackingFileSystem {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T, U, V>(
        file_system_id: T,
        base_uri: U,
        configuration: V,
    ) -> BackingFileSystem
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
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    ///The name by which this file system is identified.
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
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<BackingFileSystem> for BuilderStage2 {
    #[inline]
    fn from(value: BackingFileSystem) -> Self {
        BuilderStage2 {
            file_system_id: value.file_system_id,
            base_uri: value.base_uri,
            configuration: value.configuration,
        }
    }
}
///The stage 0 builder for the [`BackingFileSystem`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    ///The name by which this file system is identified.
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
///The stage 1 builder for the [`BackingFileSystem`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    file_system_id: String,
}
impl BuilderStage1 {
    #[inline]
    pub fn base_uri<T>(self, base_uri: T) -> BuilderStage2
    where
        T: Into<String>,
    {
        BuilderStage2 {
            file_system_id: self.file_system_id,
            base_uri: base_uri.into(),
            configuration: Default::default(),
        }
    }
}
///The stage 2 builder for the [`BackingFileSystem`] type
#[derive(Debug, Clone)]
pub struct BuilderStage2 {
    file_system_id: String,
    base_uri: String,
    configuration: std::collections::BTreeMap<String, String>,
}
impl BuilderStage2 {
    ///The name by which this file system is identified.
    #[inline]
    pub fn file_system_id<T>(mut self, file_system_id: T) -> Self
    where
        T: Into<String>,
    {
        self.file_system_id = file_system_id.into();
        self
    }
    #[inline]
    pub fn base_uri<T>(mut self, base_uri: T) -> Self
    where
        T: Into<String>,
    {
        self.base_uri = base_uri.into();
        self
    }
    #[inline]
    pub fn configuration<T>(mut self, configuration: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.configuration = configuration.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_configuration<T>(mut self, configuration: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.configuration.extend(configuration);
        self
    }
    #[inline]
    pub fn insert_configuration<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.configuration.insert(key.into(), value.into());
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> BackingFileSystem {
        BackingFileSystem {
            file_system_id: self.file_system_id,
            base_uri: self.base_uri,
            configuration: self.configuration,
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
