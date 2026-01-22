/// The JSON-serializable representation of an error.
#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[serde(crate = "conjure_object::serde")]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct SerializableError {
    #[serde(rename = "errorCode")]
    error_code: crate::ErrorCode,
    #[builder(into)]
    #[serde(rename = "errorName")]
    error_name: String,
    #[serde(
        rename = "errorInstanceId",
        serialize_with = "ser_error_instance_id_as_string",
        deserialize_with = "de_error_instance_id_as_string_or_uuid"
    )]
    error_instance_id: conjure_object::Uuid,
    #[builder(
        default,
        map(
            key(type = String, into),
            value(
                custom(
                    type = impl
                    conjure_object::serde::Serialize,
                    convert = |v|conjure_object::Any::new(
                        v
                    ).expect("value failed to serialize")
                )
            )
        )
    )]
    #[serde(
        rename = "parameters",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    parameters: std::collections::BTreeMap<String, conjure_object::Any>,
}
impl SerializableError {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        error_code: crate::ErrorCode,
        error_name: impl Into<String>,
        error_instance_id: conjure_object::Uuid,
    ) -> Self {
        Self::builder()
            .error_code(error_code)
            .error_name(error_name)
            .error_instance_id(error_instance_id)
            .build()
    }
    /// The broad category of the error.
    ///
    /// When transmitted over HTTP, this determines the response's status code.
    #[inline]
    pub fn error_code(&self) -> &crate::ErrorCode {
        &self.error_code
    }
    /// The error's name.
    ///
    /// The name is made up of a namespace and more specific error name, separated by a `:`.
    #[inline]
    pub fn error_name(&self) -> &str {
        &*self.error_name
    }
    /// A unique identifier for this error instance.
    ///
    /// This can be used to correlate reporting about the error as it transfers between components of a
    /// distributed system.
    #[inline]
    pub fn error_instance_id(&self) -> conjure_object::Uuid {
        self.error_instance_id
    }
    /// Parameters providing more information about the error.
    #[inline]
    pub fn parameters(&self) -> &std::collections::BTreeMap<String, conjure_object::Any> {
        &self.parameters
    }
}

fn ser_error_instance_id_as_string<S: serde::Serializer>(
    error_instance_id: &conjure_object::Uuid,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&error_instance_id.to_string())
}

