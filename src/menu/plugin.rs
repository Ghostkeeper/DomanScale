/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::app::{App, Plugin, Startup, Update};

use crate::menu::system;

/// A plug-in that creates and runs the main menu.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
        app.add_systems(Update, system::menu_system);
		app.add_systems(Startup, system::create_menu);
	}
}