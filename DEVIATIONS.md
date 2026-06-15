# Known Port Boundaries

This file tracks the remaining intentional boundaries of the desktop Macroquad port. The executable,
simulation, generated assets, sound assets, native smoke captures, and Ruffle reference comparisons
are verified by `scripts/verify-desktop-release.sh --with-reference`.

- Rendering uses recovered SWF vector contours for many in-game and root-frame chrome shapes, but
  this is still a targeted hand-coded renderer rather than a complete generic SWF vector/display-list
  renderer.
- Runtime presentation includes the desktop-only `Interpolate` toggle, enabled by default and
  switchable with `I`. It smooths continuous gameplay positions between the previous/current
  fixed-tick snapshots while leaving the authoritative `World`, collisions, scoring, sounds, score
  meters, charge clips, menu/help/startup frames, and sprite-162 announcements on the SWF-faithful
  30 Hz tick. Debug screenshots force the current fixed-tick snapshot for stable reference
  comparisons.
- The menu and instruction copy are recovered from decoded SWF text records, with placement checked
  against SWF frame data and Ruffle screenshots. The menu button captions, menu setting values, and
  static help-screen text now render from recovered glyph contours, as do the header links and
  version line. Static sprite-162 `Round`, `Blue Wins`, `Red Wins`, and `Match` labels also render
  from recovered glyph contours, and the round-number edit text uses recovered embedded contours for
  digits `1`, `2`, and `3`. The `rounds played` counter uses recovered font-26 digit contours for
  embedded digits `0` through `8`, even in mixed values such as `19`; only the non-embedded digit
  `9` uses SWF device-font text. The startup loading/XML-wait fields, menu `g.factor` field, and
  that counter `9` now prefer installed `Arial`, `Times New Roman`, and `Trebuchet MS` files, use
  common Linux desktop substitutes such as Liberation Sans/Serif, Carlito, and Caladea when those
  named Microsoft fonts are unavailable, and finally fall back to bundled Liberation Sans/Serif.
  Those fallback cases are still substitutes rather than exact recovered device-font glyph data.
- The checked-in generated text/contour modules for menu labels, menu values, help text, active
  ball/die/fire shapes `2`/`4`/`9`, shared button shape `22`, gravity-preview arrow shape `41`,
  panel chrome shapes `37`/`80`/`91`/`141`, match-pip shapes `147`/`151`, the goal-line shapes `84`/`87`,
  root/playfield paddle body shape `89`, paddle glow shape `90`, side/score marker shapes
  `92`/`93`/`94`, help-control shapes `105`/`110`, paddle ready-flash shape `127`, the `Gravity`
  title, the `rounds played:` label, header/version/footer chrome text, sprite-162 announcements,
  round-number glyphs, the rounds-played counter digits, sponsor-logo text, sponsor-logo shape
  `35`, score-meter placement/ramp/final-overlay tables, placement color transforms, and
  external-link hit shapes `30`/`74`/`98`/`123` are now reproducible from `gravity_arcade.swf` plus
  `reference/swf_deep.json` with `scripts/verify-generated-assets.sh`.
- Top chrome now includes the recovered `Gravity` title glyph contours, the `rounds played:` and
  version glyph contours, recovered header-link state glyph contours, and menu/help sprite `100`
  sponsor-logo text/shape contours, plus sprite-162 static announcement glyph contours. Round score
  meters still use targeted Rust drawing rather than a general display-list/vector/text renderer,
  but their SWF frame positions, scale ramps, color transforms, final-frame sprite placement, and
  overlay outline cycles are regenerated into `src/score_meter_constants.rs`; the native smoke gate
  now captures partial-ramp, max-meter overlay, round-intro, non-final win, and final-win
  announcement states with color-region assertions for regression evidence.
- The original online CGI logging/data endpoints remain intentionally inert in the Rust port; the
  startup screen exposes the recovered SWF offline abort branch rather than contacting them. That
  branch sets `offline = true` and executes raw `GotoFrame 55; Play`; because `ActionGotoFrame` is
  zero-based, it lands directly on the frame-56 menu whose `Stop` action prevents any extra logo
  transition.
- Native builds use WAV runtime assets because `quad-snd` decodes WAV/OGG locally.
  `scripts/verify-sound-assets.sh` now regenerates those WAVs from `gravity_arcade.swf`, applies the
  SWF `SoundInfo` in/out/envelope records for runtime cues, and verifies them byte-for-byte against
  the checked-in runtime assets.
