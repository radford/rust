// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


// Make sure that if a process doesn't have its stdio/stderr descriptors set up
// that we don't die in a large ball of fire

use std::os;
use std::io::process;

pub fn main () {
    let args = os::args();
    if args.len() > 1 && args[1] == ~"child" {
        for _ in range(0, 1000) {
            println!("hello?");
        }
        for _ in range(0, 1000) {
            println!("hello?");
        }
        return;
    }

    let config = process::ProcessConfig {
        program : args[0].as_slice(),
        args : &[~"child"],
        stdout: process::Ignored,
        stderr: process::Ignored,
        .. process::ProcessConfig::new()
    };

    let mut p = process::Process::configure(config).unwrap();
    println!("{}", p.wait());
}
