Tooling
====
In this document, the tooling used to create the game is described. Which tools are we using, and why?

Rust
----
Rust is the main programming language used for this game. This choice was made because of the following advantages:
- It has an active community with good tooling, which will help in bringing this project to completion.
- It is something that Ghostkeeper wants to get better at. As described for the vision of this project, he would like to develop his skills in a wider application area, and Rust fits in with that.
- Rust produces small binaries and highly efficient code, which helps with the retro feel of the game.

Libresprite / ASE files
----
To store the visual assets of the game, ASE files are used. Libresprite is an application that can create such files.

ASE files is a file format created for the Aseprite application. This application is focussed on creating pixel art, which is part of the vision of this game, and what's more, it is the industry standard for pixel art at the moment. As such, the ASE file format is currently the industry standard for exchanging pixel art. It has great features, including layers and animation. Plug-ins for Bevy and Rust packages exist to load ASE files, too, although they are not in a very stable state. This is preferrable to using a more interchangeable file format like PNG, because leaving it as ASE files keeps the sprites editable, making it easier to change the application later or by different people.

Aseprite is a proprietary application. There is an alternative however, Libresprite, which is forked from an earlier version of Aseprite. Libresprite is still free and open source. For that reason, it is a preferred tool for editing sprites. It is likely that the ASE file format can still be used for interchange between the two for the moment, but if they grow apart in the future, Libresprite's version will be the preferred file format.

Bevy
----
The game engine that powers this project is Bevy. Bevy was selected for its popularity and its completeness.

Bevy is a rewrite of a more mature game engine called Amethyst. Amethyst would have been a good choice as well, being a bit more mature and stable than Bevy. However Amethyst's community is smaller now that Bevy is released and gaining traction, since Bevy is meant to replace Amethyst. Furthermore, Bevy is more extensible with custom plug-ins, and the community has made a number of these, including a plug-in to read ASE files which we're using extensively.
