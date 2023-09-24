# Doman Scale game.
# Copyright (C) 2023 Ghostkeeper
# This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
# This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
# You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.

import time


class Generator:
	"""
	Responsible for the business logic of automatically generating music.

	This will generate the next MIDI notes to play, and add them to a buffer for playback.
	"""

	def __init__(self, player) -> None:
		"""
		Create a new generator, with its own buffer of notes to play.
		:param player: The music player that contains the playback parameters for what music is desired.
		"""
		self.player = player
		self.start_time = time.time()
		self.last_note = self.start_time
		self.buffer = []

	def generate(self) -> None:
		"""
		A continuous process (which will never terminate) that keeps the buffer filled with music.

		This function will ensure that the buffer of notes is filled with at least 4 beats (1 measure) of music. To do
		this, it will repeatedly poll the buffer to see what is in it (when the buffer is expected to deplete too far)
		and will then fill it according to the parameters given by the music player.
		"""
		self.start_time = time.time()
		self.refill_buffer()


	def refill_buffer(self):
		"""
		Gets called when the buffer is too far depleted, to refill the buffer with new notes.
		"""
		pass  # TODO