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
//!
//! # Examples
//!
//! Code generation via a build script, assuming we have a `service-api.conjure.json` file in the crate root:
//!
//! build.rs:
//!
//! ```no_run
//! use std::env;
//! use std::path::Path;
//!
//! fn main() {
//!     let input = "service-api.conjure.json";
//!     let output = Path::new(&env::var_os("OUT_DIR").unwrap()).join("service_api");
//!
//!     println!("cargo:rerun-if-changed={}", input);
//!     conjure_codegen::Config::new()
//!         .run_rustfmt(false)
//!         .strip_prefix("com.foobar.service".to_string())
//!         .generate_files(input, output)
//!         .unwrap();
//! }
//! ```
//!
//! src/lib.rs:
//!
//! ```ignore
//! mod service_api {
//!     include!(concat!(env!("OUT_DIR"), "/service_api/mod.rs"));
//! }
//! ```
//!
//! # Types
//!
//! ## Builtin
//!
//! Builtin types map directly to existing Rust types:
//!
//! | Conjure       | Rust                                |
//! | ------------- | ----------------------------------- |
//! | `string`      | `String`                            |
//! | `datetime`    | `chrono::DateTime<Utc>`             |
//! | `integer`     | `i32`                               |
//! | `double`      | `f64`                               |
//! | `safelong`    | `conjure_object::SafeLong`           |
//! | `binary`      | `serde_bytes::ByteBuf`              |
//! | `any`         | `serde_value::Value`                |
//! | `boolean`     | `bool`                              |
//! | `uuid`        | `uuid::Uuid`                        |
//! | `rid`         | `conjure_object::ResourceIdentifier` |
//! | `bearertoken` | `conjure_object::BearerToken`        |
//! | `optional<T>` | `Option<T>`                         |
//! | `list<T>`     | `Vec<T>`                            |
//! | `set<T>`      | `BTreeSet<T>`                       |
//! | `map<K, V>`   | `BTreeMap<K, V>`                    |
//!
//! Many of these are exposed by the `conjure-object` crate, which is a required dependency of crates containing the
//! generated code.
//!
//! ## Objects
//!
//! Conjure objects turn into Rust structs along with builders used to construct them:
//!
//! ```
//! # use conjure_codegen::example_types::product::{ManyFieldExample, StringAliasExample};
//! let object = ManyFieldExample::builder()
//!     .string("foo")
//!     .integer(123)
//!     .double_value(3.14)
//!     .optional_item("bar".to_string())
//!     .items(vec!["hello".to_string(), "world".to_string()])
//!     .alias(StringAliasExample("foobar".to_string()))
//!     .build();
//!
//! assert_eq!(object.string(), "foo");
//! assert_eq!(object.optional_item(), Some("bar"));
//! ```
//!
//! Objects with 3 or fewer fields also have an explicit constructor:
//!
//! ```rust
//! # use conjure_codegen::example_types::product::BooleanExample;
//! let object = BooleanExample::new(true);
//!
//! assert_eq!(object.coin(), true);
//! ```
//!
//! The generated structs implement `Debug`, `Clone`, `PartialEq`, `PartialOrd`, `Serialize`, and `Deserialize`. They
//! also implement `Eq`, `Ord`, and `Hash` if they do not contain a `double` value, and `Copy` if they consist entirely
//! of copyable primitive types.
//!
//! ## Unions
//!
//! Conjure unions turn into Rust enums. By default, unions are *extensible* through an additional `Unknown` variant.
//! This allows unions to be forward-compatible by allowing clients to deserialize variants they don't yet know about
//! and reserialize them properly:
//!
//! ```
//! # use conjure_codegen::example_types::product::UnionTypeExample;
//! # let union_value = UnionTypeExample::If(0);
//! match union_value {
//!     UnionTypeExample::StringExample(string) => {
//!         // ...
//!     }
//!     UnionTypeExample::Set(set) => {
//!         // ...
//!     }
//!     // ...
//!     UnionTypeExample::Unknown(unknown) => {
//!         println!("got unknown variant: {}", unknown.type_());
//!     }
//!     # _ => {}
//! }
//! ```
//!
//! The generated enums implement `Debug`, `Clone`, `PartialEq`, `PartialOrd`, `Serialize`, and `Deserialize`. They
//! also implement `Eq`, `Ord`, and `Hash` if they do not contain a `double` value. Union variants which are themselves
//! unions are boxed in the generated enum to avoid self-referential type definitions.
//!
//! ## Enums
//!
//! Conjure enums turn into Rust enums. By default, enums are *extensible*. This allows enums to be forward-compatible
//! by allowing clients to deserialize variants they don't yet know about and reserialize them properly:
//!
//! ```
//! # use conjure_codegen::example_types::product::EnumExample;
//! # let enum_value = EnumExample::One;
//! match enum_value {
//!     EnumExample::One => println!("found one"),
//!     EnumExample::Two => println!("found two"),
//!     EnumExample::Unknown(unknown) => println!("got unknown variant: {}", unknown),
//! }
//! ```
//!
//! The generated enums implement `Debug`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Display`,
//! `Serialize`, and `Deserialize`.
//!
//! ## Aliases
//!
//! Conjure aliases turn into Rust newtype structs that act like their inner value:
//!
//! ```
//! # use conjure_codegen::example_types::product::StringAliasExample;
//! let alias_value = StringAliasExample("hello world".to_string());
//! assert!(alias_value.starts_with("hello"));
//! ```
//!
//! The generated structs implement `Deref`, `DerefMut`, `Debug`, `Clone`, `PartialEq`, `PartialOrd`, `Serialize`, and
//! `Deserialize`. They also implement `Eq`, `Ord`, and `Hash` if they do not contain a `double` value, `Copy` if they
//! wrap a copyable primitive type, `Default` if they wrap a type implementing `Default`, and `Display` if they wrap a
//! type implementing `Display`.
//!
//! ## Errors
//!
//! Conjure errors turn into Rust structs storing the error's parameters as if it were a Conjure object. The struct
//! additionally implements the `conjure_error::ErrorType` trait which encodes the extra error metadata:
//!
//! ```
//! # use conjure_codegen::example_types::product::InvalidServiceDefinition;
//! # let (name, definition) = ("", "");
//! use conjure_error::{ErrorType, ErrorCode};
//!
//! let error = InvalidServiceDefinition::new(name, definition);
//!
//! assert_eq!(error.code(), ErrorCode::InvalidArgument);
//! assert_eq!(error.name(), "Conjure:InvalidServiceDefinition");
//! ```
//!
//! ## Services
//!
//! Conjure services turn into client- and server-side interfaces:
//!
//! ### Clients
//!
//! The client object wraps a raw HTTP client and provides methods to interact with the service's endpoints:
//!
//! ```
//! # use conjure_codegen::example_types::another::TestServiceClient;
//! # fn foo<T: conjure_http::client::Client>(http_client: T) -> Result<(), conjure_error::Error> {
//! # let auth_token = "foobar".parse().unwrap();
//! let client = TestServiceClient::new(http_client);
//! let file_systems = client.get_file_systems(&auth_token)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Servers
//!
//! Conjure generates a trait and accompanying wrapper resource which are used to implement the service's endpoints:
//!
//! ```ignore
//! struct TestServiceHandler;
//!
//! impl<T> TestService<T> for TestServiceHandler
//! where
//!     T: Read
//! {
//!     fn get_file_systems(
//!         &self,
//!         auth: AuthToken,
//!     ) -> Result<BTreeMap<String, BackingFileSystem>, Error> {
//!         // ...
//!     }
//!
//!     // ...
//! }
//!
//! let resource = TestServiceResource::new(TestServiceHandler);
//! http_server.register(resource);
//! ```
#![warn(clippy::all, missing_docs)]
#![doc(html_root_url = "https://docs.rs/conjure-codegen/0.3")]
#![recursion_limit = "256"]

