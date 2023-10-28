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

	/// The timestamp of the most recent activity on each channel.
	///
	/// Activity includes playing a note, playing a sustained note (infinite activity) or changing
	/// the instrument of the channel.
	pub most_recent_activity: [u32; 16],

	/// If there is any drone going on, this stores the drone instrument.
	///
	/// Only one drone can be playing at the same time. The drone is a continuous note playing
	/// without definite end (although they can change pitch over time).
	pub drone: Option<Instrument>
}

impl State {
	/// Get a channel to play a certain instrument in.
	///
	/// If there is already a channel set to the given instrument, that channel is simply returned.
	/// Buf if not, the channel that has seen the least use recently will get allocated for that
	/// instrument.
	///
	/// There is inherently a bit of a mismatch in time here between when the notes get planned and
	/// when they get played out. The channel gets reserved when it is planned, but it will remain
	/// unused until the note actually plays, causing a less-than-ideal use of the limited number of
	/// channels. Hopefully we'll not actually run into this issue in practice.
	///
	/// # Arguments:
	/// * instrument: The instrument to reserve a channel for.
	/// * time: The timestamp that the instrument must be changed by.
	///
	/// # Returns:
	/// A channel that the given instrument can be played on.
	pub fn get_channel(&mut self, instrument: Instrument, time: u32) -> i32 {
		for (channel, current_instrument) in self.current_instruments.iter().enumerate() {
			if channel == 9 || channel == 10 { //Don't use the percussion or drone channels.
				continue;
			}
			if *current_instrument == instrument { //The instrument is already allocated. Simply give that one.
				return channel as i32;
			}
		}

		//Find the channel with the least recent activity.
		let mut oldest_activity: u32 = 0;
		let mut oldest_channel = 0;
		for (channel, activity_timestamp) in self.most_recent_activity.iter().enumerate() {
			if channel == 9 || channel == 10 { //Don't use the percussion or drone channels.
				continue;
			}
			if *activity_timestamp < oldest_activity {
				oldest_activity = *activity_timestamp;
				oldest_channel = channel;
			}
		}
		//Change the program of that channel to have the correct instrument.
		_ = self.transmit.send(MidiMessage {
			time: time,
			channel: oldest_channel as i32,
			command: 0xC0 + oldest_channel as i32,
			data1: instrument as i32,
			data2: 0
		});
		self.current_instruments[oldest_channel] = instrument;
		self.most_recent_activity[oldest_channel] = time;
		return 0;
	}
}