/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

//! Tools for creating buttons for the game's menu.

use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::render::color::Color;
use bevy::text::TextStyle;
use bevy::ui::node_bundles::{ButtonBundle, TextBundle};
use bevy::ui::{AlignItems, JustifyContent, Style, Val};

/// Create a button showing a certain text.
///
/// The button will be constructed with the text as children.
///
/// # Arguments
/// - `parent`: The parent node under which to create this button.
/// - `text`: The text to display on the button.
pub fn construct_button(parent: &mut ChildBuilder, text: &str) {
	parent.spawn(ButtonBundle {
		style: Style {
			width: Val::Px(150.0),
			height: Val::Px(50.0),
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			..Default::default()
		},
		..Default::default()
	}).with_children(|parent| {
		parent.spawn(TextBundle::from_section(
			text,
			TextStyle {
				font_size: 40.0,
				color: Color::BLACK,
				..Default::default()
			}
		));
	});
}