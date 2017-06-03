// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(lang_items, alloc_system, compiler_builtins_lib, panic_abort)]
#![feature(global_allocator, allocator_api)]
#![crate_type = "dylib"]
#![no_std]

extern crate alloc_system;
extern crate compiler_builtins;
extern crate panic_abort;

#[global_allocator]
static A: alloc_system::System = alloc_system::System;

#[no_mangle]
pub extern fn bar() {}

#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
