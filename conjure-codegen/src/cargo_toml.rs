use conjure_object::Any;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct Manifest<'a> {
    pub package: Package<'a>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub dependencies: BTreeMap<&'a str, &'a str>,
}

#[derive(Serialize)]
pub struct Package<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub edition: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata<'a>>,
}

#[derive(Serialize)]
pub struct Metadata<'a> {
    pub sls: Sls<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Sls<'a> {
    pub recommended_product_dependencies: &'a Any,
}
