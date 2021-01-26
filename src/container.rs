//! This module provides the `Container` class.

use crate::analysis;
use crate::autotags;
use crate::beatgrid;
use crate::markers;
use crate::markers2;
use crate::overview;
use crate::util;

/// Provides a streamlined interface for retrieving Serato tag data.
///
/// Some of the data in Serato's tags is redundant and may contradict each other. This class
/// implements the same merge strategies for inconsistent data that Serato uses, too.
pub struct Container {
    analysis: Option<analysis::Analysis>,
    autotags: Option<autotags::Autotags>,
    beatgrid: Option<beatgrid::Beatgrid>,
    markers: Option<markers::Markers>,
    markers2: Option<markers2::Markers2>,
    overview: Option<overview::Overview>,
}

impl Container {
    /// Create an empty Serato tag container.
    pub fn new() -> Self {
        Self {
            analysis: None,
            autotags: None,
            beatgrid: None,
            markers: None,
            markers2: None,
            overview: None,
        }
    }

    /// Parse the `Serato Analysis` tag data.
    pub fn parse_analysis(&mut self, input: &[u8]) -> bool {
        let res = analysis::parse(input);
        let value = res.is_err();
        self.analysis = res.ok();
        value
    }

    /// Parse the `Serato AutoTags` tag data.
    pub fn parse_autotags(&mut self, input: &[u8]) -> bool {
        let res = autotags::parse(input);
        let value = res.is_err();
        self.autotags = res.ok();
        value
    }

    /// Parse the `Serato BeatGrid` tag data.
    pub fn parse_beatgrid(&mut self, input: &[u8]) -> bool {
        let res = beatgrid::parse(input);
        let value = res.is_err();
        self.beatgrid = res.ok();
        value
    }

    /// Parse the `Serato Markers_` tag data.
    pub fn parse_markers(&mut self, input: &[u8]) -> bool {
        let res = markers::parse(input);
        let value = res.is_err();
        self.markers = res.ok();
        value
    }

    /// Parse the `Serato Markers2` tag data.
    pub fn parse_markers2(&mut self, input: &[u8]) -> bool {
        let res = markers2::parse(input);
        let value = res.is_err();
        self.markers2 = res.ok();
        value
    }

    /// Parse the `Serato Overview` tag data.
    pub fn parse_overview(&mut self, input: &[u8]) -> bool {
        let res = overview::parse(input);
        let value = res.is_err();
        self.overview = res.ok();
        value
    }

    /// Returns the auto_gain value from the `Serato Autotags` tag.
    pub fn auto_gain(&self) -> Option<f64> {
        if let Some(tag) = &self.autotags {
            return Some(tag.auto_gain);
        }

        None
    }

    /// Returns the gain_db value from the `Serato Autotags` tag.
    pub fn gain_db(&self) -> Option<f64> {
        if let Some(tag) = &self.autotags {
            return Some(tag.gain_db);
        }

        None
    }

    /// Returns the beatgrid from the `Serato BeatGrid` tag.
    pub fn beatgrid(
        &self,
    ) -> Option<(&Vec<beatgrid::NonTerminalMarker>, &beatgrid::TerminalMarker)> {
        if let Some(tag) = &self.beatgrid {
            return Some((&tag.non_terminal_markers, &tag.terminal_marker));
        }

        None
    }

    /// Returns BPM lock status from the `Serato Markers2` tag.
    pub fn bpm_locked(&self) -> Option<bool> {
        if let Some(m) = &self.markers2 {
            return m.bpm_locked();
        }

        None
    }

    /// Returns cues from the `Serato Markers_` and `Serato Markers2` tags.
    ///
    /// This retrieves the `Serato Markers2` cues first, then overwrite the values with those from
    /// `Serato Markers_`. This is what Serato does too (i.e. if `Serato Markers_` and `Serato
    /// Markers2` contradict each other, Serato will use the values from `Serato Markers_`).
    pub fn cues(&self) -> Vec<markers2::CueMarker> {
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
                match marker.entry_type {
                    // If a cue is set in `Serato Markers2` but is invalid in `Serato Markers_`,
                    // remove it.
                    markers::EntryType::INVALID => {
                        map.remove(&index);
                        continue;
                    }
                    markers::EntryType::CUE => {
                        if marker.start_position_millis == None {
                            // This shouldn't be possible if the `Serato Markers_` data is valid.
                            // Ideally, this should be checked during the parsing state.
                            // FIXME: Throw error here?
                            map.remove(&index);
                            continue;
                        }

                        // If the cue is set in both `Serato Markers2` and `Serato Markers_`, use
                        // the version from `Serato Markers_`, but keep the label from `Serato
                        // Markers2` because the `Serato Markers_` tag doesn't contain labels.
                        if let Some(c) = map.remove(&index) {
                            let position_millis = marker.start_position_millis.unwrap();

                            map.insert(
                                index,
                                markers2::CueMarker {
                                    index,
                                    position_millis,
                                    color: marker.color,
                                    label: c.label,
                                },
                            );
                        }
                    }
                    _ => {
                        // This can only happen is `Markers::cues()` returns non-cue markers, which
                        // would be a bug.
                        // FIXME: Throw error here?
                    }
                }
            }
        }

        // Return the sorted list of cues.
        map.values().cloned().collect()
    }

    /// Returns loops from the `Serato Markers_` and `Serato Markers2` tags.
    ///
    /// This retrieves the `Serato Markers2` loops first, then overwrite the values with those from
    /// `Serato Markers_`. This is what Serato does too (i.e. if `Serato Markers_` and `Serato
    /// Markers2` contradict each other, Serato will use the values from `Serato Markers_`).
    pub fn loops(&self) -> Vec<markers2::LoopMarker> {
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
                if marker.entry_type != markers::EntryType::LOOP {
                    // This can only happen is `Markers::cues()` returns non-cue markers, which
                    // would be a bug.
                    // FIXME: Throw error here?
                    continue;
                }

                if marker.start_position_millis == None || marker.end_position_millis == None {
                    // This shouldn't be possible if the `Serato Markers_` data is valid.
                    // Ideally, this should be checked during the parsing state.
                    // FIXME: Throw error here?
                    map.remove(&index);
                    continue;
                }

                // If the cue is set in both `Serato Markers2` and `Serato Markers_`, use
                // the version from `Serato Markers_`, but keep the label from `Serato
                // Markers2` because the `Serato Markers_` tag doesn't contain labels.
                if let Some(c) = map.remove(&index) {
                    let start_position_millis = marker.start_position_millis.unwrap();
                    let end_position_millis = marker.end_position_millis.unwrap();

                    map.insert(
                        index,
                        markers2::LoopMarker {
                            index,
                            start_position_millis,
                            end_position_millis,
                            color: marker.color,
                            label: c.label,
                            is_locked: marker.is_locked,
                        },
                    );
                }
            }
        }

        // Return the sorted list of cues.
        map.values().cloned().collect()
    }

    /// Returns the track color from the `Serato Markers_` and `Serato Markers2` tags.
    ///
    /// This retrieves the `Serato Markers2` track color first, then overwrites the value with the
    /// one from `Serato Markers_`. This is what Serato does too (i.e. if `Serato Markers_` and
    /// `Serato Markers2` contradict each other, Serato will use the value from `Serato
    /// Markers_`).
    pub fn track_color(&self) -> Option<util::Color> {
        let mut track_color = None;

        if let Some(m) = &self.markers2 {
            track_color = m.track_color();
        }

        if let Some(m) = &self.markers {
            track_color = Some(m.track_color());
        }

        track_color
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
