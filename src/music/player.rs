/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::ecs::system::{Res, ResMut, Resource};
use bevy::time::{Time, Timer};

use crate::music::state::State;
use crate::music::synth::Synth;

#[derive(Resource)]
pub struct BeatTime {
	/// How often should a new beat trigger? Determines the BPM of the music.
	pub timer: Timer
}

pub fn play(state: Res<State>, mut beat_time: ResMut<BeatTime>, time: Res<Time>, synth: Res<Synth>) {
	if state.playing {
		beat_time.timer.tick(time.delta());
	}
	if beat_time.timer.finished() {
		synth.synth.lock().unwrap().note_on(0, 60, 100);
	}
}