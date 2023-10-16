/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

/// A message that can be queued for sending to the synthesizer.
///
/// This message contains a few bytes of data which will be the message being sent. However the
/// struct also implements a lot of helper functions to create the right MIDI messages.
pub struct MidiMessage {
	/// What timestamp this MIDI message needs to get played.
	///
	/// The timestamp is the time since the start of the music playback (when the game starts). The
	/// time is relative to the BPM, meaning that a higher BPM causes the message to be sent to the
	/// synthesizer earlier. The time is expressed as the number of 16ths of a beat since the start
	/// of the music playback. A play time of 16 means that the message gets played 1 beat after the
	/// start. A play time of 128 means that the message gets played 8 beats (2 measures) after the
	/// start.
	pub time: u32,

	/// The channel to send the MIDI message to.
	pub channel: u8,

	/// The MIDI command to send.
	pub command: u8,

	/// One of the data fields to send with the MIDI command. What this data field means depends on
	/// the MIDI command:
	/// * Note On: The pitch of the note to turn on.
	/// * Note Off: The pitch o the note to turn off.
	/// * Controller: Which parameter to control on the channel:
	///   - 0x00: Bank selection.
	///   - 0x01: Modulation course.
	///   - 0x06: Data entry coarse.
	///   - 0x0A: Panning coarse.
	///   - 0x0B: Expression coarse.
	///   - 0x21: Modulation fine.
	///   - 0x27: Data entry fine.
	///   - 0x2A: Panning fine.
	///   - 0x2B: Expression fine.
	///   - 0x40: Hold pedal.
	///   - 0x5B: Reverb.
	///   - 0x5D: Chorus.
	///   - 0x64: RPN fine.
	///   - 0x65: RPN coarse.
	///   - 0x78: Stop all sound immediately.
	///   - 0x79: Reset all controllers.
	///   - 0x7B: Stop all notes.
	/// * Program Change: Which instrument to play on that channel.
	/// * Pitch Bend: The least significant byte of the pitch bend adjustment.
	pub data1: u8,

	/// One of the data fields to send with the MIDI command. What this data field means depends on
	/// the MIDI command:
	/// * Note On: The velocity of the note to play.
	/// * Note Off: None, this is ignored.
	/// * Controller: Depending on the first data field, this data field controls the magnitude of
	/// that effect.
	pub data2: u8,
}
