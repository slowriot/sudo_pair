// Copyright 2018 Square Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied. See the License for the specific language governing
// permissions and limitations under the License.

//! description = "Macros to simplify writing sudo plugins"
//!
//! TODO: explain

// TODO: provide the Plugin object to all callbacks?

#![warn(bad_style)]
#![warn(future_incompatible)]
#![warn(nonstandard_style)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(rustdoc)]
#![warn(unused)]

#![warn(bare_trait_objects)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unstable_features)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

// this entire crate is unsafe code
#![allow(unsafe_code)]

#![cfg_attr(feature="cargo-clippy", warn(clippy))]
#![cfg_attr(feature="cargo-clippy", warn(clippy_complexity))]
#![cfg_attr(feature="cargo-clippy", warn(clippy_correctness))]
#![cfg_attr(feature="cargo-clippy", warn(clippy_pedantic))]
#![cfg_attr(feature="cargo-clippy", warn(clippy_perf))]
#![cfg_attr(feature="cargo-clippy", warn(clippy_style))]

// this warns on names that are out of our control like argv, argc, uid,
// and gid
#![cfg_attr(feature="cargo-clippy", allow(similar_names))]

// TODO: we can remove `bindgen` as a direct dependency and just bundle
// its output since it's static; these should pass much more reliably
// then
//
// #![cfg_attr(feature="cargo-clippy", warn(clippy_cargo))]

pub mod errors;
pub mod macros;
pub mod plugin;

mod version;

pub use sudo_plugin_sys as sys;

pub use self::plugin::*;
