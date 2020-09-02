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

use std::path::PathBuf;
use std::process;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Opts {
    #[structopt(
        name = "generate",
        setting = AppSettings::UnifiedHelpMessage,
        setting = AppSettings::DeriveDisplayOrder,
        setting = AppSettings::DontCollapseArgsInUsage,
    )]
    /// Generate Rust code from a conjure IR file.
    Generate(Args),
}

// FIXME move aliases over to the standard names
#[derive(StructOpt)]
struct Args {
    #[structopt(long = "exhaustive", parse(try_from_str))]
    /// Generate exhaustively matchable enums and unions
    exhaustive: bool,
    #[structopt(long = "strip-prefix", value_name = "prefix", alias = "stripPrefix")]
    /// Strip a prefix from types's package paths
    strip_prefix: Option<String>,
    /// The name of the generated crate
    #[structopt(
        long = "crate-name",
        value_name = "name",
        requires = "crate-version",
        alias = "crateName",
        alias = "productName"
    )]
    crate_name: Option<String>,
    /// The version of the generated crate
    #[structopt(
        long = "crate-version",
        value_name = "version",
        requires = "crate-name",
        alias = "crateVersion",
        alias = "productVersion"
    )]
    crate_version: Option<String>,
    #[structopt(name = "input-json", parse(from_os_str))]
    /// Path to a JSON-formatted Conjure IR file
    input_json: PathBuf,
    #[structopt(name = "output-directory", parse(from_os_str))]
    /// Directory to place generated code
    output_directory: PathBuf,
}

fn main() {
    let Opts::Generate(args) = Opts::from_args();

    let mut config = conjure_codegen::Config::new();
    config.exhaustive(args.exhaustive);
    if let Some(prefix) = args.strip_prefix {
        config.strip_prefix(prefix);
    }
    if let (Some(crate_name), Some(crate_version)) = (args.crate_name, args.crate_version) {
        config.build_crate(&crate_name, &crate_version);
    }
    let r = config.generate_files(&args.input_json, &args.output_directory);

    if let Err(e) = r {
        eprintln!("{}", e);
        for e in e.iter_causes() {
            eprintln!("Caused by: {}", e);
        }
        process::exit(1);
    }
}
