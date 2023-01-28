// Copyright (c) 2023 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use std::env;
use std::string::String;
use triseratops::library::Library;

fn main() -> Result<(), triseratops::error::Error> {
    let mut args: Vec<String> = env::args().collect();
    let _prog = args.remove(0);

    if args.len() != 1 {
        panic!("Expected exactly 1 argument!")
    }
    let path = args.remove(0);

    let library = Library::read_from_path(path)?;
    let tracks: Vec<_> = library.tracks().collect();
    println!("Library ({} tracks)", tracks.len());
    println!("{:#?}", tracks);
    let subcrates = library.subcrates();
    for subcrate in subcrates {
        let subcrate_tracks: Vec<_> = library.subcrate(&subcrate)?.collect();
        println!();
        print!("Subcrate: {} ({} tracks)", &subcrate, subcrate_tracks.len());
        println!("{:#?}", subcrate_tracks);
    }

    Ok(())
}
