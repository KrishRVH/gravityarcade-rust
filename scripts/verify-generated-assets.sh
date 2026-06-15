#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

OUT_DIR="$ROOT/target/generated-verify"
mkdir -p "$OUT_DIR"

python3 reference/extract_announce_texts.py gravity_arcade.swf --out "$OUT_DIR/announce_texts.rs"
python3 reference/extract_ball_shapes.py gravity_arcade.swf --out "$OUT_DIR/ball_shapes.rs"
python3 reference/extract_button_shape22.py gravity_arcade.swf --out "$OUT_DIR/button_shape22.rs"
python3 reference/extract_chrome_texts.py gravity_arcade.swf --out "$OUT_DIR/chrome_texts.rs"
python3 reference/extract_counter_digit_texts.py gravity_arcade.swf --out "$OUT_DIR/counter_digit_texts.rs"
python3 reference/extract_goal_line_shapes.py gravity_arcade.swf --out "$OUT_DIR/goal_line_shapes.rs"
python3 reference/extract_gravity_preview_arrow_shape41.py gravity_arcade.swf --out "$OUT_DIR/gravity_preview_arrow_shape41.rs"
python3 reference/extract_help_control_shapes.py gravity_arcade.swf --out "$OUT_DIR/help_control_shapes.rs"
python3 reference/extract_ui_texts.py gravity_arcade.swf --module help-labels --out "$OUT_DIR/help_label_texts.rs"
python3 reference/extract_match_pip_shapes.py gravity_arcade.swf --out "$OUT_DIR/match_pip_shapes.rs"
python3 reference/extract_ui_texts.py gravity_arcade.swf --module menu-labels --out "$OUT_DIR/menu_label_texts.rs"
python3 reference/extract_ui_texts.py gravity_arcade.swf --module menu-values --out "$OUT_DIR/menu_value_texts.rs"
python3 reference/extract_paddle_body_shape89.py gravity_arcade.swf --out "$OUT_DIR/paddle_body_shape89.rs"
python3 reference/extract_paddle_glow_shape90.py gravity_arcade.swf --out "$OUT_DIR/paddle_glow_shape90.rs"
python3 reference/extract_paddle_ready_flash_shape127.py gravity_arcade.swf --out "$OUT_DIR/paddle_ready_flash_shape127.rs"
python3 reference/extract_panel_chrome_shapes.py gravity_arcade.swf --out "$OUT_DIR/panel_chrome_shapes.rs"
python3 reference/extract_placement_constants.py \
    gravity_arcade.swf \
    reference/swf_deep.json \
    --out "$OUT_DIR/placement_constants.rs"
python3 reference/extract_round_number_texts.py gravity_arcade.swf --out "$OUT_DIR/round_number_texts.rs"
python3 reference/extract_score_meter_constants.py \
    reference/swf_deep.json \
    --out "$OUT_DIR/score_meter_constants.rs"
python3 reference/extract_side_marker_shapes.py gravity_arcade.swf --out "$OUT_DIR/side_marker_shapes.rs"
python3 reference/extract_sponsor_logo_shape35.py gravity_arcade.swf --out "$OUT_DIR/sponsor_logo_shape35.rs"
python3 reference/extract_sponsor_logo_texts.py gravity_arcade.swf --out "$OUT_DIR/sponsor_logo_texts.rs"
python3 reference/extract_single_text_contours.py \
    gravity_arcade.swf \
    --out "$OUT_DIR/rounds_played_text77.rs" \
    --text-id 77 \
    --font-id 26 \
    --label "rounds played:" \
    --prefix RoundsPlayed \
    --header "// Generated from gravity_arcade.swf DefineText 77 and DefineFont 26."
python3 reference/extract_single_text_contours.py \
    gravity_arcade.swf \
    --out "$OUT_DIR/top_title_text81.rs" \
    --text-id 81 \
    --font-id 54 \
    --label Gravity \
    --prefix TopTitle \
    --header "// Generated from gravity_arcade.swf DefineText 81 and DefineFont 54."

