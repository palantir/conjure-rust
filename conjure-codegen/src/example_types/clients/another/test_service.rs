use conjure_http::endpoint;
/// A Markdown description of the service.
#[conjure_http::conjure_client(name = "TestService")]
pub trait TestService<
    #[request_writer]
    O,
    #[response_body]
    I: Iterator<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        >,
> {
    /// Returns a mapping from file system id to backing file system configuration.
    #[endpoint(
        method = GET,
        path = "/catalog/fileSystems",
        name = "getFileSystems",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    fn get_file_systems(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
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
        accept = conjure_http::client::StdResponseDeserializer
    )]
    fn create_dataset(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        request: &super::super::super::objects::product::CreateDatasetRequest,
        #[header(
            name = "Test-Header",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        test_header_arg: &str,
    ) -> Result<
        super::super::super::objects::product::datasets::Dataset,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}",
        name = "getDataset",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    fn get_dataset(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<
        Option<super::super::super::objects::product::datasets::Dataset>,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw",
        name = "getRawData",
        accept = conjure_http::client::conjure::BinaryResponseDeserializer
    )]
    fn get_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<I, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-aliased",
        name = "getAliasedRawData",
        accept = conjure_http::client::conjure::BinaryResponseDeserializer
    )]
    fn get_aliased_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<I, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-maybe",
        name = "maybeGetRawData",
        accept = conjure_http::client::conjure::OptionalBinaryResponseDeserializer
    )]
    fn maybe_get_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<I>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/string-aliased",
        name = "getAliasedString",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    fn get_aliased_string(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<
        super::super::super::objects::product::AliasedString,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw",
        name = "uploadRawData",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    fn upload_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::conjure::BinaryRequestSerializer)]
        input: impl conjure_http::client::WriteBody<O>,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw-aliased",
        name = "uploadAliasedRawData",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    fn upload_aliased_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::conjure::BinaryRequestSerializer)]
        input: impl conjure_http::client::WriteBody<O>,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches",
        name = "getBranches",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    fn get_branches(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    /// Gets all branches of this dataset.
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branchesDeprecated",
        name = "getBranchesDeprecated",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    fn get_branches_deprecated(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches/{branch}/resolve",
        name = "resolveBranch",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    fn resolve_branch(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
        #[path(name = "branch", encoder = conjure_http::client::conjure::PlainEncoder)]
        branch: &str,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/testParam",
        name = "testParam",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    fn test_param(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-query-params",
        name = "testQueryParams",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    fn test_query_params(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        query: &str,
        #[query(
            name = "different",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        something: &conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_middle: Option<&conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        implicit: &conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        set_end: &std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_end: Option<&conjure_object::ResourceIdentifier>,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-no-response-query-params",
        name = "testNoResponseQueryParams",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    fn test_no_response_query_params(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        query: &str,
        #[query(
            name = "different",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        something: &conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_middle: Option<&conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        implicit: &conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        set_end: &std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_end: Option<&conjure_object::ResourceIdentifier>,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/boolean",
        name = "testBoolean",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    fn test_boolean(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/double",
        name = "testDouble",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    fn test_double(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/integer",
        name = "testInteger",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    fn test_integer(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/optional",
        name = "testPostOptional",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    fn test_post_optional(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        maybe_string: Option<&str>,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/optional-integer-double",
        name = "testOptionalIntegerAndDouble",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    fn test_optional_integer_and_double(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[query(
            name = "maybeInteger",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        maybe_integer: Option<i32>,
        #[query(
            name = "maybeDouble",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error>;
}
/// A Markdown description of the service.
#[conjure_http::conjure_client(name = "TestService")]
pub trait AsyncTestService<
    #[request_writer]
    O,
    #[response_body]
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        >,
> {
    /// Returns a mapping from file system id to backing file system configuration.
    #[endpoint(
        method = GET,
        path = "/catalog/fileSystems",
        name = "getFileSystems",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn get_file_systems(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
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
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn create_dataset(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        request: &super::super::super::objects::product::CreateDatasetRequest,
        #[header(
            name = "Test-Header",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        test_header_arg: &str,
    ) -> Result<
        super::super::super::objects::product::datasets::Dataset,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}",
        name = "getDataset",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn get_dataset(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<
        Option<super::super::super::objects::product::datasets::Dataset>,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw",
        name = "getRawData",
        accept = conjure_http::client::conjure::BinaryResponseDeserializer
    )]
    async fn get_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<I, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-aliased",
        name = "getAliasedRawData",
        accept = conjure_http::client::conjure::BinaryResponseDeserializer
    )]
    async fn get_aliased_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<I, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-maybe",
        name = "maybeGetRawData",
        accept = conjure_http::client::conjure::OptionalBinaryResponseDeserializer
    )]
    async fn maybe_get_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<I>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/string-aliased",
        name = "getAliasedString",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn get_aliased_string(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<
        super::super::super::objects::product::AliasedString,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw",
        name = "uploadRawData",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    async fn upload_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::conjure::BinaryRequestSerializer)]
        input: impl conjure_http::client::AsyncWriteBody<O> + Sync + Send,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw-aliased",
        name = "uploadAliasedRawData",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    async fn upload_aliased_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::conjure::BinaryRequestSerializer)]
        input: impl conjure_http::client::AsyncWriteBody<O> + Sync + Send,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches",
        name = "getBranches",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn get_branches(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    /// Gets all branches of this dataset.
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branchesDeprecated",
        name = "getBranchesDeprecated",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn get_branches_deprecated(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches/{branch}/resolve",
        name = "resolveBranch",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn resolve_branch(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
        #[path(name = "branch", encoder = conjure_http::client::conjure::PlainEncoder)]
        branch: &str,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/testParam",
        name = "testParam",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn test_param(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-query-params",
        name = "testQueryParams",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn test_query_params(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        query: &str,
        #[query(
            name = "different",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        something: &conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_middle: Option<&conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        implicit: &conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        set_end: &std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_end: Option<&conjure_object::ResourceIdentifier>,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-no-response-query-params",
        name = "testNoResponseQueryParams",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    async fn test_no_response_query_params(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        query: &str,
        #[query(
            name = "different",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        something: &conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_middle: Option<&conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        implicit: &conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        set_end: &std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_end: Option<&conjure_object::ResourceIdentifier>,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/boolean",
        name = "testBoolean",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn test_boolean(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/double",
        name = "testDouble",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn test_double(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/integer",
        name = "testInteger",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn test_integer(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/optional",
        name = "testPostOptional",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn test_post_optional(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        maybe_string: Option<&str>,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/optional-integer-double",
        name = "testOptionalIntegerAndDouble",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    async fn test_optional_integer_and_double(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[query(
            name = "maybeInteger",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        maybe_integer: Option<i32>,
        #[query(
            name = "maybeDouble",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error>;
}
/// A Markdown description of the service.
#[conjure_http::conjure_client(name = "TestService", local)]
pub trait LocalAsyncTestService<
    #[request_writer]
    O,
    #[response_body]
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        >,
> {
    /// Returns a mapping from file system id to backing file system configuration.
    #[endpoint(
        method = GET,
        path = "/catalog/fileSystems",
        name = "getFileSystems",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn get_file_systems(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
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
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn create_dataset(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        request: &super::super::super::objects::product::CreateDatasetRequest,
        #[header(
            name = "Test-Header",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        test_header_arg: &str,
    ) -> Result<
        super::super::super::objects::product::datasets::Dataset,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}",
        name = "getDataset",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn get_dataset(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<
        Option<super::super::super::objects::product::datasets::Dataset>,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw",
        name = "getRawData",
        accept = conjure_http::client::conjure::BinaryResponseDeserializer
    )]
    async fn get_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<I, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-aliased",
        name = "getAliasedRawData",
        accept = conjure_http::client::conjure::BinaryResponseDeserializer
    )]
    async fn get_aliased_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<I, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/raw-maybe",
        name = "maybeGetRawData",
        accept = conjure_http::client::conjure::OptionalBinaryResponseDeserializer
    )]
    async fn maybe_get_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<I>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/string-aliased",
        name = "getAliasedString",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn get_aliased_string(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<
        super::super::super::objects::product::AliasedString,
        conjure_http::private::Error,
    >;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw",
        name = "uploadRawData",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    async fn upload_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::conjure::BinaryRequestSerializer)]
        input: impl conjure_http::client::LocalAsyncWriteBody<O>,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/datasets/upload-raw-aliased",
        name = "uploadAliasedRawData",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    async fn upload_aliased_raw_data(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::conjure::BinaryRequestSerializer)]
        input: impl conjure_http::client::LocalAsyncWriteBody<O>,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches",
        name = "getBranches",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn get_branches(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    /// Gets all branches of this dataset.
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branchesDeprecated",
        name = "getBranchesDeprecated",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn get_branches_deprecated(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/branches/{branch}/resolve",
        name = "resolveBranch",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn resolve_branch(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
        #[path(name = "branch", encoder = conjure_http::client::conjure::PlainEncoder)]
        branch: &str,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/datasets/{datasetRid}/testParam",
        name = "testParam",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn test_param(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[path(
            name = "datasetRid",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-query-params",
        name = "testQueryParams",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn test_query_params(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        query: &str,
        #[query(
            name = "different",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        something: &conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_middle: Option<&conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        implicit: &conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        set_end: &std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_end: Option<&conjure_object::ResourceIdentifier>,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/test-no-response-query-params",
        name = "testNoResponseQueryParams",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    async fn test_no_response_query_params(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        query: &str,
        #[query(
            name = "different",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        something: &conjure_object::ResourceIdentifier,
        #[query(
            name = "optionalMiddle",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_middle: Option<&conjure_object::ResourceIdentifier>,
        #[query(
            name = "implicit",
            encoder = conjure_http::client::conjure::PlainEncoder
        )]
        implicit: &conjure_object::ResourceIdentifier,
        #[query(
            name = "setEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        set_end: &std::collections::BTreeSet<String>,
        #[query(
            name = "optionalEnd",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        optional_end: Option<&conjure_object::ResourceIdentifier>,
    ) -> Result<(), conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/boolean",
        name = "testBoolean",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn test_boolean(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/double",
        name = "testDouble",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn test_double(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/integer",
        name = "testInteger",
        accept = conjure_http::client::StdResponseDeserializer
    )]
    async fn test_integer(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error>;
    #[endpoint(
        method = POST,
        path = "/catalog/optional",
        name = "testPostOptional",
        accept = conjure_http::client::conjure::CollectionResponseDeserializer
    )]
    async fn test_post_optional(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[body(serializer = conjure_http::client::StdRequestSerializer)]
        maybe_string: Option<&str>,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    #[endpoint(
        method = GET,
        path = "/catalog/optional-integer-double",
        name = "testOptionalIntegerAndDouble",
        accept = conjure_http::client::conjure::EmptyResponseDeserializer
    )]
    async fn test_optional_integer_and_double(
        &self,
        #[auth]
        auth_: &conjure_object::BearerToken,
        #[query(
            name = "maybeInteger",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        maybe_integer: Option<i32>,
        #[query(
            name = "maybeDouble",
            encoder = conjure_http::client::conjure::PlainSeqEncoder
        )]
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error>;
}
