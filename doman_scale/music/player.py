# Doman Scale game.
# Copyright (C) 2023 Ghostkeeper
# This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
# This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
# You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.

import logging

logger = logging.getLogger("Music.Player")


class Player:
	"""
	Causes the playback of music to happen.

	When the music is started, this player will continuously generate music on-the-fly, that gets sent to a MIDI
	controller for playback.
	"""

	def __init__(self):
		"""
		Construct a new music player.
		"""
		self.pipe_out = None

	def play(self, pipe_out: "multiprocessing.connection.PipeConnection") -> None:
		"""
		Start the music playing.
		"""
		self.pipe_out = pipe_out
		logger.info("Starting music playback.")