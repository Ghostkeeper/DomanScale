/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use crate::music::instrument::Instrument;

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
	///
	/// These are the possible commands:
	/// * 0x80: Note off.
	/// * 0x90: Note on.
	/// * 0xB0: Controller.
	/// * 0xC0: Program change.
	/// * 0xE0: Pitch bend.
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

impl MidiMessage {
	/// Helper function to generate a "note on" MIDI message.
	///
	/// # Arguments
	/// * `time`: The timestamp at which the note must go on.
	/// * `channel`: The channel that the note must be played on. To get a proper channel for an
	/// instrument, use the `State::get_channel` function.
	/// * `pitch`: The pitch of the note.
	/// * `velocity`: The velocity (roughly, volume) of the note.
	///
	/// # Returns
	/// Returns a MIDI message that can be sent to the synthesizer.
	pub fn note_on(time: u32, channel: u8, pitch: u8, velocity: u8) -> MidiMessage {
		MidiMessage {
			time: time,
			channel: channel,
			command: 0x90, //Note on.
			data1: pitch,
			data2: velocity
		}
	}

	/// Helper function to generate a "note off" MIDI message.
	///
	/// # Arguments
	/// * `time`: The timestamp at which the note must go off.
	/// * `channel`: The channel of the note to turn off.
	/// * `pitch`: The pitch of the note to turn off.
	///
	/// # Returns
	/// Returns a MIDI message that can be sent to the synthesizer.
	pub fn note_off(time: u32, channel: u8, pitch: u8) -> MidiMessage {
		MidiMessage {
			time: time,
			channel: channel,
			command: 0x80, //Note off.
			data1: pitch,
			data2: 0
		}
	}

	/// Helper function to generate a program change MIDI message.
	///
	/// # Arguments
	/// * `time`: The timestamp at which the program should change.
	/// * `channel`: The channel to change the program of.
	/// * `instrument`: The instrument to play on that channel.
	///
	/// # Returns
	/// Returns a MIDI message that can be sent to the synthesizer.
	pub fn change_program(time: u32, channel: u8, instrument: Instrument) -> MidiMessage {
		MidiMessage {
			time: time,
			channel: channel,
			command: 0xC0, //Program change.
			data1: instrument as u8,
			data2: 0
		}
	}

	/// Helper function to generate a "stop all notes" message.
	///
	/// # Arguments
	/// * `time`: The timestamp at which to stop all notes.
	/// * `channel`: The channel to stop all notes on.
	///
	/// # Returns
	/// Returns a MIDI message that can be sent to the synthesizer.
	pub fn stop_all_notes(time: u32, channel: u8) -> MidiMessage {
		MidiMessage {
			time: time,
			channel: channel,
			command: 0xB0, //Controller.
			data1: 0x7B, //Stop all notes.
			data2: 0
		}
	}
}