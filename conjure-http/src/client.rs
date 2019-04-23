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

use conjure_error::Error;
use http::{Request, Response};
use std::io::{Cursor, Read, Write};

pub trait Client {
    type ResponseBody: Read;

    fn request(&self, req: Request<Body>) -> Result<Response<Self::ResponseBody>, Error>;
}

pub enum Body {
    Empty,
    Fixed(Vec<u8>),
    Streaming(Box<dyn WriteBody>),
}

pub trait WriteBody {
    fn write(&mut self, w: &mut dyn Write) -> Result<(), Error>;

    fn reset(&mut self) -> bool;
}

impl<T> WriteBody for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn write(&mut self, w: &mut dyn Write) -> Result<(), Error> {
        let buf = &self.get_ref().as_ref()[self.position() as usize..];
        w.write_all(buf).map_err(Error::internal_safe)?;
        self.set_position(self.get_ref().as_ref().len() as u64);
        Ok(())
    }

    fn reset(&mut self) -> bool {
        self.set_position(0);
        true
    }
}
