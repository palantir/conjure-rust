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
use bytes::Bytes;
use conjure_error::Error;
use futures::Stream;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

mod clients;
mod errors;
mod external_refs;
mod objects;
mod servers;

#[derive(Debug, PartialEq)]
struct RemoteBody(Vec<u8>);

impl Iterator for RemoteBody {
    type Item = Result<Bytes, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        Some(Ok(Bytes::from(mem::take(&mut self.0))))
    }
}

impl Stream for RemoteBody {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.0.is_empty() {
            return Poll::Ready(None);
        }

        Poll::Ready(Some(Ok(Bytes::from(mem::take(&mut self.0)))))
    }
}
