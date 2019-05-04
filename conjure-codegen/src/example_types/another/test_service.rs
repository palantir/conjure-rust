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
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        headers_.insert(
            conjure_http::private::http::header::HeaderName::from_static("test-header"),
            conjure_http::private::http::header::HeaderValue::from_shared(
                conjure_object::ToPlain::to_plain(&test_header_arg).into(),
            )
            .map_err(conjure_http::private::Error::internal_safe)?,
        );
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
        path_params_.insert(
            "datasetRid",
            conjure_object::ToPlain::to_plain(&dataset_rid),
        );
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
    ) -> Result<T::ResponseBody, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        path_params_.insert(
            "datasetRid",
            conjure_object::ToPlain::to_plain(&dataset_rid),
        );
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
    ) -> Result<T::ResponseBody, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        path_params_.insert(
            "datasetRid",
            conjure_object::ToPlain::to_plain(&dataset_rid),
        );
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
    ) -> Result<Option<T::ResponseBody>, conjure_http::private::Error> {
        let mut path_params_ = conjure_http::PathParams::new();
        path_params_.insert(
            "datasetRid",
            conjure_object::ToPlain::to_plain(&dataset_rid),
        );
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        path_params_.insert(
            "datasetRid",
            conjure_object::ToPlain::to_plain(&dataset_rid),
        );
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        U: conjure_http::client::WriteBody,
    {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        U: conjure_http::client::WriteBody,
    {
        let path_params_ = conjure_http::PathParams::new();
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        path_params_.insert(
            "datasetRid",
            conjure_object::ToPlain::to_plain(&dataset_rid),
        );
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        path_params_.insert(
            "datasetRid",
            conjure_object::ToPlain::to_plain(&dataset_rid),
        );
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        path_params_.insert(
            "datasetRid",
            conjure_object::ToPlain::to_plain(&dataset_rid),
        );
        path_params_.insert("branch", conjure_object::ToPlain::to_plain(&branch));
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        path_params_.insert(
            "datasetRid",
            conjure_object::ToPlain::to_plain(&dataset_rid),
        );
        let query_params_ = conjure_http::QueryParams::new();
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        query_params_.insert("different", conjure_object::ToPlain::to_plain(&something));
        query_params_.insert_all(
            "optionalMiddle",
            optional_middle
                .iter()
                .map(conjure_object::ToPlain::to_plain),
        );
        query_params_.insert("implicit", conjure_object::ToPlain::to_plain(&implicit));
        query_params_.insert_all(
            "setEnd",
            set_end.iter().map(conjure_object::ToPlain::to_plain),
        );
        query_params_.insert_all(
            "optionalEnd",
            optional_end.iter().map(conjure_object::ToPlain::to_plain),
        );
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        query_params_.insert("different", conjure_object::ToPlain::to_plain(&something));
        query_params_.insert_all(
            "optionalMiddle",
            optional_middle
                .iter()
                .map(conjure_object::ToPlain::to_plain),
        );
        query_params_.insert("implicit", conjure_object::ToPlain::to_plain(&implicit));
        query_params_.insert_all(
            "setEnd",
            set_end.iter().map(conjure_object::ToPlain::to_plain),
        );
        query_params_.insert_all(
            "optionalEnd",
            optional_end.iter().map(conjure_object::ToPlain::to_plain),
        );
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
        query_params_.insert_all(
            "maybeInteger",
            maybe_integer.iter().map(conjure_object::ToPlain::to_plain),
        );
        query_params_.insert_all(
            "maybeDouble",
            maybe_double.iter().map(conjure_object::ToPlain::to_plain),
        );
        let mut headers_ = conjure_http::private::http::HeaderMap::new();
        headers_.insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
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
