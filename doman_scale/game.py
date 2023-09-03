# Doman Scale game.
# Copyright (C) 2023 Ghostkeeper
# This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
# This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
# You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.

import logging
import pygame

import menu.main_menu

logger = logging.getLogger("Game")

class Game:
	"""
	A class that runs instances of the game.

	This class is responsible for orchestrating the loading of the game's resources and then choosing a starting point
	to run the game from.
	"""

	def __init__(self):
		"""
		Initialise the game, loading resources needed before the main loop is started.
		"""
		logger.info("Initialising game")
		pygame.init()
		self.running = True

		self.window_width = 640
		self.window_height = 480
		self.window = None
		self.create_window()

	def create_window(self):
		"""
		Create a window to display the game in.
		"""
		self.window = pygame.display.set_mode((self.window_width, self.window_height))
		pygame.display.set_caption("Doman Scale")
		self.window.fill((0, 0, 0))
		pygame.display.flip()

	def run(self):
		"""
		Start the main game loop.
		"""
		logger.info("Running game")

		main_menu = menu.main_menu.MainMenu(self)
		main_menu.run()