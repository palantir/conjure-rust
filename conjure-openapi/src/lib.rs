use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use conjure_codegen::{
    conjure_definition::ConjureDefinition, type_::Type, type_definition::TypeDefinition,
    AliasDefinition, ArgumentDefinition, ArgumentName, AuthType, BodyParameterType, Documentation,
    EndpointDefinition, EndpointName, EnumDefinition, EnumValueDefinition, FieldDefinition,
    FieldName, HeaderAuthType, HeaderParameterType, HttpMethod, HttpPath, ListType,
    ObjectDefinition, OptionalType, ParameterId, ParameterType, PathParameterType, PrimitiveType,
    QueryParameterType, ServiceDefinition, SetType, TypeName, UnionDefinition,
};

use anyhow::{anyhow, Error};
use openapiv3::{
    OpenAPI, Operation, Parameter, ParameterKind, ParameterSchemaOrContent, PathItem, RefOr,
    RequestBody, Responses, Schema, SchemaKind, StatusCode,
};

const EXT_SERVICE_NAME: &str = "x-service-name";
const EXT_SAFETY: &str = "x-safety";

pub fn parse_openapi(openapi_file: &Path) -> Result<ConjureDefinition, Error> {
    let file = File::open(openapi_file)?;
    let reader = BufReader::new(file);
    let openapi: OpenAPI = serde_yaml::from_reader(reader)?;

    // let version = parse_version(&openapi);
    let version = 1;
    let errors = vec![]; // TODO: Parse error types
    let (services, hoisted_from_path) = parse_services(&openapi)?;
    let types = parse_types(&openapi)?;

    // Append together types defined in the `path` block and all other types in the `schema` block.
    let all_types = types.chain(
        hoisted_from_path
            .into_iter()
            .map(Composite::into_type_definition),
    );

    let conjure = ConjureDefinition::builder()
        .version(version)
        .errors(errors)
        .services(services)
        .types(all_types)
        .build();
    Ok(conjure)
}

fn parse_services(
    openapi: &OpenAPI,
) -> Result<(impl Iterator<Item = ServiceDefinition>, Vec<Composite>), Error> {
    let paths = &openapi.paths;

    if !paths.extensions.is_empty() {
        eprintln!("OpenAPI path extensions not supported.");
    }

    let mut services: HashMap<TypeName, Vec<EndpointDefinition>> = HashMap::new();
    let mut hoist = Vec::new();
    for (name, path_ref) in &paths.paths {
        let (service, endpoints, hoist_) = parse_ref_or_path(name, path_ref)?;

        let service_name = TypeName::new(service, "default");
        services.entry(service_name).or_default().extend(endpoints);

        hoist.extend(hoist_);
    }

    let services = services.into_iter().map(|(k, v)| {
        ServiceDefinition::builder()
            .service_name(k)
            .endpoints(v)
            .build()
    });
    Ok((services, hoist))
}

fn parse_ref_or_path(
    name: &str,
    path_ref: &RefOr<PathItem>,
) -> Result<(String, Vec<EndpointDefinition>, Vec<Composite>), Error> {
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
            let service_name = path
                .extensions
                .get(EXT_SERVICE_NAME)
                .unwrap_or(&serde_json::Value::String("DefaultService".to_string()))
                .as_str()
                .unwrap()
                .to_string();

            let mut endpoints = Vec::new();
            let mut hoist = Vec::new();
            if let Some(get) = &path.get {
                let (endpoint, hoist_) = parse_operation(get, &http_path, &HttpMethod::Get)?;
                endpoints.push(endpoint);
                hoist.extend(hoist_);
            }
            if let Some(post) = &path.post {
                let (endpoint, hoist_) = parse_operation(post, &http_path, &HttpMethod::Post)?;
                endpoints.push(endpoint);
                hoist.extend(hoist_);
            }
            if let Some(put) = &path.put {
                let (endpoint, hoist_) = parse_operation(put, &http_path, &HttpMethod::Put)?;
                endpoints.push(endpoint);
                hoist.extend(hoist_);
            }
            if let Some(delete) = &path.delete {
                let (endpoint, hoist_) = parse_operation(delete, &http_path, &HttpMethod::Delete)?;
                endpoints.push(endpoint);
                hoist.extend(hoist_);
            }

            Ok((service_name, endpoints, hoist))
        }
    }
}

