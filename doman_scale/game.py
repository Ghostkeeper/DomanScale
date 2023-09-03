# Doman Scale game.
# Copyright (C) 2023 Ghostkeeper
# This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
# This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
# You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.

import logging
import pygame

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

	def run(self):
		"""
		Start the main game loop.
		"""
		logger.info("Running game")