# DLSS bridge

The games NicoLauncher builds (with llvm-mingw) reach NVIDIA DLAA/DLSS through
`dlss_bridge.dll`, because NVIDIA's NGX library links only with MSVC. That DLL
cannot be built on the player's PC, so it is built here, one per game, and each
game's `hub.toml` downloads it next to the executable with its SHA256.

- `<game>/dlss_bridge-abi<N>-<commit>.dll`: the DLL, for the bridge ABI `N`
  (`DLSS_BRIDGE_ABI` in the game's `src/video/dlss_bridge.h`) and built from
  that game commit. The game refuses a DLL of another ABI.
- `<game>/dlss_bridge-abi<N>-<commit>.txt`: where it comes from (game commit,
  NVIDIA DLSS SDK version, SHA256, the Actions run that built it).
- `NOTICE.txt` and `LICENSE-NVIDIA-DLSS-SDK-<version>.txt`: NVIDIA's notice
  and license.

Only the bridges a game's `hub.toml` points to on its main branch are kept. A
bridge goes once no game uses it any more: a player who updates gets the new one,
and one who does not keeps the DLL already next to the game.

## A new one

Actions > **DLSS bridge** > Run workflow, pick the game (and a commit, if not
the latest). It builds the DLL with MSVC, checks that the game's loader takes
it, and opens a pull request with the `[[fetch]]` for the game's `hub.toml`.
Needed whenever a game changes `src/video/dlss*` or `tools/dlss_bridge`.
