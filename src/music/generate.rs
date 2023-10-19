/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use std::cmp::max;

use crate::music::midi_message::MidiMessage;
use crate::music::state::State;
use crate::music::style::Style;

/// Starting point of the generating of music.
///
/// The purpose of this function is to fill up the buffer of MIDI messages with enough messages to
/// keep the music going for a while.
///
/// # Arguments
/// * `style`: The style of music to generate.
/// * `state`: The music generation state, and transmitting channel.
/// * `current_time`: The current beat time in the generated music.
pub fn generate(style: &Style, state: &mut State, current_time: u32) {
	if current_time + 16 < state.generated_up_to { //Generated at least 16 ticks ahead.
		return; //We generated far enough ahead.
	}
	if style.playing == false {
		state.generated_up_to = max(state.generated_up_to, current_time);
		return; //Don't want any music.
	}

	let next_beat_time = state.generated_up_to - (state.generated_up_to % 16) + 16;
	let _ = state.transmit.send(MidiMessage {
		time: next_beat_time,
		channel: 0,
		command: 0x90,
		data1: 60,
		data2: 100
	});
	state.generated_up_to = next_beat_time;
}