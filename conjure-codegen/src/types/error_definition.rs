use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorDefinition {
    error_name: Box<super::TypeName>,
    docs: Option<super::Documentation>,
    namespace: super::ErrorNamespace,
    code: super::ErrorCode,
    safe_args: Vec<super::FieldDefinition>,
    unsafe_args: Vec<super::FieldDefinition>,
}
impl ErrorDefinition {
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn error_name(&self) -> &super::TypeName {
        &*self.error_name
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn namespace(&self) -> &super::ErrorNamespace {
        &self.namespace
    }
    #[inline]
    pub fn code(&self) -> &super::ErrorCode {
        &self.code
    }
    #[inline]
    pub fn safe_args(&self) -> &[super::FieldDefinition] {
        &*self.safe_args
    }
    #[inline]
    pub fn unsafe_args(&self) -> &[super::FieldDefinition] {
        &*self.unsafe_args
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<ErrorDefinition> for BuilderStage3 {
    #[inline]
    fn from(value: ErrorDefinition) -> Self {
        BuilderStage3 {
            error_name: value.error_name,
            docs: value.docs,
            namespace: value.namespace,
            code: value.code,
            safe_args: value.safe_args,
            unsafe_args: value.unsafe_args,
        }
    }
}
///The stage 0 builder for the [`ErrorDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    #[inline]
    pub fn error_name(self, error_name: super::TypeName) -> BuilderStage1 {
        BuilderStage1 {
            error_name: Box::new(error_name),
        }
    }
}
///The stage 1 builder for the [`ErrorDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    error_name: Box<super::TypeName>,
}
impl BuilderStage1 {
    #[inline]
    pub fn namespace(self, namespace: super::ErrorNamespace) -> BuilderStage2 {
        BuilderStage2 {
            error_name: self.error_name,
            namespace: namespace,
        }
    }
}
///The stage 2 builder for the [`ErrorDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage2 {
    error_name: Box<super::TypeName>,
    namespace: super::ErrorNamespace,
}
impl BuilderStage2 {
    #[inline]
    pub fn code(self, code: super::ErrorCode) -> BuilderStage3 {
        BuilderStage3 {
            error_name: self.error_name,
            namespace: self.namespace,
            code: code,
            docs: Default::default(),
            safe_args: Default::default(),
            unsafe_args: Default::default(),
        }
    }
}
///The stage 3 builder for the [`ErrorDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage3 {
    error_name: Box<super::TypeName>,
    namespace: super::ErrorNamespace,
    code: super::ErrorCode,
    docs: Option<super::Documentation>,
    safe_args: Vec<super::FieldDefinition>,
    unsafe_args: Vec<super::FieldDefinition>,
}
impl BuilderStage3 {
    #[inline]
    pub fn error_name(mut self, error_name: super::TypeName) -> Self {
        self.error_name = Box::new(error_name);
        self
    }
    #[inline]
    pub fn namespace(mut self, namespace: super::ErrorNamespace) -> Self {
        self.namespace = namespace;
        self
    }
    #[inline]
    pub fn code(mut self, code: super::ErrorCode) -> Self {
        self.code = code;
        self
    }
    #[inline]
    pub fn docs<T>(mut self, docs: T) -> Self
    where
        T: Into<Option<super::Documentation>>,
    {
        self.docs = docs.into();
        self
    }
    #[inline]
    pub fn safe_args<T>(mut self, safe_args: T) -> Self
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        self.safe_args = safe_args.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_safe_args<T>(mut self, safe_args: T) -> Self
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        self.safe_args.extend(safe_args);
        self
    }
    #[inline]
    pub fn push_safe_args(mut self, value: super::FieldDefinition) -> Self {
        self.safe_args.push(value);
        self
    }
    #[inline]
    pub fn unsafe_args<T>(mut self, unsafe_args: T) -> Self
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        self.unsafe_args = unsafe_args.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_unsafe_args<T>(mut self, unsafe_args: T) -> Self
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        self.unsafe_args.extend(unsafe_args);
        self
    }
    #[inline]
    pub fn push_unsafe_args(mut self, value: super::FieldDefinition) -> Self {
        self.unsafe_args.push(value);
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> ErrorDefinition {
        ErrorDefinition {
            error_name: self.error_name,
            docs: self.docs,
            namespace: self.namespace,
            code: self.code,
            safe_args: self.safe_args,
            unsafe_args: self.unsafe_args,
        }
    }
}
impl ser::Serialize for ErrorDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 3usize;
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let skip_safe_args = self.safe_args.is_empty();
        if !skip_safe_args {
            size += 1;
        }
        let skip_unsafe_args = self.unsafe_args.is_empty();
        if !skip_unsafe_args {
            size += 1;
        }
        let mut s = s.serialize_struct("ErrorDefinition", size)?;
        s.serialize_field("errorName", &self.error_name)?;
        if skip_docs {
            s.skip_field("docs")?;
        } else {
            s.serialize_field("docs", &self.docs)?;
        }
        s.serialize_field("namespace", &self.namespace)?;
        s.serialize_field("code", &self.code)?;
        if skip_safe_args {
            s.skip_field("safeArgs")?;
        } else {
            s.serialize_field("safeArgs", &self.safe_args)?;
        }
        if skip_unsafe_args {
            s.skip_field("unsafeArgs")?;
        } else {
            s.serialize_field("unsafeArgs", &self.unsafe_args)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for ErrorDefinition {
    fn deserialize<D>(d: D) -> Result<ErrorDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ErrorDefinition",
            &["errorName", "docs", "namespace", "code", "safeArgs", "unsafeArgs"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ErrorDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<ErrorDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut error_name = None;
        let mut docs = None;
        let mut namespace = None;
        let mut code = None;
        let mut safe_args = None;
        let mut unsafe_args = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::ErrorName => error_name = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Namespace => namespace = Some(map_.next_value()?),
                Field_::Code => code = Some(map_.next_value()?),
                Field_::SafeArgs => safe_args = Some(map_.next_value()?),
                Field_::UnsafeArgs => unsafe_args = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let error_name = match error_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("errorName")),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        let namespace = match namespace {
            Some(v) => v,
            None => return Err(de::Error::missing_field("namespace")),
        };
        let code = match code {
            Some(v) => v,
            None => return Err(de::Error::missing_field("code")),
        };
        let safe_args = match safe_args {
            Some(v) => v,
            None => Default::default(),
        };
        let unsafe_args = match unsafe_args {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(ErrorDefinition {
            error_name,
            docs,
            namespace,
            code,
            safe_args,
            unsafe_args,
        })
    }
}
enum Field_ {
    ErrorName,
    Docs,
    Namespace,
    Code,
    SafeArgs,
    UnsafeArgs,
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
            "errorName" => Field_::ErrorName,
            "docs" => Field_::Docs,
            "namespace" => Field_::Namespace,
            "code" => Field_::Code,
            "safeArgs" => Field_::SafeArgs,
            "unsafeArgs" => Field_::UnsafeArgs,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
