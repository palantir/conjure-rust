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

//! Code generation for Conjure definitions.
#![warn(clippy::all, missing_docs)]
#![recursion_limit = "256"]

use failure::{bail, Error, ResultExt};
use proc_macro2::TokenStream;
use quote::quote;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::context::Context;
use crate::types::{ConjureDefinition, TypeDefinition};

mod aliases;
mod context;
mod enums;
mod objects;
#[allow(dead_code, clippy::all)]
mod types;
mod unions;

/// Codegen configuration.
pub struct Config {
    rustfmt: OsString,
    run_rustfmt: bool,
    exhaustive: bool,
    conjure_types_path: TokenStream,
}

impl Default for Config {
    fn default() -> Config {
        Config::new()
    }
}

impl Config {
    /// Creates a new `Config` with default settings.
    pub fn new() -> Config {
        Config {
            rustfmt: env::var_os("RUSTFMT").unwrap_or_else(|| OsString::from("rustfmt")),
            run_rustfmt: true,
            exhaustive: false,
            conjure_types_path: quote!(conjure_types),
        }
    }

    /// Controls exhaustive matchability of unions and enums.
    /// 
    /// Non-exhaustive unions and enums have the ability to deserialize and reserialize unknown variants. This enables
    /// clients to be more forward-compatible with changes made by newer servers.
    /// 
    /// Defaults to `false`.
    pub fn exhaustive(&mut self, exhaustive: bool) -> &mut Config {
        self.exhaustive = exhaustive;
        self
    }

    /// Controls the use of rustfmt to format generated source code.
    /// 
    /// Defaults to `true`.
    pub fn run_rustfmt(&mut self, run_rustfmt: bool) -> &mut Config {
        self.run_rustfmt = run_rustfmt;
        self
    }

    /// Sets the name of the binary used to format source code.
    /// 
    /// Defaults to the value of the `RUSTFMT` environment variable, or `rustfmt` if not set.
    pub fn rustfmt<T>(&mut self, rustfmt: T) -> &mut Config
    where
        T: AsRef<OsStr>,
    {
        self.rustfmt = rustfmt.as_ref().to_owned();
        self
    }

    /// Sets the module path to the root of the `conjure-types` crate.
    /// 
    /// Defaults to `conjure_types`.
    pub fn conjure_types_path(&mut self, conjure_types_path: &str) -> &mut Config {
        self.conjure_types_path = conjure_types_path.parse().unwrap();
        self
    }

    /// Generates Rust source files from a JSON-encoded Conjure IR file.
    pub fn generate_files<P, Q>(&self, ir_file: P, out_dir: Q) -> Result<(), Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        self.generate_files_inner(ir_file.as_ref(), out_dir.as_ref())
    }

    fn generate_files_inner(&self, ir_file: &Path, out_dir: &Path) -> Result<(), Error> {
        let defs = self.parse_ir(ir_file)?;

        if defs.version() != 1 {
            bail!("unsupported IR version {}", defs.version());
        }

        let modules = self.create_modules(&defs);

        fs::create_dir_all(out_dir)
            .with_context(|_| format!("error creating directory {}", out_dir.display()))?;

        for module in &modules {
            self.write_module(
                &out_dir.join(format!("{}.rs", module.module_name)),
                &module.contents,
            )?;
        }

        let root_module = self.create_root_module(&modules);
        self.write_module(&out_dir.join("mod.rs"), &root_module)?;

        Ok(())
    }

    fn parse_ir(&self, ir_file: &Path) -> Result<ConjureDefinition, Error> {
        let ir = fs::read_to_string(ir_file)
            .with_context(|_| format!("error reading file {}", ir_file.display()))?;

        let defs = serde_json::from_str(&ir)
            .with_context(|_| format!("error parsing Conjure IR file {}", ir_file.display()))?;

        Ok(defs)
    }

    fn write_module(&self, path: &Path, contents: &TokenStream) -> Result<(), Error> {
        fs::write(path, contents.to_string())
            .with_context(|_| format!("error writing module {}", path.display()))?;
        if self.run_rustfmt {
            let _ = Command::new(&self.rustfmt).arg(&path).status();
        }
        Ok(())
    }

    fn create_modules(&self, defs: &ConjureDefinition) -> Vec<Module> {
        let context = Context::new(&defs, self.exhaustive, self.conjure_types_path.clone());

        let mut modules = vec![];

        for def in defs.types() {
            let (type_name, contents) = match def {
                TypeDefinition::Enum(def) => (def.type_name(), enums::generate(&context, def)),
                TypeDefinition::Alias(def) => (def.type_name(), aliases::generate(&context, def)),
                TypeDefinition::Union(def) => (def.type_name(), unions::generate(&context, def)),
                TypeDefinition::Object(def) => (def.type_name(), objects::generate(&context, def)),
            };

            let module = Module {
                module_name: context.module_name(type_name),
                type_name: context.type_name(type_name.name()).to_string(),
                contents,
            };
            modules.push(module);
        }

        modules
    }

    fn create_root_module(&self, modules: &[Module]) -> TokenStream {
        let uses = modules.iter().map(|m| {
            let module_name = m.module_name.parse::<TokenStream>().unwrap();
            let type_name = m.type_name.parse::<TokenStream>().unwrap();
            quote! {
                #[doc(inline)]
                pub use self::#module_name::#type_name;
            }
        });

        let mods = modules.iter().map(|m| {
            let module_name = m.module_name.parse::<TokenStream>().unwrap();
            quote! {
                pub mod #module_name;
            }
        });

        quote! {
            #(#uses)*

            #(#mods)*
        }
    }
}

struct Module {
    module_name: String,
    type_name: String,
    contents: TokenStream,
}
