# Doman Scale game.
# Copyright (C) 2023 Ghostkeeper
# This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
# This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
# You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.

import pygame
import typing

import resources

if typing.TYPE_CHECKING:
	import game


class Button:
	"""
	A button shown in the interface of a menu.

	The button contains a text, and some action that happens when the button is pressed.
	"""
	def __init__(self, text: str, action: typing.Callable[[], None], position_x: float, position_y: float, width: float, game: "game.Game") -> None:
		"""
		Creates a new button, with a given text.
		:param text: The text to display on the button.
		:param action: An action to execute when the button is pressed.
		"""
		self.text = text
		self.action = action
		self.position_x = position_x
		self.position_y = position_y
		self.width = width
		self.game = game

		# Pre-render the text.
		font = resources.fonts["menu"]
		self.text_image = font.render(text, True, (0, 0, 0))
		self.text_rect = self.text_image.get_rect()

	def draw(self) -> None:
		"""
		Draws the button on the screen.
		"""
		# Draw background colour.
		pygame.draw.rect(self.game.window, (255, 255, 255), (self.position_x, self.position_y, self.width, 100))
		# Draw text in the middle.
		self.game.window.blit(self.text_image, (self.position_x + self.width / 2 - self.text_rect.width / 2, self.position_y + 100 / 2 - self.text_rect.height / 2))

	def event(self, event: pygame.event.Event) -> bool:
		"""
		Attempt to handle a game event.

		If the event needs to get swallowed by this button (the button is clicked) then `True` will be returned.
		:param event: The game event that occurred.
		:return: Whether the event is handled now by this button, and should be discarded for the rest.
		"""
		if event.type != pygame.MOUSEBUTTONUP:
			return False  # Not a mouse button press.
		click_position = pygame.mouse.get_pos()
		if click_position[0] < self.position_x or click_position[1] < self.position_y or click_position[0] >= self.position_x + self.width or click_position[1] >= self.position_y + 100:
			return False  # Outside of button bounds.
		self.action()