fn de_error_instance_id_as_string_or_uuid<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<conjure_object::Uuid, D::Error> {
    use serde::de::{Error, Visitor};

    struct ErrorInstanceIdVisitor;

    impl<'de> Visitor<'de> for ErrorInstanceIdVisitor {
        type Value = conjure_object::Uuid;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a UUID as string or binary")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            value
                .parse()
                .map_err(|e| Error::custom(format!("String is not a valid UUID: {e}")))
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if value.len() == 16 {
                conjure_object::Uuid::from_slice(value)
                    .map_err(|e| Error::custom(format!("Bytes are not a valid UUID: {e}")))
            } else {
                let s = std::str::from_utf8(value).map_err(Error::custom)?;
                self.visit_str(s)
            }
        }
    }

    deserializer.deserialize_any(ErrorInstanceIdVisitor)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ErrorType, InvalidArgument};
    use conjure_error::ErrorCode;
    use conjure_object::Any;
    use uuid::Uuid;

    // TODO(#533)
    // This is what the codegen-ed SerializableError will look like once it's fixed to encode
    // errorInstanceId as a String type.
    #[derive(
        Debug,
        Clone,
        conjure_object::serde::Serialize,
        conjure_object::serde::Deserialize,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
    )]
    #[serde(crate = "conjure_object::serde")]
    #[conjure_object::private::staged_builder::staged_builder]
    #[builder(crate = conjure_object::private::staged_builder, update, inline)]
    struct OldSerializableError {
        #[serde(rename = "errorCode")]
        error_code: ErrorCode,
        #[builder(into)]
        #[serde(rename = "errorName")]
        error_name: String,
        #[serde(rename = "errorInstanceId")]
        error_instance_id: Uuid, // Default Uuid serde
        #[builder(
            default,
            map(
                key(type = String, into),
                value(
                    custom(
                    type = impl
                    conjure_object::serde::Serialize,
                    convert = |v|conjure_object::Any::new(
                        v
                    ).expect("value failed to serialize")
                    )
                )
            )
        )]
        #[serde(
            rename = "parameters",
            skip_serializing_if = "std::collections::BTreeMap::is_empty",
            default
        )]
        parameters: std::collections::BTreeMap<String, Any>,
    }

    #[test]
    fn smile_serialize_error_instance_as_string() {
        let error_instance_id = Uuid::new_v4();
        let serialized_err =
            conjure_serde::smile::to_vec(&serializable_error(error_instance_id.clone())).unwrap();
        let error_instance_id_string_bytes = error_instance_id.to_string().into_bytes();
        assert!(serialized_err
            .windows(error_instance_id_string_bytes.len())
            .any(|seq| seq == error_instance_id_string_bytes.as_slice()));
    }

    #[test]
    fn json_serialize_error_instance_as_string() {
        let error_instance_id = Uuid::new_v4();
        let serialized_err =
            conjure_serde::json::to_string(&serializable_error(error_instance_id.clone())).unwrap();
        assert!(serialized_err.contains(error_instance_id.to_string().as_str()));
    }

    // As serialized by older conjure-rust
    #[test]
    fn smile_deserialize_error_instance_id_as_uuid() {
        let error_instance_id = Uuid::new_v4();
        let serialized_err =
            conjure_serde::smile::to_vec(&old_serializable_error(error_instance_id.clone()))
                .unwrap();

        let deserialized_err: SerializableError =
            conjure_serde::smile::client_from_slice(&serialized_err).unwrap();
        assert_eq!(deserialized_err, serializable_error(error_instance_id));
    }

    // As serialized by older conjure-rust
    #[test]
    fn json_deserialize_error_instance_id_as_uuid() {
        let error_instance_id = Uuid::new_v4();
        let serialized_err =
            conjure_serde::json::to_vec(&old_serializable_error(error_instance_id.clone()))
                .unwrap();

        let deserialized_err: SerializableError =
            conjure_serde::json::client_from_slice(&serialized_err).unwrap();
        assert_eq!(deserialized_err, serializable_error(error_instance_id));
    }

    // As serialized by new conjure-rust and conjure-java
    #[test]
    fn smile_deserialize_error_instance_id_as_string() {
        let error_instance_id = Uuid::new_v4();
        let serialized_err =
            conjure_serde::smile::to_vec(&serializable_error(error_instance_id.clone())).unwrap();

        let deserialized_err: SerializableError =
            conjure_serde::smile::client_from_slice(&serialized_err).unwrap();
        assert_eq!(deserialized_err, serializable_error(error_instance_id));
    }

    // As serialized by new conjure-rust and conjure-java
    #[test]
    fn json_deserialize_error_instance_id_as_string() {
        let error_instance_id = Uuid::new_v4();
        let serialized_err =
            conjure_serde::json::to_vec(&serializable_error(error_instance_id.clone())).unwrap();

        let deserialized_err: SerializableError =
            conjure_serde::json::client_from_slice(&serialized_err).unwrap();
        assert_eq!(deserialized_err, serializable_error(error_instance_id));
    }

    fn serializable_error(error_instance_id: Uuid) -> SerializableError {
        SerializableError::builder()
            .error_code(ErrorCode::InvalidArgument)
            .error_name(InvalidArgument::name())
            .error_instance_id(error_instance_id)
            .insert_parameters("param1", "1")
            .insert_parameters("param2", true)
            .build()
    }

    fn old_serializable_error(error_instance_id: Uuid) -> OldSerializableError {
        let err = serializable_error(error_instance_id);
        OldSerializableError::builder()
            .error_code(err.error_code)
            .error_name(err.error_name)
            .error_instance_id(err.error_instance_id)
            .parameters(err.parameters)
            .build()
    }
}
