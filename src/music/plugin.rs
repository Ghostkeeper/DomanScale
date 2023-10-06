/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::app::{App, Plugin, Update, Startup};
use bevy::ecs::system::{Commands, ResMut};
use bevy::time::{Timer, TimerMode};
use itertools::Itertools;
use rustysynth::Synthesizer;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tinyaudio::{OutputDeviceParameters, run_output_device};

use crate::music::player::{BeatTime, play};
use crate::music::state::State;
use crate::music::synth::Synth;

/// A plug-in that plays music during the game.
pub struct MusicPlugin;

impl Plugin for MusicPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, initialise);
		app.add_systems(Update, play);
	}
}

/// Initialise the music resources.
fn initialise(mut commands: Commands) {
	commands.init_resource::<State>();
	commands.insert_resource(BeatTime {
		timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating)
	});

	let synth_resource = Synth::default();
	let params = OutputDeviceParameters {
		channels_count: 2,
		sample_rate: 44100,
		channel_sample_count: 4410 //Buffer size.
	};
	let synth = synth_resource.synth.clone();
	let left_buffer = synth_resource.left_buffer.clone();
	let right_buffer = synth_resource.right_buffer.clone();
	let _device = run_output_device(params, {
		move |data| {
			synth.lock().unwrap().render(&mut left_buffer.lock().unwrap()[..], &mut right_buffer.lock().unwrap()[..]);
			for (i, value) in left_buffer.lock().unwrap().iter().interleave(right_buffer.lock().unwrap().iter()).enumerate() {
				data[i] = *value;
			}
		}
	}).unwrap();
	commands.insert_resource(synth_resource);
}