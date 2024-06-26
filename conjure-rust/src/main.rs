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

use clap::Parser;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
enum Opts {
    #[clap(name = "generate", dont_collapse_args_in_usage = true)]
    /// Generate Rust code from a conjure IR file.
    Generate(Args),
}

#[derive(Parser)]
struct Args {
    /// Generate exhaustively matchable enums and unions
    #[clap(long = "exhaustive")]
    exhaustive: bool,
    /// Strip a prefix from types's package paths
    #[clap(long = "stripPrefix", value_name = "prefix")]
    strip_prefix: Option<String>,
    /// The name of the product
    #[clap(
        long = "productName",
        value_name = "name",
        requires = "product_version"
    )]
    product_name: Option<String>,
    /// The version of the product
    #[clap(
        long = "productVersion",
        value_name = "version",
        requires = "product_name"
    )]
    product_version: Option<String>,
    /// The version of the generated crate. Defaults to `--productVersion`
    #[clap(
        long = "crateVersion",
        value_name = "version",
        requires = "product_version"
    )]
    crate_version: Option<String>,
    /// Path to a JSON-formatted Conjure IR file
    #[clap(name = "inputJson")]
    input_json: PathBuf,
    /// Directory to place generated code
    #[clap(name = "outputDirectory")]
    output_directory: PathBuf,
}

fn main() {
    let Opts::Generate(args) = Opts::parse();

    let mut config = conjure_codegen::Config::new();
    config.exhaustive(args.exhaustive);
    if let Some(prefix) = args.strip_prefix {
        config.strip_prefix(prefix);
    }
    let crate_version = args
        .crate_version
        .as_deref()
        .or(args.product_version.as_deref());
    if let (Some(product_name), Some(crate_version)) = (args.product_name, crate_version) {
        config.build_crate(&product_name, crate_version);
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
