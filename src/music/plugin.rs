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
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use spin_sleep::LoopHelper;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use tinyaudio::{OutputDeviceParameters, run_output_device};

use crate::music::midi_message::MidiMessage;
use crate::music::player::play;
use crate::music::state::State;

/// A plug-in that plays music during the game.
pub struct MusicPlugin;

impl Plugin for MusicPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, initialise);
		app.add_systems(Update, testplay);
	}
}

fn testplay(mut state: ResMut<State>) {
	_ = state.transmit.send(MidiMessage{
		time: 64,
		channel: 0,
		command: 0x90,
		data1: 60,
		data2: 100
	});
}

/// Initialise the music resources and start outputting to the sound device.
fn initialise(mut commands: Commands) {
	// Create a producer/consumer channel from the state resource to the synthesizer.
	let (mut transmitter, mut receiver) = channel();
	commands.insert_resource(State {
		playing: false,
		transmit: transmitter
	});

	//Create a synthesizer.
	let mut sf2 = File::open("airfont.sf2").unwrap();
	let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());
	let settings = SynthesizerSettings::new(44100);
	let synth = Arc::new(Mutex::new(Synthesizer::new(&sound_font, &settings).unwrap()));
	//Create the Synth resource and start rendering with it.
	let params = OutputDeviceParameters {
		channels_count: 2,
		sample_rate: 44100,
		channel_sample_count: 4410 //Buffer size.
	};

	//Create a thread that infinitely keeps rendering (as long as the parent process runs).
	let synth_arc_copy = synth.clone(); //Clone the Arc.
	thread::spawn(move || {
		let mut left_buffer = vec![0_f32; params.channel_sample_count];
		let mut right_buffer = vec![0_f32; params.channel_sample_count];
		let _device = run_output_device(params, {
			move |data| {
				synth_arc_copy.lock().unwrap().render(&mut left_buffer[..], &mut right_buffer[..]);
				for (i, value) in left_buffer.iter().interleave(right_buffer.iter()).enumerate() {
					data[i] = *value;
				}
			}
		}).unwrap();
		//Run this thread indefinitely, because the output device can't move threads and needs to stay existing.
		loop {
			thread::sleep(Duration::from_secs(60));
		}
	});

	//Create a thread that continuously reads from a note buffer to render the images in the synthesizer.
	thread::spawn(move || {
		let rate = 120.0 / 60.0 * 16.0; //120BPM, with 16-sub-beat intervals.
		let mut time = 0u32; //Current timestamp.
		let mut loop_helper = LoopHelper::builder().build_with_target_rate(rate);
		loop {
			loop_helper.loop_start();
			play(&mut receiver, time.clone(), synth.clone());
			time += 1;
			loop_helper.loop_sleep(); //Limit the loop rate.
		}
	});
}