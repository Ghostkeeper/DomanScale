/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::ecs::system::Resource;
use std::fs::File;
use std::sync::{Arc, Mutex};
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};

/// The synthesizer that generates waveform audio from MIDI music notes.
#[derive(Resource)]
pub struct Synth {
	/// The Rustysynth synthesizer.
	pub synth: Arc<Mutex<Synthesizer>>,

	/// The audio buffer for the left channel to synthesize into.
	pub left_buffer: Arc<Mutex<Vec<f32>>>,

	/// The audio buffer for the right channel to synthesize into.
	pub right_buffer: Arc<Mutex<Vec<f32>>>,
}

impl Default for Synth {
	fn default() -> Self {
		let mut sf2 = File::open("airfont.sf2").unwrap();
		let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());
		let settings = SynthesizerSettings::new(44100);
		Self {
			synth: Arc::new(Mutex::new(Synthesizer::new(&sound_font, &settings).unwrap())),
			left_buffer: Arc::new(Mutex::new(vec![0_f32; 4410])),
			right_buffer: Arc::new(Mutex::new(vec![0_f32; 4410]))
		}
	}
}