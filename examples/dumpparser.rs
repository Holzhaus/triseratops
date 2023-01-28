// Copyright (c) 2023 Jan Holthuis <jan.holthuis@rub.de>
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
use triseratops::tag::format::{flac::FLACTag, id3::ID3Tag, mp4::MP4Tag, ogg::OggTag};

fn parse_file(data: &[u8]) -> bool {
    let res = triseratops::tag::Analysis::parse_id3(data);
    if res.is_ok() {
        println!("Tag: Analysis (ID3)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Autotags::parse_id3(data);
    if res.is_ok() {
        println!("Tag: Autotags (ID3)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Beatgrid::parse_id3(data);
    if res.is_ok() {
        println!("Tag: Beatgrid (ID3)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Markers::parse_id3(data);
    if res.is_ok() {
        println!("Tag: Markers (ID3)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Markers2::parse_id3(data);
    if res.is_ok() {
        println!("Tag: Markers2 (ID3)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Overview::parse_id3(data);
    if res.is_ok() {
        println!("Tag: Overview (ID3)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Analysis::parse_flac(data);
    if res.is_ok() {
        println!("Tag: Analysis (FLAC)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Autotags::parse_flac(data);
    if res.is_ok() {
        println!("Tag: Autotags (FLAC)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Beatgrid::parse_flac(data);
    if res.is_ok() {
        println!("Tag: Beatgrid (FLAC)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Markers2::parse_flac(data);
    if res.is_ok() {
        println!("Tag: Markers2 (FLAC)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Overview::parse_flac(data);
    if res.is_ok() {
        println!("Tag: Overview (FLAC)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::RelVolAd::parse_flac(data);
    if res.is_ok() {
        println!("Tag: RelVolAd (FLAC)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Analysis::parse_mp4(data);
    if res.is_ok() {
        println!("Tag: Analysis (MP4)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Autotags::parse_mp4(data);
    if res.is_ok() {
        println!("Tag: Autotags (MP4)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Beatgrid::parse_mp4(data);
    if res.is_ok() {
        println!("Tag: Beatgrid (MP4)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Markers::parse_mp4(data);
    if res.is_ok() {
        println!("Tag: Markers (MP4)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Markers2::parse_mp4(data);
    if res.is_ok() {
        println!("Tag: Markers2 (MP4)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Overview::parse_mp4(data);
    if res.is_ok() {
        println!("Tag: Overview (MP4)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::RelVolAd::parse_mp4(data);
    if res.is_ok() {
        println!("Tag: RelVolAd (MP4)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::VidAssoc::parse_mp4(data);
    if res.is_ok() {
        println!("Tag: VidAssoc (MP4)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Analysis::parse_ogg(data);
    if res.is_ok() {
        println!("Tag: Analysis (Ogg)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    let res = triseratops::tag::Markers2::parse_ogg(data);
    if res.is_ok() {
        println!("Tag: Markers2 (Ogg)");
        println!("{:#?}", res.unwrap());
        return true;
    }
    false
}

fn main() {
    let mut files: Vec<String> = env::args().collect();
    let _prog = files.remove(0);

    if files.is_empty() {
        panic!("No files specified!");
    }

    for filename in files {
        let mut file = File::open(&filename).expect("Failed to open file!");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Failed to read data!");
        println!("File: {}", &filename);
        if !parse_file(&data) {
            println!("Unable to parse file");
        }
    }
}
