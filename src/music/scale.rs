/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

/// The musical scales to play music in.
///
/// The musical scale determines which 7 out of 12 notes are included in the scale.
#[derive(Clone, Copy)]
pub enum Scale {
	/// The major scale, having note intervals of whole, whole, half, whole, whole, whole, half and
	/// whole.
	Major,

	/// The natural minor scale, having note intervals of whole, half, whole, whole, half, whole,
	/// whole and half.
	Minor,

	/// The Hijaz (Arabic) scale, which has a special meaning in this game.
	///
	/// This scale has note intervals of half, augmented second, half, whole, half, whole, whole and
	/// augmented second.
	Hijaz,
}

impl Scale {
	/// Get the scale intervals for this musical scale.
	///
	/// These intervals are integer numbers counting half-notes from the base note of a chord. Each
	/// scale contains 7 notes that are in-scale. These notes are listed in the result.
	pub fn intervals(&self) -> [u32; 7] {
		match self {
			Scale::Major => [0, 2, 4, 5, 7, 9, 11],
			Scale::Minor => [0, 2, 3, 5, 7, 8, 10],
			Scale::Hijaz => [0, 1, 4, 5, 7, 8, 10]
		}
	}
}