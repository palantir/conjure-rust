use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use conjure_codegen::{
    conjure_definition::ConjureDefinition, type_::Type, type_definition::TypeDefinition,
    AliasDefinition, ArgumentDefinition, ArgumentName, AuthType, BodyParameterType, Documentation,
    EndpointDefinition, EndpointName, EnumDefinition, EnumValueDefinition, FieldDefinition,
    FieldName, HeaderAuthType, HeaderParameterType, HttpMethod, HttpPath, ListType, LogSafety,
    ObjectDefinition, OptionalType, ParameterId, ParameterType, PathParameterType, PrimitiveType,
    QueryParameterType, ServiceDefinition, SetType, TypeName, UnionDefinition,
};

use anyhow::{anyhow, Error};
use openapiv3::{
    OpenAPI, Operation, Parameter, ParameterKind, ParameterSchemaOrContent, PathItem, RefOr,
    RequestBody, Responses, Schema, SchemaKind, StatusCode,
};

mod hoist;
mod util;

use hoist::Hoist;
use util::*;

const CONJURE_IR_VERSION: i32 = 1;
const EXT_SAFETY: &str = "x-safety";
const EXT_PACKAGE: &str = "x-package";
const EXT_SERVICE: &str = "x-service";
const EXT_DEFAULT_PACKAGE: &str = "x-default-package";
const EXT_DEFAULT_SERVICE: &str = "x-default-service";

const DEFAULT_PACKAGE_NAME: &str = "default";
const DEFAULT_SERVICE_NAME: &str = "DefaultService";

/// Main entrypoint
pub fn parse_openapi_file(openapi_file: &Path) -> Result<ConjureDefinition, Error> {
    let file = File::open(openapi_file)?;
    let reader = BufReader::new(file);
    let openapi: OpenAPI = serde_yaml::from_reader(reader)?;
    parse_openapi(openapi)
}

pub fn parse_openapi(openapi: OpenAPI) -> Result<ConjureDefinition, Error> {
    let errors = vec![]; // TODO: Parse error types
    let (services, mut hoist) = parse_paths(&openapi)?.explode();
    let types = parse_components(&openapi)?;

    // Append together types defined in the `path` block and all other types in the `schema` block.
    hoist.extend(types);

    let conjure = ConjureDefinition::builder()
        .version(CONJURE_IR_VERSION)
        .errors(errors)
        .services(services)
        .types(hoist.into_iter())
        .build();
    Ok(conjure)
}

/// Entrypoint to parse the `paths:` block of OpenAPI v3 spec.
///
/// *Extensions*
///   - `paths.x-default-package`: Sets the default package for service and all nested schemas. Can be overwritten at the path level.
///   - `paths.x-default-service`: Sets the default service name for all paths. Can be overwritten at the path level.
///   - `paths.{path}.x-service`: Per-path level service name.
///   - `paths.{path}.x-package`: Per-path level package.
/// ```yaml
/// paths:
///   x-default-package: com.palantir.foo
///   x-default-service: FooService
///   /dummy:
///     x-package: com.palantir.bar
///     x-service: BarService
/// ```
fn parse_paths(openapi: &OpenAPI) -> Result<Hoist<impl Iterator<Item = ServiceDefinition>>, Error> {
    let paths = &openapi.paths;

    let default_package =
        get_extension_str(&paths.extensions, EXT_DEFAULT_PACKAGE).unwrap_or(DEFAULT_PACKAGE_NAME);
    let default_service =
        get_extension_str(&paths.extensions, EXT_DEFAULT_SERVICE).unwrap_or(DEFAULT_SERVICE_NAME);

    let mut services: HashMap<TypeName, Vec<EndpointDefinition>> = HashMap::new();
    let mut hoist = Hoist::new(());
    for (name, path_ref) in &paths.paths {
        let hoist_ = parse_ref_or_path(name, path_ref, default_package, default_service)?;
        let (service_name, endpoints) = hoist.extend(hoist_);

        services.entry(service_name).or_default().extend(endpoints);
    }

    let services = services.into_iter().map(|(k, v)| {
        ServiceDefinition::builder()
            .service_name(k)
            .endpoints(v)
            .build()
    });
    Ok(hoist.wrap(services))
}

