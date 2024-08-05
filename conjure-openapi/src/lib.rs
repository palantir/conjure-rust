use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use conjure_codegen::{
    conjure_definition::ConjureDefinition, type_::Type, type_definition::TypeDefinition,
    AliasDefinition, FieldDefinition, FieldName, ListType, ObjectDefinition, OptionalType,
    PrimitiveType, ServiceDefinition, SetType, TypeName,
};

use anyhow::{anyhow, Error};
use openapiv3::{AdditionalProperties, OpenAPI, RefOr, Schema, SchemaKind};

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

    let mut schema_map = HashMap::new();
    for (name, schema_ref) in &components.schemas {
        println!("{:#?}", name);
        let schema = schema_ref.resolve(openapi); // Resolve here because all top-level types are resolved
        let obj_def = parse_schema_into_type_definition(name, schema)?;
        let obj_name = get_type_name_from_type(&obj_def);
        schema_map.insert(obj_name, obj_def);
    }
    println!("{:#?}", schema_map);

    let types = schema_map.values().cloned().collect::<Vec<_>>();
    Ok(types.into_iter())
}

fn parse_schema_into_type_definition(
    type_name: &str,
    schema: &Schema,
) -> Result<TypeDefinition, Error> {
    let conjure_type = match &schema.kind {
        SchemaKind::Type(type_) => parse_type_definition(type_name, type_)?,
        SchemaKind::AllOf { .. } => {
            return Err(anyhow!(
                "SchemaKind::AllOf not supported. {{ type_name: {} }}",
                type_name
            ))
        }
        SchemaKind::AnyOf { .. } => {
            return Err(anyhow!(
                "SchemaKind::AnyOf not supported. {{ type_name: {} }}",
                type_name
            ))
        }
        SchemaKind::OneOf { .. } => {
            return Err(anyhow!(
                "SchemaKind::OneOf not supported. {{ type_name: {} }}",
                type_name
            ))
        }
        SchemaKind::Not { .. } => {
            return Err(anyhow!(
                "SchemaKind::Not not supported. {{ type_name: {} }}",
                type_name
            ))
        }
        SchemaKind::Any(_) => {
            return Err(anyhow!(
                "SchemaKind::Any not supported. {{ type_name: {} }}",
                type_name
            ))
        }
    };

    Ok(conjure_type)
}

fn parse_type_definition(
    type_name: &str,
    type_: &openapiv3::Type,
) -> Result<TypeDefinition, Error> {
    let type_name_ = TypeName::builder()
        .name(type_name)
        .package("default")
        .build();

    match type_ {
        openapiv3::Type::Object(obj) => {
            let required_props = &obj.required;
            let mut obj_builder = ObjectDefinition::builder().type_name(type_name_);

            for (field_name, schema_ref) in &obj.properties {
                let field_type = parse_schema_to_type(schema_ref)?;
                let field_name_ = FieldName(field_name.to_string());

                // If not labeled as required we make it optional
                let field_type = if required_props.contains(field_name) {
                    field_type
                } else {
                    Type::Optional(OptionalType::new(field_type))
                };

                let field = FieldDefinition::new(field_name_, field_type);
                obj_builder = obj_builder.push_fields(field);
            }

            Ok(TypeDefinition::Object(obj_builder.build()))
        }
        _ => Ok(TypeDefinition::Alias(AliasDefinition::new(
            type_name_,
            parse_type_to_type(type_)?,
        ))),
    }
}

fn parse_schema_to_type(schema: &RefOr<Schema>) -> Result<Type, Error> {
    if let Some(schema) = schema.as_item() {
        let type_ = match &schema.kind {
            SchemaKind::Type(type_) => parse_type_to_type(type_)?,
            _ => {
                return Err(anyhow!(
                    "Expected OpenApi SchemaKind::Type to parse into Conjure Type."
                ));
            }
        };
        Ok(type_)
    } else if let Some(schema_ref) = schema.as_ref_str() {
        let type_name_ = get_type_name_from_ref_str(schema_ref);
        let type_name = TypeName::new(type_name_, "default");
        Ok(Type::Reference(type_name))
    } else {
        unreachable!()
    }
}

fn parse_type_to_type(type_: &openapiv3::Type) -> Result<Type, Error> {
    let conjure_type = match type_ {
        openapiv3::Type::Boolean {} => Type::Primitive(conjure_codegen::PrimitiveType::Boolean),
        openapiv3::Type::Integer(_) => Type::Primitive(conjure_codegen::PrimitiveType::Integer),
        openapiv3::Type::Number(_) => Type::Primitive(PrimitiveType::Double),
        openapiv3::Type::String(_) => Type::Primitive(PrimitiveType::String),
        openapiv3::Type::Array(array_info) => {
            let item_type = array_info
                .items
                .as_ref()
                .map(|item_type| parse_schema_to_type(item_type))
                .transpose()?;

            let Some(item_type) = item_type else {
                return Err(anyhow!("Empty array item type."));
            };

            if array_info.unique_items {
                Type::Set(SetType::new(item_type))
            } else {
                Type::List(ListType::new(item_type))
            }
        }
        openapiv3::Type::Object(_obj) => {
            let Some(additional_props) = &_obj.additional_properties else {
                return Err(anyhow!(
                    "Can't parse openapiv3::Type::ObjectType into a Conjure Type."
                ));
            };

            match additional_props {
                AdditionalProperties::Any(_) => {
                    return Err(anyhow!(
                        "Can't parse openapiv3::Type::ObjectType into a Conjure Type."
                    ))
                }
                AdditionalProperties::Schema(schema) => return parse_schema_to_type(schema),
            }
        }
    };
    Ok(conjure_type)
}

fn get_type_name_from_type(type_def: &TypeDefinition) -> TypeName {
    let type_name = match type_def {
        TypeDefinition::Alias(alias_def) => alias_def.type_name(),
        TypeDefinition::Enum(enum_def) => enum_def.type_name(),
        TypeDefinition::Object(obj_def) => obj_def.type_name(),
        TypeDefinition::Union(union_def) => union_def.type_name(),
    };
    type_name.to_owned()
}

fn get_type_name_from_ref_str(ref_str: &str) -> &str {
    let idx = ref_str.rfind('/').unwrap_or(0);
    &ref_str[idx + 1..]
}

#[cfg(test)]
mod tests {
    use conjure_codegen::Config;

    use super::*;

    #[test]
    fn print_info() {
        let data = include_str!("../data/umi.yaml");
        let openapi: OpenAPI = serde_yaml::from_str(data).expect("Could not deserialize input");
        println!("{:#?}", &openapi.info);
    }

    #[test]
    fn print_objects() {
        let data = include_str!("../data/umi.yaml");
        let openapi: OpenAPI = serde_yaml::from_str(data).expect("Could not deserialize input");
        println!("{:#?}", &openapi.components);
    }

    #[test]
    fn test_parse_types() {
        let data = include_str!("../data/umi.yaml");
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
                Path::new("/Volumes/git/conjure-rust/conjure-openapi/data/umi.yaml"),
                Path::new("/Volumes/git/conjure-rust/conjure-openapi/out"),
                parse_openapi,
            )
            .unwrap();
    }
}
