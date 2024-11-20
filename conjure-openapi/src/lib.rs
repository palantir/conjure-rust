use std::{fs::File, io::BufReader, path::Path};

use conjure_codegen::{
    conjure_definition::ConjureDefinition, type_::Type, type_definition::TypeDefinition,
    AliasDefinition, EnumDefinition, EnumValueDefinition, FieldDefinition, FieldName, ListType,
    ObjectDefinition, OptionalType, PrimitiveType, ServiceDefinition, SetType, TypeName,
    UnionDefinition,
};

use anyhow::{anyhow, Error};
use openapiv3::{OpenAPI, RefOr, Schema, SchemaKind};

pub fn parse_openapi(openapi_file: &Path) -> Result<ConjureDefinition, Error> {
    let file = File::open(openapi_file)?;
    let reader = BufReader::new(file);
    let openapi: OpenAPI = serde_yaml::from_reader(reader)?;

    // let version = parse_version(&openapi);
    let version = 1;
    let errors = vec![]; //TODO
    let services = parse_services(&openapi)?;
    let types = parse_types(&openapi)?;

    let conjure = ConjureDefinition::builder()
        .version(version)
        .errors(errors)
        .services(services)
        .types(types)
        .build();
    Ok(conjure)
}

fn parse_services(_openapi: &OpenAPI) -> Result<impl Iterator<Item = ServiceDefinition>, Error> {
    Ok(vec![].into_iter())
}

fn parse_types(openapi: &OpenAPI) -> Result<impl Iterator<Item = TypeDefinition>, Error> {
    let components = &openapi.components;

    if !components.responses.is_empty() {
        eprintln!("Object parsing not supported. {{ object_type: Response }}");
    }
    if !components.parameters.is_empty() {
        eprintln!("Object parsing not supported. {{ object_type: Parameter }}");
    }
    if !components.examples.is_empty() {
        eprintln!("Object parsing not supported. {{ object_type: Example }}");
    }
    if !components.request_bodies.is_empty() {
        eprintln!("Object parsing not supported. {{ object_type: RequestBody }}");
    }
    if !components.headers.is_empty() {
        eprintln!("Object parsing not supported. {{ object_type: Header }}");
    }
    if !components.links.is_empty() {
        eprintln!("Object parsing not supported. {{ object_type: Link }}");
    }
    if !components.callbacks.is_empty() {
        eprintln!("Object parsing not supported. {{ object_type: Callback }}");
    }
    if !components.extensions.is_empty() {
        eprintln!("Object parsing not supported. {{ object_type: ComponentsExtention }}");
    }

    if !components.security_schemes.is_empty() {
        eprintln!("TODO: IMPL SECURITY SCHEMA");
    }

    let mut types = Vec::new();
    for (name, schema_ref) in &components.schemas {
        let (obj_def, hoist) = parse_ref_or_schema(name, schema_ref)?;
        for hoist_type in hoist {
            types.push(hoist_type.into_type_definition());
        }
        let type_def = obj_def.into_type_definition(name);
        types.push(type_def);
    }

    Ok(types.into_iter())
}

enum HoistType {
    Object(ObjectDefinition),
    Enum(EnumDefinition),
    Union(UnionDefinition),
}

impl HoistType {
    fn get_reference(&self) -> Type {
        let type_name = match self {
            Self::Object(o) => o.type_name(),
            Self::Enum(e) => e.type_name(),
            Self::Union(u) => u.type_name(),
        };
        Type::Reference(type_name.clone())
    }

    fn into_type_definition(self) -> TypeDefinition {
        match self {
            HoistType::Object(o) => TypeDefinition::Object(o),
            HoistType::Enum(e) => TypeDefinition::Enum(e),
            HoistType::Union(u) => TypeDefinition::Union(u),
        }
    }
}

enum AnonymousType {
    Type(Type),
    Hoist(HoistType),
}