fn parse_ref_or_path(
    name: &str,
    path_ref: &RefOr<PathItem>,
    default_package: &str,
    default_service: &str,
) -> Result<Hoist<(TypeName, Vec<EndpointDefinition>)>, Error> {
    match path_ref {
        RefOr::Reference { reference } => Err(anyhow!(
            "References are not supported in path schemas. {{ path_reference: {} }}",
            reference
        )),
        RefOr::Item(path) => {
            if path.head.is_some() {
                eprintln!("Path field is not supported. {{ path_field: head }}");
            }
            if path.patch.is_some() {
                eprintln!("Path field is not supported. {{ path_field: patch }}");
            }
            if path.trace.is_some() {
                eprintln!("Path field is not supported. {{ path_field: trace }}");
            }
            if !path.servers.is_empty() {
                eprintln!("Path field is not supported. {{ path_field: servers }}");
            }
            if path.summary.is_some() || path.description.is_some() {
                eprintln!("Path level summary and description are not supported. Put descriptions in Http method level.");
            }

            let http_path = HttpPath::from(name.to_string());
            let service_name = get_extension_str(&path.extensions, EXT_SERVICE)
                .unwrap_or(default_service)
                .to_string();
            let service_package = get_extension_str(&path.extensions, EXT_PACKAGE)
                .unwrap_or(default_package)
                .to_string();

            let mut endpoints = Vec::new();
            let mut hoist = Hoist::new(());
            if let Some(get) = &path.get {
                let hoist_ = parse_operation(get, &http_path, &HttpMethod::Get, &service_package)?;
                let endpoint = hoist.extend(hoist_);
                endpoints.push(endpoint);
            }
            if let Some(post) = &path.post {
                let hoist_ =
                    parse_operation(post, &http_path, &HttpMethod::Post, &service_package)?;
                let endpoint = hoist.extend(hoist_);
                endpoints.push(endpoint);
            }
            if let Some(put) = &path.put {
                let hoist_ = parse_operation(put, &http_path, &HttpMethod::Put, &service_package)?;
                let endpoint = hoist.extend(hoist_);
                endpoints.push(endpoint);
            }
            if let Some(delete) = &path.delete {
                let hoist_ =
                    parse_operation(delete, &http_path, &HttpMethod::Delete, &service_package)?;
                let endpoint = hoist.extend(hoist_);
                endpoints.push(endpoint);
            }

            Ok(hoist.wrap((TypeName::new(service_name, service_package), endpoints)))
        }
    }
}

fn parse_operation(
    operation: &Operation,
    path: &HttpPath,
    method: &HttpMethod,
    package_name: &str,
) -> Result<Hoist<EndpointDefinition>, Error> {
    let endpoint_name = operation
        .operation_id
        .clone()
        .map(EndpointName::from)
        .unwrap_or_else(|| make_endpoint_name(path, method));
    let endpoint_name = EndpointName::from(to_pascal_case(&endpoint_name));
    let docs = operation.description.clone().map(Documentation::from);

    let mut hoist = Hoist::new(());
    let mut args = Vec::new();
    for parameter in &operation.parameters {
        let hoist_ = parse_parameter(parameter, &endpoint_name, package_name)?;
        let arg = hoist.extend(hoist_);
        args.push(arg);
    }

    if let Some(request_body) = &operation.request_body {
        let hoist_ = parse_request_body(&endpoint_name, request_body, package_name)?;
        let arg = hoist.extend(hoist_);
        args.push(arg);
    }

    // Parse out response body. Append hoisted types if we find any
    let response_ = parse_responses(&endpoint_name, &operation.responses, package_name)?;
    let returns = response_.map(|hoist_| hoist.extend(hoist_));

    let endpoint = EndpointDefinition::builder()
        .endpoint_name(endpoint_name)
        .http_method(method.clone())
        .http_path(path.clone())
        .args(args)
        .returns(returns)
        .docs(docs)
        // TODO: Support CookieAuth
        .auth(AuthType::Header(HeaderAuthType::new()))
        .build();

    Ok(hoist.wrap(endpoint))
}

