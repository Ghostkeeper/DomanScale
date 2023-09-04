# Doman Scale game.
# Copyright (C) 2023 Ghostkeeper
# This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
# This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
# You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.

import logging
import pygame
import typing

import menu.button

if typing.TYPE_CHECKING:
	import game

logger = logging.getLogger("MainMenu")

class MainMenu:
	"""
	This class creates the main menu on the screen, allowing interaction with the menu through clicking buttons or using
	the keyboard.
	"""

	def __init__(self, game: "game.Game"):
		"""
		Create the resources for the main menu on the screen.
		:param game: The main game. Once the menu is exited (and the game starts), this will be told what scene is next.
		"""
		logger.info("Creating MainMenu")
		self.running = True

		# TODO: For now, the main menu is just for debugging the music system.
		self.objects = [
			menu.button.Button("Play", lambda: game.music_controller.start(), 100, 100, 440, game)
		]

	def run(self):
		"""
		Run the main loop of the main window.
		"""
		while self.running:
			for event in pygame.event.get():
				if event.type == pygame.QUIT:
					self.running = False
				for object in self.objects:
					if object.event(event):
						break  # Event was handled by this object and shouldn't be handled again.

			for object in self.objects:
				object.draw()
			pygame.display.update()