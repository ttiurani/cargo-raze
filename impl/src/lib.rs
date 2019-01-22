// Copyright 2018 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate cargo;
extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate slug;
extern crate tempdir;
extern crate tera;
extern crate toml;
extern crate docopt;
extern crate failure;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

pub mod bazel;
pub mod context;
pub mod license;
pub mod metadata;
pub mod planning;
pub mod rendering;
pub mod settings;
pub mod util;