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
//! | Conjure       | Rust                                 |
//! | ------------- | ------------------------------------ |
//! | `string`      | `String`                             |
//! | `datetime`    | `chrono::DateTime<Utc>`              |
//! | `integer`     | `i32`                                |
//! | `double`      | `f64`                                |
//! | `safelong`    | `conjure_object::SafeLong`           |
//! | `binary`      | `bytes::Bytes`                       |
//! | `any`         | `conjure_object::Any`                |
//! | `boolean`     | `bool`                               |
//! | `uuid`        | `uuid::Uuid`                         |
//! | `rid`         | `conjure_object::ResourceIdentifier` |
//! | `bearertoken` | `conjure_object::BearerToken`        |
//! | `optional<T>` | `Option<T>`                          |
//! | `list<T>`     | `Vec<T>`                             |
//! | `set<T>`      | `BTreeSet<T>`                        |
//! | `map<K, V>`   | `BTreeMap<K, V>`                     |
//!
//! Many of these are exposed by the `conjure-object` crate, which is a required dependency of crates containing the
//! generated code.
//!
//! ### `double`
//!
//! Rust's `f64` type does not implement `Ord`, `Eq`, or `Hash`, which requires some special casing. Sets and maps keyed
//! directly by `double` are represented as `BTreeSet<DoubleKey>` and `BTreeMap<DoubleKey, T>`, where the
//! [`DoubleKey`](conjure_object::DoubleKey) type wraps `f64` and implements those traits by ordering `NaN` greater
//! than all other values and equal to itself.
//!
//! Conjure aliases, objects, and unions wrapping `double` types have trait implementations which use the same logic.
//!
//! ## Objects
//!
//! Conjure objects turn into Rust structs along with builders used to construct them:
//!
//! ```
//! # use conjure_codegen::example_types::objects::product::{ManyFieldExample, StringAliasExample};
//! let object = ManyFieldExample::builder()
//!     .string("foo")
//!     .integer(123)
//!     .double_value(3.14)
//!     .alias(StringAliasExample("foobar".to_string()))
//!     .optional_item("bar".to_string())
//!     .items(vec!["hello".to_string(), "world".to_string()])
//!     .build();
//!
//! assert_eq!(object.string(), "foo");
//! assert_eq!(object.optional_item(), Some("bar"));
//! ```
//!
//! The builder types are by-value and infallible - the compiler will prevent code from compiling if all required
//! fields are not set. The API requires that all required fields be set first strictly in declaration order,
//! after which optional fields can be set in any order.
//!
//! Objects with 3 or fewer required fields also have an explicit constructor:
//!
//! ```rust
//! # use conjure_codegen::example_types::objects::product::BooleanExample;
//! let object = BooleanExample::new(true);
//!
//! assert_eq!(object.coin(), true);
//! ```
//!
//! The generated structs implement `Debug`, `Clone`, `PartialEq`, Eq, `PartialOrd`, `Ord`, `Hash`, `Serialize`, and
//! `Deserialize`. They implement `Copy` if they consist entirely of copyable primitive types.
//!
//! ## Unions
//!
//! Conjure unions turn into Rust enums. By default, unions are *extensible* through an additional `Unknown` variant.
//! This allows unions to be forward-compatible by allowing clients to deserialize variants they don't yet know about
//! and reserialize them properly:
//!
//! ```
//! # use conjure_codegen::example_types::objects::product::UnionTypeExample;
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
//! The generated enums implement `Debug`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Serialize`, and
//! `Deserialize`. Union variants which are themselves unions are boxed in the generated enum to avoid self-referential
//! type definitions.
//!
//! ## Enums
//!
//! Conjure enums turn into Rust enums. By default, enums are *extensible*. This allows enums to be forward-compatible
//! by allowing clients to deserialize variants they don't yet know about and reserialize them properly:
//!
//! ```
//! # use conjure_codegen::example_types::objects::product::EnumExample;
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
//! # use conjure_codegen::example_types::objects::product::StringAliasExample;
//! let alias_value = StringAliasExample("hello world".to_string());
//! assert!(alias_value.starts_with("hello"));
//! ```
//!
//! The generated structs implement `Deref`, `DerefMut`, `Debug`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`,
//! `Hash`, `Serialize`, and `Deserialize`. They implement `Copy` if they wrap a copyable primitive type, `Default`
//! if they wrap a type implementing `Default`, `FromIterator` if they wrap a type implementing `FromIterator`, and
//! `Display` if they wrap a type implementing `Display`. They also implement `From<T>`, where `T` is the fully
//! de-aliased inner type.
//!
//! ## Errors
//!
//! Conjure errors turn into Rust structs storing the error's parameters as if it were a Conjure object. The struct
//! additionally implements the `conjure_error::ErrorType` trait which encodes the extra error metadata:
//!
//! ```
//! # use conjure_codegen::example_types::errors::product::InvalidServiceDefinition;
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
//! The client object wraps a raw HTTP client and provides methods to interact with the service's endpoints. Both
//! synchronous and asynchronous clients are provided:
//!
//! Synchronous:
//! ```
//! use conjure_http::client::Service;
//! # use conjure_codegen::example_types::clients::another::{TestService, TestServiceClient};
//! # fn foo<T: conjure_http::client::Client>(http_client: T) -> Result<(), conjure_error::Error> {
//! # let auth_token = "foobar".parse().unwrap();
//! let client = TestServiceClient::new(http_client);
//! let file_systems = client.get_file_systems(&auth_token)?;
//! # Ok(())
//! # }
//! ```
//!
//! Asynchronous:
//! ```
//! use conjure_http::client::AsyncService;
//! # use conjure_codegen::example_types::clients::another::{AsyncTestService, AsyncTestServiceClient};
//! # async fn foo<T>(http_client: T) -> Result<(), conjure_error::Error> where T: conjure_http::client::AsyncClient + Sync + Send, T::ResponseBody: 'static + Send {
//! # let auth_token = "foobar".parse().unwrap();
//! let client = AsyncTestServiceClient::new(http_client);
//! let file_systems = client.get_file_systems(&auth_token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Servers
//!
//! Conjure generates a trait and accompanying wrapper resource which are used to implement the service's endpoints.
//! Both synchronous and asynchronous servers are supported:
//!
//! Synchronous:
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
//!
//! Asynchronous:
//! ```ignore
//! struct TestServiceHandler;
//!
//! impl<T> AsyncTestService<T> for TestServiceHandler
//! where
//!     T: AsyncRead + 'static + Send
//! {
//!     async fn get_file_systems(
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
//!
//! ### Endpoint Tags
//!
//! * `server-request-context` - The generated server trait method will have an additional
//!     `RequestContext` argument providing lower level access to request and response information.
//! * `server-limit-request-size: <size>` - Sets the maximum request body size for endpoints with
//!     serializable request bodies. `<size>` should be a human-readable byte count (e.g. `50Mi` or
//!     `100Ki`). Defaults to `50Mi`.
#![warn(clippy::all, missing_docs)]
#![allow(clippy::needless_doctest_main)]
#![recursion_limit = "256"]

