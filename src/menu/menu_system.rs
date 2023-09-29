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
use bevy::render::color::Color;
use bevy::text::TextStyle;
use bevy::ui::node_bundles::{ButtonBundle, NodeBundle, TextBundle};
use bevy::ui::{AlignItems, JustifyContent, Style, Val};

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
	}).with_children(|parent| {
		parent.spawn(ButtonBundle {
			style: Style {
				width: Val::Px(150.0),
				height: Val::Px(65.0),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			background_color: Color::rgb(0.5, 0.0, 0.0).into(),
			..Default::default()
		}).with_children(|parent| {
			parent.spawn(TextBundle::from_section(
				"Play",
				TextStyle {
					font_size: 40.0,
					color: Color::WHITE,
					..Default::default()
				}
			));
		});
	});
}