/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

//! Define a plug-in that plays music during the game.

use bevy::app::{App, Plugin, Update};

use crate::music::{events, control};

/// A plug-in that plays music during the game.
pub struct MusicPlugin;

impl Plugin for MusicPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<events::PlayMusic>();
        app.add_systems(Update, control::play);
	}
}