rustfmt \
    "$OUT_DIR/announce_texts.rs" \
    "$OUT_DIR/ball_shapes.rs" \
    "$OUT_DIR/button_shape22.rs" \
    "$OUT_DIR/chrome_texts.rs" \
    "$OUT_DIR/counter_digit_texts.rs" \
    "$OUT_DIR/goal_line_shapes.rs" \
    "$OUT_DIR/gravity_preview_arrow_shape41.rs" \
    "$OUT_DIR/help_control_shapes.rs" \
    "$OUT_DIR/help_label_texts.rs" \
    "$OUT_DIR/match_pip_shapes.rs" \
    "$OUT_DIR/menu_label_texts.rs" \
    "$OUT_DIR/menu_value_texts.rs" \
    "$OUT_DIR/paddle_body_shape89.rs" \
    "$OUT_DIR/paddle_glow_shape90.rs" \
    "$OUT_DIR/paddle_ready_flash_shape127.rs" \
    "$OUT_DIR/panel_chrome_shapes.rs" \
    "$OUT_DIR/placement_constants.rs" \
    "$OUT_DIR/round_number_texts.rs" \
    "$OUT_DIR/rounds_played_text77.rs" \
    "$OUT_DIR/score_meter_constants.rs" \
    "$OUT_DIR/side_marker_shapes.rs" \
    "$OUT_DIR/sponsor_logo_shape35.rs" \
    "$OUT_DIR/sponsor_logo_texts.rs" \
    "$OUT_DIR/top_title_text81.rs"

cmp "$OUT_DIR/announce_texts.rs" src/announce_texts.rs
cmp "$OUT_DIR/ball_shapes.rs" src/ball_shapes.rs
cmp "$OUT_DIR/button_shape22.rs" src/button_shape22.rs
cmp "$OUT_DIR/chrome_texts.rs" src/chrome_texts.rs
cmp "$OUT_DIR/counter_digit_texts.rs" src/counter_digit_texts.rs
cmp "$OUT_DIR/goal_line_shapes.rs" src/goal_line_shapes.rs
cmp "$OUT_DIR/gravity_preview_arrow_shape41.rs" src/gravity_preview_arrow_shape41.rs
cmp "$OUT_DIR/help_control_shapes.rs" src/help_control_shapes.rs
cmp "$OUT_DIR/help_label_texts.rs" src/help_label_texts.rs
cmp "$OUT_DIR/match_pip_shapes.rs" src/match_pip_shapes.rs
cmp "$OUT_DIR/menu_label_texts.rs" src/menu_label_texts.rs
cmp "$OUT_DIR/menu_value_texts.rs" src/menu_value_texts.rs
cmp "$OUT_DIR/paddle_body_shape89.rs" src/paddle_body_shape89.rs
cmp "$OUT_DIR/paddle_glow_shape90.rs" src/paddle_glow_shape90.rs
cmp "$OUT_DIR/paddle_ready_flash_shape127.rs" src/paddle_ready_flash_shape127.rs
cmp "$OUT_DIR/panel_chrome_shapes.rs" src/panel_chrome_shapes.rs
cmp "$OUT_DIR/placement_constants.rs" src/placement_constants.rs
cmp "$OUT_DIR/round_number_texts.rs" src/round_number_texts.rs
cmp "$OUT_DIR/rounds_played_text77.rs" src/rounds_played_text77.rs
cmp "$OUT_DIR/score_meter_constants.rs" src/score_meter_constants.rs
cmp "$OUT_DIR/side_marker_shapes.rs" src/side_marker_shapes.rs
cmp "$OUT_DIR/sponsor_logo_shape35.rs" src/sponsor_logo_shape35.rs
cmp "$OUT_DIR/sponsor_logo_texts.rs" src/sponsor_logo_texts.rs
cmp "$OUT_DIR/top_title_text81.rs" src/top_title_text81.rs

echo "ok generated SWF assets match checked-in Rust modules"
