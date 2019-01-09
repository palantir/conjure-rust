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
        raw(
            setting = "AppSettings::UnifiedHelpMessage",
            setting = "AppSettings::DeriveDisplayOrder",
            setting = "AppSettings::DontCollapseArgsInUsage",
        )
    )]
    /// Generate Rust code from a conjure IR file.
    Generate(Args),
}

#[derive(StructOpt)]
struct Args {
    #[structopt(long = "exhaustive")]
    /// Generate exhaustively matchable enums and unions
    exhaustive: bool,
    #[structopt(long = "conjure-types-path", name = "path")]
    /// The module path to the conjure-types crate root
    conjure_types_path: Option<String>,
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
    if let Some(conjure_types_path) = &args.conjure_types_path {
        config.conjure_types_path(conjure_types_path);
    }
    let r = config
        .exhaustive(args.exhaustive)
        .generate_files(&args.input_json, &args.output_directory);

    if let Err(e) = r {
        eprintln!("{}", e);
        for e in e.iter_causes() {
            eprintln!("Caused by: {}", e);
        }
        process::exit(1);
    }
}
