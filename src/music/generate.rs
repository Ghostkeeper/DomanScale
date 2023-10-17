/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::ecs::system::ResMut;

use crate::music::state::State;
use crate::music::style::Style;

/// Starting point of the generating of music.
///
/// The purpose of this function is to fill up the buffer of MIDI messages with enough messages to
/// keep the music going for a while.
///
/// # Arguments
/// * state: The music generation state.
pub fn generate(style: Style, state: State, current_time: u32) {
	if state.generated_up_to < state.time
}