//! Traits for the different tag types ([ID3](id3::ID3Tag), [FLAC](flac::FLACTag), etc.)

pub mod enveloped;
pub mod flac;
pub mod id3;
pub mod mp4;
pub mod ogg;

use crate::error::Error;

pub trait Tag: Sized {
    const NAME: &'static str;
    fn parse(input: &[u8]) -> Result<Self, Error>;
}
