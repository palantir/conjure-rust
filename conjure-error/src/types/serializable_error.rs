use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[doc = "The JSON-serializable representation of an error."]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SerializableError {
    error_code: super::ErrorCode,
    error_name: String,
    error_instance_id: conjure_object::Uuid,
    parameters: std::collections::BTreeMap<String, conjure_object::Value>,
}
impl SerializableError {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[doc = "The broad category of the error."]
    #[doc = ""]
    #[doc = "When transmitted over HTTP, this determines the response\'s status code."]
    #[inline]
    pub fn error_code(&self) -> &super::ErrorCode {
        &self.error_code
    }
    #[doc = "The error\'s name."]
    #[doc = ""]
    #[doc = "The name is made up of a namespace and more specific error name, separated by a `:`."]
    #[inline]
    pub fn error_name(&self) -> &str {
        &*self.error_name
    }
    #[doc = "A unique identifier for this error instance."]
    #[doc = ""]
    #[doc = "This can be used to correlate reporting about the error as it transfers between components of a"]
    #[doc = "distributed system."]
    #[inline]
    pub fn error_instance_id(&self) -> conjure_object::Uuid {
        self.error_instance_id
    }
    #[doc = "Parameters providing more information about the error."]
    #[inline]
    pub fn parameters(&self) -> &std::collections::BTreeMap<String, conjure_object::Value> {
        &self.parameters
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    error_code: Option<super::ErrorCode>,
    error_name: Option<String>,
    error_instance_id: Option<conjure_object::Uuid>,
    parameters: std::collections::BTreeMap<String, conjure_object::Value>,
}
impl Builder {
    #[doc = "The broad category of the error."]
    #[doc = ""]
    #[doc = "When transmitted over HTTP, this determines the response\'s status code."]
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn error_code(&mut self, error_code: super::ErrorCode) -> &mut Self {
        self.error_code = Some(error_code);
        self
    }
    #[doc = "The error\'s name."]
    #[doc = ""]
    #[doc = "The name is made up of a namespace and more specific error name, separated by a `:`."]
    #[doc = r""]
    #[doc = r" Required."]
    pub fn error_name<T>(&mut self, error_name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.error_name = Some(error_name.into());
        self
    }
    #[doc = "A unique identifier for this error instance."]
    #[doc = ""]
    #[doc = "This can be used to correlate reporting about the error as it transfers between components of a"]
    #[doc = "distributed system."]
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn error_instance_id(&mut self, error_instance_id: conjure_object::Uuid) -> &mut Self {
        self.error_instance_id = Some(error_instance_id);
        self
    }
    #[doc = "Parameters providing more information about the error."]
    pub fn parameters<T>(&mut self, parameters: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, conjure_object::Value)>,
    {
        self.parameters = parameters.into_iter().collect();
        self
    }
    #[doc = "Parameters providing more information about the error."]
    pub fn extend_parameters<T>(&mut self, parameters: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, conjure_object::Value)>,
    {
        self.parameters.extend(parameters);
        self
    }
    #[doc = "Parameters providing more information about the error."]
    pub fn insert_parameters<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: conjure_object::serde::Serialize,
    {
        self.parameters.insert(
            key.into(),
            conjure_object::serde_value::to_value(value).expect("value failed to serialize"),
        );
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> SerializableError {
        SerializableError {
            error_code: self
                .error_code
                .clone()
                .expect("field error_code was not set"),
            error_name: self
                .error_name
                .clone()
                .expect("field error_name was not set"),
            error_instance_id: self
                .error_instance_id
                .clone()
                .expect("field error_instance_id was not set"),
            parameters: self.parameters.clone(),
        }
    }
}
impl From<SerializableError> for Builder {
    #[inline]
    fn from(_v: SerializableError) -> Builder {
        Builder {
            error_code: Some(_v.error_code),
            error_name: Some(_v.error_name),
            error_instance_id: Some(_v.error_instance_id),
            parameters: _v.parameters,
        }
    }
}
impl ser::Serialize for SerializableError {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 3usize;
        let skip_parameters = self.parameters.is_empty();
        if !skip_parameters {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"errorCode", &self.error_code)?;
        map.serialize_entry(&"errorName", &self.error_name)?;
        map.serialize_entry(&"errorInstanceId", &self.error_instance_id)?;
        if !skip_parameters {
            map.serialize_entry(&"parameters", &self.parameters)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for SerializableError {
    fn deserialize<D>(d: D) -> Result<SerializableError, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "SerializableError",
            &["errorCode", "errorName", "errorInstanceId", "parameters"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = SerializableError;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<SerializableError, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut error_code = None;
        let mut error_name = None;
        let mut error_instance_id = None;
        let mut parameters = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::ErrorCode => error_code = Some(map_.next_value()?),
                Field_::ErrorName => error_name = Some(map_.next_value()?),
                Field_::ErrorInstanceId => error_instance_id = Some(map_.next_value()?),
                Field_::Parameters => parameters = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let error_code = match error_code {
            Some(v) => v,
            None => return Err(de::Error::missing_field("errorCode")),
        };
        let error_name = match error_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("errorName")),
        };
        let error_instance_id = match error_instance_id {
            Some(v) => v,
            None => return Err(de::Error::missing_field("errorInstanceId")),
        };
        let parameters = match parameters {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(SerializableError {
            error_code,
            error_name,
            error_instance_id,
            parameters,
        })
    }
}
enum Field_ {
    ErrorCode,
    ErrorName,
    ErrorInstanceId,
    Parameters,
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
            "errorCode" => Field_::ErrorCode,
            "errorName" => Field_::ErrorName,
            "errorInstanceId" => Field_::ErrorInstanceId,
            "parameters" => Field_::Parameters,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
