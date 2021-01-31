//! Generic structs

/// Represents a 3-Byte RGB color value.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Represents 2-Byte version value.
#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

/// A [cue point](https://support.serato.com/hc/en-us/articles/360000067696-Cue-Points).
#[derive(Debug, Clone)]
pub struct Cue {
    pub index: u8,
    pub position_millis: u32,
    pub color: Color,
    pub label: String,
}

/// A [saved loops](https://serato.com/latest/blog/17885/pro-tip-trigger-saved-loops).
#[derive(Debug, Clone)]
pub struct Loop {
    pub index: u8,
    pub start_position_millis: u32,
    pub end_position_millis: u32,
    pub color: Color,
    pub is_locked: bool,
    pub label: String,
}

/// A [Serato Flip](https://serato.com/dj/pro/expansions/flip) performances.
#[derive(Debug, Clone)]
pub struct Flip {
    pub index: u8,
    pub is_enabled: bool,
    pub label: String,
    pub is_loop: bool,
    pub actions: Vec<FlipAction>,
}

/// An action inside of a [`Flip`](Flip) performance.
///
/// The last action is always a jump action where the source position is the time when the Flip
/// recording was stopped. If looping is enabled, it's target position is the source position of
/// the first entry. If not, the target position of that last entry is the same as its source
/// position.
#[derive(Debug, Clone)]
pub enum FlipAction {
    Censor(CensorFlipAction),
    Jump(JumpFlipAction),
    Unknown(UnknownFlipAction),
}

/// A "Censor" action inside of a [`Flip`](Flip) performance.
///
/// Actions of this type are used for censoring (playback speed factor is -1.0) and are followed
/// with a jump marker from `end_position_seconds` to the playback position that the track would be
/// at without the reverse playback.
#[derive(Debug, Clone)]
pub struct CensorFlipAction {
    /// The start position of the censoring.
    ///
    /// When playback reaches this position, the censoring starts.
    pub start_position_seconds: f64,

    /// The end position of the censoring.
    pub end_position_seconds: f64,

    /// The playback speed factor (usually -1.0).
    pub speed_factor: f64,
}

/// A "Jump" action inside of a [`Flip`](Flip) performance.
#[derive(Debug, Clone)]
pub struct JumpFlipAction {
    /// The source position of the jump.
    ///
    /// When playback reaches this position, the jump is performed.
    pub source_position_seconds: f64,

    /// The target position of the jump.
    pub target_position_seconds: f64,
}

/// A unknown action inside of a [`Flip`](Flip) performance that we don't have a parser for.
#[derive(Debug, Clone)]
pub struct UnknownFlipAction {
    pub id: u8,
    pub data: Vec<u8>,
}
