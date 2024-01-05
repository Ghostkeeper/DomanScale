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
#[derive(Clone, Copy)]
pub enum Pitch {
	/// Pitch of C.
	C = 0,

	/// Pitch of C#.
	CS = 1,

	/// Pitch of D.
	D = 2,

	/// Pitch of D#.
	DS = 3,

	/// Pitch of E.
	E = 4,

	/// Pitch of F.
	F = 5,

	/// Pitch of F#.
	FS = 6,

	/// Pitch of G.
	G = 7,

	/// Pitch of G#.
	GS = 8,

	/// Pitch of A.
	A = 9,

	/// Pitch of A#.
	AS = 10,

	/// Pitch of B.
	B = 11
}