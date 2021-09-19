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
                conjure_http::private::Option::Some("0.1.0"),
                "getFileSystems",
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
                conjure_http::private::Option::Some("0.1.0"),
                "createDataset",
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
                conjure_http::private::Option::Some("0.1.0"),
                "getDataset",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_).await
    }
    pub async fn get_raw_data(
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
                conjure_http::private::Option::Some("0.1.0"),
                "getRawData",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::decode_binary_response(response_)
    }
    pub async fn get_aliased_raw_data(
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
                conjure_http::private::Option::Some("0.1.0"),
                "getAliasedRawData",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::decode_binary_response(response_)
    }
    pub async fn maybe_get_raw_data(
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
                conjure_http::private::Option::Some("0.1.0"),
                "maybeGetRawData",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::decode_optional_binary_response(response_)
    }
    pub async fn get_aliased_string(
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
                conjure_http::private::Option::Some("0.1.0"),
                "getAliasedString",
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
        let mut request_ = conjure_http::private::encode_binary_request(input as _);
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
                conjure_http::private::Option::Some("0.1.0"),
                "uploadRawData",
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
        let mut request_ = conjure_http::private::encode_binary_request(input as _);
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
                conjure_http::private::Option::Some("0.1.0"),
                "uploadAliasedRawData",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_empty_response(response_).await
    }
    pub async fn get_branches(
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
                conjure_http::private::Option::Some("0.1.0"),
                "getBranches",
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
                conjure_http::private::Option::Some("0.1.0"),
                "getBranchesDeprecated",
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
                conjure_http::private::Option::Some("0.1.0"),
                "resolveBranch",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_default_serializable_response(response_).await
    }
    pub async fn test_param(
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
                conjure_http::private::Option::Some("0.1.0"),
                "testParam",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testQueryParams",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testNoResponseQueryParams",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_empty_response(response_).await
    }
    pub async fn test_boolean(
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
                conjure_http::private::Option::Some("0.1.0"),
                "testBoolean",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn test_double(
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
                conjure_http::private::Option::Some("0.1.0"),
                "testDouble",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn test_integer(
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
                conjure_http::private::Option::Some("0.1.0"),
                "testInteger",
            ));
        let response_ = self.0.send(request_).await?;
        conjure_http::private::async_decode_serializable_response(response_).await
    }
    pub async fn test_post_optional(
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
                conjure_http::private::Option::Some("0.1.0"),
                "testPostOptional",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testOptionalIntegerAndDouble",
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
                conjure_http::private::Option::Some("0.1.0"),
                "getFileSystems",
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
                conjure_http::private::Option::Some("0.1.0"),
                "createDataset",
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
                conjure_http::private::Option::Some("0.1.0"),
                "getDataset",
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
                conjure_http::private::Option::Some("0.1.0"),
                "getRawData",
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
                conjure_http::private::Option::Some("0.1.0"),
                "getAliasedRawData",
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
                conjure_http::private::Option::Some("0.1.0"),
                "maybeGetRawData",
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
                conjure_http::private::Option::Some("0.1.0"),
                "getAliasedString",
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
                conjure_http::private::Option::Some("0.1.0"),
                "uploadRawData",
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
                conjure_http::private::Option::Some("0.1.0"),
                "uploadAliasedRawData",
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
                conjure_http::private::Option::Some("0.1.0"),
                "getBranches",
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
                conjure_http::private::Option::Some("0.1.0"),
                "getBranchesDeprecated",
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
                conjure_http::private::Option::Some("0.1.0"),
                "resolveBranch",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testParam",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testQueryParams",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testNoResponseQueryParams",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testBoolean",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testDouble",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testInteger",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testPostOptional",
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
                conjure_http::private::Option::Some("0.1.0"),
                "testOptionalIntegerAndDouble",
            ));
        let response_ = self.0.send(request_)?;
        conjure_http::private::decode_empty_response(response_)
    }
}
use conjure_http::server::{AsyncResponse as _, Response as _};
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
pub trait AsyncTestService<I, O> {
    #[doc = "The body type returned by the `get_raw_data` method."]
    type GetRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    #[doc = "The body type returned by the `get_aliased_raw_data` method."]
    type GetAliasedRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    #[doc = "The body type returned by the `maybe_get_raw_data` method."]
    type MaybeGetRawDataBody: conjure_http::server::AsyncWriteBody<O> + 'static + Send;
    #[doc = "Returns a mapping from file system id to backing file system configuration."]
    fn get_file_systems<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<
                        std::collections::BTreeMap<
                            String,
                            super::super::product::datasets::BackingFileSystem,
                        >,
                        conjure_http::private::Error,
                    >,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn create_dataset<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        request: super::super::product::CreateDatasetRequest,
        test_header_arg: String,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<
                        super::super::product::datasets::Dataset,
                        conjure_http::private::Error,
                    >,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn get_dataset<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<
                        Option<super::super::product::datasets::Dataset>,
                        conjure_http::private::Error,
                    >,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn get_raw_data<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<Self::GetRawDataBody, conjure_http::private::Error>,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn get_aliased_raw_data<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<Self::GetAliasedRawDataBody, conjure_http::private::Error>,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn maybe_get_raw_data<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<
                        Option<Self::MaybeGetRawDataBody>,
                        conjure_http::private::Error,
                    >,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn get_aliased_string<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<
                        super::super::product::AliasedString,
                        conjure_http::private::Error,
                    >,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn upload_raw_data<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        input: I,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<Output = Result<(), conjure_http::private::Error>>
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn upload_aliased_raw_data<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        input: I,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<Output = Result<(), conjure_http::private::Error>>
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn get_branches<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<
                        std::collections::BTreeSet<String>,
                        conjure_http::private::Error,
                    >,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    #[doc = "Gets all branches of this dataset."]
    fn get_branches_deprecated<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<
                        std::collections::BTreeSet<String>,
                        conjure_http::private::Error,
                    >,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn resolve_branch<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
        branch: String,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<Option<String>, conjure_http::private::Error>,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn test_param<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        dataset_rid: conjure_object::ResourceIdentifier,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<Option<String>, conjure_http::private::Error>,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn test_query_params<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        query: String,
        something: conjure_object::ResourceIdentifier,
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        implicit: conjure_object::ResourceIdentifier,
        set_end: std::collections::BTreeSet<String>,
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<Output = Result<i32, conjure_http::private::Error>>
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn test_no_response_query_params<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        query: String,
        something: conjure_object::ResourceIdentifier,
        optional_middle: Option<conjure_object::ResourceIdentifier>,
        implicit: conjure_object::ResourceIdentifier,
        set_end: std::collections::BTreeSet<String>,
        optional_end: Option<conjure_object::ResourceIdentifier>,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<Output = Result<(), conjure_http::private::Error>>
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn test_boolean<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<Output = Result<bool, conjure_http::private::Error>>
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn test_double<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<Output = Result<f64, conjure_http::private::Error>>
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn test_integer<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<Output = Result<i32, conjure_http::private::Error>>
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn test_post_optional<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        maybe_string: Option<String>,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<Option<String>, conjure_http::private::Error>,
                >
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
    fn test_optional_integer_and_double<'life0, 'async_trait>(
        &'life0 self,
        auth_: conjure_object::BearerToken,
        maybe_integer: Option<i32>,
        maybe_double: Option<f64>,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<Output = Result<(), conjure_http::private::Error>>
                + 'async_trait
                + Send,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'life0;
}
pub struct TestServiceResource<T>(T);
impl<T> TestServiceResource<T> {
    #[doc = r" Creates a new resource."]
    pub fn new(handler: T) -> TestServiceResource<T> {
        TestServiceResource(handler)
    }
}
struct GetFileSystemsHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for GetFileSystemsHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.get_file_systems(auth_)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
}
struct CreateDatasetHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for CreateDatasetHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let test_header_arg =
            conjure_http::private::parse_required_header(headers_, "testHeaderArg", "Test-Header")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        let request = body_.accept(conjure_http::private::SerializableRequestBodyVisitor::new())?;
        let response = service_.0.create_dataset(auth_, request, test_header_arg)?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
}
struct GetDatasetHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for GetDatasetHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.get_dataset(auth_, dataset_rid)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
}
struct GetRawDataHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for GetRawDataHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.get_raw_data(auth_, dataset_rid)?;
        conjure_http::private::BinaryResponse(response).accept(response_visitor_)
    }
}
struct GetAliasedRawDataHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R>
    for GetAliasedRawDataHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.get_aliased_raw_data(auth_, dataset_rid)?;
        conjure_http::private::BinaryResponse(response).accept(response_visitor_)
    }
}
struct MaybeGetRawDataHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R>
    for MaybeGetRawDataHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.maybe_get_raw_data(auth_, dataset_rid)?;
        conjure_http::private::OptionalBinaryResponse(response).accept(response_visitor_)
    }
}
struct GetAliasedStringHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R>
    for GetAliasedStringHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.get_aliased_string(auth_, dataset_rid)?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
}
struct UploadRawDataHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for UploadRawDataHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        let input = body_.accept(conjure_http::private::BinaryRequestBodyVisitor)?;
        service_.0.upload_raw_data(auth_, input)?;
        conjure_http::private::EmptyResponse.accept(response_visitor_)
    }
}
struct UploadAliasedRawDataHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R>
    for UploadAliasedRawDataHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        let input = body_.accept(conjure_http::private::BinaryRequestBodyVisitor)?;
        service_.0.upload_aliased_raw_data(auth_, input)?;
        conjure_http::private::EmptyResponse.accept(response_visitor_)
    }
}
struct GetBranchesHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for GetBranchesHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.get_branches(auth_, dataset_rid)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
}
struct GetBranchesDeprecatedHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R>
    for GetBranchesDeprecatedHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.get_branches_deprecated(auth_, dataset_rid)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
}
struct ResolveBranchHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for ResolveBranchHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let branch = conjure_http::private::parse_path_param(path_params_, "branch")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.resolve_branch(auth_, dataset_rid, branch)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
}
struct TestParamHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for TestParamHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.test_param(auth_, dataset_rid)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
}
struct TestQueryParamsHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R>
    for TestQueryParamsHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        query_params_: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let something =
            conjure_http::private::parse_query_param(query_params_, "something", "different")?;
        let mut optional_middle: Option<conjure_object::ResourceIdentifier> = Default::default();
        conjure_http::private::parse_optional_query_param(
            query_params_,
            "optionalMiddle",
            "optionalMiddle",
            &mut optional_middle,
        )?;
        let implicit =
            conjure_http::private::parse_query_param(query_params_, "implicit", "implicit")?;
        let mut set_end: std::collections::BTreeSet<String> = Default::default();
        conjure_http::private::parse_set_query_param(
            query_params_,
            "setEnd",
            "setEnd",
            &mut set_end,
        )?;
        let mut optional_end: Option<conjure_object::ResourceIdentifier> = Default::default();
        conjure_http::private::parse_optional_query_param(
            query_params_,
            "optionalEnd",
            "optionalEnd",
            &mut optional_end,
        )?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        let query = body_.accept(conjure_http::private::SerializableRequestBodyVisitor::new())?;
        let response = service_.0.test_query_params(
            auth_,
            query,
            something,
            optional_middle,
            implicit,
            set_end,
            optional_end,
        )?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
}
struct TestNoResponseQueryParamsHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R>
    for TestNoResponseQueryParamsHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        query_params_: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let something =
            conjure_http::private::parse_query_param(query_params_, "something", "different")?;
        let mut optional_middle: Option<conjure_object::ResourceIdentifier> = Default::default();
        conjure_http::private::parse_optional_query_param(
            query_params_,
            "optionalMiddle",
            "optionalMiddle",
            &mut optional_middle,
        )?;
        let implicit =
            conjure_http::private::parse_query_param(query_params_, "implicit", "implicit")?;
        let mut set_end: std::collections::BTreeSet<String> = Default::default();
        conjure_http::private::parse_set_query_param(
            query_params_,
            "setEnd",
            "setEnd",
            &mut set_end,
        )?;
        let mut optional_end: Option<conjure_object::ResourceIdentifier> = Default::default();
        conjure_http::private::parse_optional_query_param(
            query_params_,
            "optionalEnd",
            "optionalEnd",
            &mut optional_end,
        )?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        let query = body_.accept(conjure_http::private::SerializableRequestBodyVisitor::new())?;
        service_.0.test_no_response_query_params(
            auth_,
            query,
            something,
            optional_middle,
            implicit,
            set_end,
            optional_end,
        )?;
        conjure_http::private::EmptyResponse.accept(response_visitor_)
    }
}
struct TestBooleanHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for TestBooleanHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.test_boolean(auth_)?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
}
struct TestDoubleHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for TestDoubleHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.test_double(auth_)?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
}
struct TestIntegerHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R> for TestIntegerHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = service_.0.test_integer(auth_)?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
}
struct TestPostOptionalHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R>
    for TestPostOptionalHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        let maybe_string =
            body_.accept(conjure_http::private::DefaultSerializableRequestBodyVisitor::new())?;
        let response = service_.0.test_post_optional(auth_, maybe_string)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
}
struct TestOptionalIntegerAndDoubleHandler_;
impl<T, B, R> conjure_http::server::Handler<TestServiceResource<T>, B, R>
    for TestOptionalIntegerAndDoubleHandler_