use crate::context::Context;
use crate::types::objects::{ConjureDefinition, TypeDefinition};
use anyhow::{bail, Context as _, Error};
use context::BaseModule;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

mod aliases;
mod cargo_toml;
mod clients;
mod context;
mod enums;
mod errors;
mod objects;
mod servers;
#[allow(dead_code, clippy::all)]
#[rustfmt::skip]
mod types;
mod human_size;
mod unions;

/// Examples of generated Conjure code.
///
/// This module is only intended to be present in documentation; it shouldn't be relied on by any library code.
#[cfg(feature = "example-types")]
#[allow(warnings)]
#[rustfmt::skip]
pub mod example_types;

struct CrateInfo {
    name: String,
    version: String,
}

/// Codegen configuration.
pub struct Config {
    exhaustive: bool,
    strip_prefix: Option<String>,
    version: Option<String>,
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
            exhaustive: false,
            strip_prefix: None,
            version: None,
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

    /// No longer used.
    #[deprecated(note = "no longer used", since = "1.2.0")]
    pub fn run_rustfmt(&mut self, _run_rustfmt: bool) -> &mut Config {
        self
    }

    /// No longer used.
    #[deprecated(note = "no longer used", since = "1.2.0")]
    pub fn rustfmt<T>(&mut self, _rustfmt: T) -> &mut Config
    where
        T: AsRef<OsStr>,
    {
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

    /// Sets the version included in endpoint metadata for generated client bindings.
    ///
    /// Defaults to the version passed to [`Self::build_crate`], or `None` otherwise.
    pub fn version<T>(&mut self, version: T) -> &mut Config
    where
        T: Into<Option<String>>,
    {
        self.version = version.into();
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
            self.write_rustfmt_toml(out_dir)?;
        }

        modules.render(&src_dir, lib_root)?;

        Ok(())
    }

    fn parse_ir(&self, ir_file: &Path) -> Result<ConjureDefinition, Error> {
        let ir = fs::read_to_string(ir_file)
            .with_context(|| format!("error reading file {}", ir_file.display()))?;

        let defs = conjure_serde::json::client_from_str(&ir)
            .with_context(|| format!("error parsing Conjure IR file {}", ir_file.display()))?;

        Ok(defs)
    }

