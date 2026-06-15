# Gravity Arcade Rust

Rust 2024 + macroquad rewrite of `gravity_arcade.swf`.

The port is built from the SWF itself plus `curveball-rust` as the project-structure reference:
the simulation core is headless and tested, while the binary owns macroquad rendering and input.

## Run

```bash
cargo run
```

This project is intentionally desktop-only for now; browser/wasm targets are rejected at compile
time.

Controls:

- Blue player, right side: `Up` / `Down`, fire with `Left`.
- Red player, left side: `W` / `S`, fire with `D`.
- Desktop presentation: `I` toggles `Interpolate`, which smooths continuous gameplay positions and
  drawable scale/alpha/color fields between the SWF-faithful 30 Hz logic ticks without changing
  simulation, scoring, sounds, or frame-timed animations.
- Menu, help, and back navigation use the on-screen buttons, matching the SWF.

## Verification

```bash
cargo fmt --check
cargo test
cargo build
cargo clippy --all-targets -- -D warnings
scripts/verify-generated-assets.sh
scripts/verify-sound-assets.sh
scripts/capture-native-smoke.sh
```

The desktop release gate is available as one command:

```bash
scripts/verify-desktop-release.sh
scripts/verify-desktop-release.sh --with-reference
```

The `--with-reference` variant also runs the Ruffle/SWF visual comparison and needs the same browser
and network access described below.

Optional Ruffle reference captures for visual comparison against the original SWF:

```bash
scripts/capture-reference-swf.sh
scripts/compare-reference-smoke.sh --capture
```

This writes menu, menu-setting variant, help, help-back return, idle first-playfield, and both
players' fired-ball playfield PNGs plus browser console/network logs to `target/reference-swf/`. It
needs the gstack `browse` binary and network access for the pinned Ruffle web player. The comparison
script captures both the original SWF-through-Ruffle and the native macroquad smoke frames, then
checks their image deltas against antialias-tolerant thresholds.
The startup/loading frame is covered by native smoke captures; the Ruffle reference harness reaches
the stopped menu before browser navigation returns, and its manual-play overlay is not suitable for
deterministic startup-frame capture.

Debug-only deterministic screenshots are available for visual parity checks:

```bash
GRAVITYARCADE_WARP=menu GRAVITYARCADE_SHOT=target/debug/gravityarcade-menu.png cargo run
GRAVITYARCADE_WARP=help GRAVITYARCADE_SHOT=target/debug/gravityarcade-help.png cargo run
```

`GRAVITYARCADE_WARP` accepts `startup`, `menu`, `menu_polarisation_opposite`,
`menu_polarisation_same`, `menu_polarisation_all`, `menu_matches_5`, `menu_matches_7`,
`menu_matches_1`, `menu_gravity_low`, `menu_gravity_high`, `menu_gravity_very_high`,
`menu_gravity_black_hole`, `menu_speed_fast`, `help`, `playing_idle`, `playing`, `playing_red`,
`score_ramps`, `score_max`, `round_intro_1`, `round_intro`, `blue_win`, `red_win`, `blue_final`,
and `red_final`.
Append `:ticks` to `GRAVITYARCADE_SHOT` to advance fixed SWF ticks before capture.
The smoke script captures the key native screens to `target/debug/visual-smoke/` and validates that
each PNG is a nonblank `550x400` SWF-stage image, including score-meter and sprite-162
announcement timeline states. The targeted score and announcement captures also assert expected
color-region evidence, so the gate fails if those states collapse to a blank or wrong-frame image.

## Reference Artifacts

- `DEVIATIONS.md`: current accepted port boundaries and verification notes.
- `gravity_arcade.swf`: original Flash 5 game, 550x400 at 30 FPS.
- `reference/extract_actions.py`: local SWF5/AS1 action extractor written for this port.
- `reference/extract_shapes.py`: local SWF shape-record extractor for auditing vector paths,
  shared fill boundaries, and optional reconstructed fill contours.