use failure::{bail, Error, ResultExt};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt::Write;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::context::Context;
use crate::types::{ConjureDefinition, TypeDefinition};

mod aliases;
mod clients;
mod context;
mod enums;
mod errors;
mod objects;
mod servers;
#[allow(dead_code, clippy::all)]
mod types;
mod unions;

/// Examples of generated Conjure code.
///
/// This module is only intended to be present in documentation; it shouldn't be relied on by any library code.
#[cfg(feature = "example-types")]
#[allow(warnings)]
pub mod example_types;

struct CrateInfo {
    name: String,
    version: String,
}

/// Codegen configuration.
pub struct Config {
    rustfmt: OsString,
    run_rustfmt: bool,
    exhaustive: bool,
    strip_prefix: Option<String>,
    build_crate: Option<CrateInfo>,
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
            strip_prefix: None,
            build_crate: None,
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

    /// Sets a prefix that will be stripped from package names.
    ///
    /// Defaults to `None`.
    pub fn strip_prefix<T>(&mut self, strip_prefix: T) -> &mut Config
    where
        T: Into<Option<String>>,
    {
        self.strip_prefix = strip_prefix.into();
        self
    }

    /// Switches generation to create a full crate.
    ///
    /// Defaults to just generating a single module.
    pub fn build_crate(&mut self, name: &str, version: &str) -> &mut Config {
        self.build_crate = Some(CrateInfo {
            name: name.to_string(),
            version: version.to_string(),
        });
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
        let (src_dir, lib_root) = if self.build_crate.is_some() {
            (out_dir.join("src"), true)
        } else {
            (out_dir.to_path_buf(), false)
        };

        if let Some(info) = &self.build_crate {
            self.write_cargo_toml(out_dir, info, &defs)?;
        }

        modules.render(self, &src_dir, lib_root)?;

        // rustfmt sometimes takes 2 runs to converge (?!)
        for _ in 0..2 {
            if self.run_rustfmt {
                let file_name = if lib_root { "lib.rs" } else { "mod.rs" };
                let _ = Command::new(&self.rustfmt)
                    .arg(&src_dir.join(file_name))
                    .status();
            }
        }

        Ok(())
    }

