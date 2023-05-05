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

mod client;
mod path;

/// Creates a Conjure client type implementing the annotated trait.
///
/// For a trait named `MyService`, the macro will create a type named `MyServiceClient` which
/// implements the Conjure `Client` and `MyService` traits.
///
/// # Endpoints
///
/// Each method corresponds to a separate HTTP endpoint, and is expected to take `&self` and return
/// `Result<T, Error>`.  Each must be annotated with `#[endpoint]`, which has several
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
/// * `#[path]` - A path parameter. The path templaste must contain a parameter component matching
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

/// A no-op attribute macro required due to technical limitations of Rust's macro system.
#[proc_macro_attribute]
pub fn endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