impl AnonymousType {
    fn into_type_definition(self, name: &str) -> TypeDefinition {
        match self {
            AnonymousType::Type(ty_) => {
                let type_name = TypeName::builder().name(name).package("default").build();
                TypeDefinition::Alias(AliasDefinition::new(type_name, ty_))
            }
            AnonymousType::Hoist(hoist) => hoist.into_type_definition(),
        }
    }
}

fn parse_ref_or_schema(
    name: &str,
    schema_ref: &RefOr<Schema>,
) -> Result<(AnonymousType, Vec<HoistType>), Error> {
    match schema_ref {
        RefOr::Reference { reference } => {
            let type_name_ = get_type_name_from_ref_str(reference);
            let type_name = TypeName::new(type_name_, "default");
            Ok((AnonymousType::Type(Type::Reference(type_name)), Vec::new()))
        }
        RefOr::Item(schema) => match &schema.kind {
            SchemaKind::Type(type_) => parse_type(name, type_),
            SchemaKind::OneOf { one_of } => {
                let type_name = TypeName::new(name, "default");
                let mut union_builder = UnionDefinition::builder().type_name(type_name);

                let mut hoist = Vec::new();
                for (n, schema) in one_of.iter().enumerate() {
                    let hoist_name = to_pascal_case(&format!("{}_Variant_{}", name, n));
                    let field_name = FieldName::from(format!("variant{}", n));

                    let (field_type, hoist_) = match parse_ref_or_schema(&hoist_name, schema)? {
                        (AnonymousType::Hoist(hoist_type), mut hoist__) => {
                            let ref_ = hoist_type.get_reference();
                            hoist__.push(hoist_type);
                            (ref_, hoist__)
                        }
                        (AnonymousType::Type(type_), hoist__) => (type_, hoist__),
                    };
                    hoist.extend(hoist_);

                    let field = FieldDefinition::new(field_name, field_type);
                    union_builder = union_builder.push_union_(field);
                }

                Ok((
                    AnonymousType::Hoist(HoistType::Union(union_builder.build())),
                    hoist,
                ))
            }
            SchemaKind::AllOf { .. } => Err(anyhow!(
                "SchemaKind::AllOf not supported. {{ type_name: {} }}",
                name
            )),
            SchemaKind::AnyOf { .. } => Err(anyhow!(
                "SchemaKind::AnyOf not supported. {{ type_name: {} }}",
                name
            )),

            SchemaKind::Not { .. } => Err(anyhow!(
                "SchemaKind::Not not supported. {{ type_name: {} }}",
                name
            )),
            SchemaKind::Any(_) => Err(anyhow!(
                "SchemaKind::Any not supported. {{ type_name: {} }}",
                name
            )),
        },
    }
}

