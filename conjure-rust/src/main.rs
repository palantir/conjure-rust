// Copyright 2018 Palantir Technologies, Inc.
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
#![warn(clippy::all)]

use clap::{ArgAction, Parser};
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[clap(rename_all = "camelCase")]
enum Opts {
    #[clap(dont_collapse_args_in_usage = true)]
    /// Generate Rust code from a conjure IR file.
    Generate(Args),
}

#[derive(Parser)]
#[clap(rename_all = "camelCase")]
struct Args {
    /// Generate exhaustively matchable enums and unions
    #[clap(
        long,
        default_missing_value = "true",
        default_value = "false",
        num_args = 0..=1,
        require_equals = true,
        action = ArgAction::Set,
    )]
    exhaustive: bool,
    /// Include empty collection fields in serialized output
    #[clap(
        long,
        default_missing_value = "true",
        default_value = "false",
        num_args = 0..=1,
        require_equals = true,
        action = ArgAction::Set,
    )]
    serialize_empty_collections: bool,
    /// Strip a prefix from types's package paths
    #[clap(long, value_name = "prefix")]
    strip_prefix: Option<String>,
    /// The name of the product
    #[clap(long, value_name = "name", requires = "product_version")]
    product_name: Option<String>,
    /// The version of the product
    #[clap(long, value_name = "version", requires = "product_name")]
    product_version: Option<String>,
    /// The version of the generated crate. Defaults to `--productVersion`
    #[clap(long, value_name = "version", requires = "product_version")]
    crate_version: Option<String>,
    /// Use external references with package name stripping
    #[clap(long)]
    use_external_references: bool,
    /// Path to a JSON-formatted Conjure IR file
    input_json: PathBuf,
    /// Directory to place generated code
    output_directory: PathBuf,

    #[clap(
        long = "extraManifestJson",
        value_name = "json",
        value_parser = parse_extra_manifest_json,
        help = r#"Extra manifest configuration as a JSON object.
This JSON will be converted to TOML and merged into the generated Cargo.toml manifest.

Example:
    --extraManifestJson '{
        "package": { "publish": ["some-registry-name"], "license": "MIT" },
        "dependencies": { "serde": { "version": "1.0", "features": ["default"] } },
        "features": { "fancy-feature": ["foo", "bar"] }
    }'

Use single quotes to avoid shell escaping issues."#
)]
    extra_manifest: Option<toml::Value>,
}

/// Parse a JSON string into a toml::Value
fn parse_extra_manifest_json(s: &str) -> Result<toml::Value, String> {
    let json_value: serde_json::Value =
        serde_json::from_str(s).map_err(|e| format!("Invalid JSON: {}", e))?;
    toml::Value::try_from(json_value).map_err(|e| format!("Cannot convert JSON to TOML: {}", e))
}

fn main() {
    let Opts::Generate(args) = Opts::parse();

    let mut config = conjure_codegen::Config::new();
    config
        .exhaustive(args.exhaustive)
        .serialize_empty_collections(args.serialize_empty_collections);
    if let Some(prefix) = args.strip_prefix {
        config.strip_prefix(prefix);
    }
    config.use_external_references(args.use_external_references);
    let crate_version = args
        .crate_version
        .as_deref()
        .or(args.product_version.as_deref());
    if let (Some(product_name), Some(crate_version)) = (args.product_name, crate_version) {
        config.build_crate(&product_name, crate_version);
    }
    if let Some(extra_manifest_config) = args.extra_manifest {
        config.extra_manifest_config(extra_manifest_config);
    }
    if let Some(product_version) = args.product_version {
        config.version(product_version);
    }
    let r = config.generate_files(&args.input_json, &args.output_directory);

    if let Err(e) = r {
        eprintln!("{e:?}");
        process::exit(1);
    }
}
