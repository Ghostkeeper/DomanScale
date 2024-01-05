/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::log::warn;
use rustysynth::Synthesizer;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

use crate::music::midi_message::MidiMessage;


/// Play the MIDI messages in a receiver queue for this time instant.
///
/// This assumes that the messages in the receiver are ordered by their time instant. If they are
/// not, some messages may be skipped.
pub fn play(next_message: &mut Option<MidiMessage>, receiver: &mut Receiver<MidiMessage>, time: u32, synth: Arc<Mutex<Synthesizer>>) {
	loop {
		match next_message {
			None => {
				*next_message = receiver.try_recv().ok();
				if next_message.is_none() { //Still nothing?
					return; //No messages in the queue. Wait for the next call then.
				}
			}
			Some(message) => {
				if message.time > time {
					return; //Next note is in the future. Need to wait a while, until we're called with that time.
				}
				else {
					if message.time < time {
						warn!("Accidentally skipped a MIDI message. Are messages out of order?");
					}
					synth.lock().unwrap().process_midi_message(message.channel as i32, message.command as i32, message.data1 as i32, message.data2 as i32);
					*next_message = None;
				}
			}
		}
	}
}