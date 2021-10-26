#[doc = "A Markdown description of the service."]
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
    #[doc = "Returns a mapping from file system id to backing file system configuration."]
    pub async fn get_file_systems(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<String, super::super::product::datasets::BackingFileSystem>,
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getFileSystems",
                "/catalog/fileSystems",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_).await
    }
    pub async fn create_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        request: &super::super::product::CreateDatasetRequest,
        test_header_arg: &str,
    ) -> Result<super::super::product::datasets::Dataset, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_serializable_request(&request);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_header(&mut request_, "test-header", &test_header_arg)?;
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "createDataset",
                "/catalog/datasets",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn get_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<super::super::product::datasets::Dataset>, conjure_http::private::Error>
    {
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getDataset",
                "/catalog/datasets/{datasetRid}",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_).await
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getRawData",
                "/catalog/datasets/{datasetRid}/raw",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getAliasedRawData",
                "/catalog/datasets/{datasetRid}/raw-aliased",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "maybeGetRawData",
                "/catalog/datasets/{datasetRid}/raw-maybe",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getAliasedString",
                "/catalog/datasets/{datasetRid}/string-aliased",
            ));
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
        conjure_http::private::pin_mut!(input);
        let mut request_ = conjure_http::private::async_encode_binary_request(input as _);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets/upload-raw");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "uploadRawData",
                "/catalog/datasets/upload-raw",
            ));
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
        conjure_http::private::pin_mut!(input);
        let mut request_ = conjure_http::private::async_encode_binary_request(input as _);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets/upload-raw-aliased");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "uploadAliasedRawData",
                "/catalog/datasets/upload-raw-aliased",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getBranches",
                "/catalog/datasets/{datasetRid}/branches",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_).await
    }
    #[doc = "Gets all branches of this dataset."]
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getBranchesDeprecated",
                "/catalog/datasets/{datasetRid}/branchesDeprecated",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_).await
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "resolveBranch",
                "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_).await
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testParam",
                "/catalog/datasets/{datasetRid}/testParam",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_).await
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
        let mut request_ = conjure_http::private::async_encode_serializable_request(&query);
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testQueryParams",
                "/catalog/test-query-params",
            ));
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
        let mut request_ = conjure_http::private::async_encode_serializable_request(&query);
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testNoResponseQueryParams",
                "/catalog/test-no-response-query-params",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testBoolean",
                "/catalog/boolean",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testDouble",
                "/catalog/double",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testInteger",
                "/catalog/integer",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn test_post_optional(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_string: Option<&str>,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::async_encode_serializable_request(&maybe_string);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/optional");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testPostOptional",
                "/catalog/optional",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_).await
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testOptionalIntegerAndDouble",
                "/catalog/optional-integer-double",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_empty_response(response_).await
    }
}
#[doc = "A Markdown description of the service."]
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
    #[doc = "Returns a mapping from file system id to backing file system configuration."]
    pub fn get_file_systems(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<String, super::super::product::datasets::BackingFileSystem>,
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getFileSystems",
                "/catalog/fileSystems",
            ));
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
        conjure_http::private::encode_header(&mut request_, "test-header", &test_header_arg)?;
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "createDataset",
                "/catalog/datasets",
            ));
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_serializable_response(response_)
    }
    pub fn get_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<super::super::product::datasets::Dataset>, conjure_http::private::Error>
    {
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getDataset",
                "/catalog/datasets/{datasetRid}",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getRawData",
                "/catalog/datasets/{datasetRid}/raw",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getAliasedRawData",
                "/catalog/datasets/{datasetRid}/raw-aliased",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "maybeGetRawData",
                "/catalog/datasets/{datasetRid}/raw-maybe",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getAliasedString",
                "/catalog/datasets/{datasetRid}/string-aliased",
            ));
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
        let mut input = input;
        let mut request_ = conjure_http::private::encode_binary_request(&mut input as _);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets/upload-raw");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "uploadRawData",
                "/catalog/datasets/upload-raw",
            ));
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
        let mut input = input;
        let mut request_ = conjure_http::private::encode_binary_request(&mut input as _);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/datasets/upload-raw-aliased");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_empty_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "uploadAliasedRawData",
                "/catalog/datasets/upload-raw-aliased",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getBranches",
                "/catalog/datasets/{datasetRid}/branches",
            ));
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_default_serializable_response(response_)
    }
    #[doc = "Gets all branches of this dataset."]
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "getBranchesDeprecated",
                "/catalog/datasets/{datasetRid}/branchesDeprecated",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "resolveBranch",
                "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testParam",
                "/catalog/datasets/{datasetRid}/testParam",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testQueryParams",
                "/catalog/test-query-params",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testNoResponseQueryParams",
                "/catalog/test-no-response-query-params",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testBoolean",
                "/catalog/boolean",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testDouble",
                "/catalog/double",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testInteger",
                "/catalog/integer",
            ));
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_serializable_response(response_)
    }
    pub fn test_post_optional(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_string: Option<&str>,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ = conjure_http::private::encode_serializable_request(&maybe_string);
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = conjure_http::private::UriBuilder::new();
        path_.push_literal("/catalog/optional");
        *request_.uri_mut() = path_.build();
        conjure_http::private::encode_header_auth(&mut request_, auth_);
        conjure_http::private::encode_serializable_response_headers(&mut request_);
        request_
            .extensions_mut()
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testPostOptional",
                "/catalog/optional",
            ));
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
            .insert(conjure_http::client::Endpoint::new(
                "TestService",
                conjure_http::private::Option::None,
                "testOptionalIntegerAndDouble",
                "/catalog/optional-integer-double",
            ));
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_empty_response(response_)
    }
}
#[doc = "A Markdown description of the service."]
pub trait TestService<I, O> {
    #[doc = "The body type returned by the `get_raw_data` method."]
    type GetRawDataBody: conjure_http::server::WriteBody<O> + 'static;
    #[doc = "The body type returned by the `get_aliased_raw_data` method."]
    type GetAliasedRawDataBody: conjure_http::server::WriteBody<O> + 'static;
    #[doc = "The body type returned by the `maybe_get_raw_data` method."]
    type MaybeGetRawDataBody: conjure_http::server::WriteBody<O> + 'static;
    #[doc = "Returns a mapping from file system id to backing file system configuration."]
    fn get_file_systems(
        &self,
        auth_: conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<String, super::super::product::datasets::BackingFileSystem>,
        conjure_http::private::Error,
    >;
    fn create_dataset(
        &self,
        auth_: conjure_object::BearerToken,
        request: super::super::product::CreateDatasetRequest,
        test_header_arg: String,
    ) -> Result<super::super::product::datasets::Dataset, conjure_http::private::Error>;
    fn get_dataset(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<super::super::product::datasets::Dataset>, conjure_http::private::Error>;
    fn get_raw_data(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Self::GetRawDataBody, conjure_http::private::Error>;
    fn get_aliased_raw_data(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Self::GetAliasedRawDataBody, conjure_http::private::Error>;
    fn maybe_get_raw_data(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<Self::MaybeGetRawDataBody>, conjure_http::private::Error>;
    fn get_aliased_string(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<super::super::product::AliasedString, conjure_http::private::Error>;
    fn upload_raw_data(
        &self,
        auth_: conjure_object::BearerToken,
        input: I,
    ) -> Result<(), conjure_http::private::Error>;
    fn upload_aliased_raw_data(
        &self,
        auth_: conjure_object::BearerToken,
        input: I,
    ) -> Result<(), conjure_http::private::Error>;
    fn get_branches(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    #[doc = "Gets all branches of this dataset."]
    fn get_branches_deprecated(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    fn resolve_branch(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
        branch: String,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    fn test_param(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    fn test_query_params(
        &self,
        auth_: conjure_object::BearerToken,
        query: String,
        something: conjure_object::ResourceIdentifier,
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        implicit: conjure_object::ResourceIdentifier,
        set_end: std::collections::BTreeSet<String>,
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> Result<i32, conjure_http::private::Error>;
    fn test_no_response_query_params(
        &self,
        auth_: conjure_object::BearerToken,
        query: String,
        something: conjure_object::ResourceIdentifier,
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        implicit: conjure_object::ResourceIdentifier,
        set_end: std::collections::BTreeSet<String>,
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> Result<(), conjure_http::private::Error>;
    fn test_boolean(
        &self,
        auth_: conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error>;
    fn test_double(
        &self,
        auth_: conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error>;
    fn test_integer(
        &self,
        auth_: conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error>;
    fn test_post_optional(
        &self,
        auth_: conjure_object::BearerToken,
        maybe_string: Option<String>,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    fn test_optional_integer_and_double(
        &self,
        auth_: conjure_object::BearerToken,
        maybe_integer: Option<i32>,
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error>;
}
#[doc = "A Markdown description of the service."]
#[conjure_http::private::async_trait]
pub trait AsyncTestService<I, O> {
    #[doc = "The body type returned by the `get_raw_data` method."]
    type GetRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    #[doc = "The body type returned by the `get_aliased_raw_data` method."]
    type GetAliasedRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    #[doc = "The body type returned by the `maybe_get_raw_data` method."]
    type MaybeGetRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    #[doc = "Returns a mapping from file system id to backing file system configuration."]
    async fn get_file_systems(
        &self,
        auth_: conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<String, super::super::product::datasets::BackingFileSystem>,
        conjure_http::private::Error,
    >;
    async fn create_dataset(
        &self,
        auth_: conjure_object::BearerToken,
        request: super::super::product::CreateDatasetRequest,
        test_header_arg: String,
    ) -> Result<super::super::product::datasets::Dataset, conjure_http::private::Error>;
    async fn get_dataset(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<super::super::product::datasets::Dataset>, conjure_http::private::Error>;
    async fn get_raw_data(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Self::GetRawDataBody, conjure_http::private::Error>;
    async fn get_aliased_raw_data(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Self::GetAliasedRawDataBody, conjure_http::private::Error>;
    async fn maybe_get_raw_data(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<Self::MaybeGetRawDataBody>, conjure_http::private::Error>;
    async fn get_aliased_string(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<super::super::product::AliasedString, conjure_http::private::Error>;
    async fn upload_raw_data(
        &self,
        auth_: conjure_object::BearerToken,
        input: I,
    ) -> Result<(), conjure_http::private::Error>;
    async fn upload_aliased_raw_data(
        &self,
        auth_: conjure_object::BearerToken,
        input: I,
    ) -> Result<(), conjure_http::private::Error>;
    async fn get_branches(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    #[doc = "Gets all branches of this dataset."]
    async fn get_branches_deprecated(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error>;
    async fn resolve_branch(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
        branch: String,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    async fn test_param(
        &self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    async fn test_query_params(
        &self,
        auth_: conjure_object::BearerToken,
        query: String,
        something: conjure_object::ResourceIdentifier,
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        implicit: conjure_object::ResourceIdentifier,
        set_end: std::collections::BTreeSet<String>,
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> Result<i32, conjure_http::private::Error>;
    async fn test_no_response_query_params(
        &self,
        auth_: conjure_object::BearerToken,
        query: String,
        something: conjure_object::ResourceIdentifier,
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        implicit: conjure_object::ResourceIdentifier,
        set_end: std::collections::BTreeSet<String>,
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> Result<(), conjure_http::private::Error>;
    async fn test_boolean(
        &self,
        auth_: conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error>;
    async fn test_double(
        &self,
        auth_: conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error>;
    async fn test_integer(
        &self,
        auth_: conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error>;
    async fn test_post_optional(
        &self,
        auth_: conjure_object::BearerToken,
        maybe_string: Option<String>,
    ) -> Result<Option<String>, conjure_http::private::Error>;
    async fn test_optional_integer_and_double(
        &self,
        auth_: conjure_object::BearerToken,
        maybe_integer: Option<i32>,
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error>;
}
pub struct TestServiceEndpoints<T>(conjure_http::private::Arc<T>);
impl<T> TestServiceEndpoints<T> {
    #[doc = r" Creates a new resource."]
    pub fn new(handler: T) -> TestServiceEndpoints<T> {
        TestServiceEndpoints(conjure_http::private::Arc::new(handler))
    }
}
impl<T, I, O> conjure_http::server::Service<I, O> for TestServiceEndpoints<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn endpoints(&self) -> Vec<Box<dyn conjure_http::server::Endpoint<I, O> + Sync + Send>> {
        vec![
            Box::new(GetFileSystemsEndpoint_(self.0.clone())),
            Box::new(CreateDatasetEndpoint_(self.0.clone())),
            Box::new(GetDatasetEndpoint_(self.0.clone())),
            Box::new(GetRawDataEndpoint_(self.0.clone())),
            Box::new(GetAliasedRawDataEndpoint_(self.0.clone())),
            Box::new(MaybeGetRawDataEndpoint_(self.0.clone())),
            Box::new(GetAliasedStringEndpoint_(self.0.clone())),
            Box::new(UploadRawDataEndpoint_(self.0.clone())),
            Box::new(UploadAliasedRawDataEndpoint_(self.0.clone())),
            Box::new(GetBranchesEndpoint_(self.0.clone())),
            Box::new(GetBranchesDeprecatedEndpoint_(self.0.clone())),
            Box::new(ResolveBranchEndpoint_(self.0.clone())),
            Box::new(TestParamEndpoint_(self.0.clone())),
            Box::new(TestQueryParamsEndpoint_(self.0.clone())),
            Box::new(TestNoResponseQueryParamsEndpoint_(self.0.clone())),
            Box::new(TestBooleanEndpoint_(self.0.clone())),
            Box::new(TestDoubleEndpoint_(self.0.clone())),
            Box::new(TestIntegerEndpoint_(self.0.clone())),
            Box::new(TestPostOptionalEndpoint_(self.0.clone())),
            Box::new(TestOptionalIntegerAndDoubleEndpoint_(self.0.clone())),
        ]
    }
}
impl<T, I, O> conjure_http::server::AsyncService<I, O> for TestServiceEndpoints<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    fn endpoints(&self) -> Vec<Box<dyn conjure_http::server::AsyncEndpoint<I, O> + Sync + Send>> {
        vec![
            Box::new(GetFileSystemsEndpoint_(self.0.clone())),
            Box::new(CreateDatasetEndpoint_(self.0.clone())),
            Box::new(GetDatasetEndpoint_(self.0.clone())),
            Box::new(GetRawDataEndpoint_(self.0.clone())),
            Box::new(GetAliasedRawDataEndpoint_(self.0.clone())),
            Box::new(MaybeGetRawDataEndpoint_(self.0.clone())),
            Box::new(GetAliasedStringEndpoint_(self.0.clone())),
            Box::new(UploadRawDataEndpoint_(self.0.clone())),
            Box::new(UploadAliasedRawDataEndpoint_(self.0.clone())),
            Box::new(GetBranchesEndpoint_(self.0.clone())),
            Box::new(GetBranchesDeprecatedEndpoint_(self.0.clone())),
            Box::new(ResolveBranchEndpoint_(self.0.clone())),
            Box::new(TestParamEndpoint_(self.0.clone())),
            Box::new(TestQueryParamsEndpoint_(self.0.clone())),
            Box::new(TestNoResponseQueryParamsEndpoint_(self.0.clone())),
            Box::new(TestBooleanEndpoint_(self.0.clone())),
            Box::new(TestDoubleEndpoint_(self.0.clone())),
            Box::new(TestIntegerEndpoint_(self.0.clone())),
            Box::new(TestPostOptionalEndpoint_(self.0.clone())),
            Box::new(TestOptionalIntegerAndDoubleEndpoint_(self.0.clone())),
        ]
    }
}
struct GetFileSystemsEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for GetFileSystemsEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "fileSystems",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/fileSystems"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "getFileSystems"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for GetFileSystemsEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_file_systems(auth_)?;
            Ok(conjure_http::private::encode_default_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for GetFileSystemsEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_file_systems(auth_).await?;
            Ok(conjure_http::private::async_encode_default_serializable_response(&response_))
        })
        .await
    }
}
struct CreateDatasetEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for CreateDatasetEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::POST
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "createDataset"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for CreateDatasetEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            let request = conjure_http::private::decode_serializable_request(&parts_, body_)?;
            let test_header_arg = conjure_http::private::parse_required_header(
                &parts_,
                "testHeaderArg",
                "Test-Header",
            )?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.create_dataset(auth_, request, test_header_arg)?;
            Ok(conjure_http::private::encode_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for CreateDatasetEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            let request =
                conjure_http::private::async_decode_serializable_request(&parts_, body_).await?;
            let test_header_arg = conjure_http::private::parse_required_header(
                &parts_,
                "testHeaderArg",
                "Test-Header",
            )?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self
                .0
                .create_dataset(auth_, request, test_header_arg)
                .await?;
            Ok(conjure_http::private::async_encode_serializable_response(
                &response_,
            ))
        })
        .await
    }
}
struct GetDatasetEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for GetDatasetEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Parameter {
                name: conjure_http::private::Cow::Borrowed("datasetRid"),
                regex: None,
            },
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/{datasetRid}"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "getDataset"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for GetDatasetEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_dataset(auth_, dataset_rid)?;
            Ok(conjure_http::private::encode_default_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for GetDatasetEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_dataset(auth_, dataset_rid).await?;
            Ok(conjure_http::private::async_encode_default_serializable_response(&response_))
        })
        .await
    }
}
struct GetRawDataEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for GetRawDataEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Parameter {
                name: conjure_http::private::Cow::Borrowed("datasetRid"),
                regex: None,
            },
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed("raw")),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/{datasetRid}/raw"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "getRawData"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for GetRawDataEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_raw_data(auth_, dataset_rid)?;
            Ok(conjure_http::private::encode_binary_response(response_))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for GetRawDataEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_raw_data(auth_, dataset_rid).await?;
            Ok(conjure_http::private::async_encode_binary_response(
                response_,
            ))
        })
        .await
    }
}
struct GetAliasedRawDataEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for GetAliasedRawDataEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Parameter {
                name: conjure_http::private::Cow::Borrowed("datasetRid"),
                regex: None,
            },
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "raw-aliased",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/{datasetRid}/raw-aliased"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "getAliasedRawData"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for GetAliasedRawDataEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_aliased_raw_data(auth_, dataset_rid)?;
            Ok(conjure_http::private::encode_binary_response(response_))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for GetAliasedRawDataEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_aliased_raw_data(auth_, dataset_rid).await?;
            Ok(conjure_http::private::async_encode_binary_response(
                response_,
            ))
        })
        .await
    }
}
struct MaybeGetRawDataEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for MaybeGetRawDataEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Parameter {
                name: conjure_http::private::Cow::Borrowed("datasetRid"),
                regex: None,
            },
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "raw-maybe",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/{datasetRid}/raw-maybe"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "maybeGetRawData"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for MaybeGetRawDataEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.maybe_get_raw_data(auth_, dataset_rid)?;
            Ok(conjure_http::private::encode_optional_binary_response(
                response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for MaybeGetRawDataEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.maybe_get_raw_data(auth_, dataset_rid).await?;
            Ok(conjure_http::private::async_encode_optional_binary_response(response_))
        })
        .await
    }
}
struct GetAliasedStringEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for GetAliasedStringEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Parameter {
                name: conjure_http::private::Cow::Borrowed("datasetRid"),
                regex: None,
            },
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "string-aliased",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/{datasetRid}/string-aliased"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "getAliasedString"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for GetAliasedStringEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_aliased_string(auth_, dataset_rid)?;
            Ok(conjure_http::private::encode_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for GetAliasedStringEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_aliased_string(auth_, dataset_rid).await?;
            Ok(conjure_http::private::async_encode_serializable_response(
                &response_,
            ))
        })
        .await
    }
}
struct UploadRawDataEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for UploadRawDataEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::POST
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "upload-raw",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/upload-raw"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "uploadRawData"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for UploadRawDataEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            let input = conjure_http::private::decode_binary_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            self.0.upload_raw_data(auth_, input)?;
            Ok(conjure_http::private::encode_empty_response())
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for UploadRawDataEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            let input = conjure_http::private::decode_binary_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            self.0.upload_raw_data(auth_, input).await?;
            Ok(conjure_http::private::async_encode_empty_response())
        })
        .await
    }
}
struct UploadAliasedRawDataEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for UploadAliasedRawDataEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::POST
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "upload-raw-aliased",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/upload-raw-aliased"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "uploadAliasedRawData"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for UploadAliasedRawDataEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            let input = conjure_http::private::decode_binary_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            self.0.upload_aliased_raw_data(auth_, input)?;
            Ok(conjure_http::private::encode_empty_response())
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for UploadAliasedRawDataEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            let input = conjure_http::private::decode_binary_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            self.0.upload_aliased_raw_data(auth_, input).await?;
            Ok(conjure_http::private::async_encode_empty_response())
        })
        .await
    }
}
struct GetBranchesEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for GetBranchesEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Parameter {
                name: conjure_http::private::Cow::Borrowed("datasetRid"),
                regex: None,
            },
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "branches",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/{datasetRid}/branches"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "getBranches"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for GetBranchesEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_branches(auth_, dataset_rid)?;
            Ok(conjure_http::private::encode_default_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for GetBranchesEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_branches(auth_, dataset_rid).await?;
            Ok(conjure_http::private::async_encode_default_serializable_response(&response_))
        })
        .await
    }
}
struct GetBranchesDeprecatedEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for GetBranchesDeprecatedEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Parameter {
                name: conjure_http::private::Cow::Borrowed("datasetRid"),
                regex: None,
            },
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "branchesDeprecated",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/{datasetRid}/branchesDeprecated"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "getBranchesDeprecated"
    }
    fn deprecated(&self) -> Option<&str> {
        Some("use getBranches instead")
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for GetBranchesDeprecatedEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_branches_deprecated(auth_, dataset_rid)?;
            Ok(conjure_http::private::encode_default_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for GetBranchesDeprecatedEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.get_branches_deprecated(auth_, dataset_rid).await?;
            Ok(conjure_http::private::async_encode_default_serializable_response(&response_))
        })
        .await
    }
}
struct ResolveBranchEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for ResolveBranchEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Parameter {
                name: conjure_http::private::Cow::Borrowed("datasetRid"),
                regex: None,
            },
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "branches",
            )),
            conjure_http::server::PathSegment::Parameter {
                name: conjure_http::private::Cow::Borrowed("branch"),
                regex: Some(conjure_http::private::Cow::Borrowed(".+")),
            },
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "resolve",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "resolveBranch"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for ResolveBranchEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let branch = conjure_http::private::parse_path_param(&parts_, "branch")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.resolve_branch(auth_, dataset_rid, branch)?;
            Ok(conjure_http::private::encode_default_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for ResolveBranchEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let branch = conjure_http::private::parse_path_param(&parts_, "branch")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.resolve_branch(auth_, dataset_rid, branch).await?;
            Ok(conjure_http::private::async_encode_default_serializable_response(&response_))
        })
        .await
    }
}
struct TestParamEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for TestParamEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "datasets",
            )),
            conjure_http::server::PathSegment::Parameter {
                name: conjure_http::private::Cow::Borrowed("datasetRid"),
                regex: None,
            },
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "testParam",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/datasets/{datasetRid}/testParam"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "testParam"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for TestParamEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_param(auth_, dataset_rid)?;
            Ok(conjure_http::private::encode_default_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for TestParamEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let dataset_rid = conjure_http::private::parse_path_param(&parts_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_param(auth_, dataset_rid).await?;
            Ok(conjure_http::private::async_encode_default_serializable_response(&response_))
        })
        .await
    }
}
struct TestQueryParamsEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for TestQueryParamsEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::POST
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "test-query-params",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/test-query-params"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "testQueryParams"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for TestQueryParamsEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            let query_params_ = conjure_http::private::parse_query_params(&parts_);
            let query = conjure_http::private::decode_serializable_request(&parts_, body_)?;
            let something =
                conjure_http::private::parse_query_param(&query_params_, "something", "different")?;
            let mut optional_middle: Option<conjure_object::ResourceIdentifier> =
                Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "optionalMiddle",
                "optionalMiddle",
                &mut optional_middle,
            )?;
            let implicit =
                conjure_http::private::parse_query_param(&query_params_, "implicit", "implicit")?;
            let mut set_end: std::collections::BTreeSet<String> = Default::default();
            conjure_http::private::parse_set_query_param(
                &query_params_,
                "setEnd",
                "setEnd",
                &mut set_end,
            )?;
            let mut optional_end: Option<conjure_object::ResourceIdentifier> = Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "optionalEnd",
                "optionalEnd",
                &mut optional_end,
            )?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_query_params(
                auth_,
                query,
                something,
                optional_middle,
                implicit,
                set_end,
                optional_end,
            )?;
            Ok(conjure_http::private::encode_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for TestQueryParamsEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            let query_params_ = conjure_http::private::parse_query_params(&parts_);
            let query =
                conjure_http::private::async_decode_serializable_request(&parts_, body_).await?;
            let something =
                conjure_http::private::parse_query_param(&query_params_, "something", "different")?;
            let mut optional_middle: Option<conjure_object::ResourceIdentifier> =
                Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "optionalMiddle",
                "optionalMiddle",
                &mut optional_middle,
            )?;
            let implicit =
                conjure_http::private::parse_query_param(&query_params_, "implicit", "implicit")?;
            let mut set_end: std::collections::BTreeSet<String> = Default::default();
            conjure_http::private::parse_set_query_param(
                &query_params_,
                "setEnd",
                "setEnd",
                &mut set_end,
            )?;
            let mut optional_end: Option<conjure_object::ResourceIdentifier> = Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "optionalEnd",
                "optionalEnd",
                &mut optional_end,
            )?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self
                .0
                .test_query_params(
                    auth_,
                    query,
                    something,
                    optional_middle,
                    implicit,
                    set_end,
                    optional_end,
                )
                .await?;
            Ok(conjure_http::private::async_encode_serializable_response(
                &response_,
            ))
        })
        .await
    }
}
struct TestNoResponseQueryParamsEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for TestNoResponseQueryParamsEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::POST
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "test-no-response-query-params",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/test-no-response-query-params"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "testNoResponseQueryParams"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for TestNoResponseQueryParamsEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            let query_params_ = conjure_http::private::parse_query_params(&parts_);
            let query = conjure_http::private::decode_serializable_request(&parts_, body_)?;
            let something =
                conjure_http::private::parse_query_param(&query_params_, "something", "different")?;
            let mut optional_middle: Option<conjure_object::ResourceIdentifier> =
                Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "optionalMiddle",
                "optionalMiddle",
                &mut optional_middle,
            )?;
            let implicit =
                conjure_http::private::parse_query_param(&query_params_, "implicit", "implicit")?;
            let mut set_end: std::collections::BTreeSet<String> = Default::default();
            conjure_http::private::parse_set_query_param(
                &query_params_,
                "setEnd",
                "setEnd",
                &mut set_end,
            )?;
            let mut optional_end: Option<conjure_object::ResourceIdentifier> = Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "optionalEnd",
                "optionalEnd",
                &mut optional_end,
            )?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            self.0.test_no_response_query_params(
                auth_,
                query,
                something,
                optional_middle,
                implicit,
                set_end,
                optional_end,
            )?;
            Ok(conjure_http::private::encode_empty_response())
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for TestNoResponseQueryParamsEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            let query_params_ = conjure_http::private::parse_query_params(&parts_);
            let query =
                conjure_http::private::async_decode_serializable_request(&parts_, body_).await?;
            let something =
                conjure_http::private::parse_query_param(&query_params_, "something", "different")?;
            let mut optional_middle: Option<conjure_object::ResourceIdentifier> =
                Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "optionalMiddle",
                "optionalMiddle",
                &mut optional_middle,
            )?;
            let implicit =
                conjure_http::private::parse_query_param(&query_params_, "implicit", "implicit")?;
            let mut set_end: std::collections::BTreeSet<String> = Default::default();
            conjure_http::private::parse_set_query_param(
                &query_params_,
                "setEnd",
                "setEnd",
                &mut set_end,
            )?;
            let mut optional_end: Option<conjure_object::ResourceIdentifier> = Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "optionalEnd",
                "optionalEnd",
                &mut optional_end,
            )?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            self.0
                .test_no_response_query_params(
                    auth_,
                    query,
                    something,
                    optional_middle,
                    implicit,
                    set_end,
                    optional_end,
                )
                .await?;
            Ok(conjure_http::private::async_encode_empty_response())
        })
        .await
    }
}
struct TestBooleanEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for TestBooleanEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "boolean",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/boolean"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "testBoolean"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for TestBooleanEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_boolean(auth_)?;
            Ok(conjure_http::private::encode_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for TestBooleanEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_boolean(auth_).await?;
            Ok(conjure_http::private::async_encode_serializable_response(
                &response_,
            ))
        })
        .await
    }
}
struct TestDoubleEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for TestDoubleEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "double",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/double"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "testDouble"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for TestDoubleEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_double(auth_)?;
            Ok(conjure_http::private::encode_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for TestDoubleEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_double(auth_).await?;
            Ok(conjure_http::private::async_encode_serializable_response(
                &response_,
            ))
        })
        .await
    }
}
struct TestIntegerEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for TestIntegerEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "integer",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/integer"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "testInteger"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for TestIntegerEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_integer(auth_)?;
            Ok(conjure_http::private::encode_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for TestIntegerEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_integer(auth_).await?;
            Ok(conjure_http::private::async_encode_serializable_response(
                &response_,
            ))
        })
        .await
    }
}
struct TestPostOptionalEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for TestPostOptionalEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::POST
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "optional",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/optional"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "testPostOptional"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for TestPostOptionalEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            let maybe_string =
                conjure_http::private::decode_optional_serializable_request(&parts_, body_)?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_post_optional(auth_, maybe_string)?;
            Ok(conjure_http::private::encode_default_serializable_response(
                &response_,
            ))
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for TestPostOptionalEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            let maybe_string =
                conjure_http::private::async_decode_optional_serializable_request(&parts_, body_)
                    .await?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            let response_ = self.0.test_post_optional(auth_, maybe_string).await?;
            Ok(conjure_http::private::async_encode_default_serializable_response(&response_))
        })
        .await
    }
}
struct TestOptionalIntegerAndDoubleEndpoint_<T>(conjure_http::private::Arc<T>);
impl<T> conjure_http::server::EndpointMetadata for TestOptionalIntegerAndDoubleEndpoint_<T> {
    fn method(&self) -> conjure_http::private::Method {
        conjure_http::private::Method::GET
    }
    fn path(&self) -> &[conjure_http::server::PathSegment] {
        &[
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "catalog",
            )),
            conjure_http::server::PathSegment::Literal(conjure_http::private::Cow::Borrowed(
                "optional-integer-double",
            )),
        ]
    }
    fn template(&self) -> &str {
        "/catalog/optional-integer-double"
    }
    fn service_name(&self) -> &str {
        "TestService"
    }
    fn name(&self) -> &str {
        "testOptionalIntegerAndDouble"
    }
    fn deprecated(&self) -> Option<&str> {
        None
    }
}
impl<T, I, O> conjure_http::server::Endpoint<I, O> for TestOptionalIntegerAndDoubleEndpoint_<T>
where
    T: TestService<I, O> + 'static + Sync + Send,
    I: Iterator<Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>>,
{
    fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::ResponseBody<O>> {
        conjure_http::private::wrap(|_safe_params| {
            let (parts_, body_) = request.into_parts();
            let query_params_ = conjure_http::private::parse_query_params(&parts_);
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let mut maybe_integer: Option<i32> = Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "maybeInteger",
                "maybeInteger",
                &mut maybe_integer,
            )?;
            let mut maybe_double: Option<f64> = Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "maybeDouble",
                "maybeDouble",
                &mut maybe_double,
            )?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            self.0
                .test_optional_integer_and_double(auth_, maybe_integer, maybe_double)?;
            Ok(conjure_http::private::encode_empty_response())
        })
    }
}
#[conjure_http::private::async_trait]
impl<T, I, O> conjure_http::server::AsyncEndpoint<I, O> for TestOptionalIntegerAndDoubleEndpoint_<T>
where
    T: AsyncTestService<I, O> + 'static + Sync + Send,
    I: conjure_http::private::Stream<
            Item = Result<conjure_http::private::Bytes, conjure_http::private::Error>,
        > + Sync
        + Send,
{
    async fn handle(
        &self,
        request: conjure_http::private::Request<I>,
    ) -> conjure_http::private::Response<conjure_http::server::AsyncResponseBody<O>>
    where
        I: 'async_trait,
    {
        conjure_http::__async_wrap!(|_safe_params| async {
            let (parts_, body_) = request.into_parts();
            let query_params_ = conjure_http::private::parse_query_params(&parts_);
            conjure_http::private::decode_empty_request(&parts_, body_)?;
            let mut maybe_integer: Option<i32> = Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "maybeInteger",
                "maybeInteger",
                &mut maybe_integer,
            )?;
            let mut maybe_double: Option<f64> = Default::default();
            conjure_http::private::parse_optional_query_param(
                &query_params_,
                "maybeDouble",
                "maybeDouble",
                &mut maybe_double,
            )?;
            let auth_ = conjure_http::private::parse_header_auth(&parts_)?;
            self.0
                .test_optional_integer_and_double(auth_, maybe_integer, maybe_double)
                .await?;
            Ok(conjure_http::private::async_encode_empty_response())
        })
        .await
    }
}
