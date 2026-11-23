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
      "image": "art/announcements/2026-11-help-wanted.jpg",
      "body": [
        "A line is a paragraph.",
        "- A line that starts with \"- \" is an item of a list"
      ],
      "buttons": [
        { "label": "I can help", "url": "https://github.com/snicolast/NicoLauncher-Hub/issues/..." }
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
| `image` | no | A picture across the top of the window, in place of the megaphone (or of the game's picture and logo). See below. |
| `body` | yes | Lines of plain text. No HTML or Markdown: links go in `buttons`. |
| `buttons` | no | Up to two `{ label, url }`, opened in the web browser; the first is the main one. "Close" is always there. Only `https://` URLs are accepted. |
| `before_launcher` | no | A launcher version, such as `0.3.21`. Only launchers older than it show the announcement: for "update the launcher to use…". Launchers older than 0.3.21 do not know the key and show it too, which is what it is for. |

## Pictures

`image` is a path in this repository, such as
`art/announcements/2026-11-help-wanted.jpg`: put the picture in
`art/announcements/` in the same commit as the announcement. A full
`https://raw.githubusercontent.com/...` URL also works, for a picture already
in a game's repository (its `docs/img/`, say). Pictures from any other site
are left out: the launcher's page does not load them.

- The frame is 12:5, 620 pixels wide in the window. Make it **1200 × 500**
  (JPG, under about 300 KB) and it is shown whole; any other shape is cut to
  the frame from its middle.
- The top left and top right corners carry the label and the close button, and
  the bottom fades into the text: keep what matters in the middle.
- With `game`, the picture replaces the game's picture and logo; the game's
  name is still in the line under it.
- If the picture cannot be loaded (no connection, a wrong path), the window
  looks as it would without one. The picture is fetched as soon as the
  announcement arrives, so it is usually there when the window opens.
- Launchers older than this key ignore it and show the announcement without the
  picture.

Unknown keys are ignored, so later versions can add some (a Spanish text,
say) without breaking launchers already installed. A file that cannot be read
shows no announcements; it never stops the launcher.

`examples/announcements.json` has two complete ones: a general "Help wanted"
with a picture in this repository (a made-up path, to show its shape), and one
for a single game with a picture from the game's repository.
