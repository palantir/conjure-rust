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
        deserialize_with = "error_instance_id_as_string_or_uuid"
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

fn error_instance_id_as_string_or_uuid<'de, D: serde::Deserializer<'de>>(
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
    struct SerializableErrorWithStringErrorInstanceId {
        #[serde(rename = "errorCode")]
        error_code: ErrorCode,
        #[builder(into)]
        #[serde(rename = "errorName")]
        error_name: String,
        #[serde(rename = "errorInstanceId")]
        error_instance_id: String, // Not UUID
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
    fn smile_deserialize_error_instance_id_as_uuid() {
        let error_instance_id = Uuid::new_v4();
        let serialized_err =
            serde_smile::to_vec(&with_uuid_error_instance_id(error_instance_id.clone())).unwrap();

        let deserialized_err: SerializableError = serde_smile::from_slice(&serialized_err).unwrap();
        assert_eq!(
            deserialized_err,
            with_uuid_error_instance_id(error_instance_id)
        );
    }

    #[test]
    fn json_deserialize_error_instance_id_as_uuid() {
        let error_instance_id = Uuid::new_v4();
        let serialized_err =
            serde_json::to_vec(&with_uuid_error_instance_id(error_instance_id.clone())).unwrap();

        let deserialized_err: SerializableError = serde_json::from_slice(&serialized_err).unwrap();
        assert_eq!(
            deserialized_err,
            with_uuid_error_instance_id(error_instance_id)
        );
    }

    #[test]
    fn smile_deserialize_error_instance_id_as_string() {
        let error_instance_id = Uuid::new_v4();
        let serialized_err = serde_smile::to_vec(&with_string_error_instance_id(
            error_instance_id.to_string(),
        ))
        .unwrap();

        let deserialized_err: SerializableError = serde_smile::from_slice(&serialized_err).unwrap();
        assert_eq!(
            deserialized_err,
            with_uuid_error_instance_id(error_instance_id)
        );
    }

    #[test]
    fn json_deserialize_error_instance_id_as_string() {
        let error_instance_id = Uuid::new_v4();
        let serialized_err = serde_json::to_vec(&with_string_error_instance_id(
            error_instance_id.to_string(),
        ))
        .unwrap();

        let deserialized_err: SerializableError = serde_json::from_slice(&serialized_err).unwrap();
        assert_eq!(
            deserialized_err,
            with_uuid_error_instance_id(error_instance_id)
        );
    }

    #[test]
    fn smile_deserialize_error_instance_id_as_invalid_string() {
        let serialized_err =
            serde_smile::to_vec(&with_string_error_instance_id("0A-not a uuid".to_string()))
                .unwrap();

        let res: Result<SerializableError, _> = serde_smile::from_slice(&serialized_err);
        assert_eq!(
            res.unwrap_err().to_string(),
            "String is not a valid UUID: invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 4"
        );
    }

    #[test]
    fn json_deserialize_error_instance_id_as_invalid_string() {
        let serialized_err =
            serde_json::to_vec(&with_string_error_instance_id("0A-not a uuid".to_string()))
                .unwrap();

        let res: Result<SerializableError, _> = serde_json::from_slice(&serialized_err);
        assert_eq!(
            res.unwrap_err().to_string(),
            "String is not a valid UUID: invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 4 at line 1 column 103"
        );
    }

    fn with_uuid_error_instance_id(error_instance_id: Uuid) -> SerializableError {
        SerializableError::builder()
            .error_code(ErrorCode::InvalidArgument)
            .error_name(InvalidArgument::name())
            .error_instance_id(error_instance_id)
            .insert_parameters("param1", "1")
            .insert_parameters("param2", true)
            .build()
    }

    fn with_string_error_instance_id(
        error_instance_id: String,
    ) -> SerializableErrorWithStringErrorInstanceId {
        // Generate the template error, but then create our custom type with the string UUID
        let err = with_uuid_error_instance_id(Uuid::new_v4());
        SerializableErrorWithStringErrorInstanceId::builder()
            .error_code(err.error_code)
            .error_name(err.error_name)
            .error_instance_id(error_instance_id)
            .parameters(err.parameters)
            .build()
    }
}
