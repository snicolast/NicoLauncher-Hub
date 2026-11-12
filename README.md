# NicoLauncher

One place to install, build, update and play [snicolast](https://github.com/snicolast)'s native PC ports of arcade games.

- [Star Wars Trilogy Native](https://github.com/snicolast/StarWarsTrilogyNative)
- [The Lost World Native](https://github.com/snicolast/LostWorldNative)
- [Sega Rally 2 Native](https://github.com/snicolast/SegaRally2Native)

## Download

Get `NicoLauncher-setup.exe` from the [latest release](https://github.com/snicolast/NicoLauncher-Hub/releases/latest) and run it. Windows 10 or 11, 64-bit.

Windows may warn that the installer is from an unknown publisher (it is not code-signed yet): choose **More info**, then **Run anyway**.

## How it works

1. Choose a folder for your games.
2. Pick a game and press **Install**. The launcher downloads the project's source code and the tools to build it. Nothing else is installed on your PC, and Visual Studio is not needed.
3. When it asks, point it at **your own** copy of the game's ROM file. The game is built on your PC from it.
4. Press **Play**. When a game has something new, the launcher tells you and updates it with one click.

The launcher updates itself too.

## What it does not do

- It does not include, download or link to game ROMs. You use your own copy.
- It does not download or share built games. Each one is built on your PC from its source code.
- These are unofficial fan projects, not affiliated with the owners of the original games.

## This repository

It holds what installed launchers read from the internet: the launcher's news (`news.txt`), the list of games (`registry.json`) and the releases of the launcher itself. Problems with a game go to that game's repository; problems with the launcher go to [Issues](https://github.com/snicolast/NicoLauncher-Hub/issues) here.
