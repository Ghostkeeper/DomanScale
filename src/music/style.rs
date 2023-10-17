/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::ecs::system::Resource;

/// Defines the style of the music that should be generated.
///
/// This resource can adjust a number of parameters related to the music style. The music generator
/// will read these when it is time to fill up the buffer with newly generated music.
#[derive(Resource)]
pub struct Style {
	/// Whether any music should be generated at all.
	pub playing: bool
}