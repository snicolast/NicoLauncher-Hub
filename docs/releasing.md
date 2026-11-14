# Releasing the launcher

Besides the code, this repository holds what installed launchers read from the
internet:

- `news.txt`: the launcher's own news, shown on Home.
- `registry.json`: the list of projects. It is also built into the launcher, for
  when it cannot be read online, so adding a project needs no release.
- Its Releases: the installer and the portable zip, from where the launcher
  updates itself.

An update is
only installed when it is signed with the launcher's private key, so a release
nobody but the key's owner made is refused.

Only the launcher is released this way. Projects are never published built.

## Once: the signing key

1. On your PC, in the repository folder:

   ```
   npx @tauri-apps/cli@2 signer generate -w nicolauncher.key
   ```

   It asks for a password and writes `nicolauncher.key` (private) and
   `nicolauncher.key.pub` (public).

2. Keep `nicolauncher.key` and its password somewhere safe, outside the
   repository. **If they are lost, launchers already installed can never be
   updated again**: they only trust this key. Never commit the private key.

3. In GitHub: Settings > Secrets and variables > Actions > New repository secret.
   - `TAURI_SIGNING_PRIVATE_KEY`: the whole content of `nicolauncher.key`
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: the password

4. Put the content of `nicolauncher.key.pub` in `src-tauri/tauri.conf.json`,
   as `plugins.updater.pubkey`. The public key is not a secret.

## Each release

1. Add the release's entry to `news.txt`, starting with `Launcher X.Y.Z: `
   (the new version). On their first start after updating, launchers show
   the entries of every version newer than the one the user had.
2. Set the new version in `Cargo.toml` (`[workspace.package] version`) and in
   `src-tauri/tauri.conf.json` (`version`). Merge that.
3. Tag it and push the tag:

   ```
   git tag v0.2.0
   git push origin v0.2.0
   ```

The Release workflow builds the installer, signs it and publishes
`NicoLauncher-setup.exe`, its signature and `latest.json` as a release of
this repository, with `NicoLauncher-portable.zip` and its signature for
portable launchers ([portable.md](portable.md)). Installed launchers look at its
`releases/latest/download/latest.json` when they start and offer the
update in the sidebar.

## Not covered yet

The installer is not code-signed for Windows (a paid certificate), so
SmartScreen warns on the first install. That is separate from the update
signature above.
