// Copyright 2019 Palantir Technologies, Inc.
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

pub use crate::private::client::*;
pub use crate::private::server::*;
pub use async_trait::async_trait;
pub use bytes::Bytes;
pub use conjure_error::Error;
pub use conjure_serde::json;
pub use futures_core::Stream;
pub use http::{self, Extensions, Method, Request, Response};
pub use pin_utils::pin_mut;
pub use std::borrow::Cow;
pub use std::future::Future;
pub use std::option::Option;
pub use std::pin::Pin;
pub use std::sync::Arc;

use bytes::BytesMut;
use conjure_error::InvalidArgument;
use futures_util::TryStreamExt;
use http::HeaderValue;

mod client;
mod server;

pub(crate) const APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");
const APPLICATION_OCTET_STREAM: HeaderValue = HeaderValue::from_static("application/octet-stream");

// slightly nontrivial to avoid a copy for single-chunk bodies
fn read_body<I>(mut body: I, limit: Option<usize>) -> Result<Bytes, Error>
where
    I: Iterator<Item = Result<Bytes, Error>>,
{
    let first = match body.next().transpose()? {
        Some(bytes) => bytes,
        None => return Ok(Bytes::new()),
    };
    check_limit(&first, limit)?;

    let mut buf = BytesMut::new();
    match body.next().transpose()? {
        Some(second) => {
            buf.reserve(first.len() + second.len());
            buf.extend_from_slice(&first);
            buf.extend_from_slice(&second);
        }
        None => return Ok(first),
    };
    check_limit(&buf, limit)?;

    for bytes in body {
        buf.extend_from_slice(&bytes?);
        check_limit(&buf, limit)?;
    }

    Ok(buf.freeze())
}

async fn async_read_body<I>(body: I, limit: Option<usize>) -> Result<Bytes, Error>
where
    I: Stream<Item = Result<Bytes, Error>>,
{
    pin_mut!(body);

    let first = match body.try_next().await? {
        Some(bytes) => bytes,
        None => return Ok(Bytes::new()),
    };
    check_limit(&first, limit)?;

    let mut buf = BytesMut::new();
    match body.try_next().await? {
        Some(second) => {
            buf.reserve(first.len() + second.len());
            buf.extend_from_slice(&first);
            buf.extend_from_slice(&second);
        }
        None => return Ok(first),
    }
    check_limit(&buf, limit)?;

    while let Some(bytes) = body.try_next().await? {
        buf.extend_from_slice(&bytes);
        check_limit(&buf, limit)?;
    }

    Ok(buf.freeze())
}

fn check_limit(buf: &[u8], limit: Option<usize>) -> Result<(), Error> {
    let limit = match limit {
        Some(limit) => limit,
        None => return Ok(()),
    };

    if buf.len() > limit {
        return Err(Error::service_safe(
            "body too large",
            InvalidArgument::new(),
        ));
    }

    Ok(())
}
