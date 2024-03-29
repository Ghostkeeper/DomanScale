/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use crate::music::pitch::Pitch;
use crate::music::scale::Scale;

/// These are the available musical melodies.
///
/// Each theme is a little recognisable melody.
#[derive(Clone, Copy)]
pub enum Theme {
	/// Theme of the land of Doman, and main theme for the game.
	Doman
}

impl Theme {
	/// Get the melody belonging to this theme.
	///
	/// # Returns
	/// The melody is a vector of measures, and each measure contains a vector of notes. Each note
	/// is a pair combining (timestamp, pitch). The pitch here is relative to the base note, and
	/// depends on the scale.
	pub fn melody(&self) -> Vec<Vec<(u32, i8)>> {
		match self {
			Theme::Doman => vec![vec![(0, 0), (32, 4)], vec![(0, 3), (16, 2), (24, 1)], vec![(0, 0), (32, 3)], vec![(0, 2), (16, 1), (24, 0)]],
		}
	}

	/// Get the chord progression belonging to this theme.
	///
	/// # Returns
	/// The chord progression is a vector of pitches and scales, one for each measure. The pitch is
	/// the pitch of the ground tone, and the scale is the set of tones played during that measure.
	pub fn chord_progression(&self) -> Vec<(Pitch, Scale)> {
		match self {
			Theme::Doman => vec![(Pitch::F, Scale::Major), (Pitch::F, Scale::Major), (Pitch::E, Scale::Minor), (Pitch::E, Scale::Minor)],
		}
	}
}