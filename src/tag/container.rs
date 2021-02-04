//! Provides the [`TagContainer` struct](TagContainer), a simple interface to access data in
//! Serato's tags.
use super::{
    beatgrid, format::flac::FLACTag, format::id3::ID3Tag, format::mp4::MP4Tag, format::ogg::OggTag,
    generic, markers, Autotags, Beatgrid, Markers, Markers2, Overview,
};
use crate::error::Error;

/// Provides a streamlined interface for retrieving Serato tag data.
///
/// If you're not interested in the low-level format details, just use this instead of the
/// low-level structs (e.g. [`Markers`](Markers), [`Markers2`](Markers2), etc.)
///
/// Some of the data in Serato's tags is redundant and may contradict each other. This class
/// implements the same merge strategies for inconsistent data that Serato uses, too.
pub struct TagContainer {
    autotags: Option<Autotags>,
    beatgrid: Option<Beatgrid>,
    markers: Option<Markers>,
    markers2: Option<Markers2>,
    overview: Option<Overview>,
}

/// The tag type of the data.
///
/// The format of the Serato tag data differs between tag types.
/// Therefore it's necessary to tell the parser from what kind of the the data originates from.
pub enum TagFormat {
    ID3,
    FLAC,
    MP4,
    Ogg,
}

impl TagContainer {
    /// Create an empty Serato tag container.
    pub fn new() -> Self {
        Self {
            autotags: None,
            beatgrid: None,
            markers: None,
            markers2: None,
            overview: None,
        }
    }

    /// Parse the [`Serato Autotags`](Autotags) tag.
    pub fn parse_autotags(&mut self, input: &[u8], tag_format: TagFormat) -> Result<(), Error> {
        match tag_format {
            TagFormat::ID3 => {
                self.autotags = Some(Autotags::parse_id3(input)?);
            }
            TagFormat::FLAC => {
                self.autotags = Some(Autotags::parse_flac(input)?);
            }
            TagFormat::MP4 => {
                self.autotags = Some(Autotags::parse_mp4(input)?);
            }
            _ => return Err(Error::UnsupportedTagFormat),
        }
        Ok(())
    }

    /// Parse the [`Serato BeatGrid`](Beatgrid) tag.
    pub fn parse_beatgrid(&mut self, input: &[u8], tag_format: TagFormat) -> Result<(), Error> {
        match tag_format {
            TagFormat::ID3 => {
                self.beatgrid = Some(Beatgrid::parse_id3(input)?);
            }
            TagFormat::FLAC => {
                self.beatgrid = Some(Beatgrid::parse_flac(input)?);
            }
            TagFormat::MP4 => {
                self.beatgrid = Some(Beatgrid::parse_mp4(input)?);
            }
            _ => return Err(Error::UnsupportedTagFormat),
        }
        Ok(())
    }

    /// Parse the [`Serato Markers_`](Markers) tag.
    pub fn parse_markers(&mut self, input: &[u8], tag_format: TagFormat) -> Result<(), Error> {
        match tag_format {
            TagFormat::ID3 => {
                self.markers = Some(Markers::parse_id3(input)?);
            }
            TagFormat::MP4 => {
                self.markers = Some(Markers::parse_mp4(input)?);
            }
            _ => return Err(Error::UnsupportedTagFormat),
        }
        Ok(())
    }

    /// Parse the [`Serato Markers2`](Markers2) tag.
    pub fn parse_markers2(&mut self, input: &[u8], tag_format: TagFormat) -> Result<(), Error> {
        match tag_format {
            TagFormat::ID3 => {
                self.markers2 = Some(Markers2::parse_id3(input)?);
            }
            TagFormat::FLAC => {
                self.markers2 = Some(Markers2::parse_flac(input)?);
            }
            TagFormat::MP4 => {
                self.markers2 = Some(Markers2::parse_mp4(input)?);
            }
            TagFormat::Ogg => {
                self.markers2 = Some(Markers2::parse_ogg(input)?);
            }
        }
        Ok(())
    }

    /// Parse the [`Serato Overview`](Overview) tag.
    pub fn parse_overview(&mut self, input: &[u8], tag_format: TagFormat) -> Result<(), Error> {
        match tag_format {
            TagFormat::ID3 => {
                self.overview = Some(Overview::parse_id3(input)?);
            }
            TagFormat::FLAC => {
                self.overview = Some(Overview::parse_flac(input)?);
            }
            TagFormat::MP4 => {
                self.overview = Some(Overview::parse_mp4(input)?);
            }
            _ => return Err(Error::UnsupportedTagFormat),
        }
        Ok(())
    }

    /// Returns the [`auto_gain`](Autotags::auto_gain) value from the [`Serato Autotags`](Autotags) tag.
    pub fn auto_gain(&self) -> Option<f64> {
        if let Some(tag) = &self.autotags {
            return Some(tag.auto_gain);
        }

        None
    }

    /// Returns the [`gain_db`](Autotags::gain_db) value from the [`Serato Autotags`](Autotags) tag.
    pub fn gain_db(&self) -> Option<f64> {
        if let Some(tag) = &self.autotags {
            return Some(tag.gain_db);
        }

        None
    }

    /// Returns the beatgrid from the [`Serato BeatGrid`](Beatgrid) tag.
    pub fn beatgrid(
        &self,
    ) -> Option<(&Vec<beatgrid::NonTerminalMarker>, &beatgrid::TerminalMarker)> {
        if let Some(tag) = &self.beatgrid {
            return Some((&tag.non_terminal_markers, &tag.terminal_marker));
        }

        None
    }

    /// Returns BPM lock status from the [`Serato Markers2`](Markers2) tag.
    pub fn bpm_locked(&self) -> Option<bool> {
        if let Some(m) = &self.markers2 {
            return m.bpm_locked();
        }

        None
    }

