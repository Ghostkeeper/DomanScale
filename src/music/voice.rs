/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

/// Voices, or sections, of an orchestration.
///
/// These represent the different instrument groups of an orchestra. In a broader sense, these are
/// the different voices of a musical composition. These are not just the classical orchestral
/// sections, but more of the roles that they fill in a composition.
///
/// Each voice is assigned a unique MIDI channel. The musical style then determines which
/// instruments are used to make each voice. The channel number of each section is the values in
/// this enum.
pub enum Voice {
	/// A continuous background voice that fills the empty space between notes, usually in the lows.
	Drone = 10,

	/// A low voice that fills the lower end of the spectrum and indicates the bass note of the
	/// chord.
	Bass = 0,

	/// The most salient voice, transferring the melody and character of the music to the listener.
	Lead = 1,

	/// Counters or accompanies the lead voice.
	Secondary = 2,

	/// Counters or accompanies the lead and secondary voices.
	Tertiary = 3,

	/// Follows the lead voice, at least partially, but usually at an offset chord or using a
	/// different instrument.
	Harmony = 4,

	/// Rhythmic section but with a bit of tonality to it, similar to bass but for filling the
	/// highs.
	Rhythm = 5,

	/// Rhythmic section without tonality to it, a strong indicator of the speed of the music, and
	/// filling the very highs and lows of the spectrum.
	Percussion = 11,
}