fn parse_operation(
    operation: &Operation,
    path: &HttpPath,
    method: &HttpMethod,
) -> Result<(EndpointDefinition, Vec<Composite>), Error> {
    let endpoint_name = operation
        .operation_id
        .clone()
        .map(EndpointName::from)
        .unwrap_or_else(|| make_endpoint_name(path, method));
    let docs = operation.description.clone().map(Documentation::from);

    let mut hoist = Vec::new();
    let mut args = Vec::new();
    for parameter in &operation.parameters {
        let (arg, hoist_) = parse_parameter(parameter)?;
        hoist.extend(hoist_);
        args.push(arg);
    }

    if let Some(request_body) = &operation.request_body {
        let (body_arg, hoist_) = parse_body(request_body)?;
        hoist.extend(hoist_);
        args.push(body_arg);
    }

    // Parse out response body. Append hoisted types if we find any
    let response_ = parse_responses(&operation.responses)?;
    let returns = response_.map(|(type_, hoist_)| {
        hoist.extend(hoist_);
        type_
    });

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

    Ok((endpoint, hoist))
}

/// Parse OpenAPI Header, Path and Query parameters into Conjure Argument definitions.
/// The process of parsing may generate additional "hoisted" types that we need to pass along.
fn parse_parameter(
    parameter_ref: &RefOr<Parameter>,
) -> Result<(ArgumentDefinition, Vec<Composite>), Error> {
    match parameter_ref {
        RefOr::Reference { reference } => Err(anyhow!(
            "Reference Parameters not supported. {{ parameter_reference: {} }}",
            reference
        )),
        RefOr::Item(parameter) => {
            let arg_name = ArgumentName::from(parameter.name.clone());
            let docs = parameter.description.clone().map(Documentation::from);

            let param_id = ParameterId::from(parameter.name.clone());
            let param_type = match parameter.kind {
                ParameterKind::Cookie { .. } => {
                    return Err(anyhow!(
                        "Cookie parameters not supported. {{ parameter_name: {} }}",
                        parameter.name
                    ))
                }
                ParameterKind::Header { .. } => {
                    ParameterType::Header(HeaderParameterType::new(param_id))
                }
                ParameterKind::Path { .. } => ParameterType::Path(PathParameterType::new()),
                ParameterKind::Query { .. } => {
                    ParameterType::Query(QueryParameterType::new(param_id))
                }
            };

            // Recursively resolve the parameter type.
            let (type_, hoist) = match &parameter.format {
                ParameterSchemaOrContent::Content(_) => {
                    return Err(anyhow!(
                        "Parameter content field not supported. {{ parameter_name: {} }}",
                        parameter.name
                    ))
                }
                ParameterSchemaOrContent::Schema(schema_ref) => {
                    parse_ref_or_schema(&parameter.name, schema_ref)?.get_type_and_hoist()
                }
            };

            Ok((
                ArgumentDefinition::builder()
                    .arg_name(arg_name)
                    .type_(type_)
                    .param_type(param_type)
                    .docs(docs)
                    .build(),
                hoist,
            ))
        }
    }
}

fn parse_body(
    body_ref: &RefOr<RequestBody>,
) -> Result<(ArgumentDefinition, Vec<Composite>), Error> {
    let param_type = ParameterType::Body(BodyParameterType::new());
    match body_ref {
        RefOr::Reference { reference } => {
            let arg_name = ArgumentName::from("body".to_string());
            let type_ = Type::Reference(TypeName::new(
                get_type_name_from_ref_str(reference),
                "default",
            ));
            Ok((
                ArgumentDefinition::builder()
                    .arg_name(arg_name)
                    .type_(type_)
                    .param_type(param_type)
                    .build(),
                Vec::new(),
            ))
        }
        RefOr::Item(body) => {
            let arg_name = ArgumentName::from("body".to_string());
            let docs = body.description.clone().map(Documentation::from);

            let mut type_ = None;
            for (content_type, media) in &body.content {
                if content_type.to_lowercase() != "application/json" {
                    eprintln!("Content types that are not application/json are not supported. {{content_type: {}}}", content_type);
                    continue;
                }
                // Must be application/json
                type_ = media
                    .schema
                    .as_ref()
                    .map(|schema_ref| parse_ref_or_schema("", schema_ref))
                    .transpose()?;
            }

            let Some(type_) = type_ else {
                return Err(anyhow!("Expected request body type definition."));
            };
            let (type_, hoist) = type_.get_type_and_hoist();

            let type_ = if body.required {
                type_
            } else {
                Type::Optional(OptionalType::new(type_))
            };

            Ok((
                ArgumentDefinition::builder()
                    .arg_name(arg_name)
                    .type_(type_)
                    .param_type(param_type)
                    .docs(docs)
                    .build(),
                hoist,
            ))
        }
    }
}

