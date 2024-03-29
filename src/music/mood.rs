/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use crate::music::instrument::Instrument;
use crate::music::theme::Theme;

/// The mood of the music largely determines the theme and instrumentation being played.
#[derive(Clone, Copy)]
pub enum Mood {
	/// Adventurous music is grandiose and upbeat, with lots of orchestral instruments forming a
	/// full sound.
	Adventurous,
}

impl Mood {
	/// Get which drone is associated with each mood.
	///
	/// The drone will automatically start playing when the mood is set.
	///
	/// # Returns
	/// An instrument, if any, to be used as drone.
	pub fn drone(&self) -> Option<Instrument> {
		match self {
			Mood::Adventurous => Some(Instrument::Cello)
		}
	}

	/// Get the themes that are applicable to a mood.
	///
	/// # Returns
	/// A list of themes applicable. There will be at least one theme for each mood.
	pub fn themes(&self) -> Vec<Theme> {
		match self {
			Mood::Adventurous => vec![Theme::Doman]
		}
	}
}