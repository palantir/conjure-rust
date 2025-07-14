// Copyright 2025 Palantir Technologies, Inc.
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

use std::mem::discriminant;

use thiserror::Error;
use toml::Value;

#[derive(Debug, Error)]
pub(crate) enum TomlMergeError {
    #[error("merge failed due to type mismatch: left: {left}, right: {right}")]
    TypeMismatch { left: Value, right: Value },
}

pub(crate) fn left_merge(left: &mut Value, right: &Value) -> Result<(), TomlMergeError> {
    match (left, right) {
        (Value::Table(left_table), Value::Table(right_table)) => {
            for (key, right_val) in right_table {
                match left_table.get_mut(key) {
                    Some(left_val) => left_merge(left_val, right_val)?,
                    None => {
                        left_table.insert(key.clone(), right_val.clone());
                    }
                }
            }
            Ok(())
        }
        (Value::Array(left_array), Value::Array(right_array)) => {
            left_array.extend(right_array.clone());
            Ok(())
        }
        (left_val, right_val) if discriminant(left_val) != discriminant(right_val) => {
            Err(TomlMergeError::TypeMismatch {
                left: left_val.clone(),
                right: right_val.clone(),
            })
        }
        _ => {
            // Do nothing (i.e. keep left value)
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_left_merge_basic() {
        let left_str = r#"
            [package]
            name = "foo-lib"
            version = "0.0.0"
            edition = "2018"

            [[package.metadata.sls.recommended-product-dependencies]]
            maximum-version = "0.x.x"
            minimum-version = "0.1.0"
            optional = false
            product-group = "com.acme.foo"
            product-name = "foo"

            [dependencies]
            conjure-error = "4.10.0"
            conjure-http = "4.10.0"
            conjure-object = "4.10.0"
        "#;

        let right_str = r#"
            [package]
            version = "0.1.0"
            publish = ["some-registry-name"]
            license = "MIT"

            [dependencies]
            serde = { version = "2.0", features = ["derive"] }
            anyhow = "1.0"

            [features]
            fancy-feature = ["foo", "bar"]
        "#;

        let mut left = Value::Table(left_str.parse().unwrap());
        let right = Value::Table(right_str.parse().unwrap());

        left_merge(&mut left, &right).expect("merge should succeed");

        // [package] fields from left are kept
        let package = left.get("package").unwrap();
        assert_eq!(
            package.get("name").unwrap(),
            &Value::String("foo-lib".to_string())
        );
        assert_eq!(
            package.get("version").unwrap(),
            &Value::String("0.0.0".to_string()) // conflicting right field ignored
        );
        assert_eq!(
            package.get("edition").unwrap(),
            &Value::String("2018".to_string())
        );
        // [package] fields from right are added
        assert_eq!(
            package.get("publish").unwrap(),
            &Value::Array(vec![Value::String("some-registry-name".to_string())])
        );
        assert_eq!(
            package.get("license").unwrap(),
            &Value::String("MIT".to_string())
        );

        // Deep nested array of tables: [[package.metadata.sls.recommended-product-dependencies]]
        let sls = package
            .get("metadata")
            .unwrap()
            .get("sls")
            .unwrap()
            .get("recommended-product-dependencies")
            .unwrap();
        assert!(sls.is_array());
        let arr = sls.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let dep = &arr[0];
        assert_eq!(
            dep.get("maximum-version").unwrap(),
            &Value::String("0.x.x".to_string())
        );
        assert_eq!(
            dep.get("minimum-version").unwrap(),
            &Value::String("0.1.0".to_string())
        );
        assert_eq!(dep.get("optional").unwrap(), &Value::Boolean(false));
        assert_eq!(
            dep.get("product-group").unwrap(),
            &Value::String("com.acme.foo".to_string())
        );
        assert_eq!(
            dep.get("product-name").unwrap(),
            &Value::String("foo".to_string())
        );

        // [dependencies] from left are kept
        let dependencies = left.get("dependencies").unwrap();
        assert_eq!(
            dependencies.get("conjure-error").unwrap(),
            &Value::String("4.10.0".to_string())
        );
        assert_eq!(
            dependencies.get("conjure-http").unwrap(),
            &Value::String("4.10.0".to_string())
        );
        assert_eq!(
            dependencies.get("conjure-object").unwrap(),
            &Value::String("4.10.0".to_string())
        );
        // [dependencies] from right are are added
        assert_eq!(
            dependencies.get("serde").unwrap().get("version").unwrap(),
            &Value::String("2.0".to_string())
        );
        assert_eq!(
            dependencies.get("serde").unwrap().get("features").unwrap(),
            &Value::Array(vec![Value::String("derive".to_string())])
        );
        assert_eq!(
            dependencies.get("anyhow").unwrap(),
            &Value::String("1.0".to_string())
        );

        // [features] table from right is added
        let features = left.get("features").unwrap();
        assert_eq!(
            features.get("fancy-feature").unwrap(),
            &Value::Array(vec![
                Value::String("foo".to_string()),
                Value::String("bar".to_string())
            ])
        );
    }

    #[test]
    fn test_left_merge_empty_right() {
        let left_str = r#"
            [package]
            license = "MIT"
        "#;
        let right_str = "";
        let mut left = Value::Table(left_str.parse().unwrap());
        let right = Value::Table(right_str.parse().unwrap_or(Default::default()));

        let expected = left.clone();
        left_merge(&mut left, &right).expect("merge should succeed");
        assert_eq!(left, expected);
    }

    #[test]
    fn test_left_merge_empty_left() {
        let left_str = "";
        let right_str = r#"
            [dependencies]
            anyhow = "1.0"
        "#;
        let mut left = Value::Table(left_str.parse().unwrap_or(Default::default()));
        let right = Value::Table(right_str.parse().unwrap());

        left_merge(&mut left, &right).expect("merge should succeed");
        assert_eq!(
            left.get("dependencies").unwrap().get("anyhow").unwrap(),
            &Value::String("1.0".to_string())
        );
    }

    #[test]
    fn test_left_merge_type_mismatch() {
        let left_str = r#"
            [package]
            license = "MIT"
        "#;
        let right_str = r#"
            [package]
            license = ["MIT", "Apache-2.0"]
        "#;
        let mut left = Value::Table(left_str.parse().unwrap());
        let right = Value::Table(right_str.parse().unwrap());

        let result = left_merge(&mut left, &right);
        assert!(matches!(result, Err(TomlMergeError::TypeMismatch { .. })));
    }
}
