use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct TypeName {
    #[builder(into)]
    name: String,
    #[builder(into)]
    package: String,
}
impl TypeName {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(name: impl Into<String>, package: impl Into<String>) -> Self {
        Self::builder().name(name).package(package).build()
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
