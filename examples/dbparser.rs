// Copyright (c) 2024 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use std::env;
use std::fs::File;
use std::io::Read;
use std::string::String;
use triseratops::library::database;

fn main() -> Result<(), triseratops::error::Error> {
    let mut args: Vec<String> = env::args().collect();
    let _prog = args.remove(0);

    if args.len() != 1 {
        panic!("Expected exactly 1 argument!")
    }
    let filename = args.remove(0);

    let mut file = File::open(filename).expect("Failed to open file!");
    let mut data = vec![];
    file.read_to_end(&mut data).expect("Failed to read data!");

    let fields = database::parse(&data)?;
    println!("{fields:#?}");
    Ok(())
}