- `reference/extract_sounds.py`: extracts the original SWF MP3 `DefineSound` streams.
- `scripts/verify-sound-assets.sh`: extracts the original SWF MP3 `DefineSound` streams, converts
  them to native WAV assets, trims the SWF MP3 seek-sample delay, applies the SWF `SoundInfo`
  in/out/envelope records used by runtime cues, and compares them plus `reference/sounds.json` to
  the checked-in files.
- `scripts/verify-generated-assets.sh`: regenerates checked-in SWF contour/placement/score-meter
  modules from `gravity_arcade.swf` plus `reference/swf_deep.json`, formats them, and compares them
  to `src/`.
- `reference/actions.txt`: readable ActionScript bytecode dump for frame, clip, and button handlers.
- `reference/actions.json`: structured form of the same action dump.
- `reference/swf_deep.json`: visual tag, shape, text, and placement dump from the curveball-rust parser.
- `reference/sounds.json`: extracted sound manifest.
- `assets/sounds/`: runtime WAV assets used by macroquad's native decoder, regenerated from the
  original SWF sound streams plus SWF MP3 seek-sample and `SoundInfo` cue metadata.
- `assets/fonts/`: bundled fallbacks for SWF edit text with no embedded glyph outlines. At runtime
  the port first tries common installed `Arial`, `Times New Roman`, and `Trebuchet MS` font paths,
  then common Linux-compatible substitutes such as Liberation Sans/Serif, Carlito, and Caladea,
  before falling back to bundled OFL Liberation Sans/Serif.

Important SWF-derived behavior and runtime choices:

- World origin: `(273.45, 214.45)`.
- The macroquad window presents the SWF stage at `1650x1200` by default, using a camera transform so
  simulation, hit testing, and rendering still operate in the original `550x400` stage coordinates.
  The window is resizable; non-`11:8` sizes letterbox/pillarbox the stage instead of stretching it,
  and mouse input is remapped through the visible stage viewport. High-DPI rendering is enabled so
  Retina-style displays get a full-resolution backing framebuffer, and SWF-derived text/logo
  textures are rasterized at the default window scale to avoid blurry upscaling on standard-density
  displays. The runtime also requests display refresh pacing via miniquad `swap_interval = 1`,
  matching the curveball-rust macroquad shell pattern while the simulation itself remains fixed at
  the SWF's `30 Hz`.
- Ball bounds: `x = -235..235`, `y = -170..170`.
- Paddle bounds: `y = -150..150`, paddle half-height `30`.
- Active paddle clips follow sprite `140`'s initial placements: blue starts at local y `-0.25`,
  red at local y `-0.3`; sprite `132` then carries those `_y` values through movement, collision,
  and shot launch.
- Paddle movement follows the SWF `ty = y + v` clamp: crossing either paddle bound snaps to the
  bound and resets paddle velocity to `0`.
- Paddle collision uses the SWF's inclusive boundary check, so contact at exactly `30` px from the
  paddle center still reflects.
- Player input preserves sprite `132`'s key bindings: blue uses `Up`/`Down`/`Left`; red uses
  `W`/`S`/`D`. Menu and back navigation stay mouse-driven like the original button records.
- Startup keeps the local loading edit fields while assets initialize, then enters the frame-56 menu
  directly. The original SWF's `retrieving online data` CGI wait and offline abort branch are not
  present in the Rust runtime.
- Fixed gameplay tick: `30 Hz`; macroquad frame deltas are capped at `250 ms` before entering the
  fixed-step accumulator, matching the curveball-rust runtime shell and preventing window stalls from
  fast-forwarding through a large backlog of SWF ticks.
