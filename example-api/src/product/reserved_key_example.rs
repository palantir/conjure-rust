use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReservedKeyExample {
    package: String,
    interface: String,
    field_name_with_dashes: String,
    primitve_field_name_with_dashes: i32,
    memoized_hash_code: i32,
}
impl ReservedKeyExample {
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn package(&self) -> &str {
        &*self.package
    }
    #[inline]
    pub fn interface(&self) -> &str {
        &*self.interface
    }
    #[inline]
    pub fn field_name_with_dashes(&self) -> &str {
        &*self.field_name_with_dashes
    }
    #[inline]
    pub fn primitve_field_name_with_dashes(&self) -> i32 {
        self.primitve_field_name_with_dashes
    }
    #[inline]
    pub fn memoized_hash_code(&self) -> i32 {
        self.memoized_hash_code
    }
}
///A builder for the `ReservedKeyExample` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    package: Option<String>,
    interface: Option<String>,
    field_name_with_dashes: Option<String>,
    primitve_field_name_with_dashes: Option<i32>,
    memoized_hash_code: Option<i32>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn package<T>(&mut self, package: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.package = Some(package.into());
        self
    }
    ///
    /// Required.
    #[inline]
    pub fn interface<T>(&mut self, interface: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.interface = Some(interface.into());
        self
    }
    ///
    /// Required.
    #[inline]
    pub fn field_name_with_dashes<T>(&mut self, field_name_with_dashes: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.field_name_with_dashes = Some(field_name_with_dashes.into());
        self
    }
    ///
    /// Required.
    #[inline]
    pub fn primitve_field_name_with_dashes(
        &mut self,
        primitve_field_name_with_dashes: i32,
    ) -> &mut Self {
        self.primitve_field_name_with_dashes = Some(primitve_field_name_with_dashes);
        self
    }
    ///
    /// Required.
    #[inline]
    pub fn memoized_hash_code(&mut self, memoized_hash_code: i32) -> &mut Self {
        self.memoized_hash_code = Some(memoized_hash_code);
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> ReservedKeyExample {
        ReservedKeyExample {
            package: self.package.clone().expect("field package was not set"),
            interface: self.interface.clone().expect("field interface was not set"),
            field_name_with_dashes: self
                .field_name_with_dashes
                .clone()
                .expect("field field_name_with_dashes was not set"),
            primitve_field_name_with_dashes: self
                .primitve_field_name_with_dashes
                .clone()
                .expect("field primitve_field_name_with_dashes was not set"),
            memoized_hash_code: self
                .memoized_hash_code
                .clone()
                .expect("field memoized_hash_code was not set"),
        }
    }
}
impl From<ReservedKeyExample> for Builder {
    #[inline]
    fn from(_v: ReservedKeyExample) -> Builder {
        Builder {
            package: Some(_v.package),
            interface: Some(_v.interface),
            field_name_with_dashes: Some(_v.field_name_with_dashes),
            primitve_field_name_with_dashes: Some(_v.primitve_field_name_with_dashes),
            memoized_hash_code: Some(_v.memoized_hash_code),
        }
    }
}
impl ser::Serialize for ReservedKeyExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 5usize;
        let mut s = s.serialize_struct("ReservedKeyExample", size)?;
        s.serialize_field("package", &self.package)?;
        s.serialize_field("interface", &self.interface)?;
        s.serialize_field("field-name-with-dashes", &self.field_name_with_dashes)?;
        s.serialize_field(
            "primitve-field-name-with-dashes",
            &self.primitve_field_name_with_dashes,
        )?;
        s.serialize_field("memoizedHashCode", &self.memoized_hash_code)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for ReservedKeyExample {
    fn deserialize<D>(d: D) -> Result<ReservedKeyExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ReservedKeyExample",
            &[
                "package",
                "interface",
                "field-name-with-dashes",
                "primitve-field-name-with-dashes",
                "memoizedHashCode",
            ],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ReservedKeyExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<ReservedKeyExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut package = None;
        let mut interface = None;
        let mut field_name_with_dashes = None;
        let mut primitve_field_name_with_dashes = None;
        let mut memoized_hash_code = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Package => package = Some(map_.next_value()?),
                Field_::Interface => interface = Some(map_.next_value()?),
                Field_::FieldNameWithDashes => {
                    field_name_with_dashes = Some(map_.next_value()?);
                }
                Field_::PrimitveFieldNameWithDashes => {
                    primitve_field_name_with_dashes = Some(map_.next_value()?);
                }
                Field_::MemoizedHashCode => memoized_hash_code = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let package = match package {
            Some(v) => v,
            None => return Err(de::Error::missing_field("package")),
        };
        let interface = match interface {
            Some(v) => v,
            None => return Err(de::Error::missing_field("interface")),
        };
        let field_name_with_dashes = match field_name_with_dashes {
            Some(v) => v,
            None => return Err(de::Error::missing_field("field-name-with-dashes")),
        };
        let primitve_field_name_with_dashes = match primitve_field_name_with_dashes {
            Some(v) => v,
            None => {
                return Err(de::Error::missing_field("primitve-field-name-with-dashes"));
            }
        };
        let memoized_hash_code = match memoized_hash_code {
            Some(v) => v,
            None => return Err(de::Error::missing_field("memoizedHashCode")),
        };
        Ok(ReservedKeyExample {
            package,
            interface,
            field_name_with_dashes,
            primitve_field_name_with_dashes,
            memoized_hash_code,
        })
    }
}
enum Field_ {
    Package,
    Interface,
    FieldNameWithDashes,
    PrimitveFieldNameWithDashes,
    MemoizedHashCode,
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
            "package" => Field_::Package,
            "interface" => Field_::Interface,
            "field-name-with-dashes" => Field_::FieldNameWithDashes,
            "primitve-field-name-with-dashes" => Field_::PrimitveFieldNameWithDashes,
            "memoizedHashCode" => Field_::MemoizedHashCode,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
