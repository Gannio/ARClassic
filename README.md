# ARClassic: Classic Mode Selection Tool
## Prerequisites
- [ARCropolis 3.0.0 or higher](https://github.com/Raytwo/ARCropolis/releases/latest)
- [Skyline](https://github.com/skyline-dev/skyline/releases/tag/beta)
- [ARCExplorer](https://github.com/ScanMountGoat/ArcExplorer) (For grabbing Classic Mode Routes from the main game).


## Setup
1. Download the latest version from the link, then install to the root of your SD card.
2. Using ARCExplorer, extract all files named standard_route_[NameOfFighter].prc from rom://ui/param/standard and ui/param_patch/standard
3. Copy the above files to sd://ultimate/ClassicRoutes/Vanilla Routes/.
4. (Optional) For any additional Classic Mode mods you have, create a folder (or folders, if you have many mods that can be seperated) within sd:/ultimate/ClassicRoutes/(Whatever you want to name the folder) to store the .prc files in. This will allow you to choose them seperately from the default ones. Note they do not have to be the name of a character's route, so you can rename them to something more representative of the route instead.

## In-Game Usage
Choose a character on the Classic Mode Select screen. This will open a prompt that will let you choose which folder group to use (if more than 1 are present), followed by the specific route. Both prompts also offer a 'Default' button to choose the character's normal route, and a 'Random' button to randomly choose from the current options.

Once a route has been chosen, it stays until the character's route is unloaded. If you want to choose another route with the same character, choose another character first and enter their difficulty select screen, then back out and choose the character you actually want.

- Some default route names differ from the character's English name (see VanillaRoutes.csv in the source code for a full list).

This route will stay until it's unloaded from memory, so to change the route for a character after already choosing, choose another character, then go back after entering the difficulty screen.

The route will also not change name to reflect the new route (this is because the names are stored seperately from the actual routes).

Finally, assist trophy/pokeball odds will *not* change from the character's original route, though normal item distributions will. One notable example is Young Link, who's assist trophies are exclusively Zelda-themed.

## ARClassic Extras
The folders within ultimate/mod in the Release folder contains new routes for Giga Bowser, Squirtle, Ivysaur, and Charizard. These files are required in order for the program to latch onto these characters, however I've created original routes for them as a bonus! A Character Expansion mod is required in order to experience these routes with their respective characters.

#Squirtle: Explorers of Smash
Team up with your partner (Diddy Kong) on a route inspired by the events of Pokemon Mystery Dungeon: Explorers of Time/Darkness/Sky!

#Ivysaur: A Plant for All Seasons
Ivysaur's route involves fighting through a series of opponents representing the four seasons (with 2 stages per season when including the bonus stage).

#Charizard: The Undefeated Flames of Red and Blue
As the only standalone character present in previous games that is not in Ultimate (as a standalone that is), its route covers a collection of newcomers, stage bosses, and items from Super Smash Bros. for 3DS and Super Smash Bros. for Wii U.

#Giga Bowser Route: The Wrath of Giga Bowser
A route inspired by [The Wrath of Giga Bowser](https://www.youtube.com/watch?v=L1nGBIGKnuU), a machinima from Super Smash Bros. Melee dating back to 2006.

Finally, the add-on mod contains files for the mii fighters should anyone ever figure out how to enable them in the CSS, however they're currently just duplicates of Giga Bowser's route for the purposes of hooking.

Credit to Coolsonickirby for his [Arc Randomizer](https://github.com/Coolsonickirby/arc-randomizer), from which this project was built on, and jam1garner's [Smash Minecraft Skins](https://github.com/jam1garner/smash-minecraft-skins), from which UI components were adapted from.

Note: This mod is *potentially not Wi-Fi safe*. This mostly comes down to the fact that Global Smash Power is an online element, and I can't say for certain whether a higher-than-normal GSP score on a Classic Mode route will result in a ban. Extra characters seem to not save their scores, however I don't have enough knowledge to confirm this. If you want to absolutely avoid a ban, I would recommend using a separate save file that you don't intend to go online with (and an emuMMC but that's for modding in general).