- The desktop-only `Interpolate` presentation toggle is enabled by default and can be switched with
  `I`. It draws paddle positions, paddle charge glow scale/tint, the ready overlay scale/alpha,
  active ball positions/scale, and scored-ball burst positions/scale/alpha between the
  previous/current fixed-tick snapshots at the display refresh cadence. The authoritative `World`
  still advances only on the SWF's `30 Hz` tick, and score meters, burn thresholds, stun flashes,
  sounds, match pips, menu/help frames, and sprite-162 announcements remain tick/frame exact.
  Debug screenshots force the current fixed-tick snapshot, and the `Interpolate` notice is
  transient, so visual regression captures stay comparable with Ruffle.
- Round intro gate: `60` ticks from the score-reset frame to the SWF `gunReady = true` frame.
  The visual `Round` + number display follows sprite 162's `newgame` timeline separately:
  it appears at frame `212`, is full-size/full-alpha from `225..255`, and fades/scales out
  through frame `267`, so it remains visible for `13` ticks after guns unlock. The round-start
  sound also starts at sprite 162 frame `212`, not when the menu button is pressed or scores reset.
- Gravity factors for menu levels 1-5: `10, 17, 50, 120, 210`.
- Speed modes: normal reflection `1.0`, fast reflection `1.15`; the SWF `Reflektieren()` branch
  returns clipped speeds directly at `ballMaxSpeed + 1` (`71`), so capped rebounds do not receive
  the fast-mode multiplier.
- Menu `rounds played` mirrors the root `gamesPlayed` variable; sprite 162's `cgiPusher()`
  increments it once for each blue/red match-win announce. The top `Gravity` title renders from
  `DefineText` 81 and font 54's recovered glyph contours; the `rounds played:` label renders from
  `DefineText` 77 and font 26's recovered glyph contours. The version line renders from
  `DefineText` 73 and font 26's recovered glyph contours. The dynamic counter field keeps the
  root-frame SWF placement and right-aligned `DefineEditText` behavior. Font `26` is named
  `Trebuchet MS` and embeds digits `0` through `8`; those digits render from recovered contours,
  while the non-embedded digit `9` alone uses installed `Trebuchet MS` when available, Linux
  desktop substitutes such as Carlito/Liberation Sans next, and bundled Liberation Sans as the final
  local fallback. The Rust runtime always shows the menu/help `rounds played` counter because the
  removed offline startup branch is no longer a reachable state. Frame 57 keeps depth 44's
  version/`:neokolor` link sprite on the help screen, while frame 58 removes it for gameplay.
- The menu top `g.factor` `DefineEditText` field displays the current gravity factor
  `10`/`17`/`50`/`120`/`210` at its root-frame placement. The SWF names font `71` as
  `Times New Roman` with zero embedded glyphs, so the port uses installed `Times New Roman` when
  available, Linux Liberation Serif/Caladea substitutes next, and bundled Liberation Serif through
  macroquad's explicit font path otherwise.
- Menu rendering follows root frame 56 depth groups: low-depth buttons, labels, previews, value
  fields, top gravity factor, version, and rounds-played text draw before the frame mask/title; goal
  lines, static paddles, glows, panel outline, side markers, left sponsor link, and sponsor logo then
  draw in their recovered high-depth order.
- Menu button captions and menu setting values render from recovered glyph contours for their
  `DefineText` records, including `single match`, gravity labels `low`/`medium`/`high`/`very high`/
  `black hole`, the red `enabled` SpeedGravity value, and the original `gravity strenght` typo.
  The help `back` button also renders from its recovered glyph contours.
- Menu buttons and the help-screen bottom `back` button use shared shape `22` bounds, fill,
  outline, rounded quadratic edge path, and hit shape regenerated into `src/button_shape22.rs`;
  their outlines scale from each `PlaceObject2` matrix, including frame 57's non-uniform help-back
  scale.
