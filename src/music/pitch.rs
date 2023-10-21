/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

/// Named pitches of the chromatic scale.
///
/// This is not an absolute pitch (in Hz) but rather the 12 pitches of the chromatic scale,
/// repeating at the octave.
pub enum Pitch {
	/// Pitch of C.
	C,

	/// Pitch of C#.
	CS,

	/// Pitch of D.
	D,

	/// Pitch of D#.
	DS,

	/// Pitch of E.
	E,

	/// Pitch of F.
	F,

	/// Pitch of F#.
	FS,

	/// Pitch of G.
	G,

	/// Pitch of G#.
	GS,

	/// Pitch of A.
	A,

	/// Pitch of A#.
	AS,

	/// Pitch of B.
	B
}