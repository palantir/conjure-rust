use conjure_codegen::{type_::Type, type_definition::TypeDefinition, Documentation, LogSafety};

pub fn explode_type(type_: &TypeDefinition) -> (Type, Option<Documentation>, Option<LogSafety>) {
    (get_type(type_), get_docs(type_), get_safety(type_))
}

pub fn get_type(type_: &TypeDefinition) -> Type {
    match type_ {
        TypeDefinition::Alias(alias) => alias.alias().clone(),
        TypeDefinition::Enum(e) => Type::Reference(e.type_name().clone()),
        TypeDefinition::Object(o) => Type::Reference(o.type_name().clone()),
        TypeDefinition::Union(u) => Type::Reference(u.type_name().clone()),
    }
}

pub fn get_docs(type_: &TypeDefinition) -> Option<Documentation> {
    match type_ {
        TypeDefinition::Alias(alias) => alias.docs().cloned(),
        TypeDefinition::Enum(e) => e.docs().cloned(),
        TypeDefinition::Object(o) => o.docs().cloned(),
        TypeDefinition::Union(u) => u.docs().cloned(),
    }
}

pub fn get_safety(type_: &TypeDefinition) -> Option<LogSafety> {
    match type_ {
        TypeDefinition::Alias(alias) => alias.safety().cloned(),
        _ => None,
    }
}

pub fn get_type_name_from_ref_str(ref_str: &str) -> &str {
    let idx = ref_str.rfind('/').unwrap_or(0);
    &ref_str[idx + 1..]
}

pub fn to_pascal_case(s: &str) -> String {
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

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
    }
}