- The menu gravity-mode preview follows sprite `42` frames `2..5` for the neutral/opposite/same/all
  repel modes, including the row-specific ball offsets, signed arrow transforms, and color
  transforms. Preview balls use shape `2`'s recovered radial contour and fade (`#99ffff`,
  `#ffbe73`, `#ffbf77`, transparent at `230/255`), and arrows use shape `41`'s line path with color
  `#fffbcc`; the preview color transforms are regenerated into `src/placement_constants.rs`, while
  the arrow path is regenerated into `src/gravity_preview_arrow_shape41.rs` by
  `scripts/verify-generated-assets.sh`.
- Active ball rendering follows sprite 16's child shape layering with recovered shape contours and
  banded radial gradients:
  shape `2` uses cyan `#99ffff` through the red-owner transform `#ffbf77` and fades to transparent
  at ratio `230/255`; burning balls keep that owner glow and add shape `9` through the `#fffa11`
  fire transform fading at `250/255`. The ball owner/fire additive transforms are regenerated into
  `src/placement_constants.rs`. Static sprite-162 announcement labels render from recovered
  glyph contours: `Round` uses `#ffffff`, blue `Wins`/`Match` text uses `#88f7ff`, and red
  `Wins`/`Match` text uses `#ffb233`. Ball glow shape `2`, scored-ball die-ring shape `4`, and
  burning-fire shape `9` are regenerated into `src/ball_shapes.rs` by
  `scripts/verify-generated-assets.sh`. The round-number `DefineEditText` uses recovered embedded
  Georgia contours for digits `1`, `2`, and `3`; higher round numbers draw no substitute glyph
  because the SWF field has `UseOutlines` set and font `153` embeds no higher digit outlines.
- Help copy preserves the decoded SWF wording and frame-57 `DefineText` placements, including
  original typos such as `eatch` and `paddel`. The help title, multi-line body, bottom `back` label,
  control labels, player labels, and keycap letters render from recovered glyph contours, including
  the SWF's indented continuation lines. Help-screen keycap bases use shape `105`'s two-fill rounded
  path/outline and arrow glyphs use shape `110`'s filled path with the recovered frame-57 placement
  matrices, regenerated into `src/help_control_shapes.rs` by `scripts/verify-generated-assets.sh`.
  The help title/body, bottom back button, keycap bases, arrow glyphs, labels, and player captions
  are emitted at root frame 57 depths `2..26` before the retained frame-56 footer, frame mask, title,
  goal chrome, side markers, header back link, and sponsor logo. Root frame 57 retains
  depth `52`'s empty sprite `83`, but the visible `g.factor` edit text from menu depth `43` is
  removed before the help frame.
- Paddle firing uses the SWF `e._currentframe` charge clip directly: shots require frame `> 20`
  and the charge clip stops at frame `200`. Sprite 132 updates paddle velocity and checks the fire
  branch before assigning `ty` back to `_y`, so newly fired balls spawn at the pre-move paddle
  coordinate while using the just-updated paddle velocity for their launch angle. The attached ball
  initializes on sprite 16 frame `1` and first runs the `move` label on the following simulation
  tick.
- Score threshold: the meter clamps at `700`, but the SWF round-win branch is
  `int(score / 10) + 1 > 71`, so a round is won at `710`.
- Score meter display uses the SWF clip frame formula `int(score / 10) + 1`, clamped to frame `71`;
  scores below `10` remain on frame `1`. The drawn meter follows sprite `136`/`139` marker
  positions, per-segment scale ramps, color transforms, and the special frame-`71` max-meter
  sprite placement, including its center marker and 13-frame colored overlay outline cycle.
- The menu/help frame keeps shape `80`'s recovered `549.95x399.95` red fill/mask without inventing
  an outer-stage stroke, shape `37`'s central `#000066` panel plus darker `#000052` rounded side
  fills, the recovered shape `91` rounded white panel outline path, and static side-marker columns
  in root-frame display-list order, including duplicate endpoint entries and their fill contour
  identities. Shape `37`'s fills, shapes `80`/`91`'s outline paths, and shape `141`'s playfield
  mask corner contours are regenerated into
  `src/panel_chrome_shapes.rs` by `scripts/verify-generated-assets.sh`; active play follows root
  frame `58`, which uses borderless shape `141` at depth `46` for the red outside-panel mask,
  retains shape `80`'s rounded white panel line at depth `50`, removes the static
  side-marker/root-goal-paddle chrome, and places sprite `140` over the base panel. The retained top
  title then draws at depth `51`; depth `52`'s retained sprite `83` is visually empty, and the
  retained left `back to menu` link stays above the match pips at depth `127`.
