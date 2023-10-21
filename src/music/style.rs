/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::ecs::system::Resource;
use std::sync::{Arc, Mutex};

use crate::music::pitch::Pitch;
use crate::music::scale::Scale;

/// A resource that holds the style in a way that it can be used from Bevy's ECS as well as from the
/// music generation thread.
#[derive(Resource)]
pub struct StyleResource {
	/// A pointer to the actual style object.
	pub style: Arc<Mutex<Style>>
}

/// Defines the style of the music that should be generated.
///
/// This resource can adjust a number of parameters related to the music style. The music generator
/// will read these when it is time to fill up the buffer with newly generated music.
pub struct Style {
	/// Whether any music should be generated at all.
	pub playing: bool,

	/// Whether the generated music should have a magical overtone.
	///
	/// Most prominently, this changes the musical scale from Western to Arabic.
	pub enchanting: bool
}