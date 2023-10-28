/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

//! This file contains helper functions to play certain instruments in a characteristic manner.
//!
//! For some instruments, the helper functions cause the instruments to be played less perfectly, in
//! a more human way. For some instruments, notes can be strung together in continuous pitch.

use crate::music::instrument::Instrument;
use crate::music::state::State;
use crate::music::midi_message::MidiMessage;

/// Play guitar.
///
/// The guitar is a simple instrument, really. It just plays the note, and lets it ring forever.
/// There is not even an end note event.
pub fn guitar(state: &mut State, time: u32, pitch: i32, velocity: i32) {
	let channel = state.get_channel(Instrument::NylonGuitar, time);
	_ = state.transmit.send(MidiMessage {
		time: time,
		channel: channel,
		command: 0x90,
		data1: pitch,
		data2: velocity
	});
}