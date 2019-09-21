#[doc = "A Markdown description of the service."]
#[derive(Clone, Debug)]
pub struct TestServiceAsyncClient<T>(T);
impl<T> TestServiceAsyncClient<T>
where
    T: conjure_http::client::AsyncClient,
{
    #[doc = r" Creates a new client."]
    #[inline]
    pub fn new(client: T) -> TestServiceAsyncClient<T> {
        TestServiceAsyncClient(client)
    }
    #[doc = "Returns a mapping from file system id to backing file system configuration."]
    pub async fn get_file_systems(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<String, super::super::product::datasets::BackingFileSystem>,
        conjure_http::private::Error,
    > {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/fileSystems",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn create_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        request: &super::super::product::CreateDatasetRequest,
        test_header_arg: &str,
    ) -> Result<super::super::product::datasets::Dataset, conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        conjure_http::private::encode_header(
            &mut headers_,
            "testHeaderArg",
            "test-header",
            test_header_arg,
        )?;
        let body_ = conjure_http::private::SerializableRequestBody(request);
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::POST,
                "/catalog/datasets",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn get_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<super::super::product::datasets::Dataset>, conjure_http::private::Error>
    {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn get_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<T::BinaryBody, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::BinaryResponseVisitor;
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/raw",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn get_aliased_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<T::BinaryBody, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::BinaryResponseVisitor;
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/raw-aliased",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn maybe_get_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<T::BinaryBody>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::OptionalBinaryResponseVisitor;
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/raw-maybe",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn get_aliased_string(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<super::super::product::AliasedString, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/string-aliased",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn upload_raw_data<U>(
        &self,
        auth_: &conjure_object::BearerToken,
        input: U,
    ) -> Result<(), conjure_http::private::Error>
    where
        U: conjure_http::client::AsyncWriteBody<T::BinaryWriter> + Sync + Send,
    {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::BinaryRequestBody(input);
        let response_visitor_ = conjure_http::private::EmptyResponseVisitor;
        self.0
            .request(
                conjure_http::private::http::Method::POST,
                "/catalog/datasets/upload-raw",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn upload_aliased_raw_data<U>(
        &self,
        auth_: &conjure_object::BearerToken,
        input: U,
    ) -> Result<(), conjure_http::private::Error>
    where
        U: conjure_http::client::AsyncWriteBody<T::BinaryWriter> + Sync + Send,
    {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::BinaryRequestBody(input);
        let response_visitor_ = conjure_http::private::EmptyResponseVisitor;
        self.0
            .request(
                conjure_http::private::http::Method::POST,
                "/catalog/datasets/upload-raw-aliased",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn get_branches(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/branches",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    #[doc = "Gets all branches of this dataset."]
    #[deprecated(note = "use getBranches instead")]
    pub async fn get_branches_deprecated(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/branchesDeprecated",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn resolve_branch(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
        branch: &str,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        conjure_http::private::encode_path_param(&mut path_params_, "branch", branch);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn test_param(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/testParam",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
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
        let path_params_ = conjure_http::PathParams::new();
        let mut query_params_ = conjure_http::QueryParams::new();
        conjure_http::private::encode_query_param(&mut query_params_, "different", something);
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "optionalMiddle",
            &optional_middle,
        );
        conjure_http::private::encode_query_param(&mut query_params_, "implicit", implicit);
        conjure_http::private::encode_set_query_param(&mut query_params_, "setEnd", &set_end);
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "optionalEnd",
            &optional_end,
        );
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::SerializableRequestBody(query);
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::POST,
                "/catalog/test-query-params",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
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
        let path_params_ = conjure_http::PathParams::new();
        let mut query_params_ = conjure_http::QueryParams::new();
        conjure_http::private::encode_query_param(&mut query_params_, "different", something);
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "optionalMiddle",
            &optional_middle,
        );
        conjure_http::private::encode_query_param(&mut query_params_, "implicit", implicit);
        conjure_http::private::encode_set_query_param(&mut query_params_, "setEnd", &set_end);
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "optionalEnd",
            &optional_end,
        );
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::SerializableRequestBody(query);
        let response_visitor_ = conjure_http::private::EmptyResponseVisitor;
        self.0
            .request(
                conjure_http::private::http::Method::POST,
                "/catalog/test-no-response-query-params",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn test_boolean(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/boolean",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn test_double(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/double",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn test_integer(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/integer",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn test_post_optional(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_string: Option<&str>,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::SerializableRequestBody(maybe_string);
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0
            .request(
                conjure_http::private::http::Method::POST,
                "/catalog/optional",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
    pub async fn test_optional_integer_and_double(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_integer: Option<i32>,
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let mut query_params_ = conjure_http::QueryParams::new();
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "maybeInteger",
            &maybe_integer,
        );
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "maybeDouble",
            &maybe_double,
        );
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::EmptyResponseVisitor;
        self.0
            .request(
                conjure_http::private::http::Method::GET,
                "/catalog/optional-integer-double",
                path_params_,
                query_params_,
                headers_,
                body_,
                response_visitor_,
            )
            .await
    }
}
#[doc = "A Markdown description of the service."]
#[derive(Clone, Debug)]
pub struct TestServiceClient<T>(T);
impl<T> TestServiceClient<T>
where
    T: conjure_http::client::Client,
{
    #[doc = r" Creates a new client."]
    #[inline]
    pub fn new(client: T) -> TestServiceClient<T> {
        TestServiceClient(client)
    }
    #[doc = "Returns a mapping from file system id to backing file system configuration."]
    pub fn get_file_systems(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<
        std::collections::BTreeMap<String, super::super::product::datasets::BackingFileSystem>,
        conjure_http::private::Error,
    > {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/fileSystems",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn create_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        request: &super::super::product::CreateDatasetRequest,
        test_header_arg: &str,
    ) -> Result<super::super::product::datasets::Dataset, conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        conjure_http::private::encode_header(
            &mut headers_,
            "testHeaderArg",
            "test-header",
            test_header_arg,
        )?;
        let body_ = conjure_http::private::SerializableRequestBody(request);
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::POST,
            "/catalog/datasets",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn get_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<super::super::product::datasets::Dataset>, conjure_http::private::Error>
    {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/datasets/{datasetRid}",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn get_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<T::BinaryBody, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::BinaryResponseVisitor;
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/datasets/{datasetRid}/raw",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn get_aliased_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<T::BinaryBody, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::BinaryResponseVisitor;
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/datasets/{datasetRid}/raw-aliased",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn maybe_get_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<T::BinaryBody>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::OptionalBinaryResponseVisitor;
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/datasets/{datasetRid}/raw-maybe",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn get_aliased_string(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<super::super::product::AliasedString, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/datasets/{datasetRid}/string-aliased",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn upload_raw_data<U>(
        &self,
        auth_: &conjure_object::BearerToken,
        input: U,
    ) -> Result<(), conjure_http::private::Error>
    where
        U: conjure_http::client::WriteBody<T::BinaryWriter>,
    {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::BinaryRequestBody(input);
        let response_visitor_ = conjure_http::private::EmptyResponseVisitor;
        self.0.request(
            conjure_http::private::http::Method::POST,
            "/catalog/datasets/upload-raw",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn upload_aliased_raw_data<U>(
        &self,
        auth_: &conjure_object::BearerToken,
        input: U,
    ) -> Result<(), conjure_http::private::Error>
    where
        U: conjure_http::client::WriteBody<T::BinaryWriter>,
    {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::BinaryRequestBody(input);
        let response_visitor_ = conjure_http::private::EmptyResponseVisitor;
        self.0.request(
            conjure_http::private::http::Method::POST,
            "/catalog/datasets/upload-raw-aliased",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn get_branches(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/datasets/{datasetRid}/branches",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    #[doc = "Gets all branches of this dataset."]
    #[deprecated(note = "use getBranches instead")]
    pub fn get_branches_deprecated(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/datasets/{datasetRid}/branchesDeprecated",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn resolve_branch(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
        branch: &str,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        conjure_http::private::encode_path_param(&mut path_params_, "branch", branch);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn test_param(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        conjure_http::private::encode_path_param(&mut path_params_, "datasetRid", dataset_rid);
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/datasets/{datasetRid}/testParam",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
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
        let path_params_ = conjure_http::PathParams::new();
        let mut query_params_ = conjure_http::QueryParams::new();
        conjure_http::private::encode_query_param(&mut query_params_, "different", something);
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "optionalMiddle",
            &optional_middle,
        );
        conjure_http::private::encode_query_param(&mut query_params_, "implicit", implicit);
        conjure_http::private::encode_set_query_param(&mut query_params_, "setEnd", &set_end);
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "optionalEnd",
            &optional_end,
        );
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::SerializableRequestBody(query);
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::POST,
            "/catalog/test-query-params",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
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
        let path_params_ = conjure_http::PathParams::new();
        let mut query_params_ = conjure_http::QueryParams::new();
        conjure_http::private::encode_query_param(&mut query_params_, "different", something);
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "optionalMiddle",
            &optional_middle,
        );
        conjure_http::private::encode_query_param(&mut query_params_, "implicit", implicit);
        conjure_http::private::encode_set_query_param(&mut query_params_, "setEnd", &set_end);
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "optionalEnd",
            &optional_end,
        );
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::SerializableRequestBody(query);
        let response_visitor_ = conjure_http::private::EmptyResponseVisitor;
        self.0.request(
            conjure_http::private::http::Method::POST,
            "/catalog/test-no-response-query-params",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn test_boolean(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/boolean",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn test_double(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/double",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn test_integer(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::SerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/integer",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn test_post_optional(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_string: Option<&str>,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::SerializableRequestBody(maybe_string);
        let response_visitor_ = conjure_http::private::DefaultSerializableResponseVisitor::new();
        self.0.request(
            conjure_http::private::http::Method::POST,
            "/catalog/optional",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
    pub fn test_optional_integer_and_double(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_integer: Option<i32>,
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error> {
        let path_params_ = conjure_http::PathParams::new();
        let mut query_params_ = conjure_http::QueryParams::new();
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "maybeInteger",
            &maybe_integer,
        );
        conjure_http::private::encode_optional_query_param(
            &mut query_params_,
            "maybeDouble",
            &maybe_double,
        );
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        conjure_http::private::encode_header_auth(&mut headers_, auth_);
        let body_ = conjure_http::private::EmptyRequestBody;
        let response_visitor_ = conjure_http::private::EmptyResponseVisitor;
        self.0.request(
            conjure_http::private::http::Method::GET,
            "/catalog/optional-integer-double",
            path_params_,
            query_params_,
            headers_,
            body_,
            response_visitor_,
        )
    }
}
use conjure_http::server::Response as _;
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
pub struct TestServiceResource<T>(T);
impl<T> TestServiceResource<T> {
    #[doc = r" Creates a new resource."]
    pub fn new(handler: T) -> TestServiceResource<T> {
        TestServiceResource(handler)
    }
}
impl<T> TestServiceResource<T> {
    fn get_file_systems<B, R>(
        &self,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).get_file_systems(auth_)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
    fn create_dataset<B, R>(
        &self,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let test_header_arg =
            conjure_http::private::parse_required_header(headers_, "testHeaderArg", "Test-Header")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        let request = body_.accept(conjure_http::private::SerializableRequestBodyVisitor::new())?;
        let response = (self.0).create_dataset(auth_, request, test_header_arg)?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
    fn get_dataset<B, R>(
        &self,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).get_dataset(auth_, dataset_rid)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
    fn get_raw_data<B, R>(
        &self,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).get_raw_data(auth_, dataset_rid)?;
        conjure_http::private::BinaryResponse(response).accept(response_visitor_)
    }
    fn get_aliased_raw_data<B, R>(
        &self,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).get_aliased_raw_data(auth_, dataset_rid)?;
        conjure_http::private::BinaryResponse(response).accept(response_visitor_)
    }
    fn maybe_get_raw_data<B, R>(
        &self,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).maybe_get_raw_data(auth_, dataset_rid)?;
        conjure_http::private::OptionalBinaryResponse(response).accept(response_visitor_)
    }
    fn get_aliased_string<B, R>(
        &self,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).get_aliased_string(auth_, dataset_rid)?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
    fn upload_raw_data<B, R>(
        &self,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        let input = body_.accept(conjure_http::private::BinaryRequestBodyVisitor)?;
        (self.0).upload_raw_data(auth_, input)?;
        conjure_http::private::EmptyResponse.accept(response_visitor_)
    }
    fn upload_aliased_raw_data<B, R>(
        &self,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        let input = body_.accept(conjure_http::private::BinaryRequestBodyVisitor)?;
        (self.0).upload_aliased_raw_data(auth_, input)?;
        conjure_http::private::EmptyResponse.accept(response_visitor_)
    }
    fn get_branches<B, R>(
        &self,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).get_branches(auth_, dataset_rid)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
    fn get_branches_deprecated<B, R>(
        &self,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).get_branches_deprecated(auth_, dataset_rid)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
    fn resolve_branch<B, R>(
        &self,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let branch = conjure_http::private::parse_path_param(path_params_, "branch")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).resolve_branch(auth_, dataset_rid, branch)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
    fn test_param<B, R>(
        &self,
        path_params_: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let dataset_rid = conjure_http::private::parse_path_param(path_params_, "datasetRid")?;
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).test_param(auth_, dataset_rid)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
    fn test_query_params<B, R>(
        &self,
        _: &conjure_http::PathParams,
        query_params_: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
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
        let response = (self.0).test_query_params(
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
    fn test_no_response_query_params<B, R>(
        &self,
        _: &conjure_http::PathParams,
        query_params_: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
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
        (self.0).test_no_response_query_params(
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
    fn test_boolean<B, R>(
        &self,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).test_boolean(auth_)?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
    fn test_double<B, R>(
        &self,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).test_double(auth_)?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
    fn test_integer<B, R>(
        &self,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        body_.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
        let response = (self.0).test_integer(auth_)?;
        conjure_http::private::SerializableResponse(response).accept(response_visitor_)
    }
    fn test_post_optional<B, R>(
        &self,
        _: &conjure_http::PathParams,
        _: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
        let auth_ = conjure_http::private::parse_header_auth(headers_)?;
        let maybe_string =
            body_.accept(conjure_http::private::DefaultSerializableRequestBodyVisitor::new())?;
        let response = (self.0).test_post_optional(auth_, maybe_string)?;
        conjure_http::private::DefaultSerializableResponse(response).accept(response_visitor_)
    }
    fn test_optional_integer_and_double<B, R>(
        &self,
        _: &conjure_http::PathParams,
        query_params_: &conjure_http::QueryParams,
        headers_: &conjure_http::private::http::HeaderMap,
        body_: B,
        response_visitor_: R,
    ) -> Result<R::Output, conjure_http::private::Error>
    where
        T: TestService<B::BinaryBody, R::BinaryWriter>,
        B: conjure_http::server::RequestBody,
        R: conjure_http::server::VisitResponse,
    {
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
        (self.0).test_optional_integer_and_double(auth_, maybe_integer, maybe_double)?;
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
            conjure_http::server::Endpoint::new(
                "getFileSystems",
                conjure_http::private::http::Method::GET,
                "/catalog/fileSystems",
                TestServiceResource::get_file_systems,
                &[],
            ),
            conjure_http::server::Endpoint::new(
                "createDataset",
                conjure_http::private::http::Method::POST,
                "/catalog/datasets",
                TestServiceResource::create_dataset,
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
            ),
            conjure_http::server::Endpoint::new(
                "getDataset",
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}",
                TestServiceResource::get_dataset,
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
            ),
            conjure_http::server::Endpoint::new(
                "getRawData",
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/raw",
                TestServiceResource::get_raw_data,
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
            ),
            conjure_http::server::Endpoint::new(
                "getAliasedRawData",
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/raw-aliased",
                TestServiceResource::get_aliased_raw_data,
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
            ),
            conjure_http::server::Endpoint::new(
                "maybeGetRawData",
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/raw-maybe",
                TestServiceResource::maybe_get_raw_data,
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
            ),
            conjure_http::server::Endpoint::new(
                "getAliasedString",
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/string-aliased",
                TestServiceResource::get_aliased_string,
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
            ),
            conjure_http::server::Endpoint::new(
                "uploadRawData",
                conjure_http::private::http::Method::POST,
                "/catalog/datasets/upload-raw",
                TestServiceResource::upload_raw_data,
                &[],
            ),
            conjure_http::server::Endpoint::new(
                "uploadAliasedRawData",
                conjure_http::private::http::Method::POST,
                "/catalog/datasets/upload-raw-aliased",
                TestServiceResource::upload_aliased_raw_data,
                &[],
            ),
            conjure_http::server::Endpoint::new(
                "getBranches",
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/branches",
                TestServiceResource::get_branches,
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
            ),
            conjure_http::server::Endpoint::new(
                "getBranchesDeprecated",
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/branchesDeprecated",
                TestServiceResource::get_branches_deprecated,
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
            )
            .with_deprecated(true),
            conjure_http::server::Endpoint::new(
                "resolveBranch",
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/branches/{branch:.+}/resolve",
                TestServiceResource::resolve_branch,
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
            ),
            conjure_http::server::Endpoint::new(
                "testParam",
                conjure_http::private::http::Method::GET,
                "/catalog/datasets/{datasetRid}/testParam",
                TestServiceResource::test_param,
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
            ),
            conjure_http::server::Endpoint::new(
                "testQueryParams",
                conjure_http::private::http::Method::POST,
                "/catalog/test-query-params",
                TestServiceResource::test_query_params,
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
            ),
            conjure_http::server::Endpoint::new(
                "testNoResponseQueryParams",
                conjure_http::private::http::Method::POST,
                "/catalog/test-no-response-query-params",
                TestServiceResource::test_no_response_query_params,
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
            ),
            conjure_http::server::Endpoint::new(
                "testBoolean",
                conjure_http::private::http::Method::GET,
                "/catalog/boolean",
                TestServiceResource::test_boolean,
                &[],
            ),
            conjure_http::server::Endpoint::new(
                "testDouble",
                conjure_http::private::http::Method::GET,
                "/catalog/double",
                TestServiceResource::test_double,
                &[],
            ),
            conjure_http::server::Endpoint::new(
                "testInteger",
                conjure_http::private::http::Method::GET,
                "/catalog/integer",
                TestServiceResource::test_integer,
                &[],
            ),
            conjure_http::server::Endpoint::new(
                "testPostOptional",
                conjure_http::private::http::Method::POST,
                "/catalog/optional",
                TestServiceResource::test_post_optional,
                &[],
            ),
            conjure_http::server::Endpoint::new(
                "testOptionalIntegerAndDouble",
                conjure_http::private::http::Method::GET,
                "/catalog/optional-integer-double",
                TestServiceResource::test_optional_integer_and_double,
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
            ),
        ]
    }
}
