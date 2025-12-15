use conjure_http::endpoint;
/// A Markdown description of the service.
#[conjure_http::conjure_endpoints(name = "TestService", use_legacy_error_serialization)]
pub trait TestService<#[request_body] I, #[response_writer] O> {
    ///The body type returned by the `get_raw_data` method.
    type GetRawDataBody: conjure_http::server::WriteBody<O> + 'static;
    ///The body type returned by the `get_aliased_raw_data` method.
    type GetAliasedRawDataBody: conjure_http::server::WriteBody<O> + 'static;
    ///The body type returned by the `maybe_get_raw_data` method.
    type MaybeGetRawDataBody: conjure_http::server::WriteBody<O> + 'static;
    /// Returns a mapping from file system id to backing file system configuration.
    #[endpoint(
        method = GET,
        path = "/catalog/fileSystems",
        name = "getFileSystems",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    fn get_file_systems(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<
            String,
            super::super::super::objects::product::datasets::BackingFileSystem,
        >,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets",
        name = "createDataset",
        produces = conjure_http::server::StdResponseSerializer
    )]
    fn create_dataset(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::StdRequestDeserializer)]
        request: super::super::super::objects::product::CreateDatasetRequest,
        #[header(
            name = "Test-Header",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "testHeaderArg"
        )]
        test_header_arg: String,
    ) -> Result<
        super::super::super::objects::product::datasets::Dataset,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}",
        name = "getDataset",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    fn get_dataset(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<
        Option<super::super::super::objects::product::datasets::Dataset>,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw",
        name = "getRawData",
        produces = conjure_http::server::conjure::BinaryResponseSerializer
    )]
    fn get_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Self::GetRawDataBody, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-aliased",
        name = "getAliasedRawData",
        produces = conjure_http::server::conjure::BinaryResponseSerializer
    )]
    fn get_aliased_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Self::GetAliasedRawDataBody, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-maybe",
        name = "maybeGetRawData",
        produces = conjure_http::server::conjure::OptionalBinaryResponseSerializer
    )]
    fn maybe_get_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<Self::MaybeGetRawDataBody>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/string-aliased",
        name = "getAliasedString",
        produces = conjure_http::server::StdResponseSerializer
    )]
    fn get_aliased_string(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<
        super::super::super::objects::product::AliasedString,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw",
        name = "uploadRawData"
    )]
    fn upload_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::conjure::BinaryRequestDeserializer)]
        input: I,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw-aliased",
        name = "uploadAliasedRawData"
    )]
    fn upload_aliased_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::conjure::BinaryRequestDeserializer)]
        input: I,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches",
        name = "getBranches",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    fn get_branches(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    /// Gets all branches of this dataset.
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branchesDeprecated",
        name = "getBranchesDeprecated",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    fn get_branches_deprecated(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
        name = "resolveBranch",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    fn resolve_branch(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
        #[path(
            name = "branch",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        branch: String,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/testParam",
        name = "testParam",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    fn test_param(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-query-params",
        name = "testQueryParams",
        produces = conjure_http::server::StdResponseSerializer
    )]
    fn test_query_params(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::StdRequestDeserializer)]
        query: String,
        #[query(
            name = "different",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        something: conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalMiddle"
        )]
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        implicit: conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            decoder = conjure_http::server::conjure::FromPlainSeqDecoder<_>,
            log_as = "setEnd"
        )]
        set_end: std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalEnd"
        )]
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-no-response-query-params",
        name = "testNoResponseQueryParams"
    )]
    fn test_no_response_query_params(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::StdRequestDeserializer)]
        query: String,
        #[query(
            name = "different",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        something: conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalMiddle"
        )]
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        implicit: conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            decoder = conjure_http::server::conjure::FromPlainSeqDecoder<_>,
            log_as = "setEnd"
        )]
        set_end: std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalEnd"
        )]
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/boolean",
        name = "testBoolean",
        produces = conjure_http::server::StdResponseSerializer
    )]
    fn test_boolean(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/double",
        name = "testDouble",
        produces = conjure_http::server::StdResponseSerializer
    )]
    fn test_double(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/integer",
        name = "testInteger",
        produces = conjure_http::server::StdResponseSerializer
    )]
    fn test_integer(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/optional",
        name = "testPostOptional",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    fn test_post_optional(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(
            deserializer = conjure_http::server::conjure::OptionalRequestDeserializer,
            log_as = "maybeString"
        )]
        maybe_string: Option<String>,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/optional-integer-double",
        name = "testOptionalIntegerAndDouble"
    )]
    fn test_optional_integer_and_double(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[query(
            name = "maybeInteger",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "maybeInteger"
        )]
        maybe_integer: Option<i32>,
        #[query(
            name = "maybeDouble",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "maybeDouble"
        )]
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error>;
}
/// A Markdown description of the service.
#[conjure_http::conjure_endpoints(name = "TestService", use_legacy_error_serialization)]
pub trait AsyncTestService<#[request_body] I, #[response_writer] O> {
    ///The body type returned by the `get_raw_data` method.
    type GetRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    ///The body type returned by the `get_aliased_raw_data` method.
    type GetAliasedRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    ///The body type returned by the `maybe_get_raw_data` method.
    type MaybeGetRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    /// Returns a mapping from file system id to backing file system configuration.
    #[endpoint(
        method = GET,
        path = "/catalog/fileSystems",
        name = "getFileSystems",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn get_file_systems(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<
            String,
            super::super::super::objects::product::datasets::BackingFileSystem,
        >,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets",
        name = "createDataset",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn create_dataset(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::StdRequestDeserializer)]
        request: super::super::super::objects::product::CreateDatasetRequest,
        #[header(
            name = "Test-Header",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "testHeaderArg"
        )]
        test_header_arg: String,
    ) -> Result<
        super::super::super::objects::product::datasets::Dataset,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}",
        name = "getDataset",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn get_dataset(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<
        Option<super::super::super::objects::product::datasets::Dataset>,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw",
        name = "getRawData",
        produces = conjure_http::server::conjure::BinaryResponseSerializer
    )]
    async fn get_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Self::GetRawDataBody, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-aliased",
        name = "getAliasedRawData",
        produces = conjure_http::server::conjure::BinaryResponseSerializer
    )]
    async fn get_aliased_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Self::GetAliasedRawDataBody, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-maybe",
        name = "maybeGetRawData",
        produces = conjure_http::server::conjure::OptionalBinaryResponseSerializer
    )]
    async fn maybe_get_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<Self::MaybeGetRawDataBody>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/string-aliased",
        name = "getAliasedString",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn get_aliased_string(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<
        super::super::super::objects::product::AliasedString,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw",
        name = "uploadRawData"
    )]
    async fn upload_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::conjure::BinaryRequestDeserializer)]
        input: I,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw-aliased",
        name = "uploadAliasedRawData"
    )]
    async fn upload_aliased_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::conjure::BinaryRequestDeserializer)]
        input: I,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches",
        name = "getBranches",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn get_branches(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    /// Gets all branches of this dataset.
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branchesDeprecated",
        name = "getBranchesDeprecated",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn get_branches_deprecated(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
        name = "resolveBranch",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn resolve_branch(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
        #[path(
            name = "branch",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        branch: String,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/testParam",
        name = "testParam",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn test_param(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-query-params",
        name = "testQueryParams",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn test_query_params(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::StdRequestDeserializer)]
        query: String,
        #[query(
            name = "different",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        something: conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalMiddle"
        )]
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        implicit: conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            decoder = conjure_http::server::conjure::FromPlainSeqDecoder<_>,
            log_as = "setEnd"
        )]
        set_end: std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalEnd"
        )]
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-no-response-query-params",
        name = "testNoResponseQueryParams"
    )]
    async fn test_no_response_query_params(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::StdRequestDeserializer)]
        query: String,
        #[query(
            name = "different",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        something: conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalMiddle"
        )]
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        implicit: conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            decoder = conjure_http::server::conjure::FromPlainSeqDecoder<_>,
            log_as = "setEnd"
        )]
        set_end: std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalEnd"
        )]
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/boolean",
        name = "testBoolean",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn test_boolean(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/double",
        name = "testDouble",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn test_double(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/integer",
        name = "testInteger",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn test_integer(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/optional",
        name = "testPostOptional",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn test_post_optional(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(
            deserializer = conjure_http::server::conjure::OptionalRequestDeserializer,
            log_as = "maybeString"
        )]
        maybe_string: Option<String>,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/optional-integer-double",
        name = "testOptionalIntegerAndDouble"
    )]
    async fn test_optional_integer_and_double(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[query(
            name = "maybeInteger",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "maybeInteger"
        )]
        maybe_integer: Option<i32>,
        #[query(
            name = "maybeDouble",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "maybeDouble"
        )]
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error>;
}
/// A Markdown description of the service.
#[conjure_http::conjure_endpoints(
    name = "TestService",
    use_legacy_error_serialization,
    local
)]
pub trait LocalAsyncTestService<#[request_body] I, #[response_writer] O> {
    ///The body type returned by the `get_raw_data` method.
    type GetRawDataBody: conjure_http::server::LocalAsyncWriteBody<O> + 'static;
    ///The body type returned by the `get_aliased_raw_data` method.
    type GetAliasedRawDataBody: conjure_http::server::LocalAsyncWriteBody<O> + 'static;
    ///The body type returned by the `maybe_get_raw_data` method.
    type MaybeGetRawDataBody: conjure_http::server::LocalAsyncWriteBody<O> + 'static;
    /// Returns a mapping from file system id to backing file system configuration.
    #[endpoint(
        method = GET,
        path = "/catalog/fileSystems",
        name = "getFileSystems",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn get_file_systems(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<
            String,
            super::super::super::objects::product::datasets::BackingFileSystem,
        >,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets",
        name = "createDataset",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn create_dataset(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::StdRequestDeserializer)]
        request: super::super::super::objects::product::CreateDatasetRequest,
        #[header(
            name = "Test-Header",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "testHeaderArg"
        )]
        test_header_arg: String,
    ) -> Result<
        super::super::super::objects::product::datasets::Dataset,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}",
        name = "getDataset",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn get_dataset(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<
        Option<super::super::super::objects::product::datasets::Dataset>,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw",
        name = "getRawData",
        produces = conjure_http::server::conjure::BinaryResponseSerializer
    )]
    async fn get_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Self::GetRawDataBody, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-aliased",
        name = "getAliasedRawData",
        produces = conjure_http::server::conjure::BinaryResponseSerializer
    )]
    async fn get_aliased_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Self::GetAliasedRawDataBody, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-maybe",
        name = "maybeGetRawData",
        produces = conjure_http::server::conjure::OptionalBinaryResponseSerializer
    )]
    async fn maybe_get_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<Self::MaybeGetRawDataBody>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/string-aliased",
        name = "getAliasedString",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn get_aliased_string(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<
        super::super::super::objects::product::AliasedString,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw",
        name = "uploadRawData"
    )]
    async fn upload_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::conjure::BinaryRequestDeserializer)]
        input: I,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw-aliased",
        name = "uploadAliasedRawData"
    )]
    async fn upload_aliased_raw_data(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::conjure::BinaryRequestDeserializer)]
        input: I,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches",
        name = "getBranches",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn get_branches(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    /// Gets all branches of this dataset.
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branchesDeprecated",
        name = "getBranchesDeprecated",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn get_branches_deprecated(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
        name = "resolveBranch",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn resolve_branch(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
        #[path(
            name = "branch",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        branch: String,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/testParam",
        name = "testParam",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn test_param(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "datasetRid"
        )]
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-query-params",
        name = "testQueryParams",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn test_query_params(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::StdRequestDeserializer)]
        query: String,
        #[query(
            name = "different",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        something: conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalMiddle"
        )]
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        implicit: conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            decoder = conjure_http::server::conjure::FromPlainSeqDecoder<_>,
            log_as = "setEnd"
        )]
        set_end: std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalEnd"
        )]
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-no-response-query-params",
        name = "testNoResponseQueryParams"
    )]
    async fn test_no_response_query_params(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(deserializer = conjure_http::server::StdRequestDeserializer)]
        query: String,
        #[query(
            name = "different",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        something: conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalMiddle"
        )]
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            decoder = conjure_http::server::conjure::FromPlainDecoder
        )]
        implicit: conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            decoder = conjure_http::server::conjure::FromPlainSeqDecoder<_>,
            log_as = "setEnd"
        )]
        set_end: std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "optionalEnd"
        )]
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/boolean",
        name = "testBoolean",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn test_boolean(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/double",
        name = "testDouble",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn test_double(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/integer",
        name = "testInteger",
        produces = conjure_http::server::StdResponseSerializer
    )]
    async fn test_integer(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/optional",
        name = "testPostOptional",
        produces = conjure_http::server::conjure::CollectionResponseSerializer
    )]
    async fn test_post_optional(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[body(
            deserializer = conjure_http::server::conjure::OptionalRequestDeserializer,
            log_as = "maybeString"
        )]
        maybe_string: Option<String>,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/optional-integer-double",
        name = "testOptionalIntegerAndDouble"
    )]
    async fn test_optional_integer_and_double(
        &self,
        #[auth]
        auth_: conjure_object::BearerToken,
        #[query(
            name = "maybeInteger",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "maybeInteger"
        )]
        maybe_integer: Option<i32>,
        #[query(
            name = "maybeDouble",
            decoder = conjure_http::server::conjure::FromPlainOptionDecoder,
            log_as = "maybeDouble"
        )]
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error>;
}
