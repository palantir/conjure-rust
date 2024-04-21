use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeName {
    name: String,
    package: String,
}
impl TypeName {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T, U>(name: T, package: U) -> TypeName
    where
        T: Into<String>,
        U: Into<String>,
    {
        TypeName {
            name: name.into(),
            package: package.into(),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    ///The name of the custom Conjure type or service. It must be in UpperCamelCase. Numbers are permitted, but not at the beginning of a word. Allowed names: "FooBar", "XYCoordinate", "Build2Request". Disallowed names: "fooBar", "2BuildRequest".
    #[inline]
    pub fn name(&self) -> &str {
        &*self.name
    }
    ///A period-delimited string of package names. The package names must be lowercase. Numbers are permitted, but not at the beginning of a package name. Allowed packages: "foo", "com.palantir.bar", "com.palantir.foo.thing2". Disallowed packages: "Foo", "com.palantir.foo.2thing".
    #[inline]
    pub fn package(&self) -> &str {
        &*self.package
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<TypeName> for BuilderStage2 {
    #[inline]
    fn from(value: TypeName) -> Self {
        BuilderStage2 {
            name: value.name,
            package: value.package,
        }
    }
}
///The stage 0 builder for the [`TypeName`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    ///The name of the custom Conjure type or service. It must be in UpperCamelCase. Numbers are permitted, but not at the beginning of a word. Allowed names: "FooBar", "XYCoordinate", "Build2Request". Disallowed names: "fooBar", "2BuildRequest".
    #[inline]
    pub fn name<T>(self, name: T) -> BuilderStage1
    where
        T: Into<String>,
    {
        BuilderStage1 { name: name.into() }
    }
}
///The stage 1 builder for the [`TypeName`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    name: String,
}
impl BuilderStage1 {
    ///A period-delimited string of package names. The package names must be lowercase. Numbers are permitted, but not at the beginning of a package name. Allowed packages: "foo", "com.palantir.bar", "com.palantir.foo.thing2". Disallowed packages: "Foo", "com.palantir.foo.2thing".
    #[inline]
    pub fn package<T>(self, package: T) -> BuilderStage2
    where
        T: Into<String>,
    {
        BuilderStage2 {
            name: self.name,
            package: package.into(),
        }
    }
}
///The stage 2 builder for the [`TypeName`] type
#[derive(Debug, Clone)]
pub struct BuilderStage2 {
    name: String,
    package: String,
}
impl BuilderStage2 {
    ///The name of the custom Conjure type or service. It must be in UpperCamelCase. Numbers are permitted, but not at the beginning of a word. Allowed names: "FooBar", "XYCoordinate", "Build2Request". Disallowed names: "fooBar", "2BuildRequest".
    #[inline]
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: Into<String>,
    {
        self.name = name.into();
        self
    }
    ///A period-delimited string of package names. The package names must be lowercase. Numbers are permitted, but not at the beginning of a package name. Allowed packages: "foo", "com.palantir.bar", "com.palantir.foo.thing2". Disallowed packages: "Foo", "com.palantir.foo.2thing".
    #[inline]
    pub fn package<T>(mut self, package: T) -> Self
    where
        T: Into<String>,
    {
        self.package = package.into();
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> TypeName {
        TypeName {
            name: self.name,
            package: self.package,
        }
    }
}
impl ser::Serialize for TypeName {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 2usize;
        let mut s = s.serialize_struct("TypeName", size)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("package", &self.package)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for TypeName {
    fn deserialize<D>(d: D) -> Result<TypeName, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("TypeName", &["name", "package"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = TypeName;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<TypeName, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut name = None;
        let mut package = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Name => name = Some(map_.next_value()?),
                Field_::Package => package = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let name = match name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("name")),
        };
        let package = match package {
            Some(v) => v,
            None => return Err(de::Error::missing_field("package")),
        };
        Ok(TypeName { name, package })
    }
}
enum Field_ {
    Name,
    Package,
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
            "name" => Field_::Name,
            "package" => Field_::Package,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
