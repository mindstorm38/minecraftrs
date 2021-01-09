# Minecraft RS

Rust library for cross-version Minecraft world generation.

![](https://raw.githubusercontent.com/mindstorm38/minecraftrs/master/docs/working_generation.png)
Spot the differences! *(version: 1.2.5, biomes, terrain)*

## Supported versions
Currently, the library obviously does not support all, I started with 1.2.5 because I was interested in seed reversing challenge in this version (and I think nothing changed in the generation from 1.2.1, except for jungles maybe). For now, only biomes, terrain and ravines generation works, there are certainly some bugs, but the main API is operational as you can see in the image above.

Before implementing other versions, I plan to support all features of the version 1.2.5. Maybe not structures but definitely features. For the future of this library I'm open to suggestion for supporting more versions and obviously to contributions.

Supported and tested versions and their working features:
- 1.2.5, biomes, terrain, ravines ***(NEW)***

## Rendering ?
I'm not planning to implement any renderer for this library, if it's needed this would be created in another repository.

> For the illustration above I exported the world vertices to a `.obj` file *(check "world" example to understand how)* and then I used "3D Builder" software on Windows.

## Gallery
![](https://raw.githubusercontent.com/mindstorm38/minecraftrs/master/docs/working_ravines.png)
