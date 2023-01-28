// Copyright (c) 2023 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

extern crate id3;

use id3::Tag;
use std::env;
use std::string::String;
use triseratops::tag::format::id3::ID3Tag;
use triseratops::tag::TagFormat;

fn main() -> Result<(), triseratops::error::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut container = triseratops::tag::TagContainer::new();

    let tag = Tag::read_from_path(filename).expect("Failed to read tag!");
    for geob in tag.encapsulated_objects() {
        if !geob.description.starts_with("Serato ") {
            continue;
        }

        println!("{}", geob.description);
        println!("  Data: {} bytes", geob.data.len());
        match &geob.description[..] {
            triseratops::tag::Analysis::ID3_TAG => {
                let tag = triseratops::tag::Analysis::parse_id3(&geob.data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
            }
            triseratops::tag::Autotags::ID3_TAG => {
                let tag = triseratops::tag::Autotags::parse_id3(&geob.data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
                container.parse_autotags(&geob.data, TagFormat::ID3)?;
            }
            triseratops::tag::Beatgrid::ID3_TAG => {
                let tag = triseratops::tag::Beatgrid::parse_id3(&geob.data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
                container.parse_beatgrid(&geob.data, TagFormat::ID3)?;
            }
            triseratops::tag::Markers::ID3_TAG => {
                let tag = triseratops::tag::Markers::parse_id3(&geob.data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
                container.parse_markers(&geob.data, TagFormat::ID3)?;
            }
            triseratops::tag::Markers2::ID3_TAG => {
                let tag = triseratops::tag::Markers2::parse_id3(&geob.data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
                container.parse_markers2(&geob.data, TagFormat::ID3)?;
            }
            triseratops::tag::Overview::ID3_TAG => {
                let tag = triseratops::tag::Overview::parse_id3(&geob.data)?;
                let output = format!("{:#?}", tag);
                println!("{}", textwrap::indent(&output, "    "));
                container.parse_overview(&geob.data, TagFormat::ID3)?;
            }
            _ => (),
        }
    }

    println!();
    println!("Merged values");

    println!("  Auto Gain: {:?}", container.auto_gain());
    println!("  Gain DB: {:?}", container.gain_db());

    println!("  Cues");
    let output = format!("{:#?}", container.cues());
    println!("{}", textwrap::indent(&output, "    "));

    println!("  Loops");
    let output = format!("{:#?}", container.loops());
    println!("{}", textwrap::indent(&output, "    "));

    println!("  Track Color");
    let output = format!("{:#?}", container.track_color());
    println!("{}", textwrap::indent(&output, "    "));

    println!("  BPM Locked");
    let output = format!("{:?}", container.bpm_locked());
    println!("{}", textwrap::indent(&output, "    "));

    Ok(())
}
