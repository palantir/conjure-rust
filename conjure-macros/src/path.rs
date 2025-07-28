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

use syn::{Error, LitStr};

pub enum PathComponent {
    Literal(String),
    Parameter { name: String, regex: Option<String> },
}

pub fn parse(path_lit: &LitStr) -> Result<Vec<PathComponent>, Error> {
    let path = path_lit.value();

    if path.is_empty() {
        return Ok(vec![]);
    }

    let Some(path) = path.strip_prefix('/') else {
        return Err(Error::new_spanned(
            path_lit,
            "paths must either be empty or start with `/`",
        ));
    };

    let components = path
        .split('/')
        .map(|component| {
            match component
                .strip_prefix('{')
                .and_then(|c| c.strip_suffix('}'))
            {
                Some(parameter) => parse_parameter(parameter),
                None => Ok(PathComponent::Literal(component.to_string())),
            }
        })
        .collect::<Result<_, Error>>()?;

    Ok(components)
}

fn parse_parameter(parameter: &str) -> Result<PathComponent, Error> {
    let (name, regex) = {
        let mut p = parameter.splitn(2, ':');
        (p.next(), p.next())
    };
    match (name, regex) {
        (Some(name), Some(regex)) => Ok(PathComponent::Parameter {
            name: name.to_string(),
            regex: Some(regex.to_string()),
        }),
        (Some(name), None) => Ok(PathComponent::Parameter {
            name: name.to_string(),
            regex: None,
        }),
        _ => Err(Error::new_spanned(parameter, "invalid ")),
    }
}
