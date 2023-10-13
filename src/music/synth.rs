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

/// This resource wraps the synthesizer so that it can be called from as a resource to trigger music
/// to play.
///
/// The synthesizer is wrapped in a mutex lock so that it can be used to generate music, while other
/// threads can still add new MIDI events to be played.
#[derive(Resource)]
pub struct Synth {
	/// The Rustysynth synthesizer.
	pub synth: Arc<Mutex<Synthesizer>>
}

impl Default for Synth {
	fn default() -> Self {
		let mut sf2 = File::open("airfont.sf2").unwrap();
		let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());
		let settings = SynthesizerSettings::new(44100);
		Self {
			synth: Arc::new(Mutex::new(Synthesizer::new(&sound_font, &settings).unwrap()))
		}
	}
}