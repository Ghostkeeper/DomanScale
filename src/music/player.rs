/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use rustysynth::Synthesizer;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, TryRecvError};

use crate::music::midi_message::MidiMessage;

/// Wrap a receiver in a `TryIterResult` which is peekable.
///
/// Taken from https://stackoverflow.com/a/59448553.
pub fn try_iter_result<T>(rx: &Receiver<T>) -> TryIterResult<T> {
	TryIterResult { rx: &rx }
}

/// Wrapper around a receiver result that allows peeking on a receiver.
///
/// Taken from https://stackoverflow.com/a/59448553.
pub struct TryIterResult<'a, T: 'a> {
	/// The receiver to take the result from.
	rx: &'a Receiver<T>,
}

impl<'a, T> Iterator for TryIterResult<'a, T> {
	type Item = Result<T, TryRecvError>;

	/// Take the next iteration from the iterator.
	///
	/// This keeps the distinction that normal iterators discard, between having an empty queue, and
	/// having a queue that is disconnected from the transmitter.
	fn next(&mut self) -> Option<Result<T, TryRecvError>> {
		match self.rx.try_recv() {
			Ok(data) => Some(Ok(data)),
			Err(TryRecvError::Empty) => Some(Err(TryRecvError::Empty)),
			Err(TryRecvError::Disconnected) => None
		}
	}
}

/// Play the MIDI messages in a receiver queue for this time instant.
///
/// This assumes that the messages in the receiver are ordered by their time instant. If they are
/// not, some messages may be skipped.
pub fn play(receiver: &mut Receiver<MidiMessage>, time: u32, synth: Arc<Mutex<Synthesizer>>) {
	loop {
		match try_iter_result(receiver).peekable().peek() {
			Some(Ok(message)) => {
				if message.time > time {
					return; //Next note is in the future. Need to wait a while, until we're called with that time.
				}
				if message.time == time {
					synth.lock().unwrap().process_midi_message(message.channel, message.command, message.data1, message.data2);
					_ = receiver.recv(); //Remove this note. It successfully played.
				}
			},
			Some(Err(_)) => {
				//No notes in the queue right now. Wait for notes.
				return;
			},
			None => {
				//Channel got disconnected. We're probably closing the application then.
				return;
			}
		}
	}
}