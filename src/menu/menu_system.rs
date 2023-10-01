/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::ecs::system::Commands;
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::hierarchy::BuildChildren;
use bevy::ui::node_bundles::NodeBundle;
use bevy::ui::{AlignItems, JustifyContent, Style, Val};

use crate::menu::button;

/// System that renders and updates the menu.
pub fn menu_system() {
	println!("Menu system")
}

/// Construct the menu buttons.
pub fn create_menu(mut commands: Commands) {
	println!("Creating menu");
	commands.spawn(Camera2dBundle::default());
	commands.spawn(NodeBundle {
		style: Style {
			width: Val::Percent(100.0),
			align_items: AlignItems::Center,
			justify_content: JustifyContent::Center,
			..Default::default()
		},
		..Default::default()
	}).with_children(|parent| { button::construct_button(parent, "Play"); });
}