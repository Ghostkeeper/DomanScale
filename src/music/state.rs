/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::ecs::system::Resource;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;

use crate::music::note::Note;

/// Defines the current state of the music playback.
///
/// These are controlled from the control module. The control module adjusts the state, and the
/// player then reads the state to adjust the type of music being played.
#[derive(Resource)]
pub struct State {
	/// Whether the music system is currently running.
	pub playing: bool,

	/// A channel through which to send notes to be played.
	pub transmit: Sender<Note>
}