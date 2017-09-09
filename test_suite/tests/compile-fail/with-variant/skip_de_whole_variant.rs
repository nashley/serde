// Copyright 2017 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)] //~ ERROR: proc-macro derive panicked
//~^ HELP: variant `Unit` cannot have both #[serde(deserialize_with)] and #[serde(skip_deserializing)]
enum Enum {
    #[serde(deserialize_with = "deserialize_some_unit_variant")]
    #[serde(skip_deserializing)]
    Unit,
}

fn deserialize_some_unit_variant<'de, D>(_: D) -> StdResult<(), D::Error>
    where D: Deserializer<'de>,
{
    unimplemented!()
}

fn main() { }