where
    T: TestService<B::BinaryBody, R::BinaryWriter>,
    B: conjure_http::server::RequestBody,
    R: conjure_http::server::VisitResponse,
{
    fn handle(
        &self,
        service_: &TestServiceResource<T>,
        _: &conjure_http::PathParams,
        query_params_: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error> {
        let mut maybe_integer: Option<i32> = Default::default();
        conjure_http::private::parse_optional_query_param(
            query_params_,
            "maybeInteger",
            "maybeInteger",
            &mut maybe_integer,
        )?;
        let mut maybe_double: Option<f64> = Default::default();
        conjure_http::private::parse_optional_query_param(
            query_params_,
            "maybeDouble",
            "maybeDouble",
            &mut maybe_double,
        )?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        service_
            .0
            .test_optional_integer_and_double(auth_, maybe_integer, maybe_double)?;
        conjure_http::private::EmptyResponse.accept(response_visitor_)
    }
}
impl<T, I, O> conjure_http::server::Resource<I, O> for TestServiceResource<T>
where
    T: TestService<I, O>,
{
    const NAME: &'static str = "TestService";
    fn endpoints<B, R>() -> Vec<conjure_http::server::Endpoint<Self, B, R>>
    where
        B: conjure_http::server::RequestBody<BinaryBody = I>,
        R: conjure_http::server::VisitResponse<BinaryWriter = O>,
    {
        vec![
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getFileSystems",
                    conjure_http::private::http::Method::GET,
                    "/catalog/fileSystems",
                    &[],
                    false,
                ),
                handler: &GetFileSystemsHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "createDataset",
                    conjure_http::private::http::Method::POST,
                    "/catalog/datasets",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "testHeaderArg",
                                conjure_http::server::ParameterType::Header(
                                    conjure_http::server::HeaderParameter::new("Test-Header"),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &CreateDatasetHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getDataset",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &GetDatasetHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getRawData",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/raw",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &GetRawDataHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getAliasedRawData",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/raw-aliased",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &GetAliasedRawDataHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "maybeGetRawData",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/raw-maybe",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &MaybeGetRawDataHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getAliasedString",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/string-aliased",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &GetAliasedStringHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "uploadRawData",
                    conjure_http::private::http::Method::POST,
                    "/catalog/datasets/upload-raw",
                    &[],
                    false,
                ),
                handler: &UploadRawDataHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "uploadAliasedRawData",
                    conjure_http::private::http::Method::POST,
                    "/catalog/datasets/upload-raw-aliased",
                    &[],
                    false,
                ),
                handler: &UploadAliasedRawDataHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getBranches",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/branches",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &GetBranchesHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getBranchesDeprecated",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/branchesDeprecated",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    true,
                ),
                handler: &GetBranchesDeprecatedHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "resolveBranch",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] = &[
                            conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "branch",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            ),
                        ];
                        PARAMS
                    },
                    false,
                ),
                handler: &ResolveBranchHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testParam",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/testParam",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &TestParamHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testQueryParams",
                    conjure_http::private::http::Method::POST,
                    "/catalog/test-query-params",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] = &[
                            conjure_http::server::Parameter::new(
                                "something",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("different"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "optionalMiddle",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("optionalMiddle"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "implicit",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("implicit"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "setEnd",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("setEnd"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "optionalEnd",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("optionalEnd"),
                                ),
                            ),
                        ];
                        PARAMS
                    },
                    false,
                ),
                handler: &TestQueryParamsHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testNoResponseQueryParams",
                    conjure_http::private::http::Method::POST,
                    "/catalog/test-no-response-query-params",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] = &[
                            conjure_http::server::Parameter::new(
                                "something",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("different"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "optionalMiddle",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("optionalMiddle"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "implicit",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("implicit"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "setEnd",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("setEnd"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "optionalEnd",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("optionalEnd"),
                                ),
                            ),
                        ];
                        PARAMS
                    },
                    false,
                ),
                handler: &TestNoResponseQueryParamsHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testBoolean",
                    conjure_http::private::http::Method::GET,
                    "/catalog/boolean",
                    &[],
                    false,
                ),
                handler: &TestBooleanHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testDouble",
                    conjure_http::private::http::Method::GET,
                    "/catalog/double",
                    &[],
                    false,
                ),
                handler: &TestDoubleHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testInteger",
                    conjure_http::private::http::Method::GET,
                    "/catalog/integer",
                    &[],
                    false,
                ),
                handler: &TestIntegerHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testPostOptional",
                    conjure_http::private::http::Method::POST,
                    "/catalog/optional",
                    &[],
                    false,
                ),
                handler: &TestPostOptionalHandler_,
            },
            conjure_http::server::Endpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testOptionalIntegerAndDouble",
                    conjure_http::private::http::Method::GET,
                    "/catalog/optional-integer-double",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] = &[
                            conjure_http::server::Parameter::new(
                                "maybeInteger",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("maybeInteger"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "maybeDouble",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("maybeDouble"),
                                ),
                            ),
                        ];
                        PARAMS
                    },
                    false,
                ),
                handler: &TestOptionalIntegerAndDoubleHandler_,
            },
        ]
    }
}
struct GetFileSystemsHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for GetFileSystemsHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.get_file_systems(auth_).await?;
            conjure_http::private::AsyncDefaultSerializableResponse(response)
                .accept(response_visitor_)
        })
    }
}
struct CreateDatasetHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for CreateDatasetHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let test_header_arg = conjure_http::private::parse_required_header(
                headers_,
                "testHeaderArg",
                "Test-Header",
            )?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            let request =
                body_.accept(conjure_http::private::SerializableRequestBodyVisitor::new())?;
            let response = service_
                .0
                .create_dataset(auth_, request, test_header_arg)
                .await?;
            conjure_http::private::AsyncSerializableResponse(response).accept(response_visitor_)
        })
    }
}
struct GetDatasetHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for GetDatasetHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        path_params_: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.get_dataset(auth_, dataset_rid).await?;
            conjure_http::private::AsyncDefaultSerializableResponse(response)
                .accept(response_visitor_)
        })
    }
}
struct GetRawDataHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for GetRawDataHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        path_params_: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.get_raw_data(auth_, dataset_rid).await?;
            conjure_http::private::AsyncBinaryResponse(response).accept(response_visitor_)
        })
    }
}
struct GetAliasedRawDataHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for GetAliasedRawDataHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        path_params_: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.get_aliased_raw_data(auth_, dataset_rid).await?;
            conjure_http::private::AsyncBinaryResponse(response).accept(response_visitor_)
        })
    }
}
struct MaybeGetRawDataHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for MaybeGetRawDataHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        path_params_: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.maybe_get_raw_data(auth_, dataset_rid).await?;
            conjure_http::private::AsyncOptionalBinaryResponse(response).accept(response_visitor_)
        })
    }
}
struct GetAliasedStringHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for GetAliasedStringHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        path_params_: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.get_aliased_string(auth_, dataset_rid).await?;
            conjure_http::private::AsyncSerializableResponse(response).accept(response_visitor_)
        })
    }
}
struct UploadRawDataHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for UploadRawDataHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            let input = body_.accept(conjure_http::private::BinaryRequestBodyVisitor)?;
            service_.0.upload_raw_data(auth_, input).await?;
            conjure_http::private::AsyncEmptyResponse.accept(response_visitor_)
        })
    }
}
struct UploadAliasedRawDataHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for UploadAliasedRawDataHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            let input = body_.accept(conjure_http::private::BinaryRequestBodyVisitor)?;
            service_.0.upload_aliased_raw_data(auth_, input).await?;
            conjure_http::private::AsyncEmptyResponse.accept(response_visitor_)
        })
    }
}
struct GetBranchesHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for GetBranchesHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        path_params_: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.get_branches(auth_, dataset_rid).await?;
            conjure_http::private::AsyncDefaultSerializableResponse(response)
                .accept(response_visitor_)
        })
    }
}
struct GetBranchesDeprecatedHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for GetBranchesDeprecatedHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        path_params_: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_
                .0
                .get_branches_deprecated(auth_, dataset_rid)
                .await?;
            conjure_http::private::AsyncDefaultSerializableResponse(response)
                .accept(response_visitor_)
        })
    }
}
struct ResolveBranchHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for ResolveBranchHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        path_params_: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
            let branch = conjure_http::private::parse_path_param(path_params_, "branch")?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_
                .0
                .resolve_branch(auth_, dataset_rid, branch)
                .await?;
            conjure_http::private::AsyncDefaultSerializableResponse(response)
                .accept(response_visitor_)
        })
    }
}
struct TestParamHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for TestParamHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        path_params_: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.test_param(auth_, dataset_rid).await?;
            conjure_http::private::AsyncDefaultSerializableResponse(response)
                .accept(response_visitor_)
        })
    }
}
struct TestQueryParamsHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for TestQueryParamsHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        query_params_: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let something =
                conjure_http::private::parse_query_param(query_params_, "something", "different")?;
            let mut optional_middle: Option<conjure_object::ResourceIdentifier> =
                Default::default();
            conjure_http::private::parse_optional_query_param(
                query_params_,
                "optionalMiddle",
                "optionalMiddle",
                &mut optional_middle,
            )?;
            let implicit =
                conjure_http::private::parse_query_param(query_params_, "implicit", "implicit")?;
            let mut set_end: std::collections::BTreeSet<String> = Default::default();
            conjure_http::private::parse_set_query_param(
                query_params_,
                "setEnd",
                "setEnd",
                &mut set_end,
            )?;
            let mut optional_end: Option<conjure_object::ResourceIdentifier> = Default::default();
            conjure_http::private::parse_optional_query_param(
                query_params_,
                "optionalEnd",
                "optionalEnd",
                &mut optional_end,
            )?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            let query =
                body_.accept(conjure_http::private::SerializableRequestBodyVisitor::new())?;
            let response = service_
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
            conjure_http::private::AsyncSerializableResponse(response).accept(response_visitor_)
        })
    }
}
struct TestNoResponseQueryParamsHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for TestNoResponseQueryParamsHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        query_params_: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let something =
                conjure_http::private::parse_query_param(query_params_, "something", "different")?;
            let mut optional_middle: Option<conjure_object::ResourceIdentifier> =
                Default::default();
            conjure_http::private::parse_optional_query_param(
                query_params_,
                "optionalMiddle",
                "optionalMiddle",
                &mut optional_middle,
            )?;
            let implicit =
                conjure_http::private::parse_query_param(query_params_, "implicit", "implicit")?;
            let mut set_end: std::collections::BTreeSet<String> = Default::default();
            conjure_http::private::parse_set_query_param(
                query_params_,
                "setEnd",
                "setEnd",
                &mut set_end,
            )?;
            let mut optional_end: Option<conjure_object::ResourceIdentifier> = Default::default();
            conjure_http::private::parse_optional_query_param(
                query_params_,
                "optionalEnd",
                "optionalEnd",
                &mut optional_end,
            )?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            let query =
                body_.accept(conjure_http::private::SerializableRequestBodyVisitor::new())?;
            service_
                .0
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
            conjure_http::private::AsyncEmptyResponse.accept(response_visitor_)
        })
    }
}
struct TestBooleanHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for TestBooleanHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.test_boolean(auth_).await?;
            conjure_http::private::AsyncSerializableResponse(response).accept(response_visitor_)
        })
    }
}
struct TestDoubleHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for TestDoubleHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.test_double(auth_).await?;
            conjure_http::private::AsyncSerializableResponse(response).accept(response_visitor_)
        })
    }
}
struct TestIntegerHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for TestIntegerHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            let response = service_.0.test_integer(auth_).await?;
            conjure_http::private::AsyncSerializableResponse(response).accept(response_visitor_)
        })
    }
}
struct TestPostOptionalHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for TestPostOptionalHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        _: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            let maybe_string = body_
                .accept(conjure_http::private::DefaultSerializableRequestBodyVisitor::new())?;
            let response = service_.0.test_post_optional(auth_, maybe_string).await?;
            conjure_http::private::AsyncDefaultSerializableResponse(response)
                .accept(response_visitor_)
        })
    }
}
struct TestOptionalIntegerAndDoubleHandlerAsync_;
impl<T, B, R> conjure_http::server::AsyncHandler<TestServiceResource<T>, B, R>
    for TestOptionalIntegerAndDoubleHandlerAsync_