- Active gameplay drawing follows the recovered sprite-`140`/root-frame depth stack: blue and red
  paddles at depths `5`/`8`, dynamic ball and death clips by their original `ballID` attach depth
  around the score meters at depths `13`/`42`, action-only sprite `38` at sprite-`140` depth `11`
  for pair processing, announcement sprite `162` at root depth `58`, and match-pip sprites at root
  depths `66`/`71`.
- Round wins call `clearPlayfield()`, which resets the active ball registry and `ballID` back to
  `10` immediately. Non-final match wins then hold the blue/red win announce for `96` ticks before
  the `newgame` frame resets scores and starts the next `60`-tick intro gate; the SWF does not reset
  paddle charge clips there, so charge continues to advance frame-by-frame. Final match wins use
  sprite 162's final labels instead of an overlay: blue plays frames `269..430` (`162` ticks) and red
  plays frames `432..588` (`157` ticks) before returning to the menu.
- Non-final match-win labels use sprite 162's recovered glyph contours and text placement: blue
  appears at frame `19`, holds through `71`, and fades through `96`; red grows in from frames
  `99..115`, holds through `168`, and fades through `193`.
- Final match-win labels use sprite 162's recovered five-part `Blue Wins` / `Red Wins` + `Match`
  contour layout and pips: blue grows on frames `269..284`, holds `285..367`, fades `368..405`,
  then leaves a blank tail until the frame `430` menu jump; red grows on `432..447`, holds
  `448..530`, fades `531..565`, then leaves a blank tail until the frame `588` menu jump.
- Match pip rendering uses the SWF `DefineShape` bounds, directed quadratic fill contours, and all
  recovered fill colors for blue shape `147` (`#8c8cb0`, `#00b8d1`, `#73e8ff`, `#00c5ed`,
  `#ccf9ff`, `#4ad4ff`) and red shape `151` (`#8c8cb0`, `#ffde73`, `#d19500`, `#edaa00`,
  `#ffcc4a`, `#fff1cc`), regenerated into `src/match_pip_shapes.rs` by
  `scripts/verify-generated-assets.sh`. The in-game HUD pip positions follow sprite `163`/`164`
  frame placements.
- Match-win sounds are emitted from sprite 162 `StartSound` frames rather than the score event:
  non-final blue/red wins play `round_lost` on frames `8` and `105`; final blue/red wins play only
  `blue_match_win`/`red_match_win` on frames `269` and `432`.
- Round-intro gun enabling follows sprite 162 frame `255` as an end-of-frame announce-clip effect:
  holding fire through that frame waits one more 30 Hz tick before a ball is attached.
- Gameplay sounds follow the original SWF sound sources: attached ball sprite 16 frame `1` starts
  sound `12`, reflect bounces use sprite 16's `Sound("reflect").start()` object for sound `1`,
  merge sprite 8 frame `2` starts sound `6`, goal-line flash sprites `86`/`88`/`125`/`126` frame
  `2` start sound `85`, and sprite 132 frame `4` starts sound `131` when a burning score sends the
  defender to `paralised`. Runtime WAV assets trim the SWF MP3 seek-sample delay and bake in the SWF
  `SoundInfo` in/out points and envelope levels for the merge, shot, rollover, paddle-stun,
  round-lost, and round-start cues.
