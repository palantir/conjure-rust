#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
#[serde(crate = "conjure_object::serde")]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct TypeName {
    #[builder(into)]
    #[serde(rename = "name")]
    name: String,
    #[builder(into)]
    #[serde(rename = "package")]
    package: String,
}
impl TypeName {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(name: impl Into<String>, package: impl Into<String>) -> Self {
        Self::builder().name(name).package(package).build()
    }
    /// The name of the custom Conjure type or service. It must be in UpperCamelCase. Numbers are permitted, but not at the beginning of a word. Allowed names: "FooBar", "XYCoordinate", "Build2Request". Disallowed names: "fooBar", "2BuildRequest".
    #[inline]
    pub fn name(&self) -> &str {
        &*self.name
    }
    /// A period-delimited string of package names. The package names must be lowercase. Numbers are permitted, but not at the beginning of a package name. Allowed packages: "foo", "com.palantir.bar", "com.palantir.foo.thing2". Disallowed packages: "Foo", "com.palantir.foo.2thing".
    #[inline]
    pub fn package(&self) -> &str {
        &*self.package
    }
}
