/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

//! Managing the game's window.

use bevy::window::{PresentMode, Window, WindowPlugin};

/// Gives a `WindowPlugin` with the correct settings for this game.
///
/// This sets the title, resolution and v-sync mode of the window.
pub fn window_plugin() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			title: "Doman Scale".into(),
			resolution: (600.0, 400.0).into(),
			present_mode: PresentMode::AutoVsync,
			..Default::default()
		}),
		..Default::default()
	}
}