/// Parse OpenAPI Header, Path and Query parameters into Conjure Argument definitions.
/// The process of parsing may generate additional "hoisted" types that we need to pass along.
fn parse_parameter(
    parameter_ref: &RefOr<Parameter>,
    endpoint_name: &EndpointName,
    package_name: &str,
) -> Result<Hoist<ArgumentDefinition>, Error> {
    match parameter_ref {
        RefOr::Reference { reference } => Err(anyhow!(
            "Reference Parameters not supported. {{ parameter_reference: {} }}",
            reference
        )),
        RefOr::Item(parameter) => {
            let arg_name = ArgumentName::from(parameter.name.clone());
            let docs = parameter.description.clone().map(Documentation::from);

            let param_id = ParameterId::from(parameter.name.clone());
            let (param_type, is_optional) = match parameter.kind {
                ParameterKind::Cookie { .. } => {
                    return Err(anyhow!(
                        "Cookie parameters not supported. {{ parameter_name: {} }}",
                        parameter.name
                    ))
                }
                ParameterKind::Header { .. } => (
                    ParameterType::Header(HeaderParameterType::new(param_id)),
                    false,
                ),
                ParameterKind::Path { .. } => {
                    (ParameterType::Path(PathParameterType::new()), false)
                }
                ParameterKind::Query {
                    allow_empty_value, ..
                } => (
                    ParameterType::Query(QueryParameterType::new(param_id)),
                    allow_empty_value.unwrap_or(true),
                ),
            };

            // Recursively resolve the parameter type.
            let (type_, mut hoist) = match &parameter.format {
                ParameterSchemaOrContent::Content(_) => {
                    return Err(anyhow!(
                        "Parameter content field not supported. {{ parameter_name: {} }}",
                        parameter.name
                    ))
                }
                ParameterSchemaOrContent::Schema(schema_ref) => {
                    let type_name = TypeName::new(
                        format!("{}_{}", endpoint_name.0, parameter.name.clone()),
                        package_name,
                    );
                    parse_ref_or_schema(type_name, schema_ref)?.explode()
                }
            };
            hoist.push(&type_);

            let type_ = if is_optional {
                Type::Optional(OptionalType::new(get_type(&type_)))
            } else {
                get_type(&type_)
            };

            let arg_ = ArgumentDefinition::builder()
                .arg_name(arg_name)
                .type_(type_)
                .param_type(param_type)
                .docs(docs)
                .build();
            Ok(hoist.wrap(arg_))
        }
    }
}

fn parse_request_body(
    endpoint_name: &EndpointName,
    body_ref: &RefOr<RequestBody>,
    package_name: &str,
) -> Result<Hoist<ArgumentDefinition>, Error> {
    let param_type = ParameterType::Body(BodyParameterType::new());
    match body_ref {
        RefOr::Reference { reference } => {
            let arg_name = ArgumentName::from("body".to_string());
            let type_ = Type::Reference(TypeName::new(
                get_type_name_from_ref_str(reference),
                package_name,
            ));
            let arg_ = ArgumentDefinition::builder()
                .arg_name(arg_name)
                .type_(type_)
                .param_type(param_type)
                .build();
            Ok(Hoist::new(arg_))
        }
        RefOr::Item(body) => {
            let arg_name = ArgumentName::from("body".to_string());
            let docs = body.description.clone().map(Documentation::from);

            let mut hoist = None;
            for (content_type, media) in &body.content {
                if content_type.to_lowercase() != "application/json" {
                    eprintln!("Content types that are not application/json are not supported. {{content_type: {}}}", content_type);
                    continue;
                }
                // Must be application/json
                let type_name = TypeName::new(format!("{}_Body", endpoint_name.0), package_name);
                hoist = media
                    .schema
                    .as_ref()
                    .map(|schema_ref| parse_ref_or_schema(type_name, schema_ref))
                    .transpose()?;
            }

            let Some(hoist) = hoist else {
                return Err(anyhow!("Expected request body type definition."));
            };
            let (type_, mut hoist) = hoist.explode();
            hoist.push(&type_);

            let type_ = if body.required {
                get_type(&type_)
            } else {
                Type::Optional(OptionalType::new(get_type(&type_)))
            };

            let arg_ = ArgumentDefinition::builder()
                .arg_name(arg_name)
                .type_(type_)
                .param_type(param_type)
                .docs(docs)
                .build();
            Ok(hoist.wrap(arg_))
        }
    }
}

