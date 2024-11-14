// Copyright (c) 2024 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! Parsers for Serato's file tags
pub mod color;
pub mod format;
pub mod generic;
pub mod serato32;
mod util;

pub mod container;
pub use container::TagContainer;
pub use container::TagFormat;

pub mod analysis;
pub use self::analysis::Analysis;

pub mod autotags;
pub use self::autotags::Autotags;

pub mod beatgrid;
pub use self::beatgrid::Beatgrid;

pub mod markers;
pub use self::markers::Markers;

pub mod markers2;
pub use self::markers2::Markers2;

pub mod overview;
pub use self::overview::Overview;

pub mod relvolad;
pub use self::relvolad::RelVolAd;

pub mod vidassoc;
pub use self::vidassoc::VidAssoc;
