# ARClassic: Classic Mode Selection Tool
## Prerequisites
- [ARCropolis 1.1.3 or higher](https://github.com/Raytwo/ARCropolis/releases/latest)
- [Skyline](https://github.com/skyline-dev/skyline/releases/tag/beta)
- [ARCExplorer](https://github.com/ScanMountGoat/ArcExplorer) (For grabbing Classic Mode Routes from the main game).


## Setup
1. Download the latest version from the Releases tab, then install to the root of your SD card.
2. Using ARCExplorer, extract all files named "standard_route_[NameOfFighter].prc" from "ui/param/standard" and "ui/param_patch_standard" and move them to "/atmosphere/contents/01006A800016E000/romfs/ClassicModeSelector/ClassicDefaults/" in your SD card.
3. (Optional) For any additional Classic Mode mods you have, create a folder (or folders, if you have many mods that can be seperated) within "/atmosphere/contents/01006A800016E000/romfs/ClassicModeSelector/" to store them in. This will allow you to choose them seperately from the default ones. Note they do not have to be the name of a character's route, so you can rename them however you see fit. It is recommended these folders are appending with 'Custom' or some other word that comes after Classic, in order to keep default classic modes at ID 0.

You can also add more files to ClassicMode_FilesToCatch to load things beyond Classic Modes in a similar manner, however it's only recommended for things that are loaded by only a single file (as it will prompt you for every file for something such as fighter data).

## In-Game Usage
1. Select a character from the Classic Mode menu. THis will prompt a keyboard instruction.
2. (This step only applies if you followed #3 above) Select the ID number corresponding to your folder.
3. Type the ID number corresponding to the file you want to use (See DefaultRoutes.csv for numbers corresponding to the defaults). Alternatively, type 'R' or 'r' (Not the button R, the letter on the keyboard), to randomly choose from the folder.

This route will stay until it's unloaded from memory, so to change the route for a character after already choosing, choose another character, then go back after entering the difficulty screen.

The route will also not change name to reflect the new route (this is because the names are stored seperately from the actual routes).

Finally, assist trophy/pokeball odds will *not* change from the character's original route, though normal item distributions will. One notable example is Young Link, who's assist trophies are exclusively Zelda-themed.

Credit to Coolsonickirby for his [Arc Randomizer](https://github.com/Coolsonickirby/arc-randomizer), from which this project was built on, and jam1garner's [Smash Minecraft Skins](https://github.com/jam1garner/smash-minecraft-skins), from which UI components were adapted from.