    fn create_modules(&self, defs: &ConjureDefinition) -> ModuleTrie {
        let context = Context::new(
            defs,
            self.exhaustive,
            self.strip_prefix.as_deref(),
            self.version
                .as_deref()
                .or_else(|| self.build_crate.as_ref().map(|v| &*v.version)),
        );

        let mut root = ModuleTrie::new();

        for def in defs.types() {
            let (type_name, contents) = match def {
                TypeDefinition::Enum(def) => (def.type_name(), enums::generate(&context, def)),
                TypeDefinition::Alias(def) => (def.type_name(), aliases::generate(&context, def)),
                TypeDefinition::Union(def) => (def.type_name(), unions::generate(&context, def)),
                TypeDefinition::Object(def) => (
                    def.type_name(),
                    objects::generate(&context, BaseModule::Objects, def),
                ),
            };

            let type_ = Type {
                module_name: context.module_name(type_name),
                type_names: vec![context.type_name(type_name.name()).to_string()],
                contents,
            };
            root.insert(&context.module_path(BaseModule::Objects, type_name), type_);
        }

        for def in defs.errors() {
            let type_ = Type {
                module_name: context.module_name(def.error_name()),
                type_names: vec![context.type_name(def.error_name().name()).to_string()],
                contents: errors::generate(&context, def),
            };
            root.insert(
                &context.module_path(BaseModule::Errors, def.error_name()),
                type_,
            );
        }

        for def in defs.services() {
            let client = clients::generate(&context, def);

            let contents = quote! {
                #client
            };
            let type_ = Type {
                module_name: context.module_name(def.service_name()),
                type_names: vec![
                    format!("{}", def.service_name().name()),
                    format!("{}Client", def.service_name().name()),
                    format!("Async{}", def.service_name().name()),
                    format!("Async{}Client", def.service_name().name()),
                ],
                contents,
            };
            root.insert(
                &context.module_path(BaseModule::Clients, def.service_name()),
                type_,
            );

            let server = servers::generate(&context, def);

            let contents = quote! {
                #server
            };
            let type_ = Type {
                module_name: context.module_name(def.service_name()),
                type_names: vec![
                    context.type_name(def.service_name().name()).to_string(),
                    format!("Async{}", def.service_name().name()),
                    format!("{}Endpoints", def.service_name().name()),
                    format!("Async{}Endpoints", def.service_name().name()),
                ],
                contents,
            };
            root.insert(
                &context.module_path(BaseModule::Endpoints, def.service_name()),
                type_,
            );
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
            .with_context(|| format!("error creating directory {}", dir.display()))?;

        let metadata = def
            .extensions()
            .get("recommended-product-dependencies")
            .map(|deps| cargo_toml::Metadata {
                sls: cargo_toml::Sls {
                    recommended_product_dependencies: deps,
                },
            });

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
        let mut dependencies = BTreeMap::new();
        if needs_object {
            dependencies.insert("conjure-object", conjure_version);
        }
        if needs_error {
            dependencies.insert("conjure-error", conjure_version);
        }
        if needs_http {
            dependencies.insert("conjure-http", conjure_version);
        }

        let manifest = cargo_toml::Manifest {
            package: cargo_toml::Package {
                name: &info.name,
                version: &info.version,
                edition: "2018",
                metadata,
            },
            dependencies,
        };

        let manifest = toml::to_string_pretty(&manifest).unwrap();

        let file = dir.join("Cargo.toml");

        fs::write(&file, manifest)
            .with_context(|| format!("error writing manifest file {}", file.display()))?;

        Ok(())
    }

    fn write_rustfmt_toml(&self, dir: &Path) -> Result<(), Error> {
        let contents = "\
disable_all_formatting = true
";

        let file = dir.join("rustfmt.toml");

        fs::write(file, contents).with_context(|| "error writing rustfmt.toml")?;

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

    fn render(&self, dir: &Path, lib_root: bool) -> Result<(), Error> {
        fs::create_dir_all(dir)
            .with_context(|| format!("error creating directory {}", dir.display()))?;

        for type_ in &self.types {
            self.write_module(
                &dir.join(format!("{}.rs", type_.module_name)),
                &type_.contents,
            )?;
        }

        for (name, module) in &self.submodules {
            module.render(&dir.join(name), false)?;
        }

        let root = self.create_root_module(lib_root);
        let file_name = if lib_root { "lib.rs" } else { "mod.rs" };
        self.write_module(&dir.join(file_name), &root)?;

        Ok(())
    }

    fn write_module(&self, path: &Path, contents: &TokenStream) -> Result<(), Error> {
        let file = syn::parse2(contents.clone())?;
        let formatted = prettyplease::unparse(&file);

        fs::write(path, formatted)
            .with_context(|| format!("error writing module {}", path.display()))?;
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
