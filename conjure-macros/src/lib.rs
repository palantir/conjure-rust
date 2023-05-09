// Copyright 2022 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! Macros exposed by conjure-http.
//!
//! Do not consume directly.
// Copyright 2022 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! Macros exposed by conjure-http.
//!
//! Do not consume directly.
#![warn(missing_docs)]

use proc_macro::TokenStream;
use syn::{Error, ItemTrait, TraitItem};

mod client;
mod endpoints;
mod path;

/// Creates a Conjure client type implementing the annotated trait.
///
/// For a trait named `MyService`, the macro will create a type named `MyServiceClient` which
/// implements the Conjure `Client` and `MyService` traits.
///
/// # Endpoints
///
/// Each method corresponds to a separate HTTP endpoint, and is expected to take `&self` and return
/// `Result<T, Error>`. Each must be annotated with `#[endpoint]`, which has several
/// parameters:
///
/// * `method` - The HTTP method (e.g. `GET`). Required.
/// * `path` - The HTTP path template. Path parameters should be identified by `{name}` and must
///     make up an entire path component. Required.
/// * `accept` - A type implementing `DeserializeResponse` which will be used to create the return
///     value. Defaults to returning `()`.
///
/// Each method argument must have an annotation describing the type of parameter. One of:
///
/// * `#[path]` - A path parameter. The path template must contain a parameter component matching
///     the argument name.
///
///     Parameters:
///     * `encoder` - A type implementing `EncodeParam` which will be used to encode the value into
///         a string. Defaults to `DisplayParamEncoder`.
/// * `#[query]` - A query parameter.
///
///     Parameters:
///     * `name` - The string used as the key in the encoded URI. Required.
///     * `encoder` - A type implementing `EncodeParam` which will be used to encode the value into
///         a string. Defaults to `DisplayParamEncoder`.
/// * `#[auth]` - A `BearerToken` used to authenticate the request. A method may only have at most
///     one auth parameter.
///
///     Parameters:
///     * `cookie_name` - The name of the cookie used if the token is to be passed via a `Cookie`
///         header. If unset, it will be passed via an `Authorization` header instead.
/// * `#[header]` - A header.
///
///     Parameters:
///     * `name` - The header name. Required.
///     * `encoder` - A type implementing `EncodeHeader` which will be used to encode the value
///         into a header. Defaults to `DisplayHeaderEncoder`.
/// * `#[body]` - The request body. A method may only have at most one body parameter.
///
///     Parameters:
///     * `serializer` - A type implementing `SerializeRequest` which will be used to serialize the
///         value into a body. Defaults to `ConjureRequestSerializer`.
///
/// # Async
///
/// Both blocking and async clients are supported. For technical reasons, async trait
/// implementations must put the `#[conjure_client]` annotation *above* the `#[async_trait]`
/// annotation.
///
/// # Examples
///
/// ```rust
/// use async_trait::async_trait;
/// use conjure_error::Error;
/// use conjure_http::{conjure_client, endpoint};
/// use conjure_http::client::{
///     AsyncClient, AsyncService, Client, ConjureResponseDeserializer, DisplaySeqParamEncoder,
///     Service,
/// };
/// use conjure_object::BearerToken;
///
/// #[conjure_client]
/// trait MyService {
///     #[endpoint(method = GET, path = "/yaks/{yak_id}", accept = ConjureResponseDeserializer)]
///     fn get_yak(&self, #[auth] auth: &BearerToken, #[path] yak_id: i32) -> Result<String, Error>;
///
///     #[endpoint(method = POST, path = "/yaks")]
///     fn create_yak(
///         &self,
///         #[auth] auth_token: &BearerToken,
///         #[query(name = "parentName", encoder = DisplaySeqParamEncoder)] parent_id: Option<&str>,
///         #[body] yak: &str,
///     ) -> Result<(), Error>;
/// }
///
/// fn do_work(client: impl Client, auth: &BearerToken) -> Result<(), Error> {
///     let client = MyServiceClient::new(client);
///     client.create_yak(auth, None, "my cool yak")?;
///
///     Ok(())
/// }
///
/// #[conjure_client]
/// #[async_trait]
/// trait MyServiceAsync {
///     #[endpoint(method = GET, path = "/yaks/{yak_id}", accept = ConjureResponseDeserializer)]
///     async fn get_yak(
///         &self,
///         #[auth] auth: &BearerToken,
///         #[path] yak_id: i32,
///     ) -> Result<String, Error>;
///
///     #[endpoint(method = POST, path = "/yaks")]
///     async fn create_yak(
///         &self,
///         #[auth] auth_token: &BearerToken,
///         #[query(name = "parentName", encoder = DisplaySeqParamEncoder)] parent_id: Option<&str>,
///         #[body] yak: &str,
///     ) -> Result<(), Error>;
/// }
///
/// async fn do_work_async<C>(client: C, auth: &BearerToken) -> Result<(), Error>
/// where
///     C: AsyncClient + Sync + Send,
///     C::ResponseBody: 'static + Send,
/// {
///     let client = MyServiceAsyncClient::new(client);
///     client.create_yak(auth, None, "my cool yak").await?;
///
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn conjure_client(attr: TokenStream, item: TokenStream) -> TokenStream {
    client::generate(attr, item)
}

