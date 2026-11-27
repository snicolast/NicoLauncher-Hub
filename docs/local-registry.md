# Trying a project before it is public

The launcher lists the projects of `registry.json` in this repository. To try
a new project (or a test branch of one already listed) on one PC first, put a
`registry.local.json` next to the launcher's `config.json`:

| Launcher | Where |
| --- | --- |
| Installed | `%APPDATA%\NicoLauncher\registry.local.json` |
| Portable | next to the exe, with `config.json` |

It has a `projects` list in the same format as `registry.json`:

```json
{
  "projects": [
    {
      "id": "newgame",
      "name": "New Game",
      "repo": "owner/NewGameNative",
      "branch": "main",
      "exe": "NewGame.exe",
      "news": "news.txt",
      "input": { "label": "newgame.zip" },
      "scripts": { "install": "install.bat", "build": "build.bat", "update": "update.bat" },
      "update_state": "save/update",
      "art": { "hub_logo": "art/logos/newgame.png", "hub_icon": "art/icons/newgame.png" }
    }
  ]
}
```

- A project with the id of one in `registry.json` takes its place on that PC;
  with the id of a `coming_soon` entry, it shows as a project there instead of
  "Coming soon". The others are added at the end.
- It is read when the launcher starts and on each refresh. A broken file, or
  one with a bad entry, is skipped as a whole and the public list is used.
- The launcher downloads the code without signing in to GitHub, so the
  repository has to be public.
- When the project goes public, move its entry to `registry.json` and delete
  the local file.

Launchers older than 0.3.26 do not read the file.