    fn parse_ir(&self, ir_file: &Path) -> Result<ConjureDefinition, Error> {
        let ir = fs::read_to_string(ir_file)
            .with_context(|_| format!("error reading file {}", ir_file.display()))?;

        let defs = conjure_serde::json::server_from_str(&ir)
            .with_context(|_| format!("error parsing Conjure IR file {}", ir_file.display()))?;

        Ok(defs)
    }

    fn create_modules(&self, defs: &ConjureDefinition) -> ModuleTrie {
        let context = Context::new(
            &defs,
            self.exhaustive,
            self.strip_prefix.as_ref().map(|s| &**s),
        );

        let mut root = ModuleTrie::new();

        for def in defs.types() {
            let (type_name, contents) = match def {
                TypeDefinition::Enum(def) => (def.type_name(), enums::generate(&context, def)),
                TypeDefinition::Alias(def) => (def.type_name(), aliases::generate(&context, def)),
                TypeDefinition::Union(def) => (def.type_name(), unions::generate(&context, def)),
                TypeDefinition::Object(def) => (def.type_name(), objects::generate(&context, def)),
            };

            let type_ = Type {
                module_name: context.module_name(type_name),
                type_names: vec![context.type_name(type_name.name()).to_string()],
                contents,
            };
            root.insert(&context.module_path(&type_name), type_);
        }

        for def in defs.errors() {
            let type_ = Type {
                module_name: context.module_name(def.error_name()),
                type_names: vec![context.type_name(def.error_name().name()).to_string()],
                contents: errors::generate(&context, def),
            };
            root.insert(&context.module_path(def.error_name()), type_);
        }

        for def in defs.services() {
            let client = clients::generate(&context, def);
            let server = servers::generate(&context, def);

            let contents = quote! {
                #client
                #server
            };
            let type_ = Type {
                module_name: context.module_name(def.service_name()),
                type_names: vec![
                    format!("{}Client", def.service_name().name()),
                    context.type_name(def.service_name().name()).to_string(),
                    format!("{}Resource", def.service_name().name()),
                ],
                contents,
            };
            root.insert(&context.module_path(def.service_name()), type_);
        }

        root
    }

