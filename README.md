# triseratops - The robust, in-depth Serato Parser & Serializer

![Version](https://img.shields.io/crates/v/triseratops)
![License](https://img.shields.io/github/license/Holzhaus/triseratops)
![Build Status](https://img.shields.io/github/checks-status/Holzhaus/triseratops/main)

![triseratops - The robust, in-depth Serato Parser & Serializer](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/logo.svg)

*triseratops* is library to parse metadata and database files written by the Serato DJ software, written in Rust.

**Note:** This library is currently still under heavy development and might have breaking API changes in the future.

## FAQ

### Is this software affiliated with Serato Ltd. or its related companies?

No, this library has been written independently. All formats have been
[reverse-engineered](https://homepages.ruhr-uni-bochum.de/jan.holthuis/posts/reversing-seratos-geob-tags)
and the parsing code has been implemented from scratch.

### Does this library read tag data from MP3/AIFF/MP4/FLAC/Ogg files?

It *parses* the Serato DJ metadata embedded in those files, but it can't read
directly from those file types. You need to use a separately library (e.g.
[`id3`](https://crates.io/crates/id3) for MP3/AIFF) to read the actual tag data
from media files.

### What's the history behind this library?

Work on reversing the format and a corresponding parser was started in 2019.
The goal was to integrate such a parser into [Mixxx](https://mixxx.org), a free
and open-source DJ software to reduce vendor lock-in and allow former Serato
DJ users to migrate to Mixxx without losing their cue points, beatgrids, etc.

Mixxx currently uses a different implementation (in C++) to parse those tags,
but since it will switch to [aoide](https://gitlab.com/uklotzde/aoide-rs) for
library management at some point in the future, it made sense to write a new
parser in Rust.

## License

This software is licensed under the terms of the [Affero GPL
v3](https://www.gnu.org/licenses/agpl-3.0.html), or
(at your discretion) any later version.
