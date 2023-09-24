# Doman Scale game.
# Copyright (C) 2023 Ghostkeeper
# This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
# This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
# You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.

import multiprocessing
import pygame.mixer

import music.player


class Control:
	"""
	Controls the playing and style of background music for the game.

	The music player is started on a separate process that gets fed information asynchronously from the main game. This
	separate process listens to synchronised variables that are controlled from this control class. So from this class
	we can set the variables, but it is up to the music player to eventually cause the music to be played.

	However, this control class also holds the handles to the other process and starts the system.
	"""

	def __init__(self) -> None:
		"""
		Creates the controller, starting a separate process that will listen to queues to make music.
		"""
		pygame.mixer.music.set_soundfont("airfont.sf2")
		self.player = music.player.Player()
		pipe_out, pipe_in = multiprocessing.Pipe(duplex=False)
		start_music = lambda pipe_out: self.player.play(pipe_out)
		self.process = multiprocessing.Process(target=start_music, args=(pipe_out, ))

	def start(self):
		self.process.start()