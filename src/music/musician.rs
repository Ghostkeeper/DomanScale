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
use crate::music::voice::Voice;

/// Play guitar as lead instrument.
///
/// The guitar is a simple instrument, really. It just plays the note, and lets it ring forever.
/// There is not even an end note event.
///
/// # Arguments
/// * `state` - The music generation state to track instruments and play MIDI through.
/// * `time` - The timestamp of the note to play.
/// * `pitch` - The pitch of the note to play.
/// * `velocity` - The velocity of the note to play.
pub fn guitar_lead(state: &mut State, time: u32, pitch: i32, velocity: i32) {
	let channel = Voice::Lead as i32;
	state.set_instrument(channel, Instrument::NylonGuitar, time);
	_ = state.transmit.send(MidiMessage::note_on(time, channel, pitch, velocity));
}

/// Play cello as the drone.
///
/// When played as drone, it's played a bit softly. Notes are transitioned smoothly to create a
/// continuous drone.
///
/// # Arguments
/// * `state` - The music generation state to track instruments and play MIDI through.
/// * `time` - The timestamp of when this drone note should start.
/// * `pitch` - The pitch of the note to play.
pub fn cello_drone(state: &mut State, time: u32, pitch: i32) {
	//Change the program of that channel to have the correct instrument.
	let channel = Voice::Drone as i32;
	state.set_instrument(channel, Instrument::Cello, time);
	_ = state.transmit.send(MidiMessage::note_on(time, channel, pitch, 64));
}