<p align="center">
  <img src="docs/img/banner.png" alt="NicoLauncher" width="820">
</p>

<p align="center">
  <b>Install, update and play <a href="https://github.com/snicolast">snicolast</a>'s arcade recomps on PC.</b><br>
  Press <b>Install</b>, wait, press <b>Play</b>.
</p>

<p align="center">
  <a href="https://github.com/snicolast/NicoLauncher-Hub/releases/latest"><b>⬇️ Download for Windows</b></a>
</p>

## How to play

1. **Download** `NicoLauncher-setup.exe` from the [latest release](https://github.com/snicolast/NicoLauncher-Hub/releases/latest) and run it.
   Windows says *"Windows protected your PC"*? Click **More info**, then **Run anyway**.
2. The first time, **accept the license** and **choose a folder** for your games.
3. **Pick a game** and press **Install**.
4. **Choose your own ROM file** when it asks.
5. **Wait** a few minutes while it builds the game on your PC.
6. Press **Play**.

Updates are one click: an **Update** button shows up when a game has something new, and the launcher updates itself.

## The games

- **[Nico SWT](https://github.com/snicolast/StarWarsTrilogyNative)**: an unofficial recomp of the 1998 arcade game Star Wars Trilogy Arcade.
- **[Nico TLW](https://github.com/snicolast/LostWorldNative)**: an unofficial recomp of the 1997 arcade game The Lost World: Jurassic Park.
- **[Nico SR2](https://github.com/snicolast/SegaRally2Native)**: an unofficial recomp of the 1998 arcade game Sega Rally 2.

## You need

- Windows 10 or 11 (64-bit), internet and a few GB free.
- **Your own ROM file for each game.** Not included, and we can't tell you where to get one.

Nothing else: no Visual Studio, no Python. The launcher brings its own tools.

## Good to know

- **No ready-made downloads.** Each game is built on your PC from its source code and your ROM. Built games can't be shared.
- **Not an emulator.** The original code, translated to run directly on PC.
- **Free, forever.** There is an optional coffee button in **Credits**; it gets you nothing extra.
- **Unofficial fan projects**, not affiliated with the owners of the original games, with no obligation of support.

<details>
<summary><b>Something went wrong?</b></summary>

- Most of the time, pressing **Install** again is enough (after an internet cut, for example).
- Still failing? Press **Show log**, then **Copy log**, and open an issue in that game's repository with the log pasted in.
- Problems with the launcher itself go to [Issues](https://github.com/snicolast/NicoLauncher-Hub/issues) here.
- To remove a game, open it and press **Uninstall**. Only the game's folder is deleted.
- Want everything in one folder? Use `NicoLauncher-portable.zip` instead: see [docs/portable.md](docs/portable.md).

</details>

<details>
<summary><b>For developers</b></summary>

A Rust engine and a [Tauri 2](https://tauri.app) app with a plain HTML, CSS and JavaScript interface.

| Path | What |
| --- | --- |
| `crates/hub-core` | The engine, no interface: registry, source download, build kit, build steps, updates, news, settings |
| `src-tauri` | The app: the commands the interface calls, window, icons |
| `ui` | The interface. No build step |
| `registry.json` | The projects the launcher offers. Read from `main`, so adding one needs no release |
| `news.txt` | The launcher's own news |
| `kit.json` | The build tools the launcher downloads, each pinned by SHA-256 |
| `docs/manifest.md` | The `hub.toml` format a project uses to say how it is built |
| `docs/portable.md` | Portable mode: where the launcher keeps its files, relative paths |

To build it you need [Rust](https://rustup.rs) and Node.js, on Windows:

```
cargo test -p hub-core
npx @tauri-apps/cli@2 build --no-bundle
```

The exe is `target\release\nicolauncher.exe`. `npx @tauri-apps/cli@2 dev` runs it while you work on it. Releases are described in [docs/releasing.md](docs/releasing.md).

</details>

## License

MIT with additional terms: see [LICENSE](LICENSE). In short: keep the credit to snicolast, do not sell it, and do not change it to share built games or ROM data.
