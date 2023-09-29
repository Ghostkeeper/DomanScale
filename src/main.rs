/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::a11y::AccessibilityPlugin;
use bevy::app::{App, Update};
use bevy::input::InputPlugin;
use bevy::winit::WinitPlugin;
use bevy::MinimalPlugins;

mod menu;
mod window;

/// Creates an app with the correct plug-ins and systems, and starts it.
fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            AccessibilityPlugin,
            InputPlugin::default(),
            window::window_plugin(),
            WinitPlugin::default()
        ))
        .add_systems(Update, menu::menu_system::menu_system)
        .run();
}
