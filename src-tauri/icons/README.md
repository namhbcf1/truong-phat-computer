# System-surface icons

`public/truongphatcomputer.svg` is the approved vector source. Regenerate the compact
variants from the repository root with:

```sh
node scripts/generate-resident-icons.mjs
```

- `tray-template.png`: 36 px monochrome mask for macOS. Tauri/AppKit renders it
  at 18 points and supplies the menu-bar foreground color.
- `tray-color.png`: 64 px transparent color artwork for the system tray.
- `windows/icon.ico`: multiple native sizes for Windows taskbar, executable and
  installer icons, selected by `tauri.windows.conf.json`. The largest entry is
  first because Tauri embeds that entry for live windows; Windows can still
  choose from all sizes for executable and shortcut icons.

Color variants keep the full brand tile, so the monogram stays legible next to
round and square Windows icons. The macOS template drops the tile and the accent
bar, then flattens the monogram so the system can tint it.

Do not manually edit the derived PNG/ICO files; adjust `public/truongphatcomputer.svg` or
the documented framing in the generator.
