/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::ecs::system::Resource;
use std::sync::mpsc::Sender;

use crate::music::midi_message::MidiMessage;

/// Defines the current state of the music playback.
///
/// Anything that the playback needs to play its music.
#[derive(Resource)]
pub struct State {
	/// A channel through which to send notes to be played.
	pub transmit: Sender<MidiMessage>,

	/// We've generated music up until this timestamp.
	pub generated_up_to: u32
}