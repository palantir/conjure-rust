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
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!("/catalog/fileSystems",);
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        if response.status() == conjure_http::private::http::StatusCode::NO_CONTENT {
            Ok(Default::default())
        } else {
            conjure_http::private::json::client_from_reader(response.body_mut())
                .map_err(conjure_http::private::Error::internal)
        }
    }
    pub fn create_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        request: &super::super::product::CreateDatasetRequest,
        test_header_arg: &str,
    ) -> Result<super::super::product::datasets::Dataset, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Fixed(
                conjure_http::private::json::to_vec(&request)
                    .map_err(conjure_http::private::Error::internal)?,
            ));
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let path_ = format!("/catalog/datasets",);
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::CONTENT_TYPE,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::HeaderName::from_static("Test-Header"),
            conjure_http::private::http::header::HeaderValue::from_shared(
                conjure_object::ToPlain::to_plain(&test_header_arg).into(),
            )
            .map_err(conjure_http::private::Error::internal_safe)?,
        );
        let mut response = self.0.request(request_)?;
        conjure_http::private::json::client_from_reader(response.body_mut())
            .map_err(conjure_http::private::Error::internal)
    }
    pub fn get_dataset(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<super::super::product::datasets::Dataset>, conjure_http::private::Error>
    {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!(
            "/catalog/datasets/{}",
            conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(&dataset_rid).as_bytes(),
                conjure_http::private::PATH_SEGMENT_ENCODE_SET,
            ),
        );
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        if response.status() == conjure_http::private::http::StatusCode::NO_CONTENT {
            Ok(Default::default())
        } else {
            conjure_http::private::json::client_from_reader(response.body_mut())
                .map_err(conjure_http::private::Error::internal)
        }
    }
    pub fn get_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<T::ResponseBody, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!(
            "/catalog/datasets/{}/raw",
            conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(&dataset_rid).as_bytes(),
                conjure_http::private::PATH_SEGMENT_ENCODE_SET,
            ),
        );
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static(
                "application/octet-stream",
            ),
        );
        let response = self.0.request(request_)?;
        Ok(response.into_body())
    }
    pub fn get_aliased_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<T::ResponseBody, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!(
            "/catalog/datasets/{}/raw-aliased",
            conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(&dataset_rid).as_bytes(),
                conjure_http::private::PATH_SEGMENT_ENCODE_SET,
            ),
        );
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static(
                "application/octet-stream",
            ),
        );
        let response = self.0.request(request_)?;
        Ok(response.into_body())
    }
    pub fn maybe_get_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<T::ResponseBody>, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!(
            "/catalog/datasets/{}/raw-maybe",
            conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(&dataset_rid).as_bytes(),
                conjure_http::private::PATH_SEGMENT_ENCODE_SET,
            ),
        );
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static(
                "application/octet-stream",
            ),
        );
        let response = self.0.request(request_)?;
        if response.status() == conjure_http::private::http::StatusCode::NO_CONTENT {
            Ok(None)
        } else {
            Ok(Some(response.into_body()))
        }
    }
    pub fn get_aliased_string(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<super::super::product::AliasedString, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!(
            "/catalog/datasets/{}/string-aliased",
            conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(&dataset_rid).as_bytes(),
                conjure_http::private::PATH_SEGMENT_ENCODE_SET,
            ),
        );
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        conjure_http::private::json::client_from_reader(response.body_mut())
            .map_err(conjure_http::private::Error::internal)
    }
    pub fn upload_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        input: Box<dyn conjure_http::client::WriteBody>,
    ) -> Result<(), conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Streaming(input));
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let path_ = format!("/catalog/datasets/upload-raw",);
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::CONTENT_TYPE,
            conjure_http::private::http::header::HeaderValue::from_static(
                "application/octet-stream",
            ),
        );
        self.0.request(request_)?;
        Ok(())
    }
    pub fn upload_aliased_raw_data(
        &self,
        auth_: &conjure_object::BearerToken,
        input: Box<dyn conjure_http::client::WriteBody>,
    ) -> Result<(), conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Streaming(input));
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let path_ = format!("/catalog/datasets/upload-raw-aliased",);
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::CONTENT_TYPE,
            conjure_http::private::http::header::HeaderValue::from_static(
                "application/octet-stream",
            ),
        );
        self.0.request(request_)?;
        Ok(())
    }
    pub fn get_branches(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!(
            "/catalog/datasets/{}/branches",
            conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(&dataset_rid).as_bytes(),
                conjure_http::private::PATH_SEGMENT_ENCODE_SET,
            ),
        );
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        if response.status() == conjure_http::private::http::StatusCode::NO_CONTENT {
            Ok(Default::default())
        } else {
            conjure_http::private::json::client_from_reader(response.body_mut())
                .map_err(conjure_http::private::Error::internal)
        }
    }
    #[doc = "Gets all branches of this dataset."]
    #[deprecated(note = "use getBranches instead")]
    pub fn get_branches_deprecated(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<std::collections::BTreeSet<String>, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!(
            "/catalog/datasets/{}/branchesDeprecated",
            conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(&dataset_rid).as_bytes(),
                conjure_http::private::PATH_SEGMENT_ENCODE_SET,
            ),
        );
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        if response.status() == conjure_http::private::http::StatusCode::NO_CONTENT {
            Ok(Default::default())
        } else {
            conjure_http::private::json::client_from_reader(response.body_mut())
                .map_err(conjure_http::private::Error::internal)
        }
    }
    pub fn resolve_branch(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
        branch: &str,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!(
            "/catalog/datasets/{}/branches/{}/resolve",
            conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(&dataset_rid).as_bytes(),
                conjure_http::private::PATH_SEGMENT_ENCODE_SET,
            ),
            conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(&branch).as_bytes(),
                conjure_http::private::PATH_SEGMENT_ENCODE_SET,
            ),
        );
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        if response.status() == conjure_http::private::http::StatusCode::NO_CONTENT {
            Ok(Default::default())
        } else {
            conjure_http::private::json::client_from_reader(response.body_mut())
                .map_err(conjure_http::private::Error::internal)
        }
    }
    pub fn test_param(
        &self,
        auth_: &conjure_object::BearerToken,
        dataset_rid: &conjure_object::ResourceIdentifier,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!(
            "/catalog/datasets/{}/testParam",
            conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(&dataset_rid).as_bytes(),
                conjure_http::private::PATH_SEGMENT_ENCODE_SET,
            ),
        );
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        if response.status() == conjure_http::private::http::StatusCode::NO_CONTENT {
            Ok(Default::default())
        } else {
            conjure_http::private::json::client_from_reader(response.body_mut())
                .map_err(conjure_http::private::Error::internal)
        }
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
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Fixed(
                conjure_http::private::json::to_vec(&query)
                    .map_err(conjure_http::private::Error::internal)?,
            ));
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = format!("/catalog/test-query-params",);
        path_.push_str("?different=");
        path_.extend(conjure_http::private::percent_encode(
            conjure_object::ToPlain::to_plain(&something).as_bytes(),
            conjure_http::private::QUERY_ENCODE_SET,
        ));
        path_.push_str("&implicit=");
        path_.extend(conjure_http::private::percent_encode(
            conjure_object::ToPlain::to_plain(&implicit).as_bytes(),
            conjure_http::private::QUERY_ENCODE_SET,
        ));
        for value in optional_middle.iter() {
            path_.push_str("&optionalMiddle=");
            path_.extend(conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(value).as_bytes(),
                conjure_http::private::QUERY_ENCODE_SET,
            ));
        }
        for value in set_end.iter() {
            path_.push_str("&setEnd=");
            path_.extend(conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(value).as_bytes(),
                conjure_http::private::QUERY_ENCODE_SET,
            ));
        }
        for value in optional_end.iter() {
            path_.push_str("&optionalEnd=");
            path_.extend(conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(value).as_bytes(),
                conjure_http::private::QUERY_ENCODE_SET,
            ));
        }
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::CONTENT_TYPE,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        conjure_http::private::json::client_from_reader(response.body_mut())
            .map_err(conjure_http::private::Error::internal)
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
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Fixed(
                conjure_http::private::json::to_vec(&query)
                    .map_err(conjure_http::private::Error::internal)?,
            ));
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let mut path_ = format!("/catalog/test-no-response-query-params",);
        path_.push_str("?different=");
        path_.extend(conjure_http::private::percent_encode(
            conjure_object::ToPlain::to_plain(&something).as_bytes(),
            conjure_http::private::QUERY_ENCODE_SET,
        ));
        path_.push_str("&implicit=");
        path_.extend(conjure_http::private::percent_encode(
            conjure_object::ToPlain::to_plain(&implicit).as_bytes(),
            conjure_http::private::QUERY_ENCODE_SET,
        ));
        for value in optional_middle.iter() {
            path_.push_str("&optionalMiddle=");
            path_.extend(conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(value).as_bytes(),
                conjure_http::private::QUERY_ENCODE_SET,
            ));
        }
        for value in set_end.iter() {
            path_.push_str("&setEnd=");
            path_.extend(conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(value).as_bytes(),
                conjure_http::private::QUERY_ENCODE_SET,
            ));
        }
        for value in optional_end.iter() {
            path_.push_str("&optionalEnd=");
            path_.extend(conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(value).as_bytes(),
                conjure_http::private::QUERY_ENCODE_SET,
            ));
        }
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::CONTENT_TYPE,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        self.0.request(request_)?;
        Ok(())
    }
    pub fn test_boolean(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<bool, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!("/catalog/boolean",);
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        conjure_http::private::json::client_from_reader(response.body_mut())
            .map_err(conjure_http::private::Error::internal)
    }
    pub fn test_double(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<f64, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!("/catalog/double",);
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        conjure_http::private::json::client_from_reader(response.body_mut())
            .map_err(conjure_http::private::Error::internal)
    }
    pub fn test_integer(
        &self,
        auth_: &conjure_object::BearerToken,
    ) -> Result<i32, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let path_ = format!("/catalog/integer",);
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        conjure_http::private::json::client_from_reader(response.body_mut())
            .map_err(conjure_http::private::Error::internal)
    }
    pub fn test_post_optional(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_string: Option<&str>,
    ) -> Result<Option<String>, conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Fixed(
                conjure_http::private::json::to_vec(&maybe_string)
                    .map_err(conjure_http::private::Error::internal)?,
            ));
        *request_.method_mut() = conjure_http::private::http::Method::POST;
        let path_ = format!("/catalog/optional",);
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::CONTENT_TYPE,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        request_.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static("application/json"),
        );
        let mut response = self.0.request(request_)?;
        if response.status() == conjure_http::private::http::StatusCode::NO_CONTENT {
            Ok(Default::default())
        } else {
            conjure_http::private::json::client_from_reader(response.body_mut())
                .map_err(conjure_http::private::Error::internal)
        }
    }
    pub fn test_optional_integer_and_double(
        &self,
        auth_: &conjure_object::BearerToken,
        maybe_integer: Option<i32>,
        maybe_double: Option<f64>,
    ) -> Result<(), conjure_http::private::Error> {
        let mut request_ =
            conjure_http::private::http::Request::new(conjure_http::client::Body::Empty);
        *request_.method_mut() = conjure_http::private::http::Method::GET;
        let mut path_ = format!("/catalog/optional-integer-double",);
        let mut first_ = true;
        for value in maybe_integer.iter() {
            let ch = if first_ {
                first_ = false;
                '?'
            } else {
                '&'
            };
            path_.push(ch);
            path_.push_str("maybeInteger=");
            path_.extend(conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(value).as_bytes(),
                conjure_http::private::QUERY_ENCODE_SET,
            ));
        }
        for value in maybe_double.iter() {
            let ch = if first_ {
                first_ = false;
                '?'
            } else {
                '&'
            };
            path_.push(ch);
            path_.push_str("maybeDouble=");
            path_.extend(conjure_http::private::percent_encode(
                conjure_object::ToPlain::to_plain(value).as_bytes(),
                conjure_http::private::QUERY_ENCODE_SET,
            ));
        }
        *request_.uri_mut() = conjure_http::private::http::Uri::from_shared(path_.into())
            .expect("URI should be valid");
        request_.headers_mut().insert(
            conjure_http::private::http::header::AUTHORIZATION,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!("Bearer {}", auth_.as_str()).into(),
            )
            .expect("bearer tokens are valid headers"),
        );
        self.0.request(request_)?;
        Ok(())
    }
}
