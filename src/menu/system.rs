/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::{Commands, Query, Res};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::hierarchy::{BuildChildren, Children};
use bevy::log::{debug, trace};
use bevy::text::Text;
use bevy::ui::node_bundles::NodeBundle;
use bevy::ui::widget::Button;
use bevy::ui::{AlignItems, Interaction, JustifyContent, Style, Val};

use crate::menu::button;
use crate::music::style::StyleResource;

/// System that renders and updates the menu.
pub fn menu_system(mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>, text_query: Query<&mut Text>, music_style: Res<StyleResource>) {
	for (interaction, children) in &mut interaction_query {
		let text: &str = &text_query.get(children[0]).unwrap().sections[0].value;
		match *interaction {
			Interaction::Pressed => {
				trace!("Pressed menu button: {}", text);
				music_style.style.lock().unwrap().playing = true;
			}
			_ => {}
		}
	}
}

/// Construct the menu buttons.
pub fn create_menu(mut commands: Commands) {
	debug!("Creating menu");
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