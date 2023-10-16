/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

//! This module generates the music for the game.
//!
//! The music player is a service that always runs in the background. It can be controlled by
//! adjusting its parameters, such as the theme it should be playing, or how intense the music
//! should be. Auxiliary controls like start/stop and volume are also provided.
//!
//! All controls are sent to the music service via events. The service can then act on it of its own
//! accord and on its own time. The service will not always immediately react to everything. For
//! instance, the theme of the music can't immediately be changed. If the music theme is changed
//! halfway through a measure, the music service will set the theme to change somewhere later on,
//! with a nice transition towards it to keep the music going. Other controls, like volume, do get
//! applied immediately.

pub mod plugin;
pub mod state;

mod midi_message;
mod note;
mod player;