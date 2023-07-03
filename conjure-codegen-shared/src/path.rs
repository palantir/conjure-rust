// Copyright 2023 Palantir Technologies, Inc.
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

pub enum PathSegment {
    Literal(String),
    Parameter { name: String, regex: Option<String> },
}

pub fn parse(path: &str) -> Vec<PathSegment> {
    path.split('/')
        .skip(1)
        .map(
            |segment| match segment.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                Some(segment) => {
                    let mut it = segment.splitn(2, ':');
                    PathSegment::Parameter {
                        name: it.next().unwrap().to_string(),
                        regex: it.next().map(|s| s.to_string()),
                    }
                }
                None => PathSegment::Literal(segment.to_string()),
            },
        )
        .collect()
}
