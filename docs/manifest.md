# hub.toml

A project tells the launcher how to build it with a `hub.toml` file at the top of
its repository. It holds, as data, what each project's `build.bat` and
`tools\get_deps.bat` do today, so the launcher can do it the same way for every
project and the scripts do not have to be kept in step by hand.

**Status:** the launcher builds a project from its `hub.toml` when its folder has
one (`crates/hub-core/src/build.rs`). Neither listed project ships one yet, so today
both are still built by their own `install.bat` and `build.bat`. Updating a project
is done by the launcher (`crates/hub-core/src/update.rs`), which then rebuilds it. The format can still change until the first project
ships a `hub.toml`.

Tools named in `kit` are downloaded by the launcher itself. When one of the other
tools from `[requires]` is missing on the PC, the launcher does not install it: a
first install falls back to the project's `install.bat`, which installs the tools
and builds.

Examples for the two listed projects are in [`examples/`](../examples). The tests
keep them valid.

## What a manifest can never do

- Say where a project's own file (a game ROM) comes from. `[input]` only names the
  file. The user picks their own copy from their disk, and `{input}` is that path.
- Download anything without a SHA256. Every `[[fetch]]` is checked, and what does
  not match is thrown away.
- Touch anything outside the project's folder. Every path is relative to it, and
  `..`, drive letters and absolute paths are refused.

## Top level

```toml
schema = 1
exe = "StarWarsTrilogy.exe"
news = "news.txt"
```

| Key | | |
| --- | --- | --- |
| `schema` | required | Format version. This document describes `1`. A launcher that meets a newer one asks to be updated. |
| `exe` | required | The program the build leaves in the project's folder. It is what Play starts. |
| `news` | optional | News file in the `[N] title` format. |

Unknown keys are an error, so a misspelt key is noticed and not ignored.

## [requires]

What must be on the PC before the build. The launcher looks for each one and shows
the result in Settings.

```toml
[requires]
python = "3.10"
cmake = "3.27"
msvc = true
```

| Key | |
| --- | --- |
| `python` | Minimum Python 3. Looked for as `py -3`, `python`, then the python.org install folders. Every candidate is run and asked its version: `python` in PATH can be an old one or the Microsoft Store alias. |
| `cmake` | Minimum CMake. The one that comes with Visual Studio first, then `cmake` in PATH. |
| `msvc` | Visual Studio 2022 or later (Build Tools or any edition) with the x64 C++ tools and its CMake, and a Windows SDK with the x64 headers and libraries. |
| `kit` | Tools of the launcher's own build kit, by name. See below. |

### The build kit

```toml
[requires]
kit = ["llvm-mingw", "cmake", "ninja", "python"]
python = "3.10"
cmake = "3.27"
```

A project that names tools in `kit` is built with the launcher's own copies of
them instead of whatever is on the PC. The launcher downloads each one once
into `%LOCALAPPDATA%\NicoLauncher\kit`, checks its SHA256 and unpacks it there.
Nothing is installed on the system, no administrator permission is asked, and
deleting that folder removes all of it.

| Tool | |
| --- | --- |
| `llvm-mingw` | clang, lld and libc++ with the MinGW-w64 headers and libraries: a C and C++ compiler for Windows that needs no Visual Studio and no Windows SDK. In PATH as `clang`, `clang++`. |
| `cmake` | CMake. `{cmake}` and `{ctest}` are the kit's. |
| `ninja` | Ninja, in PATH. |
| `python` | Python. `{python}` is the kit's. |

The exact versions, where each comes from and its SHA256 are in
[`kit.json`](../kit.json); they change only with a new version of the launcher.
With `python` or `cmake` in `kit`, the version asked for is checked against the
kit's and nothing is looked for on the PC.

