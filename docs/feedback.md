# Feedback

The launcher's Feedback page has two ways out:

- **Report a problem** opens a new issue on GitHub, in the repository of the
  game or of the launcher, with the versions (and the log of a failed install
  or build) already filled in. The user reads it there and sends it. Problem
  reports are public.
- **Suggest an improvement**, **Propose a recomp** and **Ask or comment** are
  private. The launcher sends them itself to the author's mailbox,
  `nico-inbox` (this repository, `crates/nico-inbox`), which keeps them and
  sends a notice to his phone. Nothing is published, and no GitHub account
  is needed. The author can answer from the mailbox's panel, and the answer
  shows up in the launcher that sent the message, and nowhere else.

Nothing goes to GitHub Discussions any more.

## Where the launcher sends

`registry.json` names the mailbox:

```json
"feedback_inbox": "https://yoda.example.ts.net"
```

Changing it needs no launcher release. Without it (or with an address that is
not `https://`), the three private cards are greyed out. Launchers older than
0.3.21 ignore the key.

## The protocol

1. `GET {inbox}/v1/challenge` answers `{"challenge": "…", "bits": 20}`.
2. The launcher finds a number `nonce` such that SHA-256 of
   `"{challenge}:{nonce}"` starts with `bits` zero bits (well under a second).
3. `POST {inbox}/v1/message` with JSON:

```json
{
  "sender": "32 hex characters, random, made once per install",
  "challenge": "…", "nonce": 12345,
  "kind": "idea | recomp | talk",
  "about": "segarally2 | launcher | null",
  "title": "…", "system": "for recomp proposals", "text": "…",
  "contact": "optional: email or Discord name",
  "details": [["Launcher", "v0.3.21 · Installed"], ["Nico SR2", "Version 39 · abc1234"], ["Windows", "…"], ["Graphics", "…"]],
  "lang": "en", "written": 1795462067,
  "local_id": "16 hex characters, random, made for this message"
}
```

`local_id` comes from launchers 0.3.24 and later.

Answers: `200 {"status":"sent"}`; `429 {"status":"limited"}` (too many from
this sender or from everyone today: the launcher says so and keeps the form);
`403 {"status":"retry"}` (the challenge ran out or was used: the launcher asks
for a new one, once); `400 {"status":"error","error":"…"}` (refused).

When the mailbox does not answer (or answers 5xx), the launcher keeps the
message in `feedback.json` next to its settings (at most 20, for 30 days) and
sends it on a later start or Refresh. The same file holds the install's
random `sender` id.

The launcher is open source, so it has no secret: the sum, the limits per
sender and per address (5 an hour, 20 a day by default, and 300 a day from
everyone), size limits, and blocking from the panel keep spam down. Messages
with links or that talk about ROMs are marked in the panel and arrive as quiet
notices. A blocked sender is told "sent" and its messages are dropped.

## Answers

In the panel, each message has a box to answer it. The answer stays in the
mailbox, next to the message: nothing goes to GitHub or anywhere public.

After each start and Refresh, the launcher asks for the answers to its own
messages:

```
POST {inbox}/v1/replies
{"sender": "the same 32 hex characters it sends its messages with"}
```

```json
{"replies": [{"id": "1795462067-a1b2c3", "local_id": "…", "kind": "idea", "about": "segarally2",
  "title": "…", "text": "…", "written": 1795462067, "reply": {"text": "…", "at": 1795480000}}]}
```

Only messages with an answer are listed, with what the user wrote, so an
answer to a message sent by an older launcher (which kept no copy of it) can
still be shown. The sender id is random, only the launcher and the mailbox
know it, and it is never shown anywhere public: it is what keeps one user from
reading another's answers. A blocked sender gets an empty list, and one
address may ask 60 times an hour. The first time an answer is collected, the
panel says so ("Collected by the launcher").

In the launcher, a new answer brings a green notice with Read, a count on
Feedback in the sidebar, and the answer under **Your messages** on the
Feedback page, where every message sent from there is listed (the copies live
in `feedback.json`, at most 50). An answer is a one-off: the launcher offers no
way to reply to it (a new message can always be sent). Changing an answer in the panel shows it to the user again as new; removing
it before it is collected means it never arrives.

A mailbox older than this (no `/v1/replies`) just never has answers: the
launcher asks again next time and says nothing.

### Read, and an emoji back

The answer window has a row of emojis (👍 ❤️ 😄 🙏 🎉 🤔); picking one is
optional, and there is no text. When the user closes the window, the launcher
tells the mailbox the answer was read:

```
POST {inbox}/v1/read
{"sender": "…", "id": "1795462067-a1b2c3", "reaction": "❤️"}
```