- Button sounds follow `DefineButtonSound`: menu buttons and the help screen's bottom `back` button
  play sound `23` on rollover and sound `24` on mouse down. Visible
  SWF button/link actions follow their recovered `overDownToOverUp` handlers, so they activate only
  when the mouse is released over the same hit area that was pressed; the bottom `single match`
  start button also mirrors button `39`'s `overDownToOutDown` handler, starting the match if the
  pressed cursor is dragged out of its hit area. The help-screen bottom `back` button follows button
  `103`'s raw `GotoFrame 55; Play`, while header `back to menu` button `124` uses raw
  `GotoFrame 55` without `Play`; both target the frame-56 menu because `ActionGotoFrame` is
  zero-based, and the menu frame's `Stop` makes them direct menu returns. Header `back to menu`
  text links have no SWF button sound record. The OS cursor switches to the pointer hand over all
  SWF-clickable button/link hit areas, while rollover audio remains limited to the button-sound
  records.
- Burning balls: `ballE / 8 + 30 > 100`, so energy greater than `560` switches to the burning
  clip frame; frame-2 balls pierce paddles, score, and paralyze the defender.
- The playfield `_x / 50000` drift in sprite 16 frame 2 is gated by `f._currentframe == 2`; the
  bytecode reads the clip's old `_x` before assigning `tx`, so only burning balls receive that
  extra horizontal acceleration and it is based on the pre-move x coordinate.
- Ball visual scale uses the SWF `setEnergy()` formula `(ballE / 8 + 30)%`, capped at `100%`;
  the rendered core/glow/fire contours come from the ball clip's shape bounds `11.5`, `14`, and
  `5.5`.
- Scored balls are removed from the active `ballReg` physics registry immediately, but remain in a
  separate die-display list for the SWF sprite-15 11-frame expanding fade. The burst uses shape
  `4`'s recovered radial contour, ring stops (`154/255`, `212/255`, `255/255`), and the recovered
  frame scale/alpha transforms. That visual layer keeps the scored ball's post-goal velocity and
  follows the sprite-16 die-frame damping and wall-bounce rules while staying out of later gravity,
  merge, and collision scans; sprite-15 frame 3 also preserves frame 2's alpha transform because the
  move tag omits a replacement `CXFORM`.
- Paddle bodies use sprite/root shape `89` bounds `[-1.55, 1.9, -25.5, 25.5]`, fill `#ffff99`,
  rounded quadratic edge path, and a 1 px white outline regenerated into
  `src/paddle_body_shape89.rs` by `scripts/verify-generated-assets.sh`. Menu/help static root-frame
  paddle glows use shape `90` placements at depths `64`/`65`, scale `0.5314636x0.7529144`, and the
  SWF color transforms `#ddecff`/`#ffefbb`, regenerated into `src/placement_constants.rs`. Active
  player roots use sprite `140`'s depth-5/depth-8 y
  offsets, so the body, charge glow, collision center, and shot spawn all share the original clip `_y`.
  Paddle charge glow scale, tint, energy-clip offset, and frame-200 ready overlay come from the SWF
  sprite 128/129 200-frame tables, including carry-forward frames with no explicit placement tag. The
  glow body follows shape `90`'s recovered bounds, straight central fill span `-22.25..21.65`,
  vertical alpha stops `0/255`, `87/255`, `166/255`, `255/255`, and the two recovered semi-cap radial
  contours centered at `(0.15, -22.15)` and `(0.15, 21.7)` with peak stop `74/255`, regenerated into
  `src/paddle_glow_shape90.rs` by `scripts/verify-generated-assets.sh`; the ready overlay uses shape
  `127`'s filled path and fill `#666666@171`, regenerated into
  `src/paddle_ready_flash_shape127.rs`, with sprite 128/129's frame alpha applied separately.
- Goal lines use shape `84`/`87` beveled filled paths and fills `#ffa622`/`#00b3dd`, regenerated
  into `src/goal_line_shapes.rs` by `scripts/verify-generated-assets.sh`; menu/help draw the
  root-frame `35.05`/`515.0` placements, while active play resolves sprite `140`'s nested sprite
  `125`/`126` placements to `34.95`/`514.9`. Flashes use the SWF sprite 125/126 scale, translation,
  and color-transform tables, including the slight vertical scale/offset on the left flash.