where
    T: AsyncTestService<B::BinaryBody, R::BinaryWriter> + Sync + Send,
    B: conjure_http::server::RequestBody + Send,
    B::BinaryBody: Send,
    R: conjure_http::server::AsyncVisitResponse + Send,
{
    fn handle<'a>(
        &self,
        service_: &'a TestServiceResource<T>,
        _: &'a conjure_http::PathParams,
        query_params_: &'a conjure_http::QueryParams,
        headers_: &'a conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> conjure_http::private::Pin<
        Box<
            dyn conjure_http::private::Future<
                    Output = Result<R::Output, conjure_http::private::Error>,
                > + Send
                + 'a,
        >,
    >
    where
        T: 'a,
        B: 'a,
        R: 'a,
    {
        Box::pin(async move {
            let mut maybe_integer: Option<i32> = Default::default();
            conjure_http::private::parse_optional_query_param(
                query_params_,
                "maybeInteger",
                "maybeInteger",
                &mut maybe_integer,
            )?;
            let mut maybe_double: Option<f64> = Default::default();
            conjure_http::private::parse_optional_query_param(
                query_params_,
                "maybeDouble",
                "maybeDouble",
                &mut maybe_double,
            )?;
            let auth_ = conjure_http::private::parse_header_auth(headers_)?;
            body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            service_
                .0
                .test_optional_integer_and_double(auth_, maybe_integer, maybe_double)
                .await?;
            conjure_http::private::AsyncEmptyResponse.accept(response_visitor_)
        })
    }
}
impl<T, I, O> conjure_http::server::AsyncResource<I, O> for TestServiceResource<T>
where
    T: AsyncTestService<I, O> + Sync + Send,
    I: Send,
{
    const NAME: &'static str = "TestService";
    fn endpoints<B, R>() -> Vec<conjure_http::server::AsyncEndpoint<Self, B, R>>
    where
        B: conjure_http::server::RequestBody<BinaryBody = I> + Send,
        R: conjure_http::server::AsyncVisitResponse<BinaryWriter = O> + Send,
    {
        vec![
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getFileSystems",
                    conjure_http::private::http::Method::GET,
                    "/catalog/fileSystems",
                    &[],
                    false,
                ),
                handler: &GetFileSystemsHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "createDataset",
                    conjure_http::private::http::Method::POST,
                    "/catalog/datasets",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "testHeaderArg",
                                conjure_http::server::ParameterType::Header(
                                    conjure_http::server::HeaderParameter::new("Test-Header"),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &CreateDatasetHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getDataset",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &GetDatasetHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getRawData",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/raw",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &GetRawDataHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getAliasedRawData",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/raw-aliased",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &GetAliasedRawDataHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "maybeGetRawData",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/raw-maybe",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &MaybeGetRawDataHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getAliasedString",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/string-aliased",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &GetAliasedStringHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "uploadRawData",
                    conjure_http::private::http::Method::POST,
                    "/catalog/datasets/upload-raw",
                    &[],
                    false,
                ),
                handler: &UploadRawDataHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "uploadAliasedRawData",
                    conjure_http::private::http::Method::POST,
                    "/catalog/datasets/upload-raw-aliased",
                    &[],
                    false,
                ),
                handler: &UploadAliasedRawDataHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getBranches",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/branches",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &GetBranchesHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "getBranchesDeprecated",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/branchesDeprecated",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    true,
                ),
                handler: &GetBranchesDeprecatedHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "resolveBranch",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] = &[
                            conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "branch",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            ),
                        ];
                        PARAMS
                    },
                    false,
                ),
                handler: &ResolveBranchHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testParam",
                    conjure_http::private::http::Method::GET,
                    "/catalog/datasets/{datasetRid}/testParam",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] =
                            &[conjure_http::server::Parameter::new(
                                "datasetRid",
                                conjure_http::server::ParameterType::Path(
                                    conjure_http::server::PathParameter::new(),
                                ),
                            )];
                        PARAMS
                    },
                    false,
                ),
                handler: &TestParamHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testQueryParams",
                    conjure_http::private::http::Method::POST,
                    "/catalog/test-query-params",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] = &[
                            conjure_http::server::Parameter::new(
                                "something",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("different"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "optionalMiddle",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("optionalMiddle"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "implicit",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("implicit"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "setEnd",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("setEnd"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "optionalEnd",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("optionalEnd"),
                                ),
                            ),
                        ];
                        PARAMS
                    },
                    false,
                ),
                handler: &TestQueryParamsHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testNoResponseQueryParams",
                    conjure_http::private::http::Method::POST,
                    "/catalog/test-no-response-query-params",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] = &[
                            conjure_http::server::Parameter::new(
                                "something",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("different"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "optionalMiddle",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("optionalMiddle"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "implicit",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("implicit"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "setEnd",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("setEnd"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "optionalEnd",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("optionalEnd"),
                                ),
                            ),
                        ];
                        PARAMS
                    },
                    false,
                ),
                handler: &TestNoResponseQueryParamsHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testBoolean",
                    conjure_http::private::http::Method::GET,
                    "/catalog/boolean",
                    &[],
                    false,
                ),
                handler: &TestBooleanHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testDouble",
                    conjure_http::private::http::Method::GET,
                    "/catalog/double",
                    &[],
                    false,
                ),
                handler: &TestDoubleHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testInteger",
                    conjure_http::private::http::Method::GET,
                    "/catalog/integer",
                    &[],
                    false,
                ),
                handler: &TestIntegerHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testPostOptional",
                    conjure_http::private::http::Method::POST,
                    "/catalog/optional",
                    &[],
                    false,
                ),
                handler: &TestPostOptionalHandlerAsync_,
            },
            conjure_http::server::AsyncEndpoint {
                metadata: conjure_http::server::Metadata::new(
                    "testOptionalIntegerAndDouble",
                    conjure_http::private::http::Method::GET,
                    "/catalog/optional-integer-double",
                    {
                        const PARAMS: &[conjure_http::server::Parameter] = &[
                            conjure_http::server::Parameter::new(
                                "maybeInteger",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("maybeInteger"),
                                ),
                            ),
                            conjure_http::server::Parameter::new(
                                "maybeDouble",
                                conjure_http::server::ParameterType::Query(
                                    conjure_http::server::QueryParameter::new("maybeDouble"),
                                ),
                            ),
                        ];
                        PARAMS
                    },
                    false,
                ),
                handler: &TestOptionalIntegerAndDoubleHandlerAsync_,
            },
        ]
    }
}
