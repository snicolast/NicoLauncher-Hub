# Portable mode

A launcher with a `portable.txt` (or a `config.json`) next to its exe runs
portable: it keeps all its own files in that folder and writes nothing to
AppData.

| What | Installed | Portable |
| --- | --- | --- |
| Settings | `%APPDATA%\NicoLauncher\config.json` | `config.json` |
| Build kit | `%LOCALAPPDATA%\NicoLauncher\kit` | `kit\` |
| Window data (WebView2) | `%LOCALAPPDATA%\io.github.snicolast.nicolauncher` | `webview\` |
| Updates | `NicoLauncher-setup.exe` | `NicoLauncher-portable.zip` |

Each release publishes `NicoLauncher-portable.zip`: the same exe and a
[`portable.txt`](portable.txt). Both kinds can live on the same PC: each has
its own settings, kit and data.

## Relative paths

In `config.json`, `install_root` and each project's `dir` and `input` can be
relative. They start from the folder of `config.json`, in both modes:

```json
{
  "install_root": ".\\Games",
  "projects": {
    "segarally2": { "input": ".\\ROMs\\game.zip" }
  }
}
```

A portable launcher saves the folders and files picked inside its own folder
this way, so they keep working when the folder is moved or the drive gets
another letter. Anything outside it is saved as a full path.

## Updates

The updater reads the same `latest.json`. A portable launcher asks for its
`windows-x86_64-portable` entry: the zip, signed with the same key as the
installer. It downloads it, checks the signature, takes `NicoLauncher.exe` out
of it and swaps it for the running exe, which Windows allows to be renamed:
the old one becomes `NicoLauncher.exe.old` and is deleted on the next start.

## What it needs

- WebView2, which Windows 10 and 11 already have. The installer would install
  it when missing; the portable zip cannot.
- A folder the user can write to, without `[` `]` or `;` in its path (the
  build tools cannot work from one).
- No Start menu entry or uninstaller: deleting the folder removes it.
