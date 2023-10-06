/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

//! Provides systems to control the music playback.

use bevy::ecs::event::EventReader;
use bevy::ecs::system::ResMut;
use bevy::log::info;

use crate::music::events::PlayMusic;
use crate::music::state::State;

pub fn play(mut playmusic: EventReader<PlayMusic>, mut state: ResMut<State>) {
	if !playmusic.is_empty() {
		playmusic.clear();
		info!("Start music playback.");
		state.playing = true;
	}
}