fn parse_responses(
    endpoint_name: &EndpointName,
    responses: &Responses,
    package_name: &str,
) -> Result<Option<Hoist<Type>>, Error> {
    if responses.default.is_some() {
        eprintln!("Response default is not supported.");
    }

    let mut response_ref = None;
    for (response_code, response) in &responses.responses {
        if *response_code != StatusCode::Code(200) {
            eprintln!(
                "Only 200 status code responses are supported. {{status_code: {}}}",
                response_code
            );
            continue;
        }
        response_ref = Some(response);
    }

    // If no "200" is defined we assume there is no response type
    let Some(response_ref) = response_ref else {
        return Ok(None);
    };

    match response_ref {
        RefOr::Reference { reference } => {
            let type_ = Type::Reference(TypeName::new(
                get_type_name_from_ref_str(reference),
                package_name,
            ));
            Ok(Some(Hoist::new(type_)))
        }
        RefOr::Item(response) => {
            let mut hoist = None;
            for (content_type, media) in &response.content {
                if content_type.to_lowercase() != "application/json" {
                    eprintln!("Content types that are not application/json are not supported. {{content_type: {}}}", content_type);
                    continue;
                }
                // Must be application/json
                let type_name =
                    TypeName::new(format!("{}_Response", endpoint_name.0), package_name);
                hoist = media
                    .schema
                    .as_ref()
                    .map(|schema_ref| parse_ref_or_schema(type_name, schema_ref))
                    .transpose()?;
            }

            let Some(hoist) = hoist else {
                return Err(anyhow!("Expected response type definition."));
            };

            let (type_def, mut hoist) = hoist.explode();
            hoist.push(&type_def);
            Ok(Some(hoist.wrap(get_type(&type_def))))
        }
    }
}

fn make_endpoint_name(path: &HttpPath, method: &HttpMethod) -> EndpointName {
    let mut path = path.split('/');
    let mut name = path.next().unwrap_or("default").to_lowercase();
    path.for_each(|s| name.push_str(&capitalize(s)));
    name.push_str(&capitalize(method.as_str()));
    EndpointName(name)
}

/// Entrypoint to parse the `components:` block of OpenAPI v3 spec.
///
/// *Extensions*
///   - `components.x-default-package`: Sets the default package for all nested schemas. Can be overwritten at the path level.
///   - `components.schemas.{type}.x-package`: Per-object level package override.
///   - `components.schemas.{type}.x-safety`: Per-object level safety override. {`safe`|`unsafe`|`do-not-log`}
/// ```yaml
/// components:
///   x-default-package: com.palantir.foo
///   schemas:
///     BazType:
///       type: string
///       x-safety: do-not-log
///       x-package: com.palantir.baz
/// ```
fn parse_components(openapi: &OpenAPI) -> Result<Hoist<()>, Error> {
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

    if components.security_schemes.is_empty() {
        eprintln!("TODO: IMPL SECURITY SCHEMA");
    }

    let default_package = get_extension_str(&components.extensions, EXT_DEFAULT_PACKAGE)
        .unwrap_or(DEFAULT_PACKAGE_NAME);

    let mut hoist = Hoist::new(());
    for (name, schema_ref) in &components.schemas {
        let type_name = TypeName::new(name, default_package);
        let obj_def = parse_ref_or_schema(type_name, schema_ref)?;

        hoist.extend(obj_def.flatten());
    }
    Ok(hoist)
}

fn parse_ref_or_schema(
    type_name: TypeName,
    schema_ref: &RefOr<Schema>,
) -> Result<Hoist<TypeDefinition>, Error> {
    // Normalize type names
    let type_name = TypeName::new(to_pascal_case(type_name.name()), type_name.package());

    match schema_ref {
        RefOr::Reference { reference } => {
            let reference_name_ = get_type_name_from_ref_str(reference);
            let reference_name = TypeName::new(reference_name_, type_name.package());

            Ok(Hoist::new(TypeDefinition::Alias(AliasDefinition::new(
                type_name,
                Type::Reference(reference_name),
            ))))
        }
        RefOr::Item(schema) => {
            let docs = schema.description.clone().map(Documentation::from);
            let safety = get_extension_str(&schema.extensions, EXT_SAFETY)
                .map(str::to_uppercase)
                .and_then(|safety_str| match safety_str.as_str() {
                    "SAFE" => Some(LogSafety::Safe),
                    "UNSAFE" => Some(LogSafety::Unsafe),
                    "DO-NOT-LOG" => Some(LogSafety::DoNotLog),
                    _ => None,
                });

            let package =
                get_extension_str(&schema.extensions, EXT_PACKAGE).unwrap_or(type_name.package());
            let type_name = TypeName::new(type_name.name(), package);

            match &schema.kind {
                SchemaKind::Type(type_) => parse_type(type_name, type_, docs, safety),
                SchemaKind::OneOf { one_of } => {
                    let mut union_builder = UnionDefinition::builder()
                        .type_name(type_name.clone())
                        .docs(docs);

                    let mut temp_hoist = Hoist::new(());
                    for (n, schema_ref) in one_of.iter().enumerate() {
                        let field_name = FieldName::from(format!("variant{}", n));

                        let hoist_name_ = format!("{}_Variant_{}", type_name.name(), n);
                        let hoist_name = TypeName::new(hoist_name_, type_name.package());

                        let hoist_ = parse_ref_or_schema(hoist_name, schema_ref)?;
                        let field_type_def = temp_hoist.extend(hoist_); // Keep all hoisted values
                        temp_hoist.push(&field_type_def); // Also grab a ref to the actual TypeDefinition if it was a compound type

                        let (field_type, field_docs, field_safety) = explode_type(&field_type_def);

                        let field = FieldDefinition::builder()
                            .field_name(field_name)
                            .type_(field_type)
                            .docs(field_docs)
                            .safety(field_safety)
                            .build();
                        union_builder = union_builder.push_union_(field);
                    }

                    let union_ = TypeDefinition::Union(union_builder.build());
                    Ok(temp_hoist.wrap(union_))
                }
                SchemaKind::AllOf { .. } => Err(anyhow!(
                    "SchemaKind::AllOf not supported. {{ type_name: {} }}",
                    type_name.name()
                )),
                SchemaKind::AnyOf { .. } => Err(anyhow!(
                    "SchemaKind::AnyOf not supported. {{ type_name: {} }}",
                    type_name.name()
                )),

                SchemaKind::Not { .. } => Err(anyhow!(
                    "SchemaKind::Not not supported. {{ type_name: {} }}",
                    type_name.name()
                )),
                SchemaKind::Any(_) => Err(anyhow!(
                    "SchemaKind::Any not supported. {{ type_name: {} }}",
                    type_name.name()
                )),
            }
        }
    }
}

/// Parse an OpenAPI type into a conjure TypeDefinition
fn parse_type(
    type_name: TypeName,
    type_: &openapiv3::Type,
    docs: Option<Documentation>,
    safety: Option<LogSafety>,
) -> Result<Hoist<TypeDefinition>, Error> {
    match type_ {
        openapiv3::Type::Object(obj) => {
            let mut obj_builder = ObjectDefinition::builder()
                .type_name(type_name.clone())
                .docs(docs);

            let mut temp_hoist = Hoist::new(());
            let required_props = &obj.required;
            for (field_name, schema_ref) in &obj.properties {
                let field_name_ = FieldName::from(field_name.clone());

                let hoist_name_ = format!("{}_{}", type_name.name(), field_name);
                let hoist_name = TypeName::new(hoist_name_, type_name.package());

                let hoist_ = parse_ref_or_schema(hoist_name, schema_ref)?;
                let field_type_def = temp_hoist.extend(hoist_); // Keep all hoisted values
                temp_hoist.push(&field_type_def); // Also grab a ref to the actual TypeDefinition if it was a compound type

                // Destructure the TypeDefinition from the recursion
                let (field_type, field_docs, field_safety) = explode_type(&field_type_def);

                // If not labeled as required we make it optional
                let field_type = if required_props.contains(field_name) {
                    field_type
                } else {
                    Type::Optional(OptionalType::new(field_type))
                };

                let field = FieldDefinition::builder()
                    .field_name(field_name_)
                    .type_(field_type)
                    .docs(field_docs)
                    .safety(field_safety)
                    .build();
                obj_builder = obj_builder.push_fields(field);
            }

            let obj_ = obj_builder.build();
            let obj = temp_hoist.wrap(TypeDefinition::Object(obj_));
            Ok(obj)
        }
        openapiv3::Type::String(string_type) => {
            // Decide if this string type is an Enum or just a string field
            if string_type.enumeration.is_empty() {
                let alias = TypeDefinition::Alias(
                    AliasDefinition::builder()
                        .type_name(type_name)
                        .alias(Type::Primitive(PrimitiveType::String))
                        .docs(docs)
                        .safety(safety)
                        .build(),
                );
                Ok(Hoist::new(alias))
            } else {
                let mut values = Vec::new();
                for enum_ in &string_type.enumeration {
                    values.push(EnumValueDefinition::new(enum_));
                }
                let enum_ = EnumDefinition::builder()
                    .type_name(type_name)
                    .values(values)
                    .docs(docs)
                    .build();
                Ok(Hoist::new(TypeDefinition::Enum(enum_)))
            }
        }
        openapiv3::Type::Array(array_info) => {
            let item_hoist = match &array_info.items {
                None => {
                    // TODO: Handle Any type in arrays
                    return Err(anyhow!("Any type in arrays is not supported.",));
                }
                Some(item_ty) => {
                    let item_name_ = format!("{}_Item", type_name.name());
                    let item_name = TypeName::new(item_name_, type_name.package());
                    parse_ref_or_schema(item_name, item_ty)?
                }
            };

            let (item_type_def, mut hoist) = item_hoist.explode();
            hoist.push(&item_type_def);

            let item_type = get_type(&item_type_def);
            // Arrays may become set<T> or list<T>
            if array_info.unique_items {
                let set_ = TypeDefinition::Alias(
                    AliasDefinition::builder()
                        .type_name(type_name)
                        .alias(Type::Set(SetType::new(item_type)))
                        .docs(docs)
                        .safety(safety)
                        .build(),
                );
                Ok(hoist.wrap(set_))
            } else {
                let list_ = TypeDefinition::Alias(
                    AliasDefinition::builder()
                        .type_name(type_name)
                        .alias(Type::List(ListType::new(item_type)))
                        .docs(docs)
                        .safety(safety)
                        .build(),
                );
                Ok(hoist.wrap(list_))
            }
        }
        openapiv3::Type::Boolean {} => {
            let type_ = TypeDefinition::Alias(
                AliasDefinition::builder()
                    .type_name(type_name)
                    .alias(Type::Primitive(conjure_codegen::PrimitiveType::Boolean))
                    .docs(docs)
                    .safety(safety)
                    .build(),
            );
            Ok(Hoist::new(type_))
        }
        openapiv3::Type::Integer(_) => {
            let type_ = TypeDefinition::Alias(
                AliasDefinition::builder()
                    .type_name(type_name)
                    .alias(Type::Primitive(conjure_codegen::PrimitiveType::Integer))
                    .docs(docs)
                    .safety(safety)
                    .build(),
            );
            Ok(Hoist::new(type_))
        }
        openapiv3::Type::Number(_) => {
            let type_ = TypeDefinition::Alias(
                AliasDefinition::builder()
                    .type_name(type_name)
                    .alias(Type::Primitive(conjure_codegen::PrimitiveType::Double))
                    .docs(docs)
                    .safety(safety)
                    .build(),
            );
            Ok(Hoist::new(type_))
        }
    }
}

#[cfg(test)]
mod tests {
    use conjure_codegen::Config;

    use super::*;

    #[test]
    fn test_capitalize() {
        let s = "eXampLe";
        assert_eq!(capitalize(s), "Example")
    }

    #[test]
    fn test_make_endpoint_name() {
        let path = HttpPath::from("api/v2/servers".to_string());
        let method = HttpMethod::Post;
        assert_eq!(
            make_endpoint_name(&path, &method),
            EndpointName::from("apiV2ServersPost".to_string())
        )
    }

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
    fn print_paths() {
        let data = include_str!("../data/example.yaml");
        let openapi: OpenAPI = serde_yaml::from_str(data).expect("Could not deserialize input");
        println!("{:#?}", &openapi.paths);
    }

    #[test]
    fn print_objects() {
        let data = include_str!("../data/example.yaml");
        let openapi: OpenAPI = serde_yaml::from_str(data).expect("Could not deserialize input");
        println!("{:#?}", &openapi.components);
    }

    #[test]
    fn test_parse_services() {
        let data = include_str!("../data/example.yaml");
        let openapi: OpenAPI = serde_yaml::from_str(data).expect("Could not deserialize input");

        let (services, hoist) = parse_paths(&openapi).unwrap().explode();
        for item in services {
            println!("{:#?}", item);
        }
        for item in hoist.into_iter() {
            println!("{:#?}", item);
        }
    }

    #[test]
    fn test_parse_types() {
        let data = include_str!("../data/example.yaml");
        let openapi: OpenAPI = serde_yaml::from_str(data).expect("Could not deserialize input");

        let result = parse_components(&openapi).unwrap();
        for item in result.into_iter() {
            println!("{:#?}", item);
        }
    }

    #[test]
    fn test_full_conjure_definition() {
        let definition = parse_openapi_file(Path::new(
            "/Volumes/git/conjure-rust/conjure-openapi/data/example.yaml",
        ))
        .unwrap();
        println!("{:#?}", definition)
    }

    #[test]
    fn test_parse() {
        let config = Config::new();
        config
            .generate_files_inner(
                Path::new("/Volumes/git/conjure-rust/conjure-openapi/data/example.yaml"),
                Path::new("/Volumes/git/conjure-rust/conjure-openapi/out"),
                parse_openapi_file,
            )
            .unwrap();
    }
}