Every `run` step of such a project gets a clean environment: PATH is the kit's
folders and Windows' own (`System32`), and only the variables Windows itself
sets are passed on (the user's folders, TEMP, proxy settings). Another compiler,
another Python or a `CFLAGS` some installer left on the PC take no part in the
build. Name the compiler when configuring (`-DCMAKE_C_COMPILER=clang`,
`-DCMAKE_CXX_COMPILER=clang++`), so CMake does not go looking for one.

The kit only holds tools whose licenses allow passing them on, each downloaded
from its own project. It never holds anything of a game: a project is still
built on the user's PC from the user's own file.

[`examples/StarWarsTrilogyNative.kit.hub.toml`](../examples/StarWarsTrilogyNative.kit.hub.toml)
is a whole manifest that builds this way.

## [input]

A file only the user can provide.

```toml
[input]
label = "swtrilgy.zip (Rev. A)"
done_when = ["data/crom.bin", "data/drive_program.bin"]
```

| Key | | |
| --- | --- | --- |
| `label` | required | What to ask for, as the README names it. |
| `done_when` | optional | Files that exist once the input has been used. When all are there, the file is not asked for again (a rebuild or an update does not need it). |

## [[fetch]]

Files downloaded before the steps. One table per file.

```toml
[[fetch]]
name = "SDL3"
url = "https://github.com/libsdl-org/SDL/releases/download/release-3.4.18/SDL3-3.4.18.tar.gz"
sha256 = "9c75cf16330322c217dedd2e0609f1124f1b54b8633e763467b4684d0f4334a3"
to = "third_party/downloads/SDL3-3.4.18.tar.gz"
```

All four keys are required. `url` must be `https://`. A file already at `to` with
the right SHA256 is not downloaded again, and one with a different SHA256 is
replaced. A download goes to a `.part` file first and only takes the name once its
SHA256 matches.

## [[step]]

The build, in order. A step is either `run` or `copy`. The first step that fails
stops the build, and a build whose steps all pass but leave no `exe` is a failure
too. Each step is shown as `[n/total] name`.

```toml
[[step]]
name = "Extracting the ROM"
run = ["{python}", "tools/romtool.py", "{input}", "data"]
skip_if_exists = ["data/crom.bin", "data/drive_program.bin"]

[[step]]
name = "Compiling"
run = ["{cmake}", "--build", "build/cmake"]
msvc_env = true

[[step]]
name = "Copying the game"
copy = { from = "build/cmake/StarWarsTrilogy.exe", to = "StarWarsTrilogy.exe" }
```

| Key | |
| --- | --- |
| `name` | Required. Shown while the step runs. |
| `run` | A program and its arguments, run in the project's folder. There is no shell: each item is one argument, so paths with spaces need no quoting. |
| `copy` | `{ from, to }`, both inside the project's folder. |
| `msvc_env` | Run with the Visual Studio x64 environment (what `vcvars64.bat` sets). Needs `msvc = true` in `[requires]`. |
| `skip_if_exists` | The step is skipped when all these files exist. |
| `optional` | A failure is shown as a warning and the build goes on. For tests that run after the program is already built, or a copy of a file that may not be there. |

### Placeholders

Inside `run`, these are replaced before the program starts:

| | |
| --- | --- |
| `{python}` | The Python found for `[requires] python`. |
| `{cmake}`, `{ctest}` | The CMake found for `[requires]`, and the `ctest` next to it. |
| `{input}` | The full path of the file the user picked for `[input]`. |

Using one that `[requires]` or `[input]` does not ask for is an error. Write `{{`
and `}}` for literal braces.

A step can run one of the project's own scripts (`run = ["cmd", "/c", "tools\\get_dlss.bat"]`)
for anything the format does not cover.

## Achievements

Not part of `hub.toml`: a project that has achievements writes the file
`save/achievements.json` in its own folder, and the launcher shows it on the
project's page (and a count on its card in the Library). The launcher knows
nothing of any project's achievements; the file says everything, so it can be
written by any program, in any language. Without the file, nothing is shown.

The project writes it whenever its achievements or their progress change (the
games do it every time they save, and once at start). Replace it in one step
(write a temporary file and rename it over the old one), so the launcher never
reads half of it.

```json
{
  "schema": 1,
  "achievements": [
    {"id": "ace", "name": "ACE PILOT", "description": "Destroy 100 enemies", "goal": 100, "progress": 37, "unlocked": 0},
    {"id": "veteran", "name": "VETERAN", "description": "Play for 5 hours", "goal": 1035432, "progress": 312000,
     "progress_text": "1h 30m / 5h", "unlocked": 0},
    {"id": "yavin", "name": "GREAT SHOT, KID", "description": "Clear the Battle of Yavin", "goal": 1, "progress": 1, "unlocked": 1762900000}
  ]
}
```

| Key | | |
| --- | --- | --- |
| `schema` | required | Format version. This document describes `1`. A launcher that meets another one shows nothing. |
| `id` | required | Stays the same across versions of the project. |
| `name` | required | As the project shows it. |
| `description` | optional | What it asks for. |
| `goal` | optional | Progress needed, 1 (the default) for those that are done or not. |
| `progress` | optional | How far the player is, up to `goal`. Shown as a bar when `goal` is more than 1. |
| `progress_text` | optional | The progress as the project wants it written, when the plain numbers do not read well (time counted in frames, say). |
| `unlocked` | optional | When it was unlocked, in Unix time (seconds, UTC). 0 or missing: still locked. |

The list is shown in the file's order, with the unlocked ones first, newest on
top. Unknown keys are ignored, so a project can add more without breaking older
launchers.