    /// Returns cues from the [`Serato Markers_`](Markers) and [`Serato Markers2`](Markers2) tags.
    ///
    /// This retrieves the `Serato Markers2` cues first, then overwrite the values with those from
    /// `Serato Markers_`. This is what Serato does too (i.e. if `Serato Markers_` and `Serato
    /// Markers2` contradict each other, Serato will use the values from `Serato Markers_`).
    pub fn cues(&self) -> Vec<generic::Cue> {
        let mut map = std::collections::BTreeMap::new();

        // First, insert all cue from the `Serato Markers2` tag into the map.
        if let Some(m) = &self.markers2 {
            for cue in m.cues() {
                map.insert(cue.index, cue);
            }
        }

        // Now, iterate over the cue markers from the `Serato Markers_` tag.
        if let Some(m) = &self.markers {
            for (index, marker) in m.cues() {
                match marker.marker_type {
                    // If a cue is set in `Serato Markers2` but is invalid in `Serato Markers_`,
                    // remove it.
                    markers::MarkerType::Invalid => {
                        map.remove(&index);
                        continue;
                    }
                    markers::MarkerType::Cue => {
                        if marker.start_position_millis == None {
                            // This shouldn't be possible if the `Serato Markers_` data is valid.
                            // Ideally, this should be checked during the parsing state.
                            // FIXME: Throw error here?
                            map.remove(&index);
                            continue;
                        }

                        let position_millis = marker.start_position_millis.unwrap();

                        // If the cue is set in both `Serato Markers2` and `Serato Markers_`, use
                        // the version from `Serato Markers_`, but keep the label from `Serato
                        // Markers2` because the `Serato Markers_` tag doesn't contain labels.
                        let markers2_cue = map.remove(&index);
                        let label = match markers2_cue {
                            Some(c) => c.label,
                            None => String::new(),
                        };

                        map.insert(
                            index,
                            generic::Cue {
                                index,
                                position_millis,
                                color: marker.color,
                                label,
                            },
                        );
                    }
                    _ => {} // Ignore loop markers
                }
            }
        }

        // Return the sorted list of cues.
        map.values().cloned().collect()
    }

    /// Returns loops from the [`Serato Markers_`](Markers) and [`Serato Markers2`](Markers2) tags.
    ///
    /// This retrieves the `Serato Markers2` loops first, then overwrite the values with those from
    /// `Serato Markers_`. This is what Serato does too (i.e. if `Serato Markers_` and `Serato
    /// Markers2` contradict each other, Serato will use the values from `Serato Markers_`).
    pub fn loops(&self) -> Vec<generic::Loop> {
        let mut map = std::collections::BTreeMap::new();

        // First, insert all cue from the `Serato Markers2` tag into the map.
        if let Some(m) = &self.markers2 {
            for saved_loop in m.loops() {
                map.insert(saved_loop.index, saved_loop);
            }
        }

        // Now, iterate over the cue markers from the `Serato Markers_` tag.
        if let Some(m) = &self.markers {
            for (index, marker) in m.loops() {
                if marker.marker_type != markers::MarkerType::Loop {
                    // This can only happen is `Markers::cues()` returns non-cue markers, which
                    // would be a bug.
                    // FIXME: Throw error here?
                    continue;
                }

                if marker.start_position_millis == None || marker.end_position_millis == None {
                    // This may happen even for valid data, because unset loops lack the start/end
                    // position.
                    map.remove(&index);
                    continue;
                }

                let start_position_millis = marker.start_position_millis.unwrap();
                let end_position_millis = marker.end_position_millis.unwrap();

                // If the loop is set in both `Serato Markers2` and `Serato Markers_`, use
                // the version from `Serato Markers_`, but keep the label from `Serato
                // Markers2` because the `Serato Markers_` tag doesn't contain labels.
                let markers2_loop = map.remove(&index);
                let label = match markers2_loop {
                    Some(c) => c.label,
                    None => String::new(),
                };

                map.insert(
                    index,
                    generic::Loop {
                        index,
                        start_position_millis,
                        end_position_millis,
                        color: marker.color,
                        label,
                        is_locked: marker.is_locked,
                    },
                );
            }
        }

        // Return the sorted list of cues.
        map.values().cloned().collect()
    }

    /// Returns [flips](https://serato.com/dj/pro/expansions/flip) from the [`Serato Markers2`](Markers2) tag.
    pub fn flips(&self) -> Vec<generic::Flip> {
        if let Some(m) = &self.markers2 {
            return m.flips();
        }

        vec![]
    }

    /// Returns the track color from the [`Serato Markers_`](Markers) and [`Serato
    /// Markers2`](Markers2) tags.
    ///
    /// This retrieves the `Serato Markers2` track color first, then overwrites the value with the
    /// one from `Serato Markers_`. This is what Serato does too (i.e. if `Serato Markers_` and
    /// `Serato Markers2` contradict each other, Serato will use the value from `Serato
    /// Markers_`).
    pub fn track_color(&self) -> Option<generic::Color> {
        let mut track_color = None;

        if let Some(m) = &self.markers2 {
            track_color = m.track_color();
        }

        if let Some(m) = &self.markers {
            track_color = Some(m.track_color());
        }

        track_color
    }

    /// Returns the waveform overview data color from the [`Serato Overview`](Overview) tag.
    pub fn overview(&self) -> Option<&Vec<Vec<u8>>> {
        if let Some(tag) = &self.overview {
            return Some(&tag.data);
        }

        None
    }
}

impl Default for TagContainer {
    fn default() -> Self {
        Self::new()
    }
}