/// Creates a Conjure service type wrapping types implementing the annotated trait.
///
/// For a trait named `MyService`, the macro will create a type named `MyServiceEndpoints` which
/// implements the conjure `Service` trait.
///
/// # Endpoints
///
/// Each method corresponds to a separate HTTP endpoint, and is expected to take `&self` and return
/// `Result<T, Error>`. Each must be annotated with `#[endpoint]`, which has several parameters:
///
/// * `method` - The HTTP method (e.g. `GET`). Required.
/// * `path` - The HTTP path template. Path parameters should be identified by `{name}` and must
///     make up an entire path component. Required.
/// * `produces` - A type implementing `SerializeResponse` which will be used to convert the value
///     returned by the method into a response. Defaults to `EmptyResponseSerializer`.
///
/// Each method argument must have an annotation describing the type of parameter. One of:
///
/// * `#[path]` - A path parameter. The path template mut contain a parameter component matching
///     the argument name.
///
///     Parameters:
///     * `decoder` - A type implementing `DecodeParam` which will be used to decode the value.
///         Defaults to `FromStrDecoder`.
///     * `safe` - If set, the parameter will be added to the `SafeParams` response extension.
/// * `#[query]` - A query parameter.
///
///     Parameters:
///     * `name` - The string used as the key in the encoded URI. Required.
///     * `decoder` - A type implementing `DecodeParam` which will be used to decode the value.
///         Defaults to `FromStrDecoder`.
///     * `safe` - If set, the parameter will be added to the `SafeParams` response extension.
/// * `#[auth]` - A `BearerToken` used to authenticate the request.
///
///     Parameters:
///     * `cookie_name` - The name of the cookie if the token is to be parsed from a `Cookie`
///         header. If unset, it will be parsed from an `Authorization` header instead.
/// * `#[header]` - A header parameter.
///
///     Parameters:
///     * `name` - The header name. Required.
///     * `decoder` - A type implementing `DecodeHeader` which will be used to decode the value.
///         Defaults to `FromStrDecoder`.
///     * `safe` - If set, the parameter will be added to the `SafeParams` response extension.
/// * `#[body]` - The request body.
///
///     Parameters:
///     * `deserializer` - A type implementing `DeserializeRequest` which will be used to
///         deserialize the request body into a value. Defaults to `ConjureRequestDeserializer`.
///     * `safe` - If set, the parameter will be added to the `SafeParams` response extension.
/// * `#[context]` - A `RequestContext` which provides lower level access to the request.
///
/// # Async
///
/// Both blocking and async services are supported. For technical reasons, async trait
/// implementations must put the `#[conjure_endpoints]` annotation *above* the `#[async_trait]`
/// annotation.
///
/// # Examples
///
/// ```rust
/// use async_trait::async_trait;
/// use conjure_error::Error;
/// use conjure_http::{conjure_endpoints, endpoint};
/// use conjure_http::server::{ConjureResponseSerializer, FromStrOptionDecoder};
/// use conjure_object::BearerToken;
///
/// #[conjure_endpoints]
/// trait MyService {
///     #[endpoint(method = GET, path = "/yaks/{yak_id}", produces = ConjureResponseSerializer)]
///     fn get_yak(
///         &self,
///         #[auth] auth: BearerToken,
///         #[path(safe)] yak_id: i32,
///     ) -> Result<String, Error>;
///
///     #[endpoint(method = POST, path = "/yaks")]
///     fn create_yak(
///         &self,
///         #[auth] auth: BearerToken,
///         #[query(name = "parentName", decoder = FromStrOptionDecoder)] parent_id: Option<String>,
///         #[body] yak: String,
///     ) -> Result<(), Error>;
/// }
///
/// #[conjure_endpoints]
/// #[async_trait]
/// trait AsyncMyService {
///     #[endpoint(method = GET, path = "/yaks/{yak_id}", produces = ConjureResponseSerializer)]
///     async fn get_yak(
///         &self,
///         #[auth] auth: BearerToken,
///         #[path(safe)] yak_id: i32,
///     ) -> Result<String, Error>;
///
///     #[endpoint(method = POST, path = "/yaks")]
///     async fn create_yak(
///         &self,
///         #[auth] auth: BearerToken,
///         #[query(name = "parentName", decoder = FromStrOptionDecoder)] parent_id: Option<String>,
///         #[body] yak: String,
///     ) -> Result<(), Error>;
/// }
/// ```
#[proc_macro_attribute]
pub fn conjure_endpoints(attr: TokenStream, item: TokenStream) -> TokenStream {
    endpoints::generate(attr, item)
}

/// A no-op attribute macro required due to technical limitations of Rust's macro system.
#[proc_macro_attribute]
pub fn endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

struct Errors(Vec<Error>);

impl Errors {
    fn new() -> Self {
        Errors(vec![])
    }

    fn push(&mut self, error: Error) {
        self.0.push(error);
    }

    fn build(mut self) -> Result<(), Error> {
        let Some(mut error) = self.0.pop() else { return Ok(()) };
        for other in self.0 {
            error.combine(other);
        }
        Err(error)
    }
}

#[derive(Copy, Clone)]
enum Asyncness {
    Sync,
    Async,
}

impl Asyncness {
    fn resolve(trait_: &ItemTrait) -> Result<Self, Error> {
        let mut it = trait_.items.iter().filter_map(|t| match t {
            TraitItem::Fn(f) => Some(f),
            _ => None,
        });

        let Some(first) = it.next() else {
        return Ok(Asyncness::Sync);
    };

        let is_async = first.sig.asyncness.is_some();

        let mut errors = Errors::new();

        for f in it {
            if f.sig.asyncness.is_some() != is_async {
                errors.push(Error::new_spanned(
                    f,
                    "all methods must either be sync or async",
                ));
            }
        }

        errors.build()?;
        let asyncness = if is_async {
            Asyncness::Async
        } else {
            Asyncness::Sync
        };
        Ok(asyncness)
    }
}