- Empty side score markers use the root frame's black color-transformed marker fill paths and shape
  `93` outline path on menu/help screens; active gameplay omits those static root-frame columns and
  uses the score-meter sprites' frame-1 empty marker columns before drawing active fill paths from
  shape `94` (`#7fd4d4`) and shape `92` (`#ffab22`). Shapes `92`/`93`/`94` are regenerated into
  `src/side_marker_shapes.rs` by `scripts/verify-generated-assets.sh`. The clamped frame-`71`
  meter replaces those columns with the SWF final sprite's colored marker fills, center marker, and
  animated overlay outlines. The meter root positions, frame starts, scale/color ramps, final marker
  placements, and 13-frame overlay color cycles are regenerated into `src/score_meter_constants.rs`.
- Header links use SWF button hit polygons and recovered glyph contours for all three button text
  states: black up text, `#cccccc` rollover text, and white down text. The menu/help top-right
  `neodelight` logo renders sprite `100`'s depth-3 and depth-4 `DefineText` shadows from recovered
  font-32 glyph contours, then renders the depth-5 foreground from recovered shape `35` contours
  into supersampled macroquad textures; root frame 58 removes that depth-129 logo for active
  gameplay while retaining the left `back to menu` link. The SWF `GetURL` actions are mapped to the
  native system URL handler for the left header link, menu/help top-right sponsor logo, and retained
  menu/help bottom `:neokolor` version link, using recovered button hit shapes `30`/`74`/`98`/`123`
  regenerated into `src/placement_constants.rs`.
- Merged balls keep the first loop ball's color unless the partner has strictly greater energy;
  equal-energy merges do not take the partner color.
- Ball pair processing mirrors sprite 38 frame 2: the current ball keeps local `x/y/vx/vy/e`
  values while scanning later balls, merged partners are only spliced after the full pass, and
  clustered balls therefore preserve the SWF's delayed-`delBall` behavior.
- Burning-goal paralysis: `60` ticks; player sprite 132 enters the `paralised` label at frame 4
  and returns to its movement loop at frame 63. Its color transform cycles frame 4 red, frame 5
  yellow-green, frame 6 white, then repeats through frame 63. While paralysed, sprite 132 is not on
  its movement frame, so the shot-attach branch cannot run. The retained depth-2 charge child keeps
  advancing during the paralysed frames.
- The SWF typo sets `firebonusMod` but reads `firebonusMulti`; AS1/SWF5 undefined coercion makes the
  actual burn score bonus `0`.

To regenerate sound assets:

```bash
mkdir -p target/regenerated-sounds/mp3 target/regenerated-sounds/wav target/regenerated-sounds/runtime-wav
python3 reference/extract_sounds.py \
  gravity_arcade.swf \
  --out target/regenerated-sounds/mp3 \
  --manifest target/regenerated-sounds/mp3_manifest.json
for f in target/regenerated-sounds/mp3/*.mp3; do
  ffmpeg -y -hide_banner -loglevel error -i "$f" "target/regenerated-sounds/wav/$(basename "${f%.mp3}").wav"
done
python3 reference/apply_sound_info_to_wavs.py \
  gravity_arcade.swf \
  --raw-dir target/regenerated-sounds/wav \
  --out target/regenerated-sounds/runtime-wav
python3 reference/write_wav_sound_manifest.py \
  target/regenerated-sounds/mp3_manifest.json \
  --wav-dir target/regenerated-sounds/runtime-wav \
  --out target/regenerated-sounds/sounds.json
cp target/regenerated-sounds/runtime-wav/*.wav assets/sounds/
cp target/regenerated-sounds/sounds.json reference/sounds.json
scripts/verify-sound-assets.sh
```
