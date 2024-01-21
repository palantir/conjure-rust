///A Markdown description of the service.
#[derive(Clone, Debug)]
pub struct TestServiceAsyncClient<T>(T);
impl<T> conjure_http::client::AsyncService<T> for TestServiceAsyncClient<T>
where
    T: conjure_http::client::AsyncClient,
{
    fn new(client: T) -> Self {
        TestServiceAsyncClient(client)
    }
}
impl<T> TestServiceAsyncClient<T>
where
    T: conjure_http::client::AsyncClient,
{
    ///Returns a mapping from file system id to backing file system configuration.
    pub async fn get_file_systems(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<
            String,
            super::super::product::datasets::BackingFileSystem,
        >,
        conjure_http::private::Error,
    > {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/fileSystems");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getFileSystems",
                    "/catalog/fileSystems",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_)
            .await
    }
    pub async fn create_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        request: &super::super::product::CreateDatasetRequest,
        test_header_arg: &str,
    ) -> Result<super::super::product::datasets::Dataset, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_serializable_request(
            &request,
        );
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_header(
            &mut request_,
            "test-header",
            &test_header_arg,
        )?;
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "createDataset",
                    "/catalog/datasets",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn get_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<
        Option<super::super::product::datasets::Dataset>,
        conjure_http::private::Error,
    > {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getDataset",
                    "/catalog/datasets/{datasetRid}",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_)
            .await
    }
    pub async fn get_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<T::ResponseBody, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/raw");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_binary_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getRawData",
                    "/catalog/datasets/{datasetRid}/raw",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::decode_binary_response(response_)
    }
    pub async fn get_aliased_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<T::ResponseBody, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/raw-aliased");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_binary_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getAliasedRawData",
                    "/catalog/datasets/{datasetRid}/raw-aliased",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::decode_binary_response(response_)
    }
    pub async fn maybe_get_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<T::ResponseBody>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/raw-maybe");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_binary_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "maybeGetRawData",
                    "/catalog/datasets/{datasetRid}/raw-maybe",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::decode_optional_binary_response(response_)
    }
    pub async fn get_aliased_string(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<super::super::product::AliasedString, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/string-aliased");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getAliasedString",
                    "/catalog/datasets/{datasetRid}/string-aliased",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn upload_raw_data<U>(
        &self,
        auth_: &conjure_object::BearerToken,
        input: U,
    ) -> Result<(), conjure_http::private::Error>
    where
        U: conjure_http::client::AsyncWriteBody<T::BodyWriter> + Sync + Send,
    {
        let mut request_ = conjure_http::private::async_encode_binary_request(input);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets/upload-raw");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "uploadRawData",
                    "/catalog/datasets/upload-raw",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_empty_response(response_).await
    }
    pub async fn upload_aliased_raw_data<U>(
        &self,
        auth_: &conjure_object::BearerToken,
        input: U,
    ) -> Result<(), conjure_http::private::Error>
    where
        U: conjure_http::client::AsyncWriteBody<T::BodyWriter> + Sync + Send,
    {
        let mut request_ = conjure_http::private::async_encode_binary_request(input);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets/upload-raw-aliased");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "uploadAliasedRawData",
                    "/catalog/datasets/upload-raw-aliased",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_empty_response(response_).await
    }
    pub async fn get_branches(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/branches");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getBranches",
                    "/catalog/datasets/{datasetRid}/branches",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_)
            .await
    }
    ///Gets all branches of this dataset.
    #[deprecated(note = "use getBranches instead")]
    pub async fn get_branches_deprecated(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/branchesDeprecated");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getBranchesDeprecated",
                    "/catalog/datasets/{datasetRid}/branchesDeprecated",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_)
            .await
    }
    pub async fn resolve_branch(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
        branch: &str,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/branches");
        path_.push_path_parameter(&branch);
        path_.push_literal("/resolve");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "resolveBranch",
                    "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_)
            .await
    }
    pub async fn test_param(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/testParam");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testParam",
                    "/catalog/datasets/{datasetRid}/testParam",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_)
            .await
    }
    pub async fn test_query_params(
        &self,
        auth_: &conjure_object::BearerToken,
        query: &str,
        something: &conjure_object::ResourceIdentifier,
        optional_middle: Option<&conjure_object::ResourceIdentifier>,
        implicit: &conjure_object::ResourceIdentifier,
        set_end: &std::collections::BTreeSet<String>,
        optional_end: Option<&conjure_object::ResourceIdentifier>,
    ) -> Result<i32, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_serializable_request(
            &query,
        );
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/test-query-params");
        path_.push_query_parameter("different", &something);
        path_.push_optional_query_parameter("optionalMiddle", &optional_middle);
        path_.push_query_parameter("implicit", &implicit);
        path_.push_set_query_parameter("setEnd", &set_end);
        path_.push_optional_query_parameter("optionalEnd", &optional_end);
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testQueryParams",
                    "/catalog/test-query-params",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn test_no_response_query_params(
        &self,
        auth_: &conjure_object::BearerToken,
        query: &str,
        something: &conjure_object::ResourceIdentifier,
        optional_middle: Option<&conjure_object::ResourceIdentifier>,
        implicit: &conjure_object::ResourceIdentifier,
        set_end: &std::collections::BTreeSet<String>,
        optional_end: Option<&conjure_object::ResourceIdentifier>,
    ) -> Result<(), conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_serializable_request(
            &query,
        );
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/test-no-response-query-params");
        path_.push_query_parameter("different", &something);
        path_.push_optional_query_parameter("optionalMiddle", &optional_middle);
        path_.push_query_parameter("implicit", &implicit);
        path_.push_set_query_parameter("setEnd", &set_end);
        path_.push_optional_query_parameter("optionalEnd", &optional_end);
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testNoResponseQueryParams",
                    "/catalog/test-no-response-query-params",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_empty_response(response_).await
    }
    pub async fn test_boolean(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/boolean");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testBoolean",
                    "/catalog/boolean",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn test_double(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/double");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testDouble",
                    "/catalog/double",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn test_integer(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/integer");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testInteger",
                    "/catalog/integer",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn test_post_optional(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_string: Option<&str>,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_serializable_request(
            &maybe_string,
        );
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/optional");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testPostOptional",
                    "/catalog/optional",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_)
            .await
    }
    pub async fn test_optional_integer_and_double(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_integer: Option<i32>,
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/optional-integer-double");
        path_.push_optional_query_parameter("maybeInteger", &maybe_integer);
        path_.push_optional_query_parameter("maybeDouble", &maybe_double);
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testOptionalIntegerAndDouble",
                    "/catalog/optional-integer-double",
                ),
            );
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_empty_response(response_).await
    }
}
///A Markdown description of the service.
#[derive(Clone, Debug)]
pub struct TestServiceClient<T>(T);
impl<T> conjure_http::client::Service<T> for TestServiceClient<T>
where
    T: conjure_http::client::Client,
{
    fn new(client: T) -> Self {
        TestServiceClient(client)
    }
}
impl<T> TestServiceClient<T>
where
    T: conjure_http::client::Client,
{
    ///Returns a mapping from file system id to backing file system configuration.
    pub fn get_file_systems(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<
            String,
            super::super::product::datasets::BackingFileSystem,
        >,
        conjure_http::private::Error,
    > {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/fileSystems");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getFileSystems",
                    "/catalog/fileSystems",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_default_serializable_response(response_)
    }
    pub fn create_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        request: &super::super::product::CreateDatasetRequest,
        test_header_arg: &str,
    ) -> Result<super::super::product::datasets::Dataset, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_serializable_request(&request);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_header(
            &mut request_,
            "test-header",
            &test_header_arg,
        )?;
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "createDataset",
                    "/catalog/datasets",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_serializable_response(response_)
    }
    pub fn get_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<
        Option<super::super::product::datasets::Dataset>,
        conjure_http::private::Error,
    > {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getDataset",
                    "/catalog/datasets/{datasetRid}",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_default_serializable_response(response_)
    }
    pub fn get_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<T::ResponseBody, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/raw");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_binary_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getRawData",
                    "/catalog/datasets/{datasetRid}/raw",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_binary_response(response_)
    }
    pub fn get_aliased_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<T::ResponseBody, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/raw-aliased");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_binary_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getAliasedRawData",
                    "/catalog/datasets/{datasetRid}/raw-aliased",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_binary_response(response_)
    }
    pub fn maybe_get_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<T::ResponseBody>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/raw-maybe");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_binary_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "maybeGetRawData",
                    "/catalog/datasets/{datasetRid}/raw-maybe",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_optional_binary_response(response_)
    }
    pub fn get_aliased_string(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<super::super::product::AliasedString, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/string-aliased");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getAliasedString",
                    "/catalog/datasets/{datasetRid}/string-aliased",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_serializable_response(response_)
    }
    pub fn upload_raw_data<U>(
        &self,
        auth_: &conjure_object::BearerToken,
        input: U,
    ) -> Result<(), conjure_http::private::Error>
    where
        U: conjure_http::client::WriteBody<T::BodyWriter>,
    {
        let mut request_ = conjure_http::private::encode_binary_request(input);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets/upload-raw");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "uploadRawData",
                    "/catalog/datasets/upload-raw",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_empty_response(response_)
    }
    pub fn upload_aliased_raw_data<U>(
        &self,
        auth_: &conjure_object::BearerToken,
        input: U,
    ) -> Result<(), conjure_http::private::Error>
    where
        U: conjure_http::client::WriteBody<T::BodyWriter>,
    {
        let mut request_ = conjure_http::private::encode_binary_request(input);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets/upload-raw-aliased");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "uploadAliasedRawData",
                    "/catalog/datasets/upload-raw-aliased",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_empty_response(response_)
    }
    pub fn get_branches(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/branches");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getBranches",
                    "/catalog/datasets/{datasetRid}/branches",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_default_serializable_response(response_)
    }
    ///Gets all branches of this dataset.
    #[deprecated(note = "use getBranches instead")]
    pub fn get_branches_deprecated(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/branchesDeprecated");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "getBranchesDeprecated",
                    "/catalog/datasets/{datasetRid}/branchesDeprecated",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_default_serializable_response(response_)
    }
    pub fn resolve_branch(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
        branch: &str,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/branches");
        path_.push_path_parameter(&branch);
        path_.push_literal("/resolve");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "resolveBranch",
                    "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_default_serializable_response(response_)
    }
    pub fn test_param(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        path_.push_path_parameter(&dataset_rid);
        path_.push_literal("/testParam");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testParam",
                    "/catalog/datasets/{datasetRid}/testParam",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_default_serializable_response(response_)
    }
    pub fn test_query_params(
        &self,
        auth_: &conjure_object::BearerToken,
        query: &str,
        something: &conjure_object::ResourceIdentifier,
        optional_middle: Option<&conjure_object::ResourceIdentifier>,
        implicit: &conjure_object::ResourceIdentifier,
        set_end: &std::collections::BTreeSet<String>,
        optional_end: Option<&conjure_object::ResourceIdentifier>,
    ) -> Result<i32, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_serializable_request(&query);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/test-query-params");
        path_.push_query_parameter("different", &something);
        path_.push_optional_query_parameter("optionalMiddle", &optional_middle);
        path_.push_query_parameter("implicit", &implicit);
        path_.push_set_query_parameter("setEnd", &set_end);
        path_.push_optional_query_parameter("optionalEnd", &optional_end);
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testQueryParams",
                    "/catalog/test-query-params",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_serializable_response(response_)
    }
    pub fn test_no_response_query_params(
        &self,
        auth_: &conjure_object::BearerToken,
        query: &str,
        something: &conjure_object::ResourceIdentifier,
        optional_middle: Option<&conjure_object::ResourceIdentifier>,
        implicit: &conjure_object::ResourceIdentifier,
        set_end: &std::collections::BTreeSet<String>,
        optional_end: Option<&conjure_object::ResourceIdentifier>,
    ) -> Result<(), conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_serializable_request(&query);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/test-no-response-query-params");
        path_.push_query_parameter("different", &something);
        path_.push_optional_query_parameter("optionalMiddle", &optional_middle);
        path_.push_query_parameter("implicit", &implicit);
        path_.push_set_query_parameter("setEnd", &set_end);
        path_.push_optional_query_parameter("optionalEnd", &optional_end);
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testNoResponseQueryParams",
                    "/catalog/test-no-response-query-params",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_empty_response(response_)
    }
    pub fn test_boolean(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/boolean");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testBoolean",
                    "/catalog/boolean",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_serializable_response(response_)
    }
    pub fn test_double(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/double");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testDouble",
                    "/catalog/double",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_serializable_response(response_)
    }
    pub fn test_integer(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/integer");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testInteger",
                    "/catalog/integer",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_serializable_response(response_)
    }
    pub fn test_post_optional(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_string: Option<&str>,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_serializable_request(
            &maybe_string,
        );
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/optional");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testPostOptional",
                    "/catalog/optional",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_default_serializable_response(response_)
    }
    pub fn test_optional_integer_and_double(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_integer: Option<i32>,
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_empty_request();
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/optional-integer-double");
        path_.push_optional_query_parameter("maybeInteger", &maybe_integer);
        path_.push_optional_query_parameter("maybeDouble", &maybe_double);
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(
                conjure_http::client::Endpoint::new(
                    "TestService",
                    conjure_http::private::Option::Some("0.1.0"),
                    "testOptionalIntegerAndDouble",
                    "/catalog/optional-integer-double",
                ),
            );
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_empty_response(response_)
    }
}
use conjure_http::endpoint;
///A Markdown description of the service.
#[conjure_http::conjure_endpoints(name = "TestService")]
pub trait TestService<#[request_body] I, #[response_writer] O> {
    ///The body type returned by the `get_raw_data` method.
    type GetRawDataBody: conjure_http::server::WriteBody<O> + 'static;
    ///The body type returned by the `get_aliased_raw_data` method.
    type GetAliasedRawDataBody: conjure_http::server::WriteBody<O> + 'static;
    ///The body type returned by the `maybe_get_raw_data` method.
    type MaybeGetRawDataBody: conjure_http::server::WriteBody<O> + 'static;
    ///Returns a mapping from file system id to backing file system configuration.
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
            super::super::product::datasets::BackingFileSystem,
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
        request: super::super::product::CreateDatasetRequest,
        #[header(
            name = "Test-Header",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "testHeaderArg"
        )]
        test_header_arg: String,
    ) -> Result<super::super::product::datasets::Dataset, conjure_http::private::Error>;
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
        Option<super::super::product::datasets::Dataset>,
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
    ) -> Result<super::super::product::AliasedString, conjure_http::private::Error>;
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
    ///Gets all branches of this dataset.
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
///A Markdown description of the service.
#[conjure_http::conjure_endpoints(name = "TestService")]
pub trait AsyncTestService<#[request_body] I, #[response_writer] O> {
    ///The body type returned by the `get_raw_data` method.
    type GetRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    ///The body type returned by the `get_aliased_raw_data` method.
    type GetAliasedRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    ///The body type returned by the `maybe_get_raw_data` method.
    type MaybeGetRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    ///Returns a mapping from file system id to backing file system configuration.
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
            super::super::product::datasets::BackingFileSystem,
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
        request: super::super::product::CreateDatasetRequest,
        #[header(
            name = "Test-Header",
            decoder = conjure_http::server::conjure::FromPlainDecoder,
            log_as = "testHeaderArg"
        )]
        test_header_arg: String,
    ) -> Result<super::super::product::datasets::Dataset, conjure_http::private::Error>;
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
        Option<super::super::product::datasets::Dataset>,
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
    ) -> Result<super::super::product::AliasedString, conjure_http::private::Error>;
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
    ///Gets all branches of this dataset.
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
