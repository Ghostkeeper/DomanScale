/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

/// Represents a musical note that is scheduled to be played.
///
/// The music note is represented by three major attributes:
/// * The pitch.
/// * The velocity (loudness).
/// * The duration.
///
/// # Pitch
/// The pitch of the note is stored relative to the base note only, rather than absolute pitch. This
/// is represented by its scale degree: The base note is the 1st, the first scalar note above that
/// is the 2nd, then the 3rd, 4th, 5th, etc. These are stored as an integer starting from 0. A
/// standard triad chord will then always be the 1st, 3rd and 5th, represented with integers 0, 2
/// and 4. The scale loops after 7, an octave, but the integers can just keep on counting.
///
/// When the note gets played, the absolute pitch is calculated, in MIDI notes, based on which scale
/// is being played. This way, the scale can change at a whim without needing to program different
/// chords.
///
/// To represent non-scalar notes, an additional boolean is added, the ``sharp`` field, which raises
/// the pitch by a single semi-tone.
///
/// # Time and duration
/// The play time and duration of the note are also not absolute, but represented with an integer in
/// 16ths of a beat. This means that if the tempo of the music is changed, the duration and play
/// time of the note also changes. This makes it easy to play the same melodies at different tempos
/// without needing to change the note representation.
pub struct Note {
	/// The scale degree of this note.
	///
	/// A value of 0 means it's the base note. A higher number means that it's that many steps on
	/// the scale higher. A lower number means it's that many steps lower on the scale.
	pub pitch: i32,

	/// Whether this note should be played a semitone higher than normal.
	pub sharp: bool,

	/// When this note needs to get played, in 16ths of a beat relative to the start of playback.
	pub time: u32,

	/// How long the note should be played, in 16ths of a beat.
	///
	/// If the duration is 16, it should be played for 1 beat. If the duration is 64, it should be
	/// played for 4 beats, which is always 1 measure (since this music generator can only generate
	/// 4/4 measures).
	///
	/// A duration of ``u32::MAX`` represents "infinite" duration, used for drones and such. Such
	/// notes can only be played on the last channel. On that channel, the previous note will be
	/// stopped when the next note is played.
	pub duration: u32,

	/// Which channel the note gets played on.
	///
	/// The channel determines the instrument that the note gets played with. Channels 10 and 16 are
	/// reserved for drums and drones respectively.
	pub channel: u8
}