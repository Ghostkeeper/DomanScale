/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use std::sync::mpsc::Sender;

use crate::music::instrument::Instrument;
use crate::music::midi_message::MidiMessage;

/// Defines the current state of the music playback.
///
/// Anything that the playback needs to play its music.
pub struct State {
	/// A channel through which to send notes to be played.
	pub transmit: Sender<MidiMessage>,

	/// We've generated music up until this timestamp.
	pub generated_up_to: u32,

	/// The current "programs", or instruments, for each channel.
	///
	/// When a note gets planned with a certain instrument, one of the channels needs to be set to
	/// use that instrument. This array tracks which channel is currently set to which instrument.
	///
	/// Channel 10 (index 9) is always the percussion channel, as per MIDI standards. It is included
	/// in this array, but its value is really inconsequential and it will never be allowed to be
	/// set for specific instruments.
	pub current_instruments: [Instrument; 16],

	/// If there is any drone going on, this stores the drone instrument.
	///
	/// Only one drone can be playing at the same time. The drone is a continuous note playing
	/// without definite end (although they can change pitch over time).
	pub drone: Option<Instrument>,

	/// Which measure of a phrase it is.
	///
	/// Most phrases will have either 4 or 8 measures. This counts how many measures have passed
	/// since the start of the phrase.
	pub measure_in_phrase: usize,
}

impl State {
	/// Change the instrument of a specific channel to a different one.
	///
	/// This changes the stored state, and also emits a MIDI message to let the synthesizer change the channel too.
	pub fn set_instrument(&mut self, channel: i32, instrument: Instrument, time: u32) {
		if self.current_instruments[channel as usize] != instrument {
			self.current_instruments[channel as usize] = instrument;
			let _ = self.transmit.send(MidiMessage::change_program(time, channel, instrument));
		}
	}
}