fn parse_responses(responses: &Responses) -> Result<Option<(Type, Vec<Composite>)>, Error> {
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
                "default",
            ));
            Ok(Some((type_, Vec::new())))
        }
        RefOr::Item(response) => {
            let mut type_ = None;
            for (content_type, media) in &response.content {
                if content_type.to_lowercase() != "application/json" {
                    eprintln!("Content types that are not application/json are not supported. {{content_type: {}}}", content_type);
                    continue;
                }
                // Must be application/json
                type_ = media
                    .schema
                    .as_ref()
                    .map(|schema_ref| parse_ref_or_schema("", schema_ref))
                    .transpose()?;
            }

            let Some(type_) = type_ else {
                return Err(anyhow!("Expected response type definition."));
            };
            let (type_, hoist) = type_.get_type_and_hoist();

            Ok(Some((type_, hoist)))
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

    if components.security_schemes.is_empty() {
        eprintln!("TODO: IMPL SECURITY SCHEMA");
    }

    let mut types = Vec::new();
    for (name, schema_ref) in &components.schemas {
        let mut obj_def = parse_ref_or_schema(name, schema_ref)?;
        for hoist_type in obj_def.take_hoist() {
            types.push(hoist_type.into_type_definition());
        }
        let type_def = obj_def.into_type_definition(name);
        types.push(type_def);
    }

    Ok(types.into_iter())
}

enum Composite {
    Object(ObjectDefinition),
    Enum(EnumDefinition),
    Union(UnionDefinition),
}

impl Composite {
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
            Composite::Object(o) => TypeDefinition::Object(o),
            Composite::Enum(e) => TypeDefinition::Enum(e),
            Composite::Union(u) => TypeDefinition::Union(u),
        }
    }
}

enum TypeOrComposite {
    Type {
        type_: Type,
        hoist: Vec<Composite>,
    },
    Composite {
        type_: Composite,
        hoist: Vec<Composite>,
    },
}

impl TypeOrComposite {
    fn into_type_definition(self, name: &str) -> TypeDefinition {
        match self {
            TypeOrComposite::Type { type_, .. } => {
                let type_name = TypeName::builder().name(name).package("default").build();
                TypeDefinition::Alias(AliasDefinition::new(type_name, type_))
            }
            TypeOrComposite::Composite { type_, .. } => type_.into_type_definition(),
        }
    }

    fn take_hoist(&mut self) -> Vec<Composite> {
        match self {
            TypeOrComposite::Type { hoist, .. } => std::mem::take(hoist),
            TypeOrComposite::Composite { hoist, .. } => std::mem::take(hoist),
        }
    }

    fn get_type_and_hoist(self) -> (Type, Vec<Composite>) {
        match self {
            TypeOrComposite::Type { type_, hoist } => (type_, hoist),
            TypeOrComposite::Composite { type_, mut hoist } => {
                let ref_ = type_.get_reference();
                hoist.push(type_);
                (ref_, hoist)
            }
        }
    }
}

fn parse_ref_or_schema(name: &str, schema_ref: &RefOr<Schema>) -> Result<TypeOrComposite, Error> {
    match schema_ref {
        RefOr::Reference { reference } => {
            let type_name_ = get_type_name_from_ref_str(reference);
            let type_name = TypeName::new(type_name_, "default");
            Ok(TypeOrComposite::Type {
                type_: Type::Reference(type_name),
                hoist: Vec::new(),
            })
        }
        RefOr::Item(schema) => match &schema.kind {
            SchemaKind::Type(type_) => parse_type(
                name,
                type_,
                schema.description.clone().map(Documentation::from),
            ),
            SchemaKind::OneOf { one_of } => {
                let type_name = TypeName::new(name, "default");
                let docs = schema.description.clone().map(Documentation::from);
                let mut union_builder = UnionDefinition::builder().type_name(type_name).docs(docs);

                let mut hoist = Vec::new();
                for (n, schema) in one_of.iter().enumerate() {
                    let hoist_name = to_pascal_case(&format!("{}_Variant_{}", name, n));
                    let field_name = FieldName::from(format!("variant{}", n));

                    let (field_type, hoist_) =
                        parse_ref_or_schema(&hoist_name, schema)?.get_type_and_hoist();
                    hoist.extend(hoist_);

                    let field = FieldDefinition::new(field_name, field_type);
                    union_builder = union_builder.push_union_(field);
                }

                Ok(TypeOrComposite::Composite {
                    type_: Composite::Union(union_builder.build()),
                    hoist,
                })
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
    docs: Option<Documentation>,
) -> Result<TypeOrComposite, Error> {
    let type_name = TypeName::builder().name(name).package("default").build();

    match type_ {
        openapiv3::Type::Object(obj) => {
            let mut obj_builder = ObjectDefinition::builder().type_name(type_name).docs(docs);

            let required_props = &obj.required;
            let mut hoist = Vec::new();
            for (field_name, schema_ref) in &obj.properties {
                let field_name_ = FieldName::from(field_name.clone());
                let hoist_name = to_pascal_case(&format!("{}_{}", name, field_name));

                let (field_type, hoist_) =
                    parse_ref_or_schema(&hoist_name, schema_ref)?.get_type_and_hoist();
                hoist.extend(hoist_);

                // If not labeled as required we make it optional
                let field_type = if required_props.contains(field_name) {
                    field_type
                } else {
                    Type::Optional(OptionalType::new(field_type))
                };

                let field = FieldDefinition::builder()
                    .field_name(field_name_)
                    .type_(field_type)
                    .build();
                obj_builder = obj_builder.push_fields(field);
            }

            let obj_ = obj_builder.build();
            Ok(TypeOrComposite::Composite {
                type_: Composite::Object(obj_),
                hoist,
            })
        }
        openapiv3::Type::String(string_type) => {
            // Decide if this string type is an Enum or just a string field
            if string_type.enumeration.is_empty() {
                Ok(TypeOrComposite::Type {
                    type_: Type::Primitive(PrimitiveType::String),
                    hoist: Vec::new(),
                })
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

                Ok(TypeOrComposite::Composite {
                    type_: Composite::Enum(enum_),
                    hoist: Vec::new(),
                })
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
                    parse_ref_or_schema(&item_name, item_ty)?.get_type_and_hoist()
                }
            };

            // Arrays may become set<T> or list<T>
            if array_info.unique_items {
                Ok(TypeOrComposite::Type {
                    type_: Type::Set(SetType::new(item_type)),
                    hoist,
                })
            } else {
                Ok(TypeOrComposite::Type {
                    type_: Type::List(ListType::new(item_type)),
                    hoist,
                })
            }
        }
        openapiv3::Type::Boolean {} => Ok(TypeOrComposite::Type {
            type_: Type::Primitive(conjure_codegen::PrimitiveType::Boolean),
            hoist: Vec::new(),
        }),
        openapiv3::Type::Integer(_) => Ok(TypeOrComposite::Type {
            type_: Type::Primitive(conjure_codegen::PrimitiveType::Integer),
            hoist: Vec::new(),
        }),
        openapiv3::Type::Number(_) => Ok(TypeOrComposite::Type {
            type_: Type::Primitive(PrimitiveType::Double),
            hoist: Vec::new(),
        }),
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

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
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

        let (result, _types) = parse_services(&openapi).unwrap();
        for item in result {
            println!("{:#?}", item);
        }
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
    fn test_full_conjure_definition() {
        let definition = parse_openapi(Path::new(
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
                parse_openapi,
            )
            .unwrap();
    }
}
