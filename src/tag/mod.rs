//! Parsers for Serato's file tags
pub mod format;
pub mod generic;
pub mod serato32;
pub(self) mod util;

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