    fn write_cargo_toml(
        &self,
        dir: &Path,
        info: &CrateInfo,
        def: &ConjureDefinition,
    ) -> Result<(), Error> {
        fs::create_dir_all(dir)
            .with_context(|_| format!("error creating directory {}", dir.display()))?;

        let mut manifest = format!(
            r#"[package]
name = "{}"
version = "{}"
authors = []
edition = "2018"

[dependencies]
"#,
            info.name, info.version
        );

        let mut needs_object = false;
        let mut needs_error = false;
        let mut needs_http = false;

        if !def.types().is_empty() {
            needs_object = true;
        }

        if !def.errors().is_empty() {
            needs_object = true;
            needs_error = true;
        }

        if !def.services().is_empty() {
            needs_http = true;
            needs_object = true;
        }

        let conjure_version = env!("CARGO_PKG_VERSION");
        if needs_object {
            writeln!(manifest, r#"conjure-object = "{}""#, conjure_version).unwrap();
        }
        if needs_error {
            writeln!(manifest, r#"conjure-error = "{}""#, conjure_version).unwrap();
        }
        if needs_http {
            writeln!(manifest, r#"conjure-http = "{}""#, conjure_version).unwrap();
        }

        let file = dir.join("Cargo.toml");

        fs::write(&file, &manifest)
            .with_context(|_| format!("error writing manifest file {}", file.display()))?;

        Ok(())
    }
}

struct Type {
    module_name: String,
    type_names: Vec<String>,
    contents: TokenStream,
}

struct ModuleTrie {
    submodules: BTreeMap<String, ModuleTrie>,
    types: Vec<Type>,
}

impl ModuleTrie {
    fn new() -> ModuleTrie {
        ModuleTrie {
            submodules: BTreeMap::new(),
            types: vec![],
        }
    }

    fn insert(&mut self, module_path: &[String], type_: Type) {
        match module_path.split_first() {
            Some((first, rest)) => self
                .submodules
                .entry(first.clone())
                .or_insert_with(ModuleTrie::new)
                .insert(rest, type_),
            None => self.types.push(type_),
        }
    }

    fn render(&self, config: &Config, dir: &Path, lib_root: bool) -> Result<(), Error> {
        fs::create_dir_all(dir)
            .with_context(|_| format!("error creating directory {}", dir.display()))?;

        for type_ in &self.types {
            self.write_module(
                &dir.join(format!("{}.rs", type_.module_name)),
                &type_.contents,
            )?;
        }

        for (name, module) in &self.submodules {
            module.render(config, &dir.join(name), false)?;
        }

        let root = self.create_root_module(lib_root);
        let file_name = if lib_root { "lib.rs" } else { "mod.rs" };
        self.write_module(&dir.join(file_name), &root)?;

        Ok(())
    }

    fn write_module(&self, path: &Path, contents: &TokenStream) -> Result<(), Error> {
        fs::write(path, contents.to_string())
            .with_context(|_| format!("error writing module {}", path.display()))?;
        Ok(())
    }

    fn create_root_module(&self, lib_root: bool) -> TokenStream {
        let attrs = if lib_root {
            quote! {
                #![allow(warnings)]
            }
        } else {
            quote! {}
        };

        let uses = self.types.iter().map(|m| {
            let module_name = m.module_name.parse::<TokenStream>().unwrap();
            let type_names = m
                .type_names
                .iter()
                .map(|n| n.parse::<TokenStream>().unwrap());
            quote! {
                #[doc(inline)]
                pub use self::#module_name::{#(#type_names),*};
            }
        });

        let type_mods = self.types.iter().map(|m| {
            let module_name = m.module_name.parse::<TokenStream>().unwrap();
            quote! {
                pub mod #module_name;
            }
        });

        let sub_mods = self.submodules.keys().map(|v| {
            let module_name = v.parse::<TokenStream>().unwrap();
            quote! {
                pub mod #module_name;
            }
        });

        quote! {
            #attrs
            #(#uses)*

            #(#type_mods)*
            #(#sub_mods)*
        }
    }
}