fn parse_type(
    name: &str,
    type_: &openapiv3::Type,
) -> Result<(AnonymousType, Vec<HoistType>), Error> {
    let type_name = TypeName::builder().name(name).package("default").build();

    match type_ {
        openapiv3::Type::Object(obj) => {
            let mut obj_builder = ObjectDefinition::builder().type_name(type_name);

            let required_props = &obj.required;
            let mut hoist = Vec::new();
            for (field_name, schema_ref) in &obj.properties {
                let field_name_ = FieldName::from(field_name.clone());
                let hoist_name = to_pascal_case(&format!("{}_{}", name, field_name));

                let (field_type, hoist_) = match parse_ref_or_schema(&hoist_name, schema_ref)? {
                    (AnonymousType::Hoist(hoist_type), mut hoist__) => {
                        let ref_ = hoist_type.get_reference();
                        hoist__.push(hoist_type);
                        (ref_, hoist__)
                    }
                    (AnonymousType::Type(type_), hoist__) => (type_, hoist__),
                };
                hoist.extend(hoist_);

                // If not labeled as required we make it optional
                let field_type = if required_props.contains(field_name) {
                    field_type
                } else {
                    Type::Optional(OptionalType::new(field_type))
                };

                let field = FieldDefinition::new(field_name_, field_type);
                obj_builder = obj_builder.push_fields(field);
            }

            let obj_ = obj_builder.build();
            Ok((AnonymousType::Hoist(HoistType::Object(obj_)), hoist))
        }
        openapiv3::Type::Boolean {} => Ok((
            AnonymousType::Type(Type::Primitive(conjure_codegen::PrimitiveType::Boolean)),
            Vec::new(),
        )),
        openapiv3::Type::Integer(_) => Ok((
            AnonymousType::Type(Type::Primitive(conjure_codegen::PrimitiveType::Integer)),
            Vec::new(),
        )),
        openapiv3::Type::Number(_) => Ok((
            AnonymousType::Type(Type::Primitive(PrimitiveType::Double)),
            Vec::new(),
        )),
        openapiv3::Type::String(string_type) => {
            // Decide if this string type is an Enum or just a string field
            if string_type.enumeration.is_empty() {
                Ok((
                    AnonymousType::Type(Type::Primitive(PrimitiveType::String)),
                    Vec::new(),
                ))
            } else {
                let mut values = Vec::new();
                for enum_ in &string_type.enumeration {
                    values.push(EnumValueDefinition::new(enum_));
                }
                let enum_ = EnumDefinition::builder()
                    .type_name(type_name)
                    .values(values)
                    .build();

                Ok((AnonymousType::Hoist(HoistType::Enum(enum_)), Vec::new()))
            }
        }
        openapiv3::Type::Array(array_info) => {
            let (item_type, hoist) = match &array_info.items {
                None => {
                    // TODO: Handle Any type in arrays
                    return Err(anyhow!("Any type in arrays is not supported.",));
                }
                Some(item_ty) => {
                    let item_name = to_pascal_case(&format!("{}_Item", name));
                    match parse_ref_or_schema(&item_name, item_ty)? {
                        (AnonymousType::Hoist(hoist_type), mut hoist_) => {
                            let ref_ = hoist_type.get_reference();
                            hoist_.push(hoist_type);
                            (ref_, hoist_)
                        }
                        (AnonymousType::Type(type_), hoist_) => (type_, hoist_),
                    }
                }
            };

            if array_info.unique_items {
                Ok((
                    AnonymousType::Type(Type::Set(SetType::new(item_type))),
                    hoist,
                ))
            } else {
                Ok((
                    AnonymousType::Type(Type::List(ListType::new(item_type))),
                    hoist,
                ))
            }
        }
    }
}

fn get_type_name_from_ref_str(ref_str: &str) -> &str {
    let idx = ref_str.rfind('/').unwrap_or(0);
    &ref_str[idx + 1..]
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c);
            }
            capitalize_next = false;
        } else {
            capitalize_next = true;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use conjure_codegen::Config;

    use super::*;

    #[test]
    fn test_pascal_case() {
        let s = "PascalCaseString";
        assert_eq!(to_pascal_case(s), s);
        let s = "hello_world-example string";
        assert_eq!(to_pascal_case(s), "HelloWorldExampleString");
    }

    #[test]
    fn print_info() {
        let data = include_str!("../data/example.yaml");
        let openapi: OpenAPI = serde_yaml::from_str(data).expect("Could not deserialize input");
        println!("{:#?}", &openapi.info);
    }

    #[test]
    fn print_objects() {
        let data = include_str!("../data/example.yaml");
        let openapi: OpenAPI = serde_yaml::from_str(data).expect("Could not deserialize input");
        println!("{:#?}", &openapi.components);
    }

    #[test]
    fn test_parse_types() {
        let data = include_str!("../data/example.yaml");
        let openapi: OpenAPI = serde_yaml::from_str(data).expect("Could not deserialize input");

        let result = parse_types(&openapi).unwrap();
        for item in result {
            println!("{:#?}", item);
        }
    }

    #[test]
    fn test_parse() {
        let config = Config::new();
        config
            .generate_files_inner(
                Path::new("/Volumes/git/conjure-rust/conjure-openapi/data/example.yaml"),
                Path::new("/Volumes/git/conjure-rust/conjure-openapi/out"),
                parse_openapi,
            )
            .unwrap();
    }
}
