/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use std::cmp::max;

use crate::music::midi_message::MidiMessage;
use crate::music::musician::{cello_drone, guitar_lead};
use crate::music::scale::Scale;
use crate::music::state::State;
use crate::music::style::Style;

/// Starting point of the generating of music.
///
/// The purpose of this function is to fill up the buffer of MIDI messages with enough messages to
/// keep the music going for a while.
///
/// # Arguments
/// * `state`: The music generation state, and transmitting channel.
/// * `style`: The style of music to generate.
/// * `current_time`: The current beat time in the generated music.
pub fn generate(state: &mut State, style: &Style, current_time: u32) {
	if current_time + 16 < state.generated_up_to { //Generated at least 16 ticks ahead.
		return; //We generated far enough ahead.
	}
	if style.playing == false {
		state.generated_up_to = max(state.generated_up_to, current_time);
		return; //Don't want any music.
	}

	measure(state, style, state.generated_up_to);
}

/// Generate a measure (4 beats) of music.
///
/// # Arguments
/// * `state`: The music generation state, and transmitting channel.
/// * `style`: The style of music to generate.
/// * `time`: The starting time of the measure to generate. If this time is not divisible by 1
/// measure (64 ticks), it will be rounded up, meaning that there will be silence until this measure
/// starts.
fn measure(state: &mut State, style: &Style, time: u32) {
	//Round time up to nearest multiple of 64 (1 measure).
	let start_time = ((time + 64 - 1) / 64) * 64;

	let drone = style.mood.drone();
	if drone != state.drone {
		_ = state.transmit.send(MidiMessage::stop_all_notes(start_time, 10));
		cello_drone(state, start_time, 48);
		state.drone = drone;
	}

	let theme = style.mood.themes()[0];
	let melody = theme.melody();
	let this_measure = &melody[state.measure_in_phrase];
	let base_note = theme.chord_progression()[state.measure_in_phrase].0;
	let scale = if style.enchanting { Scale::Hijaz } else { theme.chord_progression()[state.measure_in_phrase].1 };
	for (time, pitch) in this_measure {
		guitar_lead(state, start_time + time, (60 + base_note as i8 + scale.intervals()[pitch.rem_euclid(7) as usize] as i8 + (pitch - 6) / 7 * 12) as u8, 127);
	}

	state.measure_in_phrase = (state.measure_in_phrase + 1) % 4; //TODO: Hard-coded 4 measures per phrase.
	state.generated_up_to = start_time + 64;
}