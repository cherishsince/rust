// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:rustdoc-trait-object-impl.rs
// build-aux-docs
// ignore-cross-compile

extern crate rustdoc_trait_object_impl;

// @has issue_32881/trait.Bar.html
// @has - '//code' "impl<'a> dyn Bar"
// @has - '//code' "impl<'a> Debug for dyn Bar"

pub use rustdoc_trait_object_impl::Bar;