`reaction` is one of those six or `null`. The mailbox answers 200, 404 (`no
such answer`: not this sender's message, or no answer any more) or 400 (any
other emoji). It keeps the time it was first read and the latest emoji, and
sends a quiet notice ("❤️ Your answer was read"). The panel then shows Read
and the emoji next to the answer: from then on the message can be archived or
deleted, the user's launcher keeps its copy. Changing the emoji later sends it
again; a changed answer is new, unread and without an emoji.

The launcher remembers what it still has to tell (`read_told` in
`feedback.json`) and retries after each refresh, so a closed window with no
connection, or a mailbox older than 0.3.0 (no `/v1/read`), is told later.

## The panel

The panel is a mail client: folders on the left (Inbox, Unread, Important,
To answer, Answered, Flagged, Archived, Blocked) with filters by kind and by
game, the list in the middle with a search box (title, text, contact and the
answer), and the open message on the right with its buttons (Important, Mark
unread, Archive, Block, Delete) and the answer. Opening a message marks it
read. Keys: `j`/`k` move through the list, `/` goes to the search. On a phone
it is one column: the list, then the message.

It is at `https://<machine>.<tailnet>.ts.net:8443/` from any machine of the
tailnet: another PC only needs Tailscale installed and signed in with the
same account.

## Setting up the mailbox (Windows, behind Tailscale)

The machine needs to be on and in your tailnet. No port is opened on the
router: Tailscale Funnel carries the requests in.

1. **The program.** Build it (`cargo build --release -p nico-inbox`) and put
   `nico-inbox.exe` in a folder of its own, say `C:\nico-inbox`.
2. **Settings.** Copy `crates/nico-inbox/config.example.toml` there as
   `config.toml`. Set `[ntfy] url` to `https://ntfy.sh/<a long random name>`
   and subscribe to that topic in the ntfy app on your phone. Keep the name
   private: anyone who knows it reads the notices.
3. **Start with Windows**, as the built-in LOCAL SERVICE account (almost no
   rights on the machine: it may read the folder and write only in `data`),
   restarted if it stops. In PowerShell as administrator (not
   `schtasks /Create`: its tasks are stopped after 72 hours):

   ```powershell
   New-Item -ItemType Directory -Force 'C:\nico-inbox\data' | Out-Null
   icacls 'C:\nico-inbox' /grant '*S-1-5-19:(OI)(CI)RX'
   icacls 'C:\nico-inbox\data' /grant '*S-1-5-19:(OI)(CI)M' /T
   $a = New-ScheduledTaskAction -Execute 'C:\nico-inbox\nico-inbox.exe' -Argument '--config "C:\nico-inbox\config.toml"' -WorkingDirectory 'C:\nico-inbox'
   $t = New-ScheduledTaskTrigger -AtStartup
   $s = New-ScheduledTaskSettingsSet -ExecutionTimeLimit ([TimeSpan]::Zero) -RestartCount 999 -RestartInterval (New-TimeSpan -Minutes 1) -StartWhenAvailable
   $p = New-ScheduledTaskPrincipal -UserId 'NT AUTHORITY\LOCALSERVICE' -LogonType ServiceAccount
   Register-ScheduledTask -TaskName 'nico-inbox' -Action $a -Trigger $t -Settings $s -Principal $p -Force
   Start-ScheduledTask -TaskName 'nico-inbox'
   ```

4. **Public part (messages only).** Tailscale must have HTTPS certificates on
   (admin console, DNS) and allow Funnel for this machine (the first command
   prints a link to allow it):

   ```bat
   tailscale funnel --bg --set-path /v1 http://127.0.0.1:8787/v1
   ```

5. **The panel (your machines only):**

   ```bat
   tailscale serve --bg --https 8443 http://127.0.0.1:8788
   ```

   It is at `https://<machine>.<tailnet>.ts.net:8443/`. Put that address in
   `[ntfy] click` so tapping a notice opens it.
6. **Tell the launcher.** Put `https://<machine>.<tailnet>.ts.net` in
   `registry.json` as `feedback_inbox`. `tailscale funnel status` shows the
   address. It is public (certificate logs list it), which is fine: only
   `/v1` is published.

`data\inbox.log` says what came in, what was limited or dropped, and when ntfy
failed. Messages (and their answers) are plain JSON files in `data\messages`: copy
the folder to keep them.

The panel only answers requests addressed to `127.0.0.1`, `localhost` or a
`*.ts.net` name, so a web page cannot reach it through a name of its own that
points at this machine. The sender's address is the last one in
`X-Forwarded-For` (Tailscale writes it), so a client cannot pass off another.

**Updating the program**: stop the task (`Stop-ScheduledTask -TaskName
nico-inbox`), replace `nico-inbox.exe`, run `icacls C:\nico-inbox\nico-inbox.exe
/reset` (a file copied or moved over keeps its own permissions, and LOCAL
SERVICE then cannot start it: task result 2147942405, access denied), start
it again. Messages and settings stay; Tailscale needs nothing new (`/v1`
already covers the answers).
