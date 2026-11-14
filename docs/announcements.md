# Announcements (`announcements.json`)

Notes from the author that are not news of a game: help wanted, a call for
testers, something every user must know. The launcher reads
`announcements.json` from this repository (main branch) when it starts, on
Refresh and every 20 minutes, the same as `news.txt`. Publishing one is a
commit to this file; no launcher release is needed.

A megaphone in the title bar beats while there is an announcement the user
has not closed yet, with its label ("Help wanted"). Pressing it opens the
announcement in the middle of the window. Closing it marks it as seen: the
megaphone stays quiet until an announcement with a new `id` arrives. An
`important` one also opens by itself when the launcher starts.

```json
{
  "schema": 1,
  "announcements": [
    {
      "id": "2026-11-help-wanted-ports",
      "date": "2026-11-14",
      "expires": "2027-03-01",
      "level": "info",
      "game": "segarally2",
      "label": "Help wanted",
      "title": "Help bring the recomps to Linux, Steam Deck and Mac",
      "body": [
        "A line is a paragraph.",
        "- A line that starts with \"- \" is an item of a list"
      ],
      "buttons": [
        { "label": "I can help", "url": "https://github.com/snicolast/NicoLauncher-Hub/discussions/..." }
      ]
    }
  ]
}
```

| Key | Required | Meaning |
| --- | --- | --- |
| `id` | yes | Unique and never reused. The launcher remembers the ids the user closed; a new id makes the megaphone beat again. To correct a typo without bothering anyone again, keep the id. |
| `date` | yes | YYYY-MM-DD, shown in the window. Newest first. |
| `expires` | no | YYYY-MM-DD. From that day on it is not shown, so an old call does not reach new users. |
| `level` | no | `info` (default): the megaphone beats. `important`: it also opens by itself at start, and the megaphone turns amber. |
| `game` | no | A project id from `registry.json`. Then it is shown only to users who have that game installed, with the game's picture. |
| `label` | no | Short tag on the megaphone and the window ("Help wanted", "Testers wanted"). Default: "Announcement". |
| `title` | yes | One line. |
| `body` | yes | Lines of plain text. No HTML or Markdown: links go in `buttons`. |
| `buttons` | no | Up to two `{ label, url }`, opened in the web browser; the first is the main one. "Close" is always there. Only `https://` URLs are accepted. |

Unknown keys are ignored, so later versions can add some (a Spanish text,
say) without breaking launchers already installed. A file that cannot be read
shows no announcements; it never stops the launcher.

`examples/announcements.json` has two complete ones: a general "Help wanted"
and one for a single game.
