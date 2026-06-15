#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use gravityarcade::sim::{
    BALL_CORE_RADIUS, BALL_FIRE_RADIUS, BALL_GLOW_RADIUS, Controls, DyingBall, GravityStrength,
    Polarisation, ROUND_INTRO_VISUAL_TICKS, RoundEvent, SCORE_METER_MAX_FRAME, STAGE_H, STAGE_W,
    Settings, Side, SpeedMode, TICK_HZ, World, paddle_charge_visual, paddle_stun_color,
    score_meter_frame, world_to_stage,
};
use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound};
use macroquad::miniquad::{self, CursorIcon};
use macroquad::prelude::*;
use std::cell::OnceCell;

mod announce_texts;
mod ball_shapes;
mod button_shape22;
mod chrome_texts;
mod counter_digit_texts;
mod goal_line_shapes;
mod gravity_preview_arrow_shape41;
mod help_control_shapes;
mod help_label_texts;
mod match_pip_shapes;
mod menu_label_texts;
mod menu_value_texts;
mod paddle_body_shape89;
mod paddle_glow_shape90;
mod paddle_ready_flash_shape127;
mod panel_chrome_shapes;
mod placement_constants;
mod round_number_texts;
mod rounds_played_text77;
mod score_meter_constants;
mod side_marker_shapes;
mod sponsor_logo_shape35;
mod sponsor_logo_texts;
mod top_title_text81;

use ball_shapes::{BALL_DIE_RING_SHAPE, BALL_FIRE_SHAPE, BALL_GLOW_SHAPE};
use match_pip_shapes::{
    BLUE_MATCH_PIP_ACCENT, BLUE_MATCH_PIP_CONTOURS, MATCH_PIP_FLATTEN_STEPS,
    RED_MATCH_PIP_CONTOURS, RED_MATCH_PIP_TOP_SHINE,
};

const BG: Color = Color::new(0.0, 0.0, 0.20, 1.0);
const PANEL: Color = swf_rgb_array(panel_chrome_shapes::PANEL_FILL_RGB);
const PANEL_SHADOW: Color = swf_rgb_array(panel_chrome_shapes::PANEL_SHADOW_RGB);
const STAGE_RED: Color = swf_rgb_array(panel_chrome_shapes::MASK_FILL_RGB);
const PADDLE_GLOW_CENTER_X: f32 = 0.15;
const PADDLE_GLOW_TOP_CENTER_Y: f32 = -22.15;
const PADDLE_GLOW_BOTTOM_CENTER_Y: f32 = 21.7;
const PADDLE_GLOW_LINEAR_BANDS: u32 = 32;
const PADDLE_GLOW_RADIAL_BANDS: u32 = 18;
#[cfg(test)]
const STATIC_PADDLE_GLOW_ROOT_FRAME: u16 = 56;
#[cfg(test)]
const STATIC_PADDLE_GLOW_CHARACTER_ID: u16 = 90;
#[cfg(test)]
const STATIC_RIGHT_PADDLE_GLOW_DEPTH: u16 = 64;
#[cfg(test)]
const STATIC_LEFT_PADDLE_GLOW_DEPTH: u16 = 65;
const STATIC_PADDLE_GLOW_SCALE_X: f32 = placement_constants::STATIC_PADDLE_GLOW_SCALE_X;
const STATIC_PADDLE_GLOW_SCALE_Y: f32 = placement_constants::STATIC_PADDLE_GLOW_SCALE_Y;
const STATIC_RIGHT_PADDLE_GLOW_COLOR: Color =
    swf_rgb_array(placement_constants::STATIC_RIGHT_PADDLE_GLOW_ADD_RGB);
const STATIC_LEFT_PADDLE_GLOW_COLOR: Color =
    swf_rgb_array(placement_constants::STATIC_LEFT_PADDLE_GLOW_ADD_RGB);
const SWF_WHITE: Color = swf_rgb_array(panel_chrome_shapes::OUTLINE_RGB);
const HELP_KEYCAP_SCALE: f32 = 0.777_298;
const HELP_ARROW_SCALE: f32 = 0.587_722_8;
const HELP_ARROW_OFFSET_X: f32 = 0.7;
const HELP_ARROW_OFFSET_Y: f32 = -1.75;
#[cfg(test)]
const HELP_BLUE_TEXT: Color = Color::new(102.0 / 255.0, 197.0 / 255.0, 1.0, 1.0);
#[cfg(test)]
const HELP_RED_TEXT: Color = Color::new(1.0, 129.0 / 255.0, 34.0 / 255.0, 1.0);
const MAX_FRAME_TIME: f64 = 0.25;
const BALL_BLUE_CORE: Color = swf_gradient_stop_color(ball_shapes::BALL_GLOW_GRADIENT_RGBA[0]);
const BALL_RED_CORE: Color = swf_rgb_array(placement_constants::BALL_RED_ADD_RGB);
const BALL_FIRE_CORE: Color = swf_rgb_array(placement_constants::BALL_FIRE_ADD_RGB);
const BALL_GLOW_EDGE_STOP_RATIO: f32 =
    swf_gradient_stop_ratio(ball_shapes::BALL_GLOW_GRADIENT_RGBA[1]);
const BALL_FIRE_EDGE_STOP_RATIO: f32 =
    swf_gradient_stop_ratio(ball_shapes::BALL_FIRE_GRADIENT_RGBA[1]);
const RADIAL_FADE_STEPS: u32 = 18;
const RADIAL_RING_BANDS: u32 = 12;
const RADIAL_SHAPE_FLATTEN_STEPS: u8 = 4;
const BALL_DIE_RING_INNER_STOP_RATIO: f32 =
    swf_gradient_stop_ratio(ball_shapes::BALL_DIE_RING_GRADIENT_RGBA[0]);
const BALL_DIE_RING_PEAK_STOP_RATIO: f32 =
    swf_gradient_stop_ratio(ball_shapes::BALL_DIE_RING_GRADIENT_RGBA[1]);
const BALL_DIE_RING_OUTER_STOP_RATIO: f32 =
    swf_gradient_stop_ratio(ball_shapes::BALL_DIE_RING_GRADIENT_RGBA[2]);
const BALL_DIE_RING_PEAK: Color =
    swf_gradient_stop_color(ball_shapes::BALL_DIE_RING_GRADIENT_RGBA[1]);
const TEXT: Color = Color::new(0.92, 0.95, 1.0, 1.0);
const STARTUP_TEXT: Color = Color::new(1.0, 254.0 / 255.0, 238.0 / 255.0, 1.0);
const MUTED: Color = Color::new(0.55, 0.55, 0.69, 1.0);
#[cfg(test)]
const HEADER_LINK_UP: Color = swf_rgb_array(chrome_texts::SPONSOR_UP.color_rgb);
#[cfg(test)]
const HEADER_LINK_OVER: Color = swf_rgb_array(chrome_texts::SPONSOR_OVER.color_rgb);
#[cfg(test)]
const HEADER_LINK_DOWN: Color = swf_rgb_array(chrome_texts::SPONSOR_DOWN.color_rgb);
const SPONSOR_LOGO_DARK: Color = swf_rgb_array(sponsor_logo_texts::DARK.color_rgb);
const SPONSOR_LOGO_OLIVE: Color = swf_rgb_array(sponsor_logo_texts::OLIVE.color_rgb);
const SPONSOR_LOGO_FILL: Color = swf_rgb_array(sponsor_logo_shape35::FILL_RGB);
const SIDE_MARKER_EMPTY_FILL: Color = swf_rgb_array(side_marker_shapes::OUTLINE_RGB);
const SCORE_MARKER_BLUE: Color = swf_rgb_array(side_marker_shapes::BLUE_FILL_RGB);
const SCORE_MARKER_RED: Color = swf_rgb_array(side_marker_shapes::RED_FILL_RGB);
const GRAVITY_PREVIEW_BLUE: Color = BALL_BLUE_CORE;
const GRAVITY_PREVIEW_RED_LEFT: Color =
    swf_rgb_array(placement_constants::GRAVITY_PREVIEW_RED_LEFT_ADD_RGB);
const GRAVITY_PREVIEW_RED_RIGHT: Color =
    swf_rgb_array(placement_constants::GRAVITY_PREVIEW_RED_RIGHT_ADD_RGB);
const GRAVITY_PREVIEW_ARROW: Color = swf_rgb_array(gravity_preview_arrow_shape41::LINE_RGB);
const MATCH_PIP_RADIUS: f32 = 11.15;
const MATCH_PIP_OUTER: Color = swf_rgb_array(match_pip_shapes::BLUE_FILL_RGB[0]);
const MATCH_PIP_BLUE_CORE: Color = swf_rgb_array(match_pip_shapes::BLUE_FILL_RGB[1]);
const MATCH_PIP_BLUE_HIGHLIGHT: Color = swf_rgb_array(match_pip_shapes::BLUE_FILL_RGB[2]);
const MATCH_PIP_BLUE_MID: Color = swf_rgb_array(match_pip_shapes::BLUE_FILL_RGB[3]);
const MATCH_PIP_BLUE_SHINE: Color = swf_rgb_array(match_pip_shapes::BLUE_FILL_RGB[4]);
const MATCH_PIP_BLUE_ACCENT: Color = swf_rgb_array(match_pip_shapes::BLUE_FILL_RGB[5]);
const MATCH_PIP_RED_HIGHLIGHT: Color = swf_rgb_array(match_pip_shapes::RED_FILL_RGB[1]);
const MATCH_PIP_RED_CORE: Color = swf_rgb_array(match_pip_shapes::RED_FILL_RGB[2]);
const MATCH_PIP_RED_MID: Color = swf_rgb_array(match_pip_shapes::RED_FILL_RGB[3]);
const MATCH_PIP_RED_ACCENT: Color = swf_rgb_array(match_pip_shapes::RED_FILL_RGB[4]);
const MATCH_PIP_RED_SHINE: Color = swf_rgb_array(match_pip_shapes::RED_FILL_RGB[5]);
const BLUE_UP_KEY: KeyCode = KeyCode::Up;
const BLUE_DOWN_KEY: KeyCode = KeyCode::Down;
const BLUE_FIRE_KEY: KeyCode = KeyCode::Left;
const RED_UP_KEY: KeyCode = KeyCode::W;
const RED_DOWN_KEY: KeyCode = KeyCode::S;
const RED_FIRE_KEY: KeyCode = KeyCode::D;
const INTERPOLATE_TOGGLE_KEY: KeyCode = KeyCode::I;
const INTERPOLATE_NOTICE_FRAMES: u8 = 60;
const MENU_BUTTON_SCALE: f32 = 0.671_630_86;
const HELP_BACK_BUTTON_SCALE_X: f32 = 0.809_463_5;
const HELP_BACK_BUTTON_SCALE_Y: f32 = 0.816_665_65;
const GRAVITY_PREVIEW_ROOT_X: f32 = 254.75;
const GRAVITY_PREVIEW_ROOT_Y: f32 = 139.85;
const GRAVITY_PREVIEW_SPRITE_SCALE: f32 = 0.422_256_47;
const GRAVITY_PREVIEW_NEGATIVE_SCALE: f32 = -GRAVITY_PREVIEW_SPRITE_SCALE;
const PANEL_CENTER_BOUNDS: SwfBounds = panel_chrome_shapes::PANEL_CENTER_BOUNDS;
const GOAL_LINE_CENTER_Y: f32 = 214.45;
#[cfg(test)]
const SIDE_MARKER_YS: [f32; 13] = [
    50.05, 77.45, 104.85, 132.25, 159.65, 187.05, 214.45, 241.85, 269.25, 296.65, 324.05, 351.45,
    378.85,
];
const STATIC_LEFT_SIDE_MARKER_X: f32 = 17.5;
const STATIC_RIGHT_SIDE_MARKER_X: f32 = 532.45;
#[cfg(test)]
const PLAYFIELD_ROOT_FRAME: u16 = 58;
#[cfg(test)]
const PLAYFIELD_SPRITE_ID: u16 = 140;
#[cfg(test)]
const PLAYFIELD_SPRITE_DEPTH: u16 = 3;
#[cfg(test)]
const PLAYFIELD_LEFT_GOAL_DEPTH: u16 = 1;
#[cfg(test)]
const PLAYFIELD_RIGHT_GOAL_DEPTH: u16 = 3;
#[cfg(test)]
const PLAYFIELD_LEFT_GOAL_CHARACTER_ID: u16 = 125;
#[cfg(test)]
const PLAYFIELD_RIGHT_GOAL_CHARACTER_ID: u16 = 126;
#[cfg(test)]
const PLAYFIELD_PAIR_CONTROLLER_DEPTH: u32 = 11;
#[cfg(test)]
const PLAYFIELD_PAIR_CONTROLLER_CHARACTER_ID: u16 = 38;
#[cfg(test)]
const PLAYFIELD_ROOT_MASK_DEPTH: u32 = 46;
#[cfg(test)]
const PLAYFIELD_ROOT_MASK_CHARACTER_ID: u16 = 141;
#[cfg(test)]
const PLAYFIELD_RETAINED_FRAME_MASK_DEPTH: u32 = 50;
#[cfg(test)]
const PLAYFIELD_RETAINED_FRAME_MASK_CHARACTER_ID: u16 = 80;
#[cfg(test)]
const PLAYFIELD_TOP_TITLE_DEPTH: u32 = 51;
#[cfg(test)]
const PLAYFIELD_RETAINED_EMPTY_FACTOR_SPRITE_DEPTH: u32 = 52;
#[cfg(test)]
const PLAYFIELD_RETAINED_EMPTY_FACTOR_SPRITE_ID: u16 = 83;
#[cfg(test)]
const PLAYFIELD_BLUE_PLAYER_DEPTH: u32 = 5;
#[cfg(test)]
const PLAYFIELD_RED_PLAYER_DEPTH: u32 = 8;
const PLAYFIELD_FIRST_DYNAMIC_BALL_DEPTH: u32 = 10;
const PLAYFIELD_BLUE_SCORE_METER_DEPTH: u32 = 13;
const PLAYFIELD_RED_SCORE_METER_DEPTH: u32 = 42;
#[cfg(test)]
const PLAYFIELD_ANNOUNCE_DEPTH: u32 = 58;
#[cfg(test)]
const PLAYFIELD_BLUE_MATCH_PIPS_DEPTH: u32 = 66;
#[cfg(test)]
const PLAYFIELD_RED_MATCH_PIPS_DEPTH: u32 = 71;
const MENU_LEFT_GOAL_X: f32 = 35.05;
const MENU_RIGHT_GOAL_X: f32 = 515.0;
const PLAYFIELD_ROOT_X: f32 = 273.45;
const PLAYFIELD_LEFT_GOAL_LOCAL_X: f32 = -238.5;
const PLAYFIELD_RIGHT_GOAL_LOCAL_X: f32 = 241.45;
const PLAYFIELD_LEFT_GOAL_X: f32 = PLAYFIELD_ROOT_X + PLAYFIELD_LEFT_GOAL_LOCAL_X;
const PLAYFIELD_RIGHT_GOAL_X: f32 = PLAYFIELD_ROOT_X + PLAYFIELD_RIGHT_GOAL_LOCAL_X;
const STAGE_BOUNDS: SwfBounds = panel_chrome_shapes::PLAYFIELD_MASK_SHAPE.bounds;
#[derive(Clone, Copy)]
struct StageFrameOptions {
    draw_goal_paddles: bool,
    draw_panel_outline: bool,
    draw_static_side_markers: bool,
    left_goal_x: f32,
    right_goal_x: f32,
}

const MENU_STAGE_FRAME: StageFrameOptions = StageFrameOptions {
    draw_goal_paddles: true,
    draw_panel_outline: true,
    draw_static_side_markers: true,
    left_goal_x: MENU_LEFT_GOAL_X,
    right_goal_x: MENU_RIGHT_GOAL_X,
};
const PLAYFIELD_STAGE_FRAME: StageFrameOptions = StageFrameOptions {
    draw_goal_paddles: false,
    draw_panel_outline: false,
    draw_static_side_markers: false,
    left_goal_x: PLAYFIELD_LEFT_GOAL_X,
    right_goal_x: PLAYFIELD_RIGHT_GOAL_X,
};
const ANNOUNCE_ROOT_X: f32 = 283.05;
const ANNOUNCE_ROOT_Y: f32 = 133.9;
const BLUE_WIN_TEXT_BOUNDS_CENTER_X: f32 = (5.75 + 62.15) * 0.5;
const RED_WIN_TEXT_BOUNDS_CENTER_X: f32 = (7.4 + 60.55) * 0.5;
const MATCH_TEXT_BOUNDS_CENTER_X: f32 = (14.95 + 52.15) * 0.5;
const ROUND_TEXT_BOUNDS_CENTER_X: f32 = (56.55 + 93.6) * 0.5;
const ROUND_NUMBER_BOUNDS_CENTER_X: f32 = (-2.0 + 50.15) * 0.5;
const WIN_TEXT_BASELINE_Y: f32 = 11.8;
const ROUND_NUMBER_BASELINE_Y: f32 = 12.0;
const BLUE_WIN_TEXT_TX: f32 = -178.95;
const BLUE_WIN_TEXT_TY: f32 = -38.05;
const WIN_TEXT_SETTLED_SCALE: f32 = 5.619_278;
const FINAL_WIN_TEXT_TY: f32 = -70.05;
const FINAL_MATCH_TEXT_SCALE: f32 = 8.593_536;
const FINAL_MATCH_TEXT_TX: f32 = -270.85;
const FINAL_MATCH_TEXT_TY: f32 = 127.4;
const FINAL_PIP_SETTLED_SCALE: f32 = 4.233_169_6;
const FINAL_PIP_TY: f32 = 73.35;
const FINAL_PIP_TXS: [f32; 3] = [-125.75, -5.3, 115.15];
const FINAL_GROW_WIN_TEXT_TX: f32 = -173.65;
const FINAL_GROW_WIN_TEXT_TY: f32 = -119.6;
const FINAL_GROW_MATCH_TEXT_TX: f32 = -265.55;
const FINAL_GROW_MATCH_TEXT_TY: f32 = 77.85;
const FINAL_GROW_PIP_TY: f32 = 23.8;
const FINAL_GROW_PIP_TXS: [f32; 3] = [-120.45, 0.0, 120.45];
const ROUND_TEXT_INNER_SCALE: f32 = 3.730_774;
const ROUND_TEXT_INNER_TX: f32 = -268.6;
const ROUND_TEXT_INNER_TY: f32 = -79.3;
const ROUND_NUMBER_INNER_SCALE: f32 = 7.398_254_4;
const ROUND_NUMBER_INNER_TX: f32 = -178.1;
const ROUND_NUMBER_INNER_TY: f32 = -47.9;
const ANNOUNCE_TEXT_FLATTEN_STEPS: u8 = 4;
const ANNOUNCE_TEXT_TEXTURE_SCALE: u16 = 8;
const ANNOUNCE_TEXT_TEXTURE_SUPERSAMPLE: u16 = 2;
const WIN_TEXT_FADE_ALPHA: [u16; 25] = [
    246, 236, 225, 215, 205, 195, 184, 174, 164, 154, 143, 133, 123, 113, 102, 92, 82, 72, 61, 51,
    41, 31, 20, 10, 0,
];
const FINAL_GROW_TRANSFORMS: [WinTextTransform; 16] = [
    WinTextTransform::new(0.340_408_33, 0.0, 5.05, 0),
    WinTextTransform::new(0.352_645_87, -0.1, 5.9, 5),
    WinTextTransform::new(0.368_743_9, -0.25, 6.9, 11),
    WinTextTransform::new(0.388_717_65, -0.4, 8.3, 19),
    WinTextTransform::new(0.412_551_88, -0.6, 9.85, 28),
    WinTextTransform::new(0.440_246_58, -0.8, 11.8, 39),
    WinTextTransform::new(0.471_817, -1.05, 13.9, 51),
    WinTextTransform::new(0.507_232_67, -1.35, 16.25, 65),
    WinTextTransform::new(0.546_524_05, -1.65, 18.95, 80),
    WinTextTransform::new(0.589_691_16, -2.0, 21.85, 97),
    WinTextTransform::new(0.636_703_5, -2.4, 25.05, 115),
    WinTextTransform::new(0.687_591_55, -2.8, 28.45, 135),
    WinTextTransform::new(0.742_340_1, -3.25, 32.15, 156),
    WinTextTransform::new(0.800_964_36, -3.7, 36.1, 179),
    WinTextTransform::new(0.863_449_1, -4.2, 40.35, 203),
    WinTextTransform::new(0.929_794_3, -4.75, 44.8, 229),
];
const BLUE_FINAL_FADE_ALPHA: [u16; 38] = [
    249, 243, 236, 229, 222, 216, 209, 202, 195, 189, 182, 175, 168, 162, 155, 148, 141, 135, 128,
    121, 115, 108, 101, 94, 88, 81, 74, 67, 61, 54, 47, 40, 34, 27, 20, 13, 7, 0,
];
const RED_FINAL_FADE_ALPHA: [u16; 35] = [
    249, 241, 234, 227, 219, 212, 205, 197, 190, 183, 176, 168, 161, 154, 146, 139, 132, 124, 117,
    110, 102, 95, 88, 80, 73, 66, 59, 51, 44, 37, 29, 22, 15, 7, 0,
];
const ROUND_INTRO_GROW_TRANSFORMS: [WinTextTransform; 14] = [
    WinTextTransform::new(0.446_685_8, 0.0, 66.4, 0),
    WinTextTransform::new(0.489_242_55, 0.0, 67.4, 20),
    WinTextTransform::new(0.531_814_6, 0.0, 68.4, 39),
    WinTextTransform::new(0.574_371_34, 0.0, 69.4, 59),
    WinTextTransform::new(0.616_943_36, 0.0, 70.4, 79),
    WinTextTransform::new(0.659_500_1, 0.0, 71.4, 98),
    WinTextTransform::new(0.702_056_9, 0.0, 72.4, 118),
    WinTextTransform::new(0.744_628_9, 0.0, 73.4, 138),
    WinTextTransform::new(0.787_185_67, 0.0, 74.4, 158),
    WinTextTransform::new(0.829_742_43, 0.0, 75.4, 177),
    WinTextTransform::new(0.872_314_45, 0.0, 76.4, 197),
    WinTextTransform::new(0.914_871_2, 0.0, 77.4, 217),
    WinTextTransform::new(0.957_443_24, 0.0, 78.4, 236),
    WinTextTransform::new(1.0, 0.0, 79.4, 256),
];
const ROUND_INTRO_FADE_TRANSFORMS: [WinTextTransform; 12] = [
    WinTextTransform::new(1.090_713_5, 0.0, 82.15, 235),
    WinTextTransform::new(1.181_442_3, 0.0, 84.9, 213),
    WinTextTransform::new(1.272_171, 0.0, 87.65, 192),
    WinTextTransform::new(1.362_884_5, 0.0, 90.4, 171),
    WinTextTransform::new(1.453_613_3, 0.0, 93.15, 149),
    WinTextTransform::new(1.544_326_8, 0.0, 95.9, 128),
    WinTextTransform::new(1.635_040_3, 0.0, 98.65, 107),
    WinTextTransform::new(1.725_769, 0.0, 101.4, 85),
    WinTextTransform::new(1.816_497_8, 0.0, 104.15, 64),
    WinTextTransform::new(1.907_211_3, 0.0, 106.9, 43),
    WinTextTransform::new(1.997_940_1, 0.0, 109.65, 21),
    WinTextTransform::new(2.088_653_6, 0.0, 112.4, 0),
];
const RED_WIN_GROW_TRANSFORMS: [WinTextTransform; 17] = [
    WinTextTransform::new(1.912_841_8, -59.1, -24.75, 0),
    WinTextTransform::new(1.981_613_2, -61.35, -25.0, 5),
    WinTextTransform::new(2.072_067_3, -64.3, -25.4, 11),
    WinTextTransform::new(2.184_311, -67.9, -25.75, 19),
    WinTextTransform::new(2.318_237_3, -72.25, -26.3, 28),
    WinTextTransform::new(2.473_861_7, -77.25, -26.75, 39),
    WinTextTransform::new(2.651_275_6, -83.0, -27.45, 51),
    WinTextTransform::new(2.850_280_8, -89.45, -28.2, 65),
    WinTextTransform::new(3.071_075_4, -96.55, -28.95, 80),
    WinTextTransform::new(3.313_644_4, -104.4, -29.8, 97),
    WinTextTransform::new(3.577_819_8, -112.95, -30.75, 115),
    WinTextTransform::new(3.863_769_5, -122.2, -31.8, 135),
    WinTextTransform::new(4.171_417, -132.15, -32.9, 156),
    WinTextTransform::new(4.500_839, -142.8, -34.05, 179),
    WinTextTransform::new(4.851_959, -154.15, -35.3, 203),
    WinTextTransform::new(5.224_777, -166.2, -36.65, 229),
    WinTextTransform::new(
        WIN_TEXT_SETTLED_SCALE,
        BLUE_WIN_TEXT_TX,
        BLUE_WIN_TEXT_TY,
        256,
    ),
];

#[derive(Debug, Clone, Copy)]
struct SwfBounds {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RectVisual {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LogoRootTransform {
    tx: f32,
    ty: f32,
    scale: f32,
}

impl LogoRootTransform {
    const fn new(tx: f32, ty: f32, scale: f32) -> Self {
        Self { tx, ty, scale }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SwfPoint {
    x: f32,
    y: f32,
}

impl SwfPoint {
    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct KeycapVisual {
    bounds: RectVisual,
    center: SwfPoint,
    scale: f32,
    line_width: f32,
    fill: Color,
    shadow: Color,
    outline: Color,
}

#[derive(Debug, Clone, Copy)]
struct HelpControlContour {
    start: SwfPoint,
    segments: &'static [SwfPathSegment],
}

#[derive(Debug, Clone, Copy)]
struct HelpKeycapShapeDefinition {
    bounds: SwfBounds,
    fill: HelpControlContour,
    shadow: HelpControlContour,
    top_stroke: HelpControlContour,
    inner_stroke: HelpControlContour,
    outer_stroke: HelpControlContour,
}

#[derive(Debug, Clone, Copy)]
struct HelpArrowShapeDefinition {
    #[allow(
        dead_code,
        reason = "Shape 110 bounds are generated for SWF drift tests; arrow placement uses the recovered PlaceObject matrix"
    )]
    bounds: SwfBounds,
    fill: HelpControlContour,
}

#[derive(Debug, Clone, Copy)]
struct GoalLineShapeDefinition {
    bounds: SwfBounds,
    start: SwfPoint,
    segments: &'static [SwfPathSegment],
}

#[derive(Debug, Clone, Copy)]
struct GravityPreviewArrowPolyline {
    start: SwfPoint,
    points: &'static [SwfPoint],
}

#[derive(Debug, Clone, Copy)]
struct GravityPreviewArrowShapeDefinition {
    #[allow(
        dead_code,
        reason = "Shape 41 bounds are generated for SWF drift tests; arrow rendering uses the recovered line paths"
    )]
    bounds: SwfBounds,
    head: GravityPreviewArrowPolyline,
    shaft: GravityPreviewArrowPolyline,
}

#[derive(Debug, Clone, Copy)]
struct SideMarkerContour {
    start: SwfPoint,
    segments: &'static [SwfPathSegment],
}

#[derive(Debug, Clone, Copy)]
struct SideMarkerShapeDefinition {
    bounds: SwfBounds,
    contour: SideMarkerContour,
}

#[derive(Debug, Clone, Copy)]
struct ButtonShapeVisual {
    rect: Rect,
    center: SwfPoint,
    scale_x: f32,
    scale_y: f32,
    line_width: f32,
    fill: Color,
    outline: Color,
}

#[derive(Debug, Clone, Copy)]
struct ButtonShapeDefinition {
    bounds: SwfBounds,
    start: SwfPoint,
    segments: &'static [SwfPathSegment],
}

#[derive(Debug, Clone, Copy)]
struct PaddleBodyShapeDefinition {
    bounds: SwfBounds,
    start: SwfPoint,
    segments: &'static [SwfPathSegment],
}

#[derive(Debug, Clone, Copy)]
struct PaddleGlowContour {
    start: SwfPoint,
    segments: &'static [SwfPathSegment],
}

#[derive(Debug, Clone, Copy)]
struct PaddleGlowShapeDefinition {
    bounds: SwfBounds,
    body_bounds: SwfBounds,
    #[allow(
        dead_code,
        reason = "Shape 90's body contour is generated for SWF drift tests; rendering uses its equivalent bounds for the banded gradient fill"
    )]
    body: PaddleGlowContour,
    top_cap: PaddleGlowContour,
    bottom_cap: PaddleGlowContour,
}

#[derive(Debug, Clone, Copy)]
struct PaddleReadyFlashShapeDefinition {
    bounds: SwfBounds,
    start: SwfPoint,
    segments: &'static [SwfPathSegment],
}

#[derive(Debug, Clone, Copy)]
struct PanelChromeContour {
    start: SwfPoint,
    segments: &'static [SwfPathSegment],
}

#[derive(Debug, Clone, Copy)]
struct PanelChromeFillShapeDefinition {
    #[allow(
        dead_code,
        reason = "Shape 37 bounds are generated for SWF drift tests; panel rendering uses its child fill contours"
    )]
    bounds: SwfBounds,
    center: PanelChromeContour,
    left_shadow: PanelChromeContour,
    right_shadow: PanelChromeContour,
}

#[derive(Debug, Clone, Copy)]
struct PanelChromeLineShapeDefinition {
    bounds: SwfBounds,
    primary: PanelChromeContour,
    lower_right: PanelChromeContour,
}

#[derive(Debug, Clone, Copy)]
struct PanelChromeMaskShapeDefinition {
    bounds: SwfBounds,
    top_left: PanelChromeContour,
    top_right: PanelChromeContour,
    bottom_left: PanelChromeContour,
    bottom_right: PanelChromeContour,
}

#[derive(Debug, Clone, Copy)]
struct PaddleBodyVisual {
    center: SwfPoint,
    line_width: f32,
    fill: Color,
    outline: Color,
}

#[derive(Debug, Clone, Copy)]
struct StaticPaddleGlowVisual {
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
    color: Color,
}

#[derive(Debug, Clone, PartialEq)]
struct MarkerGlyphVisual {
    fill_points: Vec<SwfPoint>,
    fill_center: SwfPoint,
    fill_color: Color,
    outline_points: Vec<SwfPoint>,
    outline_line_width: f32,
    outline: Color,
}

#[derive(Debug, Clone, PartialEq)]
struct ArrowGlyphVisual {
    points: Vec<SwfPoint>,
    center: SwfPoint,
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SwfAffineTransform {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    tx: f32,
    ty: f32,
}

impl SwfAffineTransform {
    const fn new(a: f32, b: f32, c: f32, d: f32, tx: f32, ty: f32) -> Self {
        Self { a, b, c, d, tx, ty }
    }

    fn transform_point(self, local: SwfPoint) -> SwfPoint {
        SwfPoint::new(
            self.b.mul_add(local.y, self.a.mul_add(local.x, self.tx)),
            self.d.mul_add(local.y, self.c.mul_add(local.x, self.ty)),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SwfTextPlacement {
    root_tx: f32,
    root_ty: f32,
    root_sx: f32,
    root_sy: f32,
    local_tx: f32,
    local_ty: f32,
    bounds_x_min: f32,
    bounds_x_max: f32,
    run_y: f32,
    run_height: f32,
}

impl SwfTextPlacement {
    const fn new(
        root: (f32, f32),
        scale: (f32, f32),
        local: (f32, f32),
        bounds_x: (f32, f32),
        run: (f32, f32),
    ) -> Self {
        Self {
            root_tx: root.0,
            root_ty: root.1,
            root_sx: scale.0,
            root_sy: scale.1,
            local_tx: local.0,
            local_ty: local.1,
            bounds_x_min: bounds_x.0,
            bounds_x_max: bounds_x.1,
            run_y: run.0,
            run_height: run.1,
        }
    }

    #[cfg(test)]
    fn visual(self) -> SwfTextVisual {
        SwfTextVisual {
            x: self.root_sx.mul_add(
                (self.bounds_x_min + self.bounds_x_max).mul_add(0.5, self.local_tx),
                self.root_tx,
            ),
            baseline_y: self
                .root_sy
                .mul_add(self.local_ty + self.run_y, self.root_ty),
            font_size: (self.run_height * self.root_sy.abs()).round().max(1.0) as u16,
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct SwfTextVisual {
    x: f32,
    baseline_y: f32,
    font_size: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SwfLineVisual {
    x: f32,
    baseline_y: f32,
    font_size: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SwfCenterTextVisual {
    center_x: f32,
    baseline_y: f32,
    font_size: u16,
}

struct SponsorLogoTexture {
    texture: Texture2D,
    placement: RectVisual,
}

struct SponsorLogoTextTextures {
    dark: SwfTextTexture,
    olive: SwfTextTexture,
}

struct SponsorLogoAssets {
    text_layers: SponsorLogoTextTextures,
    foreground: SponsorLogoTexture,
}

struct SwfTextTexture {
    texture: Texture2D,
    placement: RectVisual,
}

struct MenuLabelTextures {
    start: SwfTextTexture,
    help: SwfTextTexture,
    polarisation: SwfTextTexture,
    matches: SwfTextTexture,
    gravity: SwfTextTexture,
    speed: SwfTextTexture,
}

struct MenuValueTextures {
    question: SwfTextTexture,
    match_single: SwfTextTexture,
    match_best_of_3: SwfTextTexture,
    match_best_of_5: SwfTextTexture,
    match_best_of_7: SwfTextTexture,
    gravity_low: SwfTextTexture,
    gravity_medium: SwfTextTexture,
    gravity_high: SwfTextTexture,
    gravity_very_high: SwfTextTexture,
    gravity_black_hole: SwfTextTexture,
    speed_disabled: SwfTextTexture,
    speed_enabled: SwfTextTexture,
}

struct HelpLabelTextures {
    title: SwfTextTexture,
    body: SwfTextTexture,
    back: SwfTextTexture,
    key_w: SwfTextTexture,
    key_d: SwfTextTexture,
    key_s: SwfTextTexture,
    red_move_up: SwfTextTexture,
    red_move_down: SwfTextTexture,
    red_shoot: SwfTextTexture,
    blue_shoot: SwfTextTexture,
    blue_move_up: SwfTextTexture,
    blue_move_down: SwfTextTexture,
    player_red: SwfTextTexture,
    player_blue: SwfTextTexture,
}

struct HeaderLinkTextureSet {
    up: SwfTextTexture,
    over: SwfTextTexture,
    down: SwfTextTexture,
}

struct ChromeTextTextures {
    sponsor_link: HeaderLinkTextureSet,
    back_link: HeaderLinkTextureSet,
    version: SwfTextTexture,
}

struct AnnounceTextTexture {
    texture: Texture2D,
    local_bounds: RectVisual,
}

struct AnnounceTextTextures {
    blue_wins: AnnounceTextTexture,
    red_wins: AnnounceTextTexture,
    round: AnnounceTextTexture,
    blue_match: AnnounceTextTexture,
    red_match: AnnounceTextTexture,
    round_number_1: AnnounceTextTexture,
    round_number_2: AnnounceTextTexture,
    round_number_3: AnnounceTextTexture,
}

struct CounterDigitTexture {
    texture: Texture2D,
    local_bounds: RectVisual,
    advance: f32,
}

#[allow(
    clippy::struct_field_names,
    reason = "Digit-indexed fields make the recovered embedded glyph coverage explicit"
)]
struct CounterDigitTextures {
    digit_0: CounterDigitTexture,
    digit_1: CounterDigitTexture,
    digit_2: CounterDigitTexture,
    digit_3: CounterDigitTexture,
    digit_4: CounterDigitTexture,
    digit_5: CounterDigitTexture,
    digit_6: CounterDigitTexture,
    digit_7: CounterDigitTexture,
    digit_8: CounterDigitTexture,
}

struct GameAssets {
    top_title: OnceCell<SwfTextTexture>,
    menu_labels: OnceCell<MenuLabelTextures>,
    menu_values: OnceCell<MenuValueTextures>,
    help_labels: OnceCell<HelpLabelTextures>,
    chrome_texts: OnceCell<ChromeTextTextures>,
    announce_texts: OnceCell<AnnounceTextTextures>,
    counter_digits: OnceCell<CounterDigitTextures>,
    rounds_played_label: OnceCell<SwfTextTexture>,
    sponsor_logo: OnceCell<SponsorLogoAssets>,
    device_arial_font: Option<Font>,
    device_serif_font: Option<Font>,
    device_trebuchet_font: Option<Font>,
}

impl GameAssets {
    fn new(
        device_arial_font: Option<Font>,
        device_serif_font: Option<Font>,
        device_trebuchet_font: Option<Font>,
    ) -> Self {
        Self {
            top_title: OnceCell::new(),
            menu_labels: OnceCell::new(),
            menu_values: OnceCell::new(),
            help_labels: OnceCell::new(),
            chrome_texts: OnceCell::new(),
            announce_texts: OnceCell::new(),
            counter_digits: OnceCell::new(),
            rounds_played_label: OnceCell::new(),
            sponsor_logo: OnceCell::new(),
            device_arial_font,
            device_serif_font,
            device_trebuchet_font,
        }
    }

    fn top_title(&self) -> &SwfTextTexture {
        self.top_title.get_or_init(build_top_title_texture)
    }

    fn menu_labels(&self) -> &MenuLabelTextures {
        self.menu_labels.get_or_init(build_menu_label_textures)
    }

    fn menu_values(&self) -> &MenuValueTextures {
        self.menu_values.get_or_init(build_menu_value_textures)
    }

    fn help_labels(&self) -> &HelpLabelTextures {
        self.help_labels.get_or_init(build_help_label_textures)
    }

    fn chrome_texts(&self) -> &ChromeTextTextures {
        self.chrome_texts.get_or_init(build_chrome_textures)
    }

    fn announce_texts(&self) -> &AnnounceTextTextures {
        self.announce_texts.get_or_init(build_announce_textures)
    }

    fn counter_digits(&self) -> &CounterDigitTextures {
        self.counter_digits
            .get_or_init(build_counter_digit_textures)
    }

    fn rounds_played_label(&self) -> &SwfTextTexture {
        self.rounds_played_label
            .get_or_init(build_rounds_played_label_texture)
    }

    fn sponsor_logo(&self) -> &SponsorLogoAssets {
        self.sponsor_logo
            .get_or_init(|| build_sponsor_logo_assets(SPONSOR_LOGO_ROOT_TRANSFORM))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SponsorLogoTextPlacement {
    tx: f32,
    ty: f32,
    color: Color,
}

impl SponsorLogoTextPlacement {
    const fn new(tx: f32, ty: f32, color: Color) -> Self {
        Self { tx, ty, color }
    }

    #[cfg(test)]
    fn visual(self) -> SponsorLogoTextLayer {
        SponsorLogoTextLayer {
            visual: SwfLineVisual {
                x: SPONSOR_LOGO_ROOT_TRANSFORM.scale.mul_add(
                    SPONSOR_LOGO_TEXT_SCALE.mul_add(SPONSOR_LOGO_RUN_X, self.tx),
                    SPONSOR_LOGO_ROOT_TRANSFORM.tx,
                ),
                baseline_y: SPONSOR_LOGO_ROOT_TRANSFORM.scale.mul_add(
                    SPONSOR_LOGO_TEXT_SCALE.mul_add(SPONSOR_LOGO_RUN_Y, self.ty),
                    SPONSOR_LOGO_ROOT_TRANSFORM.ty,
                ),
                font_size: (SPONSOR_LOGO_TEXT_HEIGHT
                    * SPONSOR_LOGO_ROOT_TRANSFORM.scale
                    * SPONSOR_LOGO_TEXT_SCALE)
                    .round()
                    .max(1.0) as u16,
            },
            color: self.color,
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct SponsorLogoTextLayer {
    visual: SwfLineVisual,
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SwfRightTextVisual {
    right_x: f32,
    baseline_y: f32,
    font_size: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct WinTextTransform {
    scale: f32,
    tx: f32,
    ty: f32,
    alpha_mult: u16,
}

impl WinTextTransform {
    const fn new(scale: f32, tx: f32, ty: f32, alpha_mult: u16) -> Self {
        Self {
            scale,
            tx,
            ty,
            alpha_mult,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnnounceTextKind {
    BlueWins,
    RedWins,
    Round,
    BlueMatch,
    RedMatch,
    RoundNumber1,
    RoundNumber2,
    RoundNumber3,
}

impl AnnounceTextKind {
    const fn text(self) -> &'static str {
        match self {
            Self::BlueWins => "Blue Wins",
            Self::RedWins => "Red Wins",
            Self::Round => "Round",
            Self::BlueMatch | Self::RedMatch => "Match",
            Self::RoundNumber1 => "1",
            Self::RoundNumber2 => "2",
            Self::RoundNumber3 => "3",
        }
    }

    const fn bounds_center_x(self) -> f32 {
        match self {
            Self::BlueWins => BLUE_WIN_TEXT_BOUNDS_CENTER_X,
            Self::RedWins => RED_WIN_TEXT_BOUNDS_CENTER_X,
            Self::Round => ROUND_TEXT_BOUNDS_CENTER_X,
            Self::BlueMatch | Self::RedMatch => MATCH_TEXT_BOUNDS_CENTER_X,
            Self::RoundNumber1 | Self::RoundNumber2 | Self::RoundNumber3 => {
                ROUND_NUMBER_BOUNDS_CENTER_X
            },
        }
    }

    const fn baseline_y(self) -> f32 {
        match self {
            Self::RoundNumber1 | Self::RoundNumber2 | Self::RoundNumber3 => ROUND_NUMBER_BASELINE_Y,
            Self::BlueWins | Self::RedWins | Self::Round | Self::BlueMatch | Self::RedMatch => {
                WIN_TEXT_BASELINE_Y
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct AnnounceTextVisual {
    kind: AnnounceTextKind,
    text: &'static str,
    x: f32,
    baseline_y: f32,
    font_size: u16,
    scale: f32,
    alpha: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct AnnouncePipVisual {
    x: f32,
    y: f32,
    scale: f32,
    alpha: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MatchPipPalette {
    outer: Color,
    core: Color,
    highlight: Color,
    mid: Color,
    shine: Color,
    accent: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MatchPipVisual {
    x: f32,
    y: f32,
    side: Side,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchPipSlot {
    Outer,
    Core,
    Highlight,
    Mid,
    Shine,
    Accent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SwfPathSegment {
    Line(SwfPoint),
    Quad { control: SwfPoint, to: SwfPoint },
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RadialGradientShape {
    base_radius: f32,
    start: SwfPoint,
    segments: &'static [SwfPathSegment],
}

#[derive(Debug, Clone, Copy)]
struct MatchPipContour {
    slot: MatchPipSlot,
    start: SwfPoint,
    segments: &'static [SwfPathSegment],
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ScoreMarkerVisual {
    x: f32,
    y: f32,
    scale: f32,
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct StaticSideMarkerVisual {
    x: f32,
    y: f32,
    side: Side,
}

const STATIC_SIDE_MARKER_VISUALS: [StaticSideMarkerVisual; 30] = [
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 214.45,
        side: Side::Red,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 378.85,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 50.05,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 187.05,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 241.85,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 159.65,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 269.25,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 132.25,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 296.65,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 104.85,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 324.05,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 77.45,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 351.45,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 378.85,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_RIGHT_SIDE_MARKER_X,
        y: 50.05,
        side: Side::Red,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 214.45,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 378.85,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 50.05,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 187.05,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 241.85,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 159.65,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 269.25,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 132.25,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 296.65,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 104.85,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 324.05,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 77.45,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 351.45,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 378.85,
        side: Side::Blue,
    },
    StaticSideMarkerVisual {
        x: STATIC_LEFT_SIDE_MARKER_X,
        y: 50.05,
        side: Side::Blue,
    },
];

#[derive(Debug, Clone, Copy, PartialEq)]
struct GravityPreviewPlacement {
    sx: f32,
    sy: f32,
    tx: f32,
    ty: f32,
}

impl GravityPreviewPlacement {
    const fn new(sx: f32, sy: f32, tx: f32, ty: f32) -> Self {
        Self { sx, sy, tx, ty }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct GravityPreviewBallPlacement {
    transform: GravityPreviewPlacement,
    color: Color,
}

impl GravityPreviewBallPlacement {
    const fn new(transform: GravityPreviewPlacement, color: Color) -> Self {
        Self { transform, color }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct GravityPreviewRow {
    left_arrow: GravityPreviewPlacement,
    left_ball: GravityPreviewBallPlacement,
    right_arrow: GravityPreviewPlacement,
    right_ball: GravityPreviewBallPlacement,
    same_color: bool,
}

impl GravityPreviewRow {
    const fn new(
        left_arrow: GravityPreviewPlacement,
        left_ball: GravityPreviewBallPlacement,
        right_arrow: GravityPreviewPlacement,
        right_ball: GravityPreviewBallPlacement,
        same_color: bool,
    ) -> Self {
        Self {
            left_arrow,
            left_ball,
            right_arrow,
            right_ball,
            same_color,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct GravityPreviewModeFrame {
    swf_frame: u8,
    rows: [GravityPreviewRow; 3],
}

impl GravityPreviewModeFrame {
    #[allow(
        clippy::large_types_passed_by_value,
        reason = "Small fixed SWF preview table is built as compile-time values"
    )]
    const fn new(swf_frame: u8, rows: [GravityPreviewRow; 3]) -> Self {
        Self { swf_frame, rows }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct GravityPreviewBallVisual {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
    transparent_stop_ratio: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct GravityPreviewArrowVisual {
    start: Vec2,
    end: Vec2,
    head_a: Vec2,
    head_b: Vec2,
    line_width: f32,
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AnnounceVisual {
    Text(AnnounceTextVisual),
    Pip(AnnouncePipVisual),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Startup,
    Menu,
    Help,
    Playing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StartupPhase {
    Loading,
    XmlWait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuAction {
    Start,
    Help,
    Polarisation,
    Matches,
    Gravity,
    Speed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeaderLink {
    Sponsor,
    BackToMenu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeaderLinkState {
    Up,
    Over,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExternalLinkAction {
    NeodelightBlank,
    NeodelightSelf,
    NeokolorBlank,
}

#[derive(Debug, Clone, Copy)]
struct Button {
    visual: ButtonShapeVisual,
    action: MenuAction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct EntityVisualSnapshot {
    id: u32,
    x: f64,
    y: f64,
}

impl EntityVisualSnapshot {
    const fn from_ball(ball: gravityarcade::sim::Ball) -> Self {
        Self {
            id: ball.id,
            x: ball.x,
            y: ball.y,
        }
    }

    const fn from_dying_ball(ball: DyingBall) -> Self {
        Self {
            id: ball.id,
            x: ball.x,
            y: ball.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct GameplayVisualSnapshot {
    blue_y: f64,
    red_y: f64,
    balls: Vec<EntityVisualSnapshot>,
    dying_balls: Vec<EntityVisualSnapshot>,
}

impl GameplayVisualSnapshot {
    fn capture(world: &World) -> Self {
        Self {
            blue_y: world.blue.y,
            red_y: world.red.y,
            balls: world
                .balls
                .iter()
                .copied()
                .map(EntityVisualSnapshot::from_ball)
                .collect(),
            dying_balls: world
                .dying_balls
                .iter()
                .copied()
                .map(EntityVisualSnapshot::from_dying_ball)
                .collect(),
        }
    }

    fn interpolate(&self, current: &Self, alpha: f64) -> Self {
        let alpha = alpha.clamp(0.0, 1.0);
        Self {
            blue_y: lerp_f64(self.blue_y, current.blue_y, alpha),
            red_y: lerp_f64(self.red_y, current.red_y, alpha),
            balls: interpolate_entities(&self.balls, &current.balls, alpha),
            dying_balls: interpolate_entities(&self.dying_balls, &current.dying_balls, alpha),
        }
    }

    fn ball_position(&self, id: u32) -> Option<(f64, f64)> {
        self.balls
            .iter()
            .find(|snapshot| snapshot.id == id)
            .map(|snapshot| (snapshot.x, snapshot.y))
    }

    fn dying_ball_position(&self, id: u32) -> Option<(f64, f64)> {
        self.dying_balls
            .iter()
            .find(|snapshot| snapshot.id == id)
            .map(|snapshot| (snapshot.x, snapshot.y))
    }
}

fn interpolate_entities(
    previous: &[EntityVisualSnapshot],
    current: &[EntityVisualSnapshot],
    alpha: f64,
) -> Vec<EntityVisualSnapshot> {
    current
        .iter()
        .map(|current| {
            previous
                .iter()
                .find(|previous| previous.id == current.id)
                .map_or(*current, |previous| EntityVisualSnapshot {
                    id: current.id,
                    x: lerp_f64(previous.x, current.x, alpha),
                    y: lerp_f64(previous.y, current.y, alpha),
                })
        })
        .collect()
}

fn lerp_f64(start: f64, end: f64, alpha: f64) -> f64 {
    (end - start).mul_add(alpha, start)
}

struct Game {
    screen: Screen,
    settings: Settings,
    world: World,
    audio: AudioBank,
    assets: GameAssets,
    goal_flash: GoalFlash,
    games_played: u32,
    startup_ticks: u32,
    offline: bool,
    accumulator: f64,
    interpolate: bool,
    interpolate_notice_frames: u8,
    previous_visuals: GameplayVisualSnapshot,
    current_visuals: GameplayVisualSnapshot,
    hovered_button: Option<ButtonSurface>,
    pressed_surface: Option<ActionSurface>,
    cursor_icon: CursorIcon,
    buttons: [Button; 6],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonSurface {
    StartupAbort,
    Menu(MenuAction),
    HelpBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InteractiveSurface {
    Button,
    HeaderBack,
    ExternalLink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionSurface {
    Button(ButtonSurface),
    HeaderBack,
    ExternalLink(ExternalLinkAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SurfaceTransition {
    MenuAction(MenuAction),
    MenuImmediate { set_offline: bool },
    ExternalLink(ExternalLinkAction),
}

impl Game {
    fn new(audio: AudioBank, assets: GameAssets) -> Self {
        let settings = Settings::default();
        let world = World::new(settings);
        let visuals = GameplayVisualSnapshot::capture(&world);
        Self {
            screen: Screen::Startup,
            settings,
            world,
            audio,
            assets,
            goal_flash: GoalFlash::default(),
            games_played: 0,
            startup_ticks: 0,
            offline: false,
            accumulator: 0.0,
            interpolate: true,
            interpolate_notice_frames: 0,
            previous_visuals: visuals.clone(),
            current_visuals: visuals,
            hovered_button: None,
            pressed_surface: None,
            cursor_icon: CursorIcon::Default,
            buttons: menu_buttons(),
        }
    }

    fn mouse_pressed(&mut self, x: f32, y: f32) {
        let pressed = action_surface_at(self.screen, self.startup_ticks, &self.buttons, x, y);
        if matches!(pressed, Some(ActionSurface::Button(_))) {
            self.audio.play(Cue::ButtonPress);
        }
        self.pressed_surface = pressed;
    }

    fn mouse_released(&mut self, x: f32, y: f32) {
        let released = action_surface_at(self.screen, self.startup_ticks, &self.buttons, x, y);
        if let Some(surface) = swf_release_or_drag_out_action(self.pressed_surface.take(), released)
        {
            self.activate_surface(surface);
        }
    }

    fn mouse_dragged(&mut self, x: f32, y: f32) {
        let current = action_surface_at(self.screen, self.startup_ticks, &self.buttons, x, y);
        if let Some(surface) = swf_drag_out_action(self.pressed_surface, current) {
            self.pressed_surface = None;
            self.activate_surface(surface);
        }
    }

    fn activate_surface(&mut self, surface: ActionSurface) {
        match surface_transition(surface) {
            SurfaceTransition::MenuAction(action) => self.activate(action),
            SurfaceTransition::MenuImmediate { set_offline } => {
                if set_offline {
                    self.offline = true;
                }
                self.screen = Screen::Menu;
            },
            SurfaceTransition::ExternalLink(link) => open_external_link(link),
        }
    }

    fn activate(&mut self, action: MenuAction) {
        match action {
            MenuAction::Start => {
                self.world = World::new(self.settings);
                self.goal_flash = GoalFlash::default();
                self.sync_gameplay_visuals();
                self.screen = Screen::Playing;
            },
            MenuAction::Help => self.screen = Screen::Help,
            MenuAction::Polarisation => {
                self.settings.polarisation = self.settings.polarisation.next();
            },
            MenuAction::Matches => self.settings.cycle_matches(),
            MenuAction::Gravity => self.settings.gravity = self.settings.gravity.next(),
            MenuAction::Speed => self.settings.speed = self.settings.speed.next(),
        }
    }

    fn sync_gameplay_visuals(&mut self) {
        let visuals = GameplayVisualSnapshot::capture(&self.world);
        self.previous_visuals = visuals.clone();
        self.current_visuals = visuals;
    }

    fn gameplay_visuals(&self, forced_alpha: Option<f32>) -> GameplayVisualSnapshot {
        if !self.interpolate {
            return self.current_visuals.clone();
        }

        let alpha = forced_alpha.unwrap_or_else(|| self.interpolation_alpha());
        self.previous_visuals
            .interpolate(&self.current_visuals, f64::from(alpha))
    }

    fn interpolation_alpha(&self) -> f32 {
        if self.screen != Screen::Playing {
            return 1.0;
        }

        (self.accumulator / (1.0 / TICK_HZ)).clamp(0.0, 1.0) as f32
    }

    #[expect(
        clippy::while_float,
        reason = "Fixed-step accumulator loop follows the macroquad runtime shell pattern"
    )]
    fn update(&mut self) {
        let (x, y) = mouse_position();
        if is_key_pressed(INTERPOLATE_TOGGLE_KEY) {
            self.interpolate = !self.interpolate;
            self.interpolate_notice_frames = INTERPOLATE_NOTICE_FRAMES;
        } else {
            self.interpolate_notice_frames = self.interpolate_notice_frames.saturating_sub(1);
        }
        match self.screen {
            Screen::Startup => self.tick_startup_timeline(),
            Screen::Menu | Screen::Help | Screen::Playing => {},
        }
        let hovered = hovered_button_with_startup_timing(
            self.screen,
            self.startup_ticks,
            &self.buttons,
            x,
            y,
        );
        if hovered != self.hovered_button {
            if hovered.is_some() {
                self.audio.play(Cue::ButtonRollover);
            }
            self.hovered_button = hovered;
        }
        let cursor_icon =
            cursor_icon_for_hover(self.screen, self.startup_ticks, &self.buttons, x, y);
        if cursor_icon != self.cursor_icon {
            miniquad::window::set_mouse_cursor(cursor_icon);
            self.cursor_icon = cursor_icon;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            self.mouse_pressed(x, y);
        }
        if is_mouse_button_down(MouseButton::Left) {
            self.mouse_dragged(x, y);
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.mouse_released(x, y);
        }
        if self.screen == Screen::Playing {
            self.accumulator += current_frame_time_step();
            let dt = 1.0 / TICK_HZ;
            while self.accumulator >= dt {
                self.previous_visuals = GameplayVisualSnapshot::capture(&self.world);
                self.world.tick(read_controls());
                self.current_visuals = GameplayVisualSnapshot::capture(&self.world);
                self.goal_flash.tick();
                self.goal_flash.apply_events(&self.world.events);
                self.games_played = self
                    .games_played
                    .saturating_add(games_played_delta(&self.world.events));
                self.audio.play_events(&self.world.events);
                self.accumulator -= dt;
                if self.world.winner.is_some() && self.world.final_win_announce_ticks == 0 {
                    self.accumulator = 0.0;
                    self.screen = Screen::Menu;
                    break;
                }
            }
        }
    }

    fn draw(&self) {
        self.draw_with_interpolation_alpha(None);
    }

    fn draw_with_interpolation_alpha(&self, interpolation_alpha: Option<f32>) {
        clear_background(BG);
        match self.screen {
            Screen::Startup => self.draw_startup(),
            Screen::Menu => self.draw_menu(),
            Screen::Help => self.draw_help(),
            Screen::Playing => self.draw_gameplay(interpolation_alpha),
        }
        if self.interpolate_notice_frames > 0 {
            draw_interpolate_notice(self.interpolate);
        }
    }

    #[cfg(debug_assertions)]
    fn debug_fixed_tick(&mut self) {
        match self.screen {
            Screen::Startup => {
                self.startup_ticks = self.startup_ticks.saturating_add(1);
            },
            Screen::Playing => {
                self.previous_visuals = GameplayVisualSnapshot::capture(&self.world);
                self.world.tick(Controls::default());
                self.current_visuals = GameplayVisualSnapshot::capture(&self.world);
                self.goal_flash.tick();
                self.goal_flash.apply_events(&self.world.events);
                self.games_played = self
                    .games_played
                    .saturating_add(games_played_delta(&self.world.events));
                if self.world.winner.is_some() && self.world.final_win_announce_ticks == 0 {
                    self.screen = Screen::Menu;
                }
            },
            Screen::Menu | Screen::Help => {},
        }
    }

    #[expect(
        clippy::while_float,
        reason = "Startup timeline uses the same fixed-step accumulator as gameplay"
    )]
    fn tick_startup_timeline(&mut self) {
        self.accumulator += current_frame_time_step();
        let dt = 1.0 / TICK_HZ;
        while self.accumulator >= dt {
            self.startup_ticks = self.startup_ticks.saturating_add(1);
            self.accumulator -= dt;
        }
    }

    fn draw_startup(&self) {
        let assets = &self.assets;
        match startup_phase(self.startup_ticks) {
            StartupPhase::Loading => draw_startup_loading(assets.device_arial_font.as_ref()),
            StartupPhase::XmlWait => draw_startup_xml_wait(
                self.startup_ticks,
                assets.device_arial_font.as_ref(),
                assets.device_trebuchet_font.as_ref(),
            ),
        }
    }

    fn draw_menu(&self) {
        let assets = &self.assets;
        let menu_labels = assets.menu_labels();
        let menu_values = assets.menu_values();
        let chrome_texts = assets.chrome_texts();
        let counter_digits = assets.counter_digits();
        draw_stage_background();
        draw_panel_fill();
        draw_menu_button_by_action(MenuAction::Start, &self.buttons, menu_labels);
        draw_gravity_mode_preview(self.settings.polarisation);
        draw_swf_text_texture(menu_match_type_value_texture(
            self.settings.matches,
            menu_values,
        ));
        draw_menu_button_by_action(MenuAction::Polarisation, &self.buttons, menu_labels);
        draw_menu_button_by_action(MenuAction::Matches, &self.buttons, menu_labels);
        draw_menu_button_by_action(MenuAction::Help, &self.buttons, menu_labels);
        draw_swf_text_texture(&menu_values.question);
        draw_swf_text_texture(menu_gravity_strength_value_texture(
            self.settings.gravity,
            menu_values,
        ));
        draw_menu_button_by_action(MenuAction::Gravity, &self.buttons, menu_labels);
        draw_menu_button_by_action(MenuAction::Speed, &self.buttons, menu_labels);
        draw_swf_text_texture(menu_speed_value_texture(self.settings.speed, menu_values));
        draw_top_gravity_factor(self.settings.gravity, assets.device_serif_font.as_ref());
        draw_footer(
            assets.rounds_played_label(),
            chrome_texts,
            counter_digits,
            assets.device_trebuchet_font.as_ref(),
            self.games_played,
            rounds_played_clip_visible(Screen::Menu, self.offline),
            version_footer_visible(Screen::Menu),
        );
        draw_playfield_mask();
        draw_top_title_foreground(assets.top_title());
        draw_goal_line(
            stage_left_goal_x(MENU_STAGE_FRAME),
            Side::Red,
            0,
            &LEFT_GOAL_FLASH,
        );
        draw_goal_line(
            stage_right_goal_x(MENU_STAGE_FRAME),
            Side::Blue,
            0,
            &RIGHT_GOAL_FLASH,
        );
        draw_goal_paddle_marker(514.7, 214.2);
        draw_goal_paddle_marker(35.0, 214.15);
        for &glow in static_goal_paddle_glows(MENU_STAGE_FRAME) {
            draw_static_paddle_glow(glow);
        }
        draw_panel_outline();
        for_each_static_side_marker(MENU_STAGE_FRAME, draw_static_side_marker);
        draw_header_link(HeaderLink::Sponsor, chrome_texts);
        draw_sponsor_logo(assets.sponsor_logo());
    }

    fn draw_help(&self) {
        let assets = &self.assets;
        let chrome_texts = assets.chrome_texts();
        let counter_digits = assets.counter_digits();
        draw_stage_background();
        draw_panel_fill();
        draw_help_content(assets.help_labels());
        draw_footer(
            assets.rounds_played_label(),
            chrome_texts,
            counter_digits,
            assets.device_trebuchet_font.as_ref(),
            self.games_played,
            rounds_played_clip_visible(Screen::Help, self.offline),
            version_footer_visible(Screen::Help),
        );
        draw_playfield_mask();
        draw_top_title_foreground(assets.top_title());
        draw_goal_line(
            stage_left_goal_x(MENU_STAGE_FRAME),
            Side::Red,
            0,
            &LEFT_GOAL_FLASH,
        );
        draw_goal_line(
            stage_right_goal_x(MENU_STAGE_FRAME),
            Side::Blue,
            0,
            &RIGHT_GOAL_FLASH,
        );
        draw_goal_paddle_marker(514.7, 214.2);
        draw_goal_paddle_marker(35.0, 214.15);
        for &glow in static_goal_paddle_glows(MENU_STAGE_FRAME) {
            draw_static_paddle_glow(glow);
        }
        draw_panel_outline();
        for_each_static_side_marker(MENU_STAGE_FRAME, draw_static_side_marker);
        draw_header_link(HeaderLink::BackToMenu, chrome_texts);
        draw_sponsor_logo(assets.sponsor_logo());
    }

    fn draw_gameplay(&self, interpolation_alpha: Option<f32>) {
        let assets = &self.assets;
        let chrome_texts = assets.chrome_texts();
        let announce_texts = assets.announce_texts();
        let world = &self.world;
        let visuals = self.gameplay_visuals(interpolation_alpha);
        draw_playfield(self.goal_flash);
        draw_player_at(&world.blue, visuals.blue_y);
        draw_player_at(&world.red, visuals.red_y);
        draw_attached_balls_in_depth_range(
            world,
            &visuals,
            PLAYFIELD_FIRST_DYNAMIC_BALL_DEPTH,
            PLAYFIELD_BLUE_SCORE_METER_DEPTH,
        );
        draw_score_meter(world.blue_score, Side::Blue, world.tick);
        draw_attached_balls_in_depth_range(
            world,
            &visuals,
            PLAYFIELD_BLUE_SCORE_METER_DEPTH,
            PLAYFIELD_RED_SCORE_METER_DEPTH,
        );
        draw_score_meter(world.red_score, Side::Red, world.tick);
        draw_attached_balls_in_depth_range(
            world,
            &visuals,
            PLAYFIELD_RED_SCORE_METER_DEPTH,
            attached_ball_depth_end(world),
        );
        draw_playfield_mask();
        draw_retained_frame_mask_outline();
        draw_top_title_foreground(assets.top_title());
        draw_match_win_announce(world, announce_texts);
        draw_round_intro(world, announce_texts);
        draw_match_pips(world.blue_matches, Side::Blue);
        draw_match_pips(world.red_matches, Side::Red);
        draw_header_link(HeaderLink::BackToMenu, chrome_texts);
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct GoalFlash {
    left: u8,
    right: u8,
}

impl GoalFlash {
    fn tick(&mut self) {
        self.left = self.left.saturating_sub(1);
        self.right = self.right.saturating_sub(1);
    }

    fn apply_events(&mut self, events: &[RoundEvent]) {
        for event in events {
            match *event {
                RoundEvent::Score {
                    side: Side::Blue, ..
                } => self.left = 6,
                RoundEvent::Score {
                    side: Side::Red, ..
                } => self.right = 6,
                _ => {},
            }
        }
    }
}

const fn surface_transition(surface: ActionSurface) -> SurfaceTransition {
    match surface {
        ActionSurface::Button(ButtonSurface::Menu(action)) => SurfaceTransition::MenuAction(action),
        ActionSurface::Button(ButtonSurface::HelpBack) => {
            SurfaceTransition::MenuImmediate { set_offline: false }
        },
        ActionSurface::Button(ButtonSurface::StartupAbort) => {
            SurfaceTransition::MenuImmediate { set_offline: true }
        },
        ActionSurface::HeaderBack => SurfaceTransition::MenuImmediate { set_offline: false },
        ActionSurface::ExternalLink(link) => SurfaceTransition::ExternalLink(link),
    }
}

#[cfg(test)]
const fn swf_goto_frame_root_frame(raw_frame: u16) -> u16 {
    raw_frame + SWF_GOTO_FRAME_ROOT_FRAME_OFFSET
}

#[derive(Default)]
struct AudioBank {
    button_rollover: Option<Sound>,
    button_press: Option<Sound>,
    reflect: Option<Sound>,
    merge: Option<Sound>,
    shot: Option<Sound>,
    score_line: Option<Sound>,
    paddle_stun: Option<Sound>,
    round_lost: Option<Sound>,
    round_start: Option<Sound>,
    blue_match_win: Option<Sound>,
    red_match_win: Option<Sound>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cue {
    ButtonRollover,
    ButtonPress,
    Reflect,
    Merge,
    Shot,
    ScoreLine,
    PaddleStun,
    RoundLost,
    RoundStart,
    BlueMatchWin,
    RedMatchWin,
}

macro_rules! swf_sound_bytes {
    ($name:literal) => {
        include_bytes!(concat!("../assets/sounds/", $name, ".wav"))
    };
}

#[cfg(test)]
const AUDIO_RUNTIME_FORMAT: &str = "wav";

impl AudioBank {
    async fn load() -> Self {
        Self {
            button_rollover: load_swf_sound(swf_sound_bytes!("sound_23")).await,
            button_press: load_swf_sound(swf_sound_bytes!("sound_24")).await,
            reflect: load_swf_sound(swf_sound_bytes!("reflect")).await,
            merge: load_swf_sound(swf_sound_bytes!("merge")).await,
            shot: load_swf_sound(swf_sound_bytes!("shot")).await,
            score_line: load_swf_sound(swf_sound_bytes!("score_line")).await,
            paddle_stun: load_swf_sound(swf_sound_bytes!("paddle_stun")).await,
            round_lost: load_swf_sound(swf_sound_bytes!("round_lost")).await,
            round_start: load_swf_sound(swf_sound_bytes!("round_start")).await,
            blue_match_win: load_swf_sound(swf_sound_bytes!("blue_match_win")).await,
            red_match_win: load_swf_sound(swf_sound_bytes!("red_match_win")).await,
        }
    }

    fn play_events(&self, events: &[RoundEvent]) {
        for cue in cues_for_events(events) {
            self.play(cue);
        }
    }

    fn play(&self, cue: Cue) {
        let sound = match cue {
            Cue::ButtonRollover => &self.button_rollover,
            Cue::ButtonPress => &self.button_press,
            Cue::Reflect => &self.reflect,
            Cue::Merge => &self.merge,
            Cue::Shot => &self.shot,
            Cue::ScoreLine => &self.score_line,
            Cue::PaddleStun => &self.paddle_stun,
            Cue::RoundLost => &self.round_lost,
            Cue::RoundStart => &self.round_start,
            Cue::BlueMatchWin => &self.blue_match_win,
            Cue::RedMatchWin => &self.red_match_win,
        };
        if let Some(sound) = sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
        }
    }
}

fn cues_for_events(events: &[RoundEvent]) -> Vec<Cue> {
    let mut cues = Vec::new();
    for event in events {
        match *event {
            RoundEvent::Shot { .. } => cues.push(Cue::Shot),
            RoundEvent::PaddleHit { .. } | RoundEvent::WallBounce => cues.push(Cue::Reflect),
            RoundEvent::Merge => cues.push(Cue::Merge),
            RoundEvent::Score { burning, .. } => {
                cues.push(Cue::ScoreLine);
                if burning {
                    cues.push(Cue::PaddleStun);
                }
            },
            RoundEvent::RoundLostSound { .. } => cues.push(Cue::RoundLost),
            RoundEvent::FinalMatchWinSound { side } => {
                cues.push(match side {
                    Side::Blue => Cue::BlueMatchWin,
                    Side::Red => Cue::RedMatchWin,
                });
            },
            RoundEvent::RoundStart | RoundEvent::MatchWin { .. } => {},
            RoundEvent::RoundIntroSound => cues.push(Cue::RoundStart),
        }
    }
    cues
}

async fn load_swf_sound(bytes: &'static [u8]) -> Option<Sound> {
    load_sound_from_bytes(bytes).await.ok()
}

#[cfg(target_os = "windows")]
const ARIAL_SYSTEM_FONT_PATHS: &[&str] =
    &["C:/Windows/Fonts/arial.ttf", "C:/Windows/Fonts/arialbd.ttf"];
#[cfg(target_os = "windows")]
const TIMES_NEW_ROMAN_SYSTEM_FONT_PATHS: &[&str] =
    &["C:/Windows/Fonts/times.ttf", "C:/Windows/Fonts/timesbd.ttf"];
#[cfg(target_os = "windows")]
const TREBUCHET_SYSTEM_FONT_PATHS: &[&str] = &[
    "C:/Windows/Fonts/trebuc.ttf",
    "C:/Windows/Fonts/trebucbd.ttf",
];

#[cfg(target_os = "macos")]
const ARIAL_SYSTEM_FONT_PATHS: &[&str] = &[
    "/Library/Fonts/Arial.ttf",
    "/System/Library/Fonts/Supplemental/Arial.ttf",
];
#[cfg(target_os = "macos")]
const TIMES_NEW_ROMAN_SYSTEM_FONT_PATHS: &[&str] = &[
    "/Library/Fonts/Times New Roman.ttf",
    "/System/Library/Fonts/Supplemental/Times New Roman.ttf",
];
#[cfg(target_os = "macos")]
const TREBUCHET_SYSTEM_FONT_PATHS: &[&str] = &[
    "/Library/Fonts/Trebuchet MS.ttf",
    "/System/Library/Fonts/Supplemental/Trebuchet MS.ttf",
];

#[cfg(all(unix, not(target_os = "macos")))]
const ARIAL_SYSTEM_FONT_PATHS: &[&str] = &[
    "/usr/share/fonts/truetype/msttcorefonts/Arial.ttf",
    "/usr/share/fonts/truetype/msttcorefonts/arial.ttf",
    "/usr/local/share/fonts/Arial.ttf",
    "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
];
#[cfg(all(unix, not(target_os = "macos")))]
const TIMES_NEW_ROMAN_SYSTEM_FONT_PATHS: &[&str] = &[
    "/usr/share/fonts/truetype/msttcorefonts/Times_New_Roman.ttf",
    "/usr/share/fonts/truetype/msttcorefonts/times.ttf",
    "/usr/local/share/fonts/Times_New_Roman.ttf",
    "/usr/share/fonts/truetype/liberation2/LiberationSerif-Regular.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf",
    "/usr/share/fonts/liberation/LiberationSerif-Regular.ttf",
    "/usr/share/fonts/truetype/crosextra/Caladea-Regular.ttf",
];
#[cfg(all(unix, not(target_os = "macos")))]
const TREBUCHET_SYSTEM_FONT_PATHS: &[&str] = &[
    "/usr/share/fonts/truetype/msttcorefonts/Trebuchet_MS.ttf",
    "/usr/share/fonts/truetype/msttcorefonts/trebuc.ttf",
    "/usr/local/share/fonts/Trebuchet_MS.ttf",
    "/usr/share/fonts/truetype/crosextra/Carlito-Regular.ttf",
    "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
];

#[cfg(not(any(target_os = "windows", target_os = "macos", unix)))]
const ARIAL_SYSTEM_FONT_PATHS: &[&str] = &[];
#[cfg(not(any(target_os = "windows", target_os = "macos", unix)))]
const TIMES_NEW_ROMAN_SYSTEM_FONT_PATHS: &[&str] = &[];
#[cfg(not(any(target_os = "windows", target_os = "macos", unix)))]
const TREBUCHET_SYSTEM_FONT_PATHS: &[&str] = &[];

fn load_first_ttf_font_from_paths(paths: &[&str]) -> Option<Font> {
    paths.iter().copied().find_map(|path| {
        let bytes = std::fs::read(path).ok()?;
        load_ttf_font_from_bytes(&bytes).ok()
    })
}

fn load_device_serif_font() -> Option<Font> {
    load_first_ttf_font_from_paths(TIMES_NEW_ROMAN_SYSTEM_FONT_PATHS).or_else(|| {
        load_ttf_font_from_bytes(include_bytes!(
            "../assets/fonts/LiberationSerif-Regular.ttf"
        ))
        .ok()
    })
}

fn load_device_arial_font() -> Option<Font> {
    load_first_ttf_font_from_paths(ARIAL_SYSTEM_FONT_PATHS).or_else(|| {
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/LiberationSans-Regular.ttf")).ok()
    })
}

fn load_device_trebuchet_font() -> Option<Font> {
    load_first_ttf_font_from_paths(TREBUCHET_SYSTEM_FONT_PATHS).or_else(|| {
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/LiberationSans-Regular.ttf")).ok()
    })
}

fn games_played_delta(events: &[RoundEvent]) -> u32 {
    events
        .iter()
        .filter(|event| matches!(event, RoundEvent::MatchWin { .. }))
        .count() as u32
}

#[cfg(test)]
const fn menu_match_type_value(matches: u8) -> &'static str {
    match matches {
        3 => "best of 3",
        5 => "best of 5",
        7 => "best of 7",
        _ => "single match",
    }
}

#[cfg(test)]
const fn menu_gravity_strength_value(gravity: GravityStrength) -> &'static str {
    match gravity {
        GravityStrength::G1 => "low",
        GravityStrength::G2 => "medium",
        GravityStrength::G3 => "high",
        GravityStrength::G4 => "very high",
        GravityStrength::G5 => "black hole",
    }
}

const fn top_gravity_factor_value(gravity: GravityStrength) -> &'static str {
    match gravity {
        GravityStrength::G1 => "10",
        GravityStrength::G2 => "17",
        GravityStrength::G3 => "50",
        GravityStrength::G4 => "120",
        GravityStrength::G5 => "210",
    }
}

#[cfg(test)]
const fn menu_speed_value(speed: SpeedMode) -> &'static str {
    match speed {
        SpeedMode::Normal => "disabled",
        SpeedMode::Fast => "enabled",
    }
}

const TOP_TITLE_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (232.25, 4.65),
    (1.698_898_3, 1.698_898_3),
    (0.0, 0.0),
    (0.5, 46.05),
    (11.8, 12.0),
);
const TOP_TITLE_FLATTEN_STEPS: u8 = 4;
const TOP_TITLE_TEXTURE_SUPERSAMPLE: u16 = 4;
const MENU_VERSION_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (221.3, 378.1),
    (0.745_849_6, 0.745_849_6),
    (0.0, 0.0),
    (-101.05, 250.35),
    (11.8, 12.0),
);
const ROUNDS_PLAYED_LABEL_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (194.4, 364.35),
    (0.745_849_6, 0.745_849_6),
    (0.0, 0.0),
    (33.25, 117.05),
    (11.8, 12.0),
);
const ROUNDS_PLAYED_LABEL_FLATTEN_STEPS: u8 = 4;
const ROUNDS_PLAYED_LABEL_TEXTURE_SUPERSAMPLE: u16 = 4;
const ROUNDS_PLAYED_VALUE_TEXT: SwfRightTextVisual = SwfRightTextVisual {
    right_x: 315.46,
    baseline_y: 373.15,
    font_size: 9,
};
#[cfg(test)]
const ROUNDS_PLAYED_DEFINE_EDIT_TEXT_ID: u16 = 78;
#[cfg(test)]
const ROUNDS_PLAYED_CLIP_FRAME: u16 = 56;
#[cfg(test)]
const ROUNDS_PLAYED_CLIP_DEPTH: u16 = 47;
#[cfg(test)]
const ROUNDS_PLAYED_CLIP_ID: u16 = 79;
#[cfg(test)]
const SWF_VISIBLE_PROPERTY_INDEX: u8 = 7;
#[cfg(test)]
const COUNTER_FONT_ID: u16 = 26;
#[cfg(test)]
const COUNTER_FONT_NAME: &str = "Trebuchet MS";
#[cfg(test)]
const COUNTER_EMBEDDED_DIGITS: &str = "012345678";
#[cfg(test)]
const COUNTER_MISSING_DEVICE_DIGIT: char = '9';
#[cfg(test)]
const COUNTER_DEVICE_FONT_NAME: &str = "Trebuchet MS";
#[cfg(test)]
const COUNTER_DEVICE_FALLBACK_FONT_NAME: &str = "Liberation Sans";
const COUNTER_DIGIT_BASELINE_Y: f32 = 11.8;
const COUNTER_DIGIT_STAGE_SCALE: f32 = 0.745_849_6;
const COUNTER_DIGIT_FLATTEN_STEPS: u8 = 4;
const COUNTER_DIGIT_TEXTURE_SCALE: u16 = 8;
const COUNTER_DIGIT_TEXTURE_SUPERSAMPLE: u16 = 2;
const CHROME_TEXT_FLATTEN_STEPS: u8 = 4;
const CHROME_TEXT_TEXTURE_SUPERSAMPLE: u16 = 4;
const HEADER_LINK_TEXT_PLACEMENT: SwfTextPlacement =
    SwfTextPlacement::new((9.0, 0.2), (1.0, 1.0), (0.0, 0.0), (0.0, 0.0), (11.8, 12.0));
#[cfg(test)]
const HEADER_LINK_TEXT: SwfLineVisual = SwfLineVisual {
    x: 9.0,
    baseline_y: 12.0,
    font_size: 12,
};
const SWF_FILE_LEN_BYTES: usize = 84_367;
const STARTUP_XML_WAIT_TICK: u32 = 2;
const STARTUP_ABORT_BUTTON_FIRST_FRAME: u32 = 16;
const STARTUP_ABORT_BUTTON_FULL_ALPHA_FRAME: u32 = 90;
const STARTUP_GRAVITY_TEXT_VALUE: &str = "\"Gravity\"";
const STARTUP_RETRIEVING_TEXT_VALUE: &str = "retrieving online data";
const STARTUP_ABORT_LABEL: &str = "abort";
const STARTUP_GRAVITY_TEXT: SwfCenterTextVisual = SwfCenterTextVisual {
    center_x: 275.0,
    baseline_y: 142.5,
    font_size: 18,
};
const STARTUP_LOADING_TITLE_TEXT: SwfCenterTextVisual = SwfCenterTextVisual {
    center_x: 275.03,
    baseline_y: 191.95,
    font_size: 18,
};
const STARTUP_LOADING_PERCENT_TEXT: SwfCenterTextVisual = SwfCenterTextVisual {
    center_x: 275.03,
    baseline_y: 236.95,
    font_size: 18,
};
const STARTUP_RETRIEVING_TEXT: SwfCenterTextVisual = SwfCenterTextVisual {
    center_x: 275.0,
    baseline_y: 190.5,
    font_size: 18,
};
const STARTUP_ABORT_BUTTON_X: f32 = 274.45;
const STARTUP_ABORT_BUTTON_Y: f32 = 233.05;
const STARTUP_ABORT_BUTTON_SCALE: f32 = 0.671_630_86;
const STARTUP_ABORT_LABEL_TEXT: SwfCenterTextVisual = SwfCenterTextVisual {
    center_x: 292.88,
    baseline_y: 238.49,
    font_size: 11,
};
#[cfg(test)]
const STARTUP_ARIAL_FONT_ID: u16 = 17;
#[cfg(test)]
const STARTUP_TITLE_DEFINE_EDIT_TEXT_ID: u16 = 18;
#[cfg(test)]
const STARTUP_PERCENT_DEFINE_EDIT_TEXT_ID: u16 = 19;
#[cfg(test)]
const STARTUP_GRAVITY_DEFINE_EDIT_TEXT_ID: u16 = 20;
#[cfg(test)]
const STARTUP_RETRIEVING_DEFINE_EDIT_TEXT_ID: u16 = 21;
#[cfg(test)]
const STARTUP_ABORT_BUTTON_ID: u16 = 25;
#[cfg(test)]
const STARTUP_ABORT_ROLLOVER_SOUND_ID: u16 = 23;
#[cfg(test)]
const STARTUP_ABORT_PRESS_SOUND_ID: u16 = 24;
#[cfg(test)]
const STARTUP_ABORT_LABEL_DEFINE_TEXT_ID: u16 = 27;
#[cfg(test)]
const STARTUP_ABORT_CLIP_ID: u16 = 29;
#[cfg(test)]
const SWF_GOTO_FRAME_ROOT_FRAME_OFFSET: u16 = 1;
#[cfg(test)]
const STARTUP_ABORT_GOTO_FRAME: u16 = 55;
#[cfg(test)]
const STARTUP_MENU_ROOT_FRAME: u16 = 56;
#[cfg(test)]
const HELP_BACK_GOTO_FRAME: u16 = 55;
#[cfg(test)]
const HEADER_BACK_GOTO_FRAME: u16 = 55;
#[cfg(test)]
const STARTUP_DEVICE_FALLBACK_FONT_NAME: &str = "Liberation Sans";
const SPONSOR_HEADER_HIT_POLYGON: [SwfPoint; 4] = placement_constants::SPONSOR_HEADER_HIT;
const BACK_HEADER_HIT_POLYGON: [SwfPoint; 4] = placement_constants::BACK_HEADER_HIT;
#[cfg(test)]
const SPONSOR_LOGO_LABEL: &str = "neodelight";
const SPONSOR_LOGO_ROOT_X: f32 = placement_constants::SPONSOR_LOGO_ROOT_X;
const SPONSOR_LOGO_ROOT_Y: f32 = placement_constants::SPONSOR_LOGO_ROOT_Y;
const SPONSOR_LOGO_ROOT_TRANSFORM: LogoRootTransform = LogoRootTransform::new(
    SPONSOR_LOGO_ROOT_X,
    SPONSOR_LOGO_ROOT_Y,
    SPONSOR_LOGO_ROOT_SCALE,
);
#[cfg(test)]
const SPONSOR_LOGO_MENU_FRAME: u16 = 56;
#[cfg(test)]
const SPONSOR_LOGO_HELP_FRAME: u16 = 57;
#[cfg(test)]
const SPONSOR_LOGO_REMOVED_FRAME: u16 = 58;
#[cfg(test)]
const SPONSOR_LOGO_DEPTH: u16 = 129;
const SPONSOR_LOGO_ROOT_SCALE: f32 = placement_constants::SPONSOR_LOGO_ROOT_SCALE;
const SPONSOR_LOGO_BUTTON_TX: f32 = placement_constants::SPONSOR_LOGO_BUTTON_TX;
const SPONSOR_LOGO_BUTTON_TY: f32 = placement_constants::SPONSOR_LOGO_BUTTON_TY;
const SPONSOR_LOGO_BUTTON_HIT_BOUNDS: SwfBounds =
    placement_constants::SPONSOR_LOGO_BUTTON_HIT_BOUNDS;
const SPONSOR_LOGO_TEXT_SCALE: f32 = 3.145_111;
#[cfg(test)]
const SPONSOR_LOGO_RUN_X: f32 = 0.0;
#[cfg(test)]
const SPONSOR_LOGO_RUN_Y: f32 = 16.8;
#[cfg(test)]
const SPONSOR_LOGO_TEXT_HEIGHT: f32 = 12.0;
const SPONSOR_LOGO_TEXT_FLATTEN_STEPS: u8 = 4;
const SPONSOR_LOGO_TEXT_TEXTURE_SUPERSAMPLE: u16 = 4;
const SPONSOR_LOGO_SHAPE35_FLATTEN_STEPS: u8 = 4;
const SPONSOR_LOGO_TEXTURE_SUPERSAMPLE: u16 = 4;
const SPONSOR_LOGO_DEPTH_3_TEXT: SponsorLogoTextPlacement =
    SponsorLogoTextPlacement::new(-117.3, -50.65, SPONSOR_LOGO_DARK);
const SPONSOR_LOGO_DEPTH_4_TEXT: SponsorLogoTextPlacement =
    SponsorLogoTextPlacement::new(-118.3, -51.65, SPONSOR_LOGO_OLIVE);
const NEOKOLOR_LINK_ROOT_X: f32 = placement_constants::NEOKOLOR_LINK_ROOT_X;
const NEOKOLOR_LINK_ROOT_Y: f32 = placement_constants::NEOKOLOR_LINK_ROOT_Y;
#[cfg(test)]
const NEOKOLOR_LINK_ROOT_FRAME: u16 = 56;
#[cfg(test)]
const NEOKOLOR_LINK_HELP_FRAME: u16 = 57;
#[cfg(test)]
const NEOKOLOR_LINK_REMOVED_FRAME: u16 = 58;
#[cfg(test)]
const NEOKOLOR_LINK_DEPTH: u16 = 44;
#[cfg(test)]
const NEOKOLOR_LINK_BUTTON_ID: u16 = 75;
const NEOKOLOR_LINK_BUTTON_TX: f32 = placement_constants::NEOKOLOR_LINK_BUTTON_TX;
const NEOKOLOR_LINK_BUTTON_TY: f32 = placement_constants::NEOKOLOR_LINK_BUTTON_TY;
const NEOKOLOR_LINK_BUTTON_SCALE_X: f32 = placement_constants::NEOKOLOR_LINK_BUTTON_SCALE_X;
const NEOKOLOR_LINK_BUTTON_SCALE_Y: f32 = placement_constants::NEOKOLOR_LINK_BUTTON_SCALE_Y;
const NEOKOLOR_LINK_BUTTON_HIT_BOUNDS: SwfBounds =
    placement_constants::NEOKOLOR_LINK_BUTTON_HIT_BOUNDS;
const TOP_GRAVITY_FACTOR_TEXT: SwfLineVisual = SwfLineVisual {
    x: 345.0,
    baseline_y: 18.05,
    font_size: 12,
};
#[cfg(test)]
const TOP_GRAVITY_FACTOR_DEFINE_EDIT_TEXT_ID: u16 = 72;
#[cfg(test)]
const TOP_GRAVITY_FACTOR_FONT_ID: u16 = 71;
#[cfg(test)]
const TOP_GRAVITY_FACTOR_FONT_NAME: &str = "Times New Roman";
#[cfg(test)]
const TOP_GRAVITY_FACTOR_DEVICE_FALLBACK_FONT_NAME: &str = "Liberation Serif";
#[cfg(test)]
const TOP_GRAVITY_FACTOR_EMBEDDED_GLYPHS: usize = 0;
const MENU_LABEL_FLATTEN_STEPS: u8 = 4;
const MENU_LABEL_TEXTURE_SUPERSAMPLE: u16 = 4;
#[cfg(test)]
const MENU_START_BUTTON_DEPTH: u16 = 4;
#[cfg(test)]
const MENU_START_LABEL_DEPTH: u16 = 6;
#[cfg(test)]
const MENU_GRAVITY_PREVIEW_DEPTH: u16 = 7;
#[cfg(test)]
const MENU_MATCH_VALUE_DEPTH: u16 = 20;
#[cfg(test)]
const MENU_POLARISATION_BUTTON_DEPTH: u16 = 22;
#[cfg(test)]
const MENU_POLARISATION_LABEL_DEPTH: u16 = 24;
#[cfg(test)]
const MENU_MATCH_BUTTON_DEPTH: u16 = 25;
#[cfg(test)]
const MENU_MATCH_LABEL_DEPTH: u16 = 27;
#[cfg(test)]
const MENU_HELP_BUTTON_DEPTH: u16 = 28;
#[cfg(test)]
const MENU_HELP_LABEL_DEPTH: u16 = 30;
#[cfg(test)]
const MENU_QUESTION_DEPTH: u16 = 31;
#[cfg(test)]
const MENU_GRAVITY_VALUE_DEPTH: u16 = 32;
#[cfg(test)]
const MENU_GRAVITY_BUTTON_DEPTH: u16 = 35;
#[cfg(test)]
const MENU_GRAVITY_LABEL_DEPTH: u16 = 37;
#[cfg(test)]
const MENU_SPEED_BUTTON_DEPTH: u16 = 38;
#[cfg(test)]
const MENU_SPEED_LABEL_DEPTH: u16 = 40;
#[cfg(test)]
const MENU_SPEED_VALUE_DEPTH: u16 = 41;
#[cfg(test)]
const MENU_TOP_GRAVITY_FACTOR_DEPTH: u16 = 43;
#[cfg(test)]
const MENU_VERSION_DEPTH: u16 = 44;
#[cfg(test)]
const MENU_FRAME_MASK_DEPTH: u16 = 50;
#[cfg(test)]
const MENU_TOP_TITLE_DEPTH: u16 = 51;
const MENU_HELP_LABEL_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (217.65, 56.5),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (0.0, 0.0),
    (31.2, 120.85),
    (15.0, 16.0),
);
const MENU_POLARISATION_LABEL_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (217.65, 113.8),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (0.0, 0.0),
    (25.8, 125.8),
    (15.0, 16.0),
);
const MENU_MATCHES_LABEL_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (217.65, 171.1),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (0.0, 0.0),
    (32.35, 119.7),
    (15.0, 16.0),
);
const MENU_GRAVITY_LABEL_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (217.65, 228.35),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (0.0, 0.0),
    (15.75, 135.9),
    (15.0, 16.0),
);
const MENU_SPEED_LABEL_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (217.95, 285.65),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (0.0, 0.0),
    (25.1, 126.5),
    (15.0, 16.0),
);
const MENU_START_LABEL_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (217.65, 341.65),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (0.0, 0.0),
    (33.95, 117.55),
    (15.0, 16.0),
);
const MENU_QUESTION_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (259.3, 74.3),
    (2.156_280_5, 2.156_280_5),
    (0.0, 0.0),
    (0.6, 10.5),
    (11.8, 12.0),
);
const MENU_MATCH_SINGLE_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (237.75, 204.5),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (-21.95, -9.0),
    (19.05, 116.35),
    (15.0, 16.0),
);
const MENU_MATCH_BEST_OF_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (237.75, 204.5),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (-21.95, -9.0),
    (33.5, 102.45),
    (15.0, 16.0),
);
const MENU_GRAVITY_LOW_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (237.75, 261.75),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (-21.95, -9.0),
    (52.5, 83.55),
    (15.0, 16.0),
);
const MENU_GRAVITY_MEDIUM_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (237.75, 261.75),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (-21.95, -9.0),
    (36.25, 99.75),
    (15.0, 16.0),
);
const MENU_GRAVITY_HIGH_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (237.75, 261.75),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (-21.95, -9.0),
    (49.95, 86.0),
    (15.0, 16.0),
);
const MENU_GRAVITY_VERY_HIGH_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (237.75, 261.75),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (-21.95, -9.0),
    (31.25, 103.75),
    (15.0, 16.0),
);
const MENU_GRAVITY_BLACK_HOLE_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (237.75, 261.75),
    (MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
    (-21.95, -9.0),
    (27.35, 108.5),
    (15.0, 16.0),
);
const MENU_SPEED_DISABLED_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (246.9, 317.95),
    (0.671_997_1, 0.671_997_1),
    (-33.65, -9.0),
    (34.75, 100.75),
    (15.0, 16.0),
);
const MENU_SPEED_ENABLED_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (246.9, 317.95),
    (0.671_997_1, 0.671_997_1),
    (-33.65, -9.0),
    (35.9, 99.55),
    (15.0, 16.0),
);
const GRAVITY_PREVIEW_NEUTRAL_FRAME: GravityPreviewModeFrame = GravityPreviewModeFrame::new(
    2,
    [
        GravityPreviewRow::new(
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_SPRITE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                10.0,
                -0.45,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    2.7,
                    -0.4,
                ),
                GRAVITY_PREVIEW_BLUE,
            ),
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_NEGATIVE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                23.6,
                -0.45,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    30.85,
                    -0.4,
                ),
                GRAVITY_PREVIEW_RED_RIGHT,
            ),
            false,
        ),
        GravityPreviewRow::new(
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_SPRITE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                10.0,
                12.25,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    2.7,
                    12.25,
                ),
                GRAVITY_PREVIEW_RED_LEFT,
            ),
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_NEGATIVE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                23.6,
                12.25,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    30.85,
                    12.25,
                ),
                GRAVITY_PREVIEW_RED_RIGHT,
            ),
            true,
        ),
        GravityPreviewRow::new(
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_SPRITE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                10.0,
                24.95,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    2.7,
                    24.95,
                ),
                GRAVITY_PREVIEW_BLUE,
            ),
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_NEGATIVE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                23.6,
                24.95,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    30.85,
                    24.95,
                ),
                GRAVITY_PREVIEW_BLUE,
            ),
            true,
        ),
    ],
);
const GRAVITY_PREVIEW_OPPOSITE_REPELS_FRAME: GravityPreviewModeFrame = GravityPreviewModeFrame::new(
    3,
    [
        GravityPreviewRow::new(
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_NEGATIVE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                0.85,
                -0.45,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_NEGATIVE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    8.1,
                    -0.4,
                ),
                GRAVITY_PREVIEW_BLUE,
            ),
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_SPRITE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                32.75,
                -0.45,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_NEGATIVE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    25.5,
                    -0.4,
                ),
                GRAVITY_PREVIEW_RED_RIGHT,
            ),
            false,
        ),
        GRAVITY_PREVIEW_NEUTRAL_FRAME.rows[1],
        GRAVITY_PREVIEW_NEUTRAL_FRAME.rows[2],
    ],
);
const GRAVITY_PREVIEW_SAME_REPELS_FRAME: GravityPreviewModeFrame = GravityPreviewModeFrame::new(
    4,
    [
        GRAVITY_PREVIEW_NEUTRAL_FRAME.rows[0],
        GravityPreviewRow::new(
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_NEGATIVE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                0.85,
                12.2,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_NEGATIVE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    8.1,
                    12.25,
                ),
                GRAVITY_PREVIEW_RED_LEFT,
            ),
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_SPRITE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                32.75,
                12.2,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_NEGATIVE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    25.5,
                    12.25,
                ),
                GRAVITY_PREVIEW_RED_LEFT,
            ),
            true,
        ),
        GravityPreviewRow::new(
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_NEGATIVE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                0.85,
                24.95,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_NEGATIVE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    8.1,
                    24.95,
                ),
                GRAVITY_PREVIEW_BLUE,
            ),
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_SPRITE_SCALE,
                GRAVITY_PREVIEW_SPRITE_SCALE,
                32.75,
                24.95,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_NEGATIVE_SCALE,
                    GRAVITY_PREVIEW_SPRITE_SCALE,
                    25.5,
                    24.95,
                ),
                GRAVITY_PREVIEW_BLUE,
            ),
            true,
        ),
    ],
);
const GRAVITY_PREVIEW_ALL_REPEL_FRAME: GravityPreviewModeFrame = GravityPreviewModeFrame::new(
    5,
    [
        GravityPreviewRow::new(
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_NEGATIVE_SCALE,
                GRAVITY_PREVIEW_NEGATIVE_SCALE,
                0.55,
                -0.35,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_NEGATIVE_SCALE,
                    GRAVITY_PREVIEW_NEGATIVE_SCALE,
                    7.85,
                    -0.4,
                ),
                GRAVITY_PREVIEW_BLUE,
            ),
            GravityPreviewPlacement::new(
                GRAVITY_PREVIEW_SPRITE_SCALE,
                GRAVITY_PREVIEW_NEGATIVE_SCALE,
                32.8,
                -0.35,
            ),
            GravityPreviewBallPlacement::new(
                GravityPreviewPlacement::new(
                    GRAVITY_PREVIEW_NEGATIVE_SCALE,
                    GRAVITY_PREVIEW_NEGATIVE_SCALE,
                    25.55,
                    -0.4,
                ),
                GRAVITY_PREVIEW_RED_RIGHT,
            ),
            false,
        ),
        GRAVITY_PREVIEW_SAME_REPELS_FRAME.rows[1],
        GRAVITY_PREVIEW_SAME_REPELS_FRAME.rows[2],
    ],
);
const HELP_BACK_LABEL_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (209.7, 341.45),
    (0.809_463_5, 0.816_665_65),
    (0.0, 0.0),
    (56.4, 95.6),
    (15.0, 16.0),
);
const HELP_TITLE_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (164.35, 56.55),
    (1.439_559_9, 1.439_559_9),
    (0.0, 0.0),
    (36.85, 113.3),
    (11.8, 12.0),
);
const HELP_BODY_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (196.05, 93.65),
    (1.0, 1.0),
    (0.0, 0.0),
    (-114.15, 277.8),
    (11.8, 12.0),
);
const HELP_LABEL_FLATTEN_STEPS: u8 = 4;
const HELP_LABEL_TEXTURE_SUPERSAMPLE: u16 = 4;
#[cfg(test)]
const HELP_BODY_LINES: [&str; 8] = [
    "1. Try to shoot as many balls as possible into your opponents goal-line.",
    "2. Big balls score more points than small balls.",
    "3. Balls are subject to forces, the attract each other...",
    "...or reject eatch other - that depends on the gravity mode you play.",
    "4. Touching balls merge into bigger ones.",
    "5. Very big balls start to burn - don't touch them, they paralyze you.",
    "6. In SpeedGravity-Mode faster balls score more",
    "and the balls take up speed, when they are reflected by a paddel.",
];
#[cfg(test)]
const HELP_BODY_TEXT_X: f32 = 81.75;
#[cfg(test)]
const HELP_BODY_TEXT_FONT_SIZE: u16 = 12;
#[cfg(test)]
const HELP_BODY_TEXT_BASELINES: [f32; 8] = [
    105.45, 122.45, 139.45, 156.45, 173.45, 190.45, 207.45, 224.45,
];
const HELP_KEY_W_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (98.9, 245.65),
    (0.777_298, 0.777_298),
    (0.0, 0.0),
    (65.55, 89.75),
    (16.3, 18.0),
);
const HELP_KEY_D_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (126.5, 273.65),
    (0.777_298, 0.777_298),
    (0.0, 0.0),
    (67.6, 88.25),
    (16.3, 18.0),
);
const HELP_KEY_S_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (98.7, 273.65),
    (0.777_298, 0.777_298),
    (0.0, 0.0),
    (68.05, 87.75),
    (16.3, 18.0),
);
const HELP_RED_MOVE_UP_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (39.5, 247.95),
    (1.0, 1.0),
    (0.0, 0.0),
    (49.95, 100.1),
    (11.8, 12.0),
);
const HELP_RED_MOVE_DOWN_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (32.1, 276.95),
    (1.0, 1.0),
    (0.0, 0.0),
    (42.25, 107.8),
    (11.8, 12.0),
);
const HELP_RED_SHOOT_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (145.1, 276.95),
    (1.0, 1.0),
    (0.0, 0.0),
    (57.85, 91.75),
    (11.8, 12.0),
);
const HELP_BLUE_SHOOT_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (254.05, 276.95),
    (1.0, 1.0),
    (0.0, 0.0),
    (57.85, 91.75),
    (11.8, 12.0),
);
const HELP_BLUE_MOVE_UP_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (359.65, 247.95),
    (1.0, 1.0),
    (0.0, 0.0),
    (49.95, 100.1),
    (11.8, 12.0),
);
const HELP_BLUE_MOVE_DOWN_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (367.05, 276.95),
    (1.0, 1.0),
    (0.0, 0.0),
    (42.25, 107.8),
    (11.8, 12.0),
);
const HELP_PLAYER_RED_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (98.55, 307.45),
    (1.0, 1.0),
    (0.0, 0.0),
    (44.4, 105.75),
    (11.8, 12.0),
);
const HELP_PLAYER_BLUE_TEXT: SwfTextPlacement = SwfTextPlacement::new(
    (299.5, 307.45),
    (1.0, 1.0),
    (0.0, 0.0),
    (42.55, 107.6),
    (11.8, 12.0),
);

#[cfg(test)]
fn menu_button_label(action: MenuAction) -> (&'static str, SwfTextVisual) {
    (
        menu_label_definition(action).text,
        menu_label_placement(action).visual(),
    )
}

fn menu_label_definition(action: MenuAction) -> &'static menu_label_texts::MenuLabelDefinition {
    match action {
        MenuAction::Start => &menu_label_texts::START,
        MenuAction::Help => &menu_label_texts::HELP,
        MenuAction::Polarisation => &menu_label_texts::POLARISATION,
        MenuAction::Matches => &menu_label_texts::MATCHES,
        MenuAction::Gravity => &menu_label_texts::GRAVITY,
        MenuAction::Speed => &menu_label_texts::SPEED,
    }
}

fn menu_label_placement(action: MenuAction) -> SwfTextPlacement {
    match action {
        MenuAction::Start => MENU_START_LABEL_TEXT,
        MenuAction::Help => MENU_HELP_LABEL_TEXT,
        MenuAction::Polarisation => MENU_POLARISATION_LABEL_TEXT,
        MenuAction::Matches => MENU_MATCHES_LABEL_TEXT,
        MenuAction::Gravity => MENU_GRAVITY_LABEL_TEXT,
        MenuAction::Speed => MENU_SPEED_LABEL_TEXT,
    }
}

#[cfg(test)]
fn menu_match_type_value_definition(matches: u8) -> &'static menu_value_texts::MenuValueDefinition {
    match matches {
        3 => &menu_value_texts::MATCH_BEST_OF_3,
        5 => &menu_value_texts::MATCH_BEST_OF_5,
        7 => &menu_value_texts::MATCH_BEST_OF_7,
        _ => &menu_value_texts::MATCH_SINGLE,
    }
}

#[cfg(test)]
fn menu_match_type_value_placement(matches: u8) -> SwfTextPlacement {
    match matches {
        3 | 5 | 7 => MENU_MATCH_BEST_OF_TEXT,
        _ => MENU_MATCH_SINGLE_TEXT,
    }
}

#[cfg(test)]
fn menu_gravity_strength_value_definition(
    gravity: GravityStrength,
) -> &'static menu_value_texts::MenuValueDefinition {
    match gravity {
        GravityStrength::G1 => &menu_value_texts::GRAVITY_LOW,
        GravityStrength::G2 => &menu_value_texts::GRAVITY_MEDIUM,
        GravityStrength::G3 => &menu_value_texts::GRAVITY_HIGH,
        GravityStrength::G4 => &menu_value_texts::GRAVITY_VERY_HIGH,
        GravityStrength::G5 => &menu_value_texts::GRAVITY_BLACK_HOLE,
    }
}

#[cfg(test)]
fn menu_gravity_strength_value_placement(gravity: GravityStrength) -> SwfTextPlacement {
    match gravity {
        GravityStrength::G1 => MENU_GRAVITY_LOW_TEXT,
        GravityStrength::G2 => MENU_GRAVITY_MEDIUM_TEXT,
        GravityStrength::G3 => MENU_GRAVITY_HIGH_TEXT,
        GravityStrength::G4 => MENU_GRAVITY_VERY_HIGH_TEXT,
        GravityStrength::G5 => MENU_GRAVITY_BLACK_HOLE_TEXT,
    }
}

#[cfg(test)]
fn menu_speed_value_definition(speed: SpeedMode) -> &'static menu_value_texts::MenuValueDefinition {
    match speed {
        SpeedMode::Normal => &menu_value_texts::SPEED_DISABLED,
        SpeedMode::Fast => &menu_value_texts::SPEED_ENABLED,
    }
}

#[cfg(test)]
fn menu_speed_value_placement(speed: SpeedMode) -> SwfTextPlacement {
    match speed {
        SpeedMode::Normal => MENU_SPEED_DISABLED_TEXT,
        SpeedMode::Fast => MENU_SPEED_ENABLED_TEXT,
    }
}

fn menu_match_type_value_texture(matches: u8, values: &MenuValueTextures) -> &SwfTextTexture {
    match matches {
        3 => &values.match_best_of_3,
        5 => &values.match_best_of_5,
        7 => &values.match_best_of_7,
        _ => &values.match_single,
    }
}

fn menu_gravity_strength_value_texture(
    gravity: GravityStrength,
    values: &MenuValueTextures,
) -> &SwfTextTexture {
    match gravity {
        GravityStrength::G1 => &values.gravity_low,
        GravityStrength::G2 => &values.gravity_medium,
        GravityStrength::G3 => &values.gravity_high,
        GravityStrength::G4 => &values.gravity_very_high,
        GravityStrength::G5 => &values.gravity_black_hole,
    }
}

fn menu_speed_value_texture(speed: SpeedMode, values: &MenuValueTextures) -> &SwfTextTexture {
    match speed {
        SpeedMode::Normal => &values.speed_disabled,
        SpeedMode::Fast => &values.speed_enabled,
    }
}

#[cfg(test)]
fn menu_match_type_text(matches: u8) -> SwfTextVisual {
    match matches {
        3 | 5 | 7 => MENU_MATCH_BEST_OF_TEXT,
        _ => MENU_MATCH_SINGLE_TEXT,
    }
    .visual()
}

#[cfg(test)]
fn menu_gravity_strength_text(gravity: GravityStrength) -> SwfTextVisual {
    match gravity {
        GravityStrength::G1 => MENU_GRAVITY_LOW_TEXT,
        GravityStrength::G2 => MENU_GRAVITY_MEDIUM_TEXT,
        GravityStrength::G3 => MENU_GRAVITY_HIGH_TEXT,
        GravityStrength::G4 => MENU_GRAVITY_VERY_HIGH_TEXT,
        GravityStrength::G5 => MENU_GRAVITY_BLACK_HOLE_TEXT,
    }
    .visual()
}

#[cfg(test)]
fn menu_speed_text(speed: SpeedMode) -> SwfTextVisual {
    match speed {
        SpeedMode::Normal => MENU_SPEED_DISABLED_TEXT,
        SpeedMode::Fast => MENU_SPEED_ENABLED_TEXT,
    }
    .visual()
}

#[cfg(test)]
fn help_body_line_visual(index: usize) -> SwfLineVisual {
    SwfLineVisual {
        x: HELP_BODY_TEXT_X,
        baseline_y: HELP_BODY_TEXT_BASELINES[index],
        font_size: HELP_BODY_TEXT_FONT_SIZE,
    }
}

fn menu_buttons() -> [Button; 6] {
    [
        // SWF root frame 56 places button 52 here; raw `GotoFrame 56` opens
        // the frame-57 instruction screen.
        Button {
            visual: swf_button_visual(266.0, 63.35, MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
            action: MenuAction::Help,
        },
        // Button 48 cycles sprite 42's gravity-mode clip.
        Button {
            visual: swf_button_visual(266.0, 120.7, MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
            action: MenuAction::Polarisation,
        },
        // Button 50 cycles sprite 47's match-count clip.
        Button {
            visual: swf_button_visual(266.0, 177.95, MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
            action: MenuAction::Matches,
        },
        // Button 64 cycles sprite 63's gravity-strength clip.
        Button {
            visual: swf_button_visual(266.0, 235.25, MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
            action: MenuAction::Gravity,
        },
        // Button 66 cycles sprite 70's speed clip plus the paired preview name.
        Button {
            visual: swf_button_visual(266.3, 292.5, MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
            action: MenuAction::Speed,
        },
        // Button 39 is the bottom `single match` button; raw `GotoFrame 57; Play`
        // enters frame 58's `startmatch` label.
        Button {
            visual: swf_button_visual(266.0, 348.5, MENU_BUTTON_SCALE, MENU_BUTTON_SCALE),
            action: MenuAction::Start,
        },
    ]
}

fn swf_button_visual(tx: f32, ty: f32, scale_x: f32, scale_y: f32) -> ButtonShapeVisual {
    let bounds = button_shape22::SHAPE.bounds;
    ButtonShapeVisual {
        rect: Rect::new(
            bounds.x_min.mul_add(scale_x, tx),
            bounds.y_min.mul_add(scale_y, ty),
            (bounds.x_max - bounds.x_min) * scale_x,
            (bounds.y_max - bounds.y_min) * scale_y,
        ),
        center: SwfPoint::new(tx, ty),
        scale_x,
        scale_y,
        line_width: button_shape22::LINE_WIDTH * (scale_x.abs() + scale_y.abs()) * 0.5,
        fill: swf_rgb_array(button_shape22::FILL_RGB),
        outline: swf_rgb_array(button_shape22::LINE_RGB),
    }
}

fn help_back_visual() -> ButtonShapeVisual {
    swf_button_visual(
        268.0,
        349.2,
        HELP_BACK_BUTTON_SCALE_X,
        HELP_BACK_BUTTON_SCALE_Y,
    )
}

const fn swf_rgb_array(rgb: [u8; 3]) -> Color {
    Color::new(
        rgb[0] as f32 / 255.0,
        rgb[1] as f32 / 255.0,
        rgb[2] as f32 / 255.0,
        1.0,
    )
}

const fn swf_gradient_stop_ratio(stop: [u8; 5]) -> f32 {
    stop[0] as f32 / 255.0
}

const fn swf_gradient_stop_color(stop: [u8; 5]) -> Color {
    Color::new(
        stop[1] as f32 / 255.0,
        stop[2] as f32 / 255.0,
        stop[3] as f32 / 255.0,
        stop[4] as f32 / 255.0,
    )
}

fn swf_rgba_array(rgba: [u8; 4]) -> Color {
    Color::new(
        f32::from(rgba[0]) / 255.0,
        f32::from(rgba[1]) / 255.0,
        f32::from(rgba[2]) / 255.0,
        f32::from(rgba[3]) / 255.0,
    )
}

fn startup_phase(ticks: u32) -> StartupPhase {
    if ticks < STARTUP_XML_WAIT_TICK {
        StartupPhase::Loading
    } else {
        StartupPhase::XmlWait
    }
}

fn startup_xml_wait_frame(ticks: u32) -> u32 {
    ticks.saturating_sub(STARTUP_XML_WAIT_TICK) + 1
}

fn startup_abort_button_alpha(ticks: u32) -> Option<f32> {
    let frame = startup_xml_wait_frame(ticks);
    if frame < STARTUP_ABORT_BUTTON_FIRST_FRAME {
        return None;
    }
    if frame >= STARTUP_ABORT_BUTTON_FULL_ALPHA_FRAME {
        return Some(1.0);
    }
    let fade_frame = frame - STARTUP_ABORT_BUTTON_FIRST_FRAME;
    let fade_len = STARTUP_ABORT_BUTTON_FULL_ALPHA_FRAME - STARTUP_ABORT_BUTTON_FIRST_FRAME;
    Some(((fade_frame * 256 + fade_len / 2) / fade_len) as f32 / 256.0)
}

fn startup_abort_button_visual() -> ButtonShapeVisual {
    swf_button_visual(
        STARTUP_ABORT_BUTTON_X,
        STARTUP_ABORT_BUTTON_Y,
        STARTUP_ABORT_BUTTON_SCALE,
        STARTUP_ABORT_BUTTON_SCALE,
    )
}

#[cfg(test)]
fn startup_abort_button_contains(ticks: u32, x: f32, y: f32) -> bool {
    startup_abort_button_alpha(ticks).is_some()
        && button_shape_contains(startup_abort_button_visual(), x, y)
}

fn hovered_button(screen: Screen, buttons: &[Button], x: f32, y: f32) -> Option<ButtonSurface> {
    match screen {
        Screen::Startup => button_shape_contains(startup_abort_button_visual(), x, y)
            .then_some(ButtonSurface::StartupAbort),
        Screen::Menu => buttons
            .iter()
            .find(|button| button_shape_contains(button.visual, x, y))
            .map(|button| ButtonSurface::Menu(button.action)),
        Screen::Help => {
            button_shape_contains(help_back_visual(), x, y).then_some(ButtonSurface::HelpBack)
        },
        Screen::Playing => None,
    }
}

fn cursor_icon_for_hover(
    screen: Screen,
    startup_ticks: u32,
    buttons: &[Button],
    x: f32,
    y: f32,
) -> CursorIcon {
    if interactive_surface_at(screen, startup_ticks, buttons, x, y).is_some() {
        CursorIcon::Pointer
    } else {
        CursorIcon::Default
    }
}

fn interactive_surface_at(
    screen: Screen,
    startup_ticks: u32,
    buttons: &[Button],
    x: f32,
    y: f32,
) -> Option<InteractiveSurface> {
    if hovered_button_with_startup_timing(screen, startup_ticks, buttons, x, y).is_some() {
        return Some(InteractiveSurface::Button);
    }
    if external_link_at(screen, x, y).is_some() {
        return Some(InteractiveSurface::ExternalLink);
    }
    if matches!(screen, Screen::Help | Screen::Playing)
        && header_link_contains(HeaderLink::BackToMenu, x, y)
    {
        return Some(InteractiveSurface::HeaderBack);
    }
    None
}

fn action_surface_at(
    screen: Screen,
    startup_ticks: u32,
    buttons: &[Button],
    x: f32,
    y: f32,
) -> Option<ActionSurface> {
    if let Some(button) = hovered_button_with_startup_timing(screen, startup_ticks, buttons, x, y) {
        return Some(ActionSurface::Button(button));
    }
    if let Some(link) = external_link_at(screen, x, y) {
        return Some(ActionSurface::ExternalLink(link));
    }
    if matches!(screen, Screen::Help | Screen::Playing)
        && header_link_contains(HeaderLink::BackToMenu, x, y)
    {
        return Some(ActionSurface::HeaderBack);
    }
    None
}

fn hovered_button_with_startup_timing(
    screen: Screen,
    startup_ticks: u32,
    buttons: &[Button],
    x: f32,
    y: f32,
) -> Option<ButtonSurface> {
    let button = hovered_button(screen, buttons, x, y)?;
    if matches!(button, ButtonSurface::StartupAbort)
        && startup_abort_button_alpha(startup_ticks).is_none()
    {
        None
    } else {
        Some(button)
    }
}

fn swf_release_action(
    pressed: Option<ActionSurface>,
    released: Option<ActionSurface>,
) -> Option<ActionSurface> {
    if pressed == released { pressed } else { None }
}

fn swf_drag_out_action(
    pressed: Option<ActionSurface>,
    current: Option<ActionSurface>,
) -> Option<ActionSurface> {
    match pressed {
        Some(ActionSurface::Button(ButtonSurface::Menu(MenuAction::Start)))
            if current != pressed =>
        {
            pressed
        },
        _ => None,
    }
}

fn swf_release_or_drag_out_action(
    pressed: Option<ActionSurface>,
    released: Option<ActionSurface>,
) -> Option<ActionSurface> {
    swf_release_action(pressed, released).or_else(|| swf_drag_out_action(pressed, released))
}

fn read_controls() -> Controls {
    Controls {
        blue_up: is_key_down(BLUE_UP_KEY),
        blue_down: is_key_down(BLUE_DOWN_KEY),
        blue_fire: is_key_down(BLUE_FIRE_KEY),
        red_up: is_key_down(RED_UP_KEY),
        red_down: is_key_down(RED_DOWN_KEY),
        red_fire: is_key_down(RED_FIRE_KEY),
    }
}

fn startup_loading_title_text(total_bytes: usize) -> String {
    format!("loading {} kbyte...", total_bytes / 1024)
}

fn startup_loading_percent_text(loaded_bytes: usize, total_bytes: usize) -> String {
    let percent = loaded_bytes
        .saturating_mul(100)
        .checked_div(total_bytes)
        .unwrap_or(0);
    format!("{percent} %")
}

fn draw_startup_loading(device_arial_font: Option<&Font>) {
    draw_swf_text_center(
        STARTUP_GRAVITY_TEXT_VALUE,
        STARTUP_GRAVITY_TEXT,
        STARTUP_TEXT,
        device_arial_font,
    );
    draw_swf_text_center(
        &startup_loading_title_text(SWF_FILE_LEN_BYTES),
        STARTUP_LOADING_TITLE_TEXT,
        STARTUP_TEXT,
        device_arial_font,
    );
    draw_swf_text_center(
        &startup_loading_percent_text(SWF_FILE_LEN_BYTES, SWF_FILE_LEN_BYTES),
        STARTUP_LOADING_PERCENT_TEXT,
        STARTUP_TEXT,
        device_arial_font,
    );
}

fn draw_startup_xml_wait(
    startup_ticks: u32,
    device_arial_font: Option<&Font>,
    device_trebuchet_font: Option<&Font>,
) {
    draw_swf_text_center(
        STARTUP_RETRIEVING_TEXT_VALUE,
        STARTUP_RETRIEVING_TEXT,
        STARTUP_TEXT,
        device_arial_font,
    );
    if let Some(alpha) = startup_abort_button_alpha(startup_ticks) {
        let mut visual = startup_abort_button_visual();
        visual.fill = color_with_alpha(visual.fill, alpha);
        visual.outline = color_with_alpha(visual.outline, alpha);
        draw_button_shape(visual);
        draw_swf_text_center(
            STARTUP_ABORT_LABEL,
            STARTUP_ABORT_LABEL_TEXT,
            color_with_alpha(SWF_WHITE, alpha),
            device_trebuchet_font,
        );
    }
}

fn color_with_alpha(mut color: Color, alpha: f32) -> Color {
    color.a *= alpha;
    color
}

fn draw_stage_frame_with_flash(options: StageFrameOptions, goal_flash: GoalFlash) {
    draw_stage_background();
    draw_panel_fill();
    draw_goal_line(
        stage_left_goal_x(options),
        Side::Red,
        goal_flash.left,
        &LEFT_GOAL_FLASH,
    );
    draw_goal_line(
        stage_right_goal_x(options),
        Side::Blue,
        goal_flash.right,
        &RIGHT_GOAL_FLASH,
    );
    if options.draw_goal_paddles {
        draw_goal_paddle_marker(514.7, 214.2);
        draw_goal_paddle_marker(35.0, 214.15);
        for &glow in static_goal_paddle_glows(options) {
            draw_static_paddle_glow(glow);
        }
    }
    if panel_outline_enabled(options) {
        draw_panel_outline();
    }
    for_each_static_side_marker(options, draw_static_side_marker);
}

#[cfg(test)]
fn static_side_marker_ys(options: StageFrameOptions) -> &'static [f32] {
    if options.draw_static_side_markers {
        &SIDE_MARKER_YS
    } else {
        &[]
    }
}

fn for_each_static_side_marker(
    options: StageFrameOptions,
    mut emit: impl FnMut(StaticSideMarkerVisual),
) {
    if !options.draw_static_side_markers {
        return;
    }

    for &visual in &STATIC_SIDE_MARKER_VISUALS {
        emit(visual);
    }
}

const fn stage_left_goal_x(options: StageFrameOptions) -> f32 {
    options.left_goal_x
}

const fn stage_right_goal_x(options: StageFrameOptions) -> f32 {
    options.right_goal_x
}

fn draw_stage_background() {
    draw_swf_rect(STAGE_BOUNDS, STAGE_RED);
}

fn panel_outline_enabled(options: StageFrameOptions) -> bool {
    options.draw_panel_outline
}

const STATIC_GOAL_PADDLE_GLOWS: [StaticPaddleGlowVisual; 2] = [
    StaticPaddleGlowVisual {
        x: 514.7,
        y: 214.45,
        scale_x: STATIC_PADDLE_GLOW_SCALE_X,
        scale_y: STATIC_PADDLE_GLOW_SCALE_Y,
        color: STATIC_RIGHT_PADDLE_GLOW_COLOR,
    },
    StaticPaddleGlowVisual {
        x: 35.0,
        y: 214.4,
        scale_x: STATIC_PADDLE_GLOW_SCALE_X,
        scale_y: STATIC_PADDLE_GLOW_SCALE_Y,
        color: STATIC_LEFT_PADDLE_GLOW_COLOR,
    },
];

fn static_goal_paddle_glows(options: StageFrameOptions) -> &'static [StaticPaddleGlowVisual] {
    if options.draw_goal_paddles {
        &STATIC_GOAL_PADDLE_GLOWS
    } else {
        &[]
    }
}

fn draw_panel_outline() {
    let bounds = panel_outline_bounds();
    if bounds.x > STAGE_W as f32
        || bounds.x + bounds.w < 0.0
        || bounds.y > STAGE_H as f32
        || bounds.y + bounds.h < 0.0
    {
        return;
    }
    let primary = panel_outline_primary_points();
    let lower_right = panel_outline_lower_right_points();
    draw_open_polyline(&primary, panel_chrome_shapes::OUTLINE_LINE_WIDTH, SWF_WHITE);
    draw_open_polyline(
        &lower_right,
        panel_chrome_shapes::OUTLINE_LINE_WIDTH,
        SWF_WHITE,
    );
}

fn draw_retained_frame_mask_outline() {
    let bounds = panel_outline_bounds();
    if bounds.x > STAGE_W as f32
        || bounds.x + bounds.w < 0.0
        || bounds.y > STAGE_H as f32
        || bounds.y + bounds.h < 0.0
    {
        return;
    }
    let primary = retained_frame_mask_outline_primary_points();
    let lower_right = retained_frame_mask_outline_lower_right_points();
    draw_open_polyline(&primary, panel_chrome_shapes::OUTLINE_LINE_WIDTH, SWF_WHITE);
    draw_open_polyline(
        &lower_right,
        panel_chrome_shapes::OUTLINE_LINE_WIDTH,
        SWF_WHITE,
    );
}

fn panel_outline_bounds() -> RectVisual {
    rect_from_swf_bounds(panel_chrome_shapes::PANEL_OUTLINE_SHAPE.bounds)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HelpContentVisual {
    Title,
    Body,
    BackButton,
    BackLabel,
    Control(HelpControlVisual),
}

const HELP_CONTENT_DISPLAY_LIST: [HelpContentVisual; 24] = [
    HelpContentVisual::Title,
    HelpContentVisual::Body,
    HelpContentVisual::BackButton,
    HelpContentVisual::BackLabel,
    HelpContentVisual::Control(HelpControlVisual::RedUpKeycap),
    HelpContentVisual::Control(HelpControlVisual::RedUpKey),
    HelpContentVisual::Control(HelpControlVisual::RedShootKeycap),
    HelpContentVisual::Control(HelpControlVisual::RedShootKey),
    HelpContentVisual::Control(HelpControlVisual::RedDownKeycap),
    HelpContentVisual::Control(HelpControlVisual::RedDownKey),
    HelpContentVisual::Control(HelpControlVisual::BlueUpKeycap),
    HelpContentVisual::Control(HelpControlVisual::BlueUpArrow),
    HelpContentVisual::Control(HelpControlVisual::BlueShootKeycap),
    HelpContentVisual::Control(HelpControlVisual::BlueShootArrow),
    HelpContentVisual::Control(HelpControlVisual::BlueDownKeycap),
    HelpContentVisual::Control(HelpControlVisual::BlueDownArrow),
    HelpContentVisual::Control(HelpControlVisual::RedMoveUpLabel),
    HelpContentVisual::Control(HelpControlVisual::RedMoveDownLabel),
    HelpContentVisual::Control(HelpControlVisual::RedShootLabel),
    HelpContentVisual::Control(HelpControlVisual::BlueShootLabel),
    HelpContentVisual::Control(HelpControlVisual::BlueMoveUpLabel),
    HelpContentVisual::Control(HelpControlVisual::BlueMoveDownLabel),
    HelpContentVisual::Control(HelpControlVisual::RedPlayerLabel),
    HelpContentVisual::Control(HelpControlVisual::BluePlayerLabel),
];

#[cfg(test)]
const HELP_CONTENT_DEPTHS_AND_CHARS: [(u16, u16); 24] = [
    (2, 101),
    (3, 102),
    (4, 103),
    (6, 104),
    (7, 105),
    (8, 107),
    (9, 105),
    (10, 108),
    (11, 105),
    (12, 109),
    (13, 105),
    (14, 110),
    (15, 105),
    (16, 110),
    (17, 105),
    (18, 110),
    (19, 111),
    (20, 112),
    (21, 113),
    (22, 114),
    (23, 115),
    (24, 116),
    (25, 117),
    (26, 118),
];

#[cfg(test)]
const HELP_VERSION_DEPTH: u16 = 44;
#[cfg(test)]
const HELP_ROUNDS_PLAYED_DEPTH: u16 = 47;
#[cfg(test)]
const HELP_FRAME_MASK_DEPTH: u16 = 50;
#[cfg(test)]
const HELP_TOP_TITLE_DEPTH: u16 = 51;
#[cfg(test)]
const HELP_RETAINED_EMPTY_FACTOR_SPRITE_DEPTH: u16 = 52;
#[cfg(test)]
const HELP_RETAINED_EMPTY_FACTOR_SPRITE_ID: u16 = 83;
#[cfg(test)]
const HELP_RETAINED_EMPTY_FACTOR_CHILD_SPRITE_ID: u16 = 82;
#[cfg(test)]
const HELP_LEFT_GOAL_DEPTH: u16 = 58;
#[cfg(test)]
const HELP_RIGHT_GOAL_DEPTH: u16 = 60;
#[cfg(test)]
const HELP_RIGHT_PADDLE_DEPTH: u16 = 62;
#[cfg(test)]
const HELP_LEFT_PADDLE_DEPTH: u16 = 63;
#[cfg(test)]
const HELP_RIGHT_PADDLE_GLOW_DEPTH: u16 = 64;
#[cfg(test)]
const HELP_LEFT_PADDLE_GLOW_DEPTH: u16 = 65;
#[cfg(test)]
const HELP_PANEL_OUTLINE_DEPTH: u16 = 66;
#[cfg(test)]
const HELP_FIRST_SIDE_MARKER_DEPTH: u16 = 67;
#[cfg(test)]
const HELP_LAST_SIDE_MARKER_DEPTH: u16 = 126;
#[cfg(test)]
const HELP_BACK_LINK_DEPTH: u16 = 127;
#[cfg(test)]
const HELP_SPONSOR_LOGO_DEPTH: u16 = 129;

fn draw_help_content(labels: &HelpLabelTextures) {
    for &visual in &HELP_CONTENT_DISPLAY_LIST {
        draw_help_content_visual(visual, labels);
    }
}

fn draw_help_content_visual(visual: HelpContentVisual, labels: &HelpLabelTextures) {
    match visual {
        HelpContentVisual::Title => draw_swf_text_texture(&labels.title),
        HelpContentVisual::Body => draw_swf_text_texture(&labels.body),
        HelpContentVisual::BackButton => draw_button_shape(help_back_visual()),
        HelpContentVisual::BackLabel => draw_swf_text_texture(&labels.back),
        HelpContentVisual::Control(control) => draw_help_control_visual(control, labels),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HelpControlVisual {
    RedUpKeycap,
    RedUpKey,
    RedShootKeycap,
    RedShootKey,
    RedDownKeycap,
    RedDownKey,
    BlueUpKeycap,
    BlueUpArrow,
    BlueShootKeycap,
    BlueShootArrow,
    BlueDownKeycap,
    BlueDownArrow,
    RedMoveUpLabel,
    RedMoveDownLabel,
    RedShootLabel,
    BlueShootLabel,
    BlueMoveUpLabel,
    BlueMoveDownLabel,
    RedPlayerLabel,
    BluePlayerLabel,
}

#[cfg(test)]
const HELP_CONTROL_DISPLAY_LIST: [HelpControlVisual; 20] = [
    HelpControlVisual::RedUpKeycap,
    HelpControlVisual::RedUpKey,
    HelpControlVisual::RedShootKeycap,
    HelpControlVisual::RedShootKey,
    HelpControlVisual::RedDownKeycap,
    HelpControlVisual::RedDownKey,
    HelpControlVisual::BlueUpKeycap,
    HelpControlVisual::BlueUpArrow,
    HelpControlVisual::BlueShootKeycap,
    HelpControlVisual::BlueShootArrow,
    HelpControlVisual::BlueDownKeycap,
    HelpControlVisual::BlueDownArrow,
    HelpControlVisual::RedMoveUpLabel,
    HelpControlVisual::RedMoveDownLabel,
    HelpControlVisual::RedShootLabel,
    HelpControlVisual::BlueShootLabel,
    HelpControlVisual::BlueMoveUpLabel,
    HelpControlVisual::BlueMoveDownLabel,
    HelpControlVisual::RedPlayerLabel,
    HelpControlVisual::BluePlayerLabel,
];

#[cfg(test)]
const HELP_CONTROL_ROOT_FRAME: u16 = 57;

#[cfg(test)]
const HELP_CONTROL_DEPTHS_AND_CHARS: [(u16, u16); 20] = [
    (7, 105),
    (8, 107),
    (9, 105),
    (10, 108),
    (11, 105),
    (12, 109),
    (13, 105),
    (14, 110),
    (15, 105),
    (16, 110),
    (17, 105),
    (18, 110),
    (19, 111),
    (20, 112),
    (21, 113),
    (22, 114),
    (23, 115),
    (24, 116),
    (25, 117),
    (26, 118),
];

fn draw_help_control_visual(visual: HelpControlVisual, labels: &HelpLabelTextures) {
    match visual {
        HelpControlVisual::RedUpKeycap => draw_keycap_base(154.1, 256.15),
        HelpControlVisual::RedUpKey => draw_swf_text_texture(&labels.key_w),
        HelpControlVisual::RedShootKeycap => draw_keycap_base(181.7, 284.1),
        HelpControlVisual::RedShootKey => draw_swf_text_texture(&labels.key_d),
        HelpControlVisual::RedDownKeycap => draw_keycap_base(153.9, 284.1),
        HelpControlVisual::RedDownKey => draw_swf_text_texture(&labels.key_s),
        HelpControlVisual::BlueUpKeycap => draw_keycap_base(390.1, 256.15),
        HelpControlVisual::BlueUpArrow => draw_arrow_glyph(390.1, 256.15, ArrowDir::Up),
        HelpControlVisual::BlueShootKeycap => draw_keycap_base(362.3, 284.1),
        HelpControlVisual::BlueShootArrow => draw_arrow_glyph(362.3, 284.1, ArrowDir::Left),
        HelpControlVisual::BlueDownKeycap => draw_keycap_base(390.1, 284.1),
        HelpControlVisual::BlueDownArrow => draw_arrow_glyph(390.1, 284.1, ArrowDir::Down),
        HelpControlVisual::RedMoveUpLabel => draw_swf_text_texture(&labels.red_move_up),
        HelpControlVisual::RedMoveDownLabel => draw_swf_text_texture(&labels.red_move_down),
        HelpControlVisual::RedShootLabel => draw_swf_text_texture(&labels.red_shoot),
        HelpControlVisual::BlueShootLabel => draw_swf_text_texture(&labels.blue_shoot),
        HelpControlVisual::BlueMoveUpLabel => draw_swf_text_texture(&labels.blue_move_up),
        HelpControlVisual::BlueMoveDownLabel => draw_swf_text_texture(&labels.blue_move_down),
        HelpControlVisual::RedPlayerLabel => draw_swf_text_texture(&labels.player_red),
        HelpControlVisual::BluePlayerLabel => draw_swf_text_texture(&labels.player_blue),
    }
}

#[derive(Debug, Clone, Copy)]
enum ArrowDir {
    Left,
    Up,
    Down,
}

fn draw_arrow_glyph(cx: f32, cy: f32, dir: ArrowDir) {
    let visual = arrow_glyph_visual(cx, cy, dir);
    draw_filled_polygon_fan(&visual.points, visual.center, visual.color);
}

fn draw_keycap_base(cx: f32, cy: f32) {
    let visual = keycap_visual(cx, cy);
    if visual.bounds.x > STAGE_W as f32
        || visual.bounds.x + visual.bounds.w < 0.0
        || visual.bounds.y > STAGE_H as f32
        || visual.bounds.y + visual.bounds.h < 0.0
    {
        return;
    }

    let outer = keycap_outer_fill_points(visual);
    let inner = keycap_inner_fill_points(visual);
    draw_filled_polygon_triangulated(&outer, visual.shadow);
    draw_filled_polygon_triangulated(&inner, visual.fill);
    draw_open_polyline(
        &keycap_top_edge_points(visual),
        visual.line_width,
        visual.outline,
    );
    draw_open_polyline(
        &keycap_inner_edge_points(visual),
        visual.line_width,
        visual.outline,
    );
    draw_open_polyline(
        &keycap_outer_edge_points(visual),
        visual.line_width,
        visual.outline,
    );
}

fn keycap_visual(cx: f32, cy: f32) -> KeycapVisual {
    let shape = help_control_shapes::KEYCAP_SHAPE;
    let bounds = rect_from_bounds(cx, cy, shape.bounds, HELP_KEYCAP_SCALE);
    let line_width = help_control_shapes::KEYCAP_LINE_WIDTH * HELP_KEYCAP_SCALE;

    KeycapVisual {
        bounds,
        center: SwfPoint::new(cx, cy),
        scale: HELP_KEYCAP_SCALE,
        line_width,
        fill: swf_rgb_array(help_control_shapes::KEYCAP_FILL_RGB),
        shadow: swf_rgb_array(help_control_shapes::KEYCAP_SHADOW_RGB),
        outline: swf_rgb_array(help_control_shapes::KEYCAP_LINE_RGB),
    }
}

fn keycap_outer_fill_points(visual: KeycapVisual) -> Vec<SwfPoint> {
    keycap_contour_points(visual, help_control_shapes::KEYCAP_SHAPE.shadow, true)
}

fn keycap_inner_fill_points(visual: KeycapVisual) -> Vec<SwfPoint> {
    keycap_contour_points(visual, help_control_shapes::KEYCAP_SHAPE.fill, true)
}

fn keycap_top_edge_points(visual: KeycapVisual) -> Vec<SwfPoint> {
    keycap_contour_points(visual, help_control_shapes::KEYCAP_SHAPE.top_stroke, false)
}

fn keycap_inner_edge_points(visual: KeycapVisual) -> Vec<SwfPoint> {
    keycap_contour_points(
        visual,
        help_control_shapes::KEYCAP_SHAPE.inner_stroke,
        false,
    )
}

fn keycap_outer_edge_points(visual: KeycapVisual) -> Vec<SwfPoint> {
    keycap_contour_points(
        visual,
        help_control_shapes::KEYCAP_SHAPE.outer_stroke,
        false,
    )
}

fn keycap_contour_points(
    visual: KeycapVisual,
    contour: HelpControlContour,
    closed: bool,
) -> Vec<SwfPoint> {
    let mut points = Vec::with_capacity(
        1 + contour.segments.len() * usize::from(help_control_shapes::FLATTEN_STEPS),
    );
    let mut current = contour.start;
    points.push(keycap_stage_point(visual, current));
    for segment in contour.segments {
        match *segment {
            SwfPathSegment::Line(to) => {
                points.push(keycap_stage_point(visual, to));
                current = to;
            },
            SwfPathSegment::Quad { control, to } => {
                for step in 1..=help_control_shapes::FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(help_control_shapes::FLATTEN_STEPS);
                    points.push(keycap_stage_point(
                        visual,
                        quadratic_point(current, control, to, t),
                    ));
                }
                current = to;
            },
        }
    }
    if closed {
        pop_duplicate_last_point(&mut points);
    }
    points
}

fn keycap_stage_point(visual: KeycapVisual, local: SwfPoint) -> SwfPoint {
    SwfPoint::new(
        local.x.mul_add(visual.scale, visual.center.x),
        local.y.mul_add(visual.scale, visual.center.y),
    )
}

fn arrow_glyph_visual(cx: f32, cy: f32, dir: ArrowDir) -> ArrowGlyphVisual {
    let transform = arrow_glyph_transform(cx, cy, dir);

    ArrowGlyphVisual {
        points: arrow_glyph_points(transform),
        center: transform.transform_point(SwfPoint::new(0.0, 0.0)),
        color: swf_rgb_array(help_control_shapes::ARROW_FILL_RGB),
    }
}

fn arrow_glyph_transform(cx: f32, cy: f32, dir: ArrowDir) -> SwfAffineTransform {
    let tx = cx + HELP_ARROW_OFFSET_X;
    let ty = cy + HELP_ARROW_OFFSET_Y;
    match dir {
        ArrowDir::Left => {
            SwfAffineTransform::new(0.0, HELP_ARROW_SCALE, -HELP_ARROW_SCALE, 0.0, tx, ty)
        },
        ArrowDir::Up => {
            SwfAffineTransform::new(HELP_ARROW_SCALE, 0.0, 0.0, HELP_ARROW_SCALE, tx, ty)
        },
        ArrowDir::Down => {
            SwfAffineTransform::new(HELP_ARROW_SCALE, 0.0, 0.0, -HELP_ARROW_SCALE, tx, ty)
        },
    }
}

fn arrow_glyph_points(transform: SwfAffineTransform) -> Vec<SwfPoint> {
    let contour = help_control_shapes::ARROW_SHAPE.fill;
    let mut points = Vec::with_capacity(
        1 + contour.segments.len() * usize::from(help_control_shapes::FLATTEN_STEPS),
    );
    let mut current = contour.start;
    points.push(transform.transform_point(current));
    for segment in contour.segments {
        match *segment {
            SwfPathSegment::Line(to) => {
                points.push(transform.transform_point(to));
                current = to;
            },
            SwfPathSegment::Quad { control, to } => {
                for step in 1..=help_control_shapes::FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(help_control_shapes::FLATTEN_STEPS);
                    points
                        .push(transform.transform_point(quadratic_point(current, control, to, t)));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn rect_from_bounds(cx: f32, cy: f32, bounds: SwfBounds, scale: f32) -> RectVisual {
    RectVisual {
        x: bounds.x_min.mul_add(scale, cx),
        y: bounds.y_min.mul_add(scale, cy),
        w: (bounds.x_max - bounds.x_min) * scale,
        h: (bounds.y_max - bounds.y_min) * scale,
    }
}

fn rect_from_swf_bounds(bounds: SwfBounds) -> RectVisual {
    RectVisual {
        x: bounds.x_min,
        y: bounds.y_min,
        w: bounds.x_max - bounds.x_min,
        h: bounds.y_max - bounds.y_min,
    }
}

fn draw_swf_rect(bounds: SwfBounds, color: Color) {
    let rect = rect_from_swf_bounds(bounds);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}

#[derive(Debug, Clone, Copy)]
struct GoalFlashFrame {
    sx: f32,
    sy: f32,
    tx: f32,
    ty: f32,
    mult: f32,
    add: f32,
}

#[derive(Debug, Clone, Copy)]
struct GoalLineVisual {
    origin: SwfPoint,
    scale_x: f32,
    scale_y: f32,
    color: Color,
}

const LEFT_GOAL_FLASH: [GoalFlashFrame; 6] = [
    GoalFlashFrame {
        sx: 1.75,
        sy: 0.999_862_7,
        tx: 0.25,
        ty: 0.05,
        mult: 0.0,
        add: 255.0,
    },
    GoalFlashFrame {
        sx: 1.600_006_1,
        sy: 0.999_893_2,
        tx: 0.2,
        ty: 0.05,
        mult: 51.0,
        add: 204.0,
    },
    GoalFlashFrame {
        sx: 1.450_027_5,
        sy: 0.999_923_7,
        tx: 0.15,
        ty: 0.05,
        mult: 102.0,
        add: 153.0,
    },
    GoalFlashFrame {
        sx: 1.300_003,
        sy: 0.999_938_96,
        tx: 0.1,
        ty: 0.0,
        mult: 154.0,
        add: 102.0,
    },
    GoalFlashFrame {
        sx: 1.149_993_9,
        sy: 0.999_969_5,
        tx: 0.05,
        ty: 0.0,
        mult: 205.0,
        add: 51.0,
    },
    GoalFlashFrame {
        sx: 1.0,
        sy: 1.0,
        tx: 0.0,
        ty: 0.0,
        mult: 256.0,
        add: 0.0,
    },
];

const RIGHT_GOAL_FLASH: [GoalFlashFrame; 6] = [
    GoalFlashFrame {
        sx: 1.749_984_7,
        sy: 1.0,
        tx: 0.0,
        ty: 0.0,
        mult: 0.0,
        add: 255.0,
    },
    GoalFlashFrame {
        sx: 1.599_975_6,
        sy: 1.0,
        tx: 0.0,
        ty: 0.0,
        mult: 51.0,
        add: 204.0,
    },
    GoalFlashFrame {
        sx: 1.449_981_7,
        sy: 1.0,
        tx: 0.0,
        ty: 0.0,
        mult: 102.0,
        add: 153.0,
    },
    GoalFlashFrame {
        sx: 1.299_972_5,
        sy: 1.0,
        tx: 0.0,
        ty: 0.0,
        mult: 154.0,
        add: 102.0,
    },
    GoalFlashFrame {
        sx: 1.149_978_6,
        sy: 1.0,
        tx: 0.0,
        ty: 0.0,
        mult: 205.0,
        add: 51.0,
    },
    GoalFlashFrame {
        sx: 1.0,
        sy: 1.0,
        tx: 0.0,
        ty: 0.0,
        mult: 256.0,
        add: 0.0,
    },
];

fn draw_goal_line(x: f32, side: Side, flash_ticks: u8, flash_frames: &[GoalFlashFrame; 6]) {
    let visual = goal_line_visual(x, side, flash_ticks, flash_frames);
    let bounds = goal_line_bounds(visual);
    if bounds.x > STAGE_W as f32
        || bounds.x + bounds.w < 0.0
        || bounds.y > STAGE_H as f32
        || bounds.y + bounds.h < 0.0
    {
        return;
    }
    let points = goal_line_points(visual);
    draw_filled_polygon_fan(&points, goal_line_fill_center(visual), visual.color);
}

fn goal_line_visual(
    x: f32,
    side: Side,
    flash_ticks: u8,
    flash_frames: &[GoalFlashFrame; 6],
) -> GoalLineVisual {
    let color = goal_line_color(side);
    if flash_ticks == 0 {
        return GoalLineVisual {
            origin: SwfPoint::new(x, GOAL_LINE_CENTER_Y),
            scale_x: 1.0,
            scale_y: 1.0,
            color,
        };
    }

    let frame = flash_frames[usize::from(6 - flash_ticks.min(6))];
    GoalLineVisual {
        origin: SwfPoint::new(x + frame.tx, GOAL_LINE_CENTER_Y + frame.ty),
        scale_x: frame.sx,
        scale_y: frame.sy,
        color: apply_goal_flash_color(color, frame),
    }
}

fn goal_line_points(visual: GoalLineVisual) -> Vec<SwfPoint> {
    let shape = goal_line_shapes::SHAPE;
    let mut points = Vec::with_capacity(1 + shape.segments.len());
    points.push(goal_line_stage_point(visual, shape.start));
    for segment in shape.segments {
        match *segment {
            SwfPathSegment::Line(to) => points.push(goal_line_stage_point(visual, to)),
            SwfPathSegment::Quad { control: _, to } => {
                points.push(goal_line_stage_point(visual, to));
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn goal_line_fill_center(visual: GoalLineVisual) -> SwfPoint {
    let bounds = goal_line_shapes::SHAPE.bounds;
    goal_line_stage_point(
        visual,
        SwfPoint::new(
            (bounds.x_min + bounds.x_max) * 0.5,
            (bounds.y_min + bounds.y_max) * 0.5,
        ),
    )
}

fn goal_line_stage_point(visual: GoalLineVisual, local: SwfPoint) -> SwfPoint {
    SwfPoint::new(
        local.x.mul_add(visual.scale_x, visual.origin.x),
        local.y.mul_add(visual.scale_y, visual.origin.y),
    )
}

fn goal_line_bounds(visual: GoalLineVisual) -> RectVisual {
    let bounds = goal_line_shapes::SHAPE.bounds;
    RectVisual {
        x: bounds.x_min.mul_add(visual.scale_x, visual.origin.x),
        y: bounds.y_min.mul_add(visual.scale_y, visual.origin.y),
        w: (bounds.x_max - bounds.x_min) * visual.scale_x,
        h: (bounds.y_max - bounds.y_min) * visual.scale_y,
    }
}

fn goal_line_color(side: Side) -> Color {
    match side {
        Side::Red => swf_rgb_array(goal_line_shapes::RED_FILL_RGB),
        Side::Blue => swf_rgb_array(goal_line_shapes::BLUE_FILL_RGB),
    }
}

fn apply_goal_flash_color(color: Color, frame: GoalFlashFrame) -> Color {
    let mult = frame.mult / 256.0;
    let add = frame.add / 255.0;
    Color::new(
        color.r.mul_add(mult, add).clamp(0.0, 1.0),
        color.g.mul_add(mult, add).clamp(0.0, 1.0),
        color.b.mul_add(mult, add).clamp(0.0, 1.0),
        color.a,
    )
}

fn draw_static_side_marker(visual: StaticSideMarkerVisual) {
    draw_side_marker(visual.x, visual.y, visual.side);
}

fn draw_side_marker(x: f32, y: f32, side: Side) {
    draw_marker_glyph(marker_glyph_visual(x, y, side, SIDE_MARKER_EMPTY_FILL, 1.0));
}

fn marker_glyph_visual(
    x: f32,
    y: f32,
    side: Side,
    fill_color: Color,
    scale: f32,
) -> MarkerGlyphVisual {
    MarkerGlyphVisual {
        fill_points: marker_fill_points(x, y, side, scale),
        fill_center: marker_fill_center(x, y, side, scale),
        fill_color,
        outline_points: marker_outline_points(x, y, scale),
        outline_line_width: side_marker_shapes::OUTLINE_LINE_WIDTH * scale,
        outline: swf_rgb_array(side_marker_shapes::OUTLINE_RGB),
    }
}

fn score_marker_color(side: Side) -> Color {
    match side {
        Side::Blue => SCORE_MARKER_BLUE,
        Side::Red => SCORE_MARKER_RED,
    }
}

const fn marker_fill_shape(side: Side) -> SideMarkerShapeDefinition {
    match side {
        Side::Blue => side_marker_shapes::BLUE_FILL_SHAPE,
        Side::Red => side_marker_shapes::RED_FILL_SHAPE,
    }
}

fn marker_fill_center(x: f32, y: f32, side: Side, scale: f32) -> SwfPoint {
    let bounds = marker_fill_bounds(side);
    SwfPoint::new(
        ((bounds.x_min + bounds.x_max) * 0.5).mul_add(scale, x),
        ((bounds.y_min + bounds.y_max) * 0.5).mul_add(scale, y),
    )
}

const fn marker_fill_bounds(side: Side) -> SwfBounds {
    marker_fill_shape(side).bounds
}

fn marker_fill_points(x: f32, y: f32, side: Side, scale: f32) -> Vec<SwfPoint> {
    marker_contour_points(x, y, scale, marker_fill_shape(side).contour)
}

fn marker_outline_points(x: f32, y: f32, scale: f32) -> Vec<SwfPoint> {
    marker_contour_points(x, y, scale, side_marker_shapes::OUTLINE_SHAPE.contour)
}

fn marker_contour_points(x: f32, y: f32, scale: f32, contour: SideMarkerContour) -> Vec<SwfPoint> {
    let flatten_steps = side_marker_shapes::FLATTEN_STEPS;
    let mut points = Vec::with_capacity(1 + contour.segments.len() * usize::from(flatten_steps));
    points.push(marker_stage_point(x, y, scale, contour.start));
    let mut current = contour.start;
    for &segment in contour.segments {
        match segment {
            SwfPathSegment::Line(to) => {
                points.push(marker_stage_point(x, y, scale, to));
                current = to;
            },
            SwfPathSegment::Quad { control, to } => {
                for step in 1..=flatten_steps {
                    let t = f32::from(step) / f32::from(flatten_steps);
                    points.push(marker_stage_point(
                        x,
                        y,
                        scale,
                        quadratic_point(current, control, to, t),
                    ));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn marker_stage_point(x: f32, y: f32, scale: f32, local: SwfPoint) -> SwfPoint {
    SwfPoint::new(local.x.mul_add(scale, x), local.y.mul_add(scale, y))
}

fn draw_marker_glyph(visual: MarkerGlyphVisual) {
    draw_filled_polygon_fan(&visual.fill_points, visual.fill_center, visual.fill_color);
    draw_closed_polyline(
        &visual.outline_points,
        visual.outline_line_width,
        visual.outline,
    );
}

fn draw_goal_paddle_marker(x: f32, y: f32) {
    draw_paddle_body(paddle_body_visual(x, y));
}

fn draw_static_paddle_glow(visual: StaticPaddleGlowVisual) {
    draw_paddle_charge_glow(
        visual.x,
        visual.y,
        paddle_glow_visual(visual.scale_x, visual.scale_y, visual.color),
    );
}

fn paddle_body_visual(x: f32, y: f32) -> PaddleBodyVisual {
    PaddleBodyVisual {
        center: SwfPoint::new(x, y),
        line_width: paddle_body_shape89::LINE_WIDTH,
        fill: swf_rgb_array(paddle_body_shape89::FILL_RGB),
        outline: swf_rgb_array(paddle_body_shape89::LINE_RGB),
    }
}

fn draw_paddle_body(visual: PaddleBodyVisual) {
    let bounds = paddle_body_bounds(visual);
    if bounds.x > STAGE_W as f32
        || bounds.x + bounds.w < 0.0
        || bounds.y > STAGE_H as f32
        || bounds.y + bounds.h < 0.0
    {
        return;
    }
    let points = paddle_body_outline_points(visual);
    draw_filled_polygon_fan(&points, visual.center, visual.fill);
    draw_closed_polyline(&points, visual.line_width, visual.outline);
}

fn paddle_body_bounds(visual: PaddleBodyVisual) -> RectVisual {
    rect_from_bounds(
        visual.center.x,
        visual.center.y,
        paddle_body_shape89::SHAPE.bounds,
        1.0,
    )
}

fn draw_footer(
    rounds_played_label: &SwfTextTexture,
    chrome_texts: &ChromeTextTextures,
    counter_digits: &CounterDigitTextures,
    counter_fallback_font: Option<&Font>,
    games_played: u32,
    show_rounds_played: bool,
    include_version: bool,
) {
    if include_version {
        draw_swf_text_texture(&chrome_texts.version);
    }
    if show_rounds_played {
        draw_rounds_played_label(rounds_played_label);
        draw_rounds_played_value(games_played, counter_digits, counter_fallback_font);
    }
}

fn rounds_played_clip_visible(screen: Screen, offline: bool) -> bool {
    match screen {
        Screen::Menu => !offline,
        Screen::Help => true,
        Screen::Startup | Screen::Playing => false,
    }
}

const fn version_footer_visible(screen: Screen) -> bool {
    matches!(screen, Screen::Menu | Screen::Help)
}

fn draw_rounds_played_value(
    games_played: u32,
    counter_digits: &CounterDigitTextures,
    fallback_font: Option<&Font>,
) {
    let text = games_played.to_string();
    let width = counter_digit_text_stage_width(&text, fallback_font);
    let mut pen_x = ROUNDS_PLAYED_VALUE_TEXT.right_x - width;
    for digit in text.chars() {
        match counter_digit_source(digit) {
            CounterDigitSource::Embedded => {
                let Some(texture) = counter_digit_texture(digit, counter_digits) else {
                    continue;
                };
                draw_texture_ex(
                    &texture.texture,
                    texture
                        .local_bounds
                        .x
                        .mul_add(COUNTER_DIGIT_STAGE_SCALE, pen_x),
                    (texture.local_bounds.y - COUNTER_DIGIT_BASELINE_Y).mul_add(
                        COUNTER_DIGIT_STAGE_SCALE,
                        ROUNDS_PLAYED_VALUE_TEXT.baseline_y,
                    ),
                    SWF_WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(
                            texture.local_bounds.w * COUNTER_DIGIT_STAGE_SCALE,
                            texture.local_bounds.h * COUNTER_DIGIT_STAGE_SCALE,
                        )),
                        ..Default::default()
                    },
                );
                pen_x = texture.advance.mul_add(COUNTER_DIGIT_STAGE_SCALE, pen_x);
            },
            CounterDigitSource::Device => {
                let mut buffer = [0; 4];
                let digit_text = digit.encode_utf8(&mut buffer);
                draw_swf_text_left(
                    digit_text,
                    SwfLineVisual {
                        x: pen_x,
                        baseline_y: ROUNDS_PLAYED_VALUE_TEXT.baseline_y,
                        font_size: ROUNDS_PLAYED_VALUE_TEXT.font_size,
                    },
                    MUTED,
                    fallback_font,
                );
                pen_x += counter_digit_device_stage_width(digit, fallback_font);
            },
        }
    }
}

fn counter_digit_text_stage_width(text: &str, fallback_font: Option<&Font>) -> f32 {
    text.chars()
        .map(|digit| match counter_digit_source(digit) {
            CounterDigitSource::Embedded => counter_digit_advance(digit)
                .map_or(0.0, |advance| advance * COUNTER_DIGIT_STAGE_SCALE),
            CounterDigitSource::Device => counter_digit_device_stage_width(digit, fallback_font),
        })
        .sum()
}

#[cfg(test)]
fn counter_digit_embedded_text_width(text: &str) -> Option<f32> {
    let mut width = 0.0;
    for digit in text.chars() {
        width += counter_digit_advance(digit)?;
    }
    Some(width)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CounterDigitSource {
    Embedded,
    Device,
}

fn counter_digit_source(digit: char) -> CounterDigitSource {
    if counter_digit_definition(digit).is_some() {
        CounterDigitSource::Embedded
    } else {
        CounterDigitSource::Device
    }
}

#[cfg(test)]
fn counter_digit_sources(text: &str) -> Vec<CounterDigitSource> {
    text.chars().map(counter_digit_source).collect()
}

fn counter_digit_advance(digit: char) -> Option<f32> {
    counter_digit_definition(digit).map(|definition| f32::from(definition.advance_centipx) / 100.0)
}

fn counter_digit_device_stage_width(digit: char, fallback_font: Option<&Font>) -> f32 {
    let mut buffer = [0; 4];
    let digit_text = digit.encode_utf8(&mut buffer);
    measure_text(
        digit_text,
        fallback_font,
        ROUNDS_PLAYED_VALUE_TEXT.font_size,
        1.0,
    )
    .width
}

fn counter_digit_definition(
    digit: char,
) -> Option<&'static counter_digit_texts::CounterDigitDefinition> {
    match digit {
        '0' => Some(&counter_digit_texts::DIGIT_0),
        '1' => Some(&counter_digit_texts::DIGIT_1),
        '2' => Some(&counter_digit_texts::DIGIT_2),
        '3' => Some(&counter_digit_texts::DIGIT_3),
        '4' => Some(&counter_digit_texts::DIGIT_4),
        '5' => Some(&counter_digit_texts::DIGIT_5),
        '6' => Some(&counter_digit_texts::DIGIT_6),
        '7' => Some(&counter_digit_texts::DIGIT_7),
        '8' => Some(&counter_digit_texts::DIGIT_8),
        _ => None,
    }
}

fn counter_digit_texture(
    digit: char,
    counter_digits: &CounterDigitTextures,
) -> Option<&CounterDigitTexture> {
    match digit {
        '0' => Some(&counter_digits.digit_0),
        '1' => Some(&counter_digits.digit_1),
        '2' => Some(&counter_digits.digit_2),
        '3' => Some(&counter_digits.digit_3),
        '4' => Some(&counter_digits.digit_4),
        '5' => Some(&counter_digits.digit_5),
        '6' => Some(&counter_digits.digit_6),
        '7' => Some(&counter_digits.digit_7),
        '8' => Some(&counter_digits.digit_8),
        _ => None,
    }
}

fn draw_header_link(link: HeaderLink, chrome_texts: &ChromeTextTextures) {
    let (mouse_x, mouse_y) = mouse_position();
    let state = header_link_state(
        link,
        mouse_x,
        mouse_y,
        is_mouse_button_down(MouseButton::Left),
    );
    draw_swf_text_texture(header_link_texture(link, state, chrome_texts));
}

fn header_link_texture(
    link: HeaderLink,
    state: HeaderLinkState,
    chrome_texts: &ChromeTextTextures,
) -> &SwfTextTexture {
    let textures = match link {
        HeaderLink::Sponsor => &chrome_texts.sponsor_link,
        HeaderLink::BackToMenu => &chrome_texts.back_link,
    };
    match state {
        HeaderLinkState::Up => &textures.up,
        HeaderLinkState::Over => &textures.over,
        HeaderLinkState::Down => &textures.down,
    }
}

fn draw_menu_button_label(action: MenuAction, labels: &MenuLabelTextures) {
    draw_swf_text_texture(menu_label_texture(action, labels));
}

fn draw_menu_button_by_action(action: MenuAction, buttons: &[Button], labels: &MenuLabelTextures) {
    if let Some(button) = buttons.iter().find(|button| button.action == action) {
        draw_button_shape(button.visual);
        draw_menu_button_label(action, labels);
    }
}

fn menu_label_texture(action: MenuAction, labels: &MenuLabelTextures) -> &SwfTextTexture {
    match action {
        MenuAction::Start => &labels.start,
        MenuAction::Help => &labels.help,
        MenuAction::Polarisation => &labels.polarisation,
        MenuAction::Matches => &labels.matches,
        MenuAction::Gravity => &labels.gravity,
        MenuAction::Speed => &labels.speed,
    }
}

fn draw_sponsor_logo(sponsor_logo: &SponsorLogoAssets) {
    draw_swf_text_texture(&sponsor_logo.text_layers.dark);
    draw_swf_text_texture(&sponsor_logo.text_layers.olive);
    draw_sponsor_logo_foreground(&sponsor_logo.foreground);
}

fn draw_sponsor_logo_foreground(sponsor_logo: &SponsorLogoTexture) {
    draw_texture_ex(
        &sponsor_logo.texture,
        sponsor_logo.placement.x,
        sponsor_logo.placement.y,
        SWF_WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(sponsor_logo.placement.w, sponsor_logo.placement.h)),
            ..Default::default()
        },
    );
}

#[cfg(test)]
fn sponsor_logo_text_layers() -> [SponsorLogoTextLayer; 2] {
    [
        SPONSOR_LOGO_DEPTH_3_TEXT.visual(),
        SPONSOR_LOGO_DEPTH_4_TEXT.visual(),
    ]
}

fn draw_top_title_foreground(top_title: &SwfTextTexture) {
    draw_swf_text_texture(top_title);
}

fn draw_top_gravity_factor(gravity: GravityStrength, font: Option<&Font>) {
    draw_swf_text_left(
        top_gravity_factor_value(gravity),
        TOP_GRAVITY_FACTOR_TEXT,
        TEXT,
        font,
    );
}

fn draw_rounds_played_label(label: &SwfTextTexture) {
    draw_swf_text_texture(label);
}

fn draw_swf_text_texture(label: &SwfTextTexture) {
    draw_texture_ex(
        &label.texture,
        label.placement.x,
        label.placement.y,
        SWF_WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(label.placement.w, label.placement.h)),
            ..Default::default()
        },
    );
}

fn build_menu_label_textures() -> MenuLabelTextures {
    MenuLabelTextures {
        start: build_menu_label_texture(MenuAction::Start),
        help: build_menu_label_texture(MenuAction::Help),
        polarisation: build_menu_label_texture(MenuAction::Polarisation),
        matches: build_menu_label_texture(MenuAction::Matches),
        gravity: build_menu_label_texture(MenuAction::Gravity),
        speed: build_menu_label_texture(MenuAction::Speed),
    }
}

fn build_menu_label_texture(action: MenuAction) -> SwfTextTexture {
    let definition = menu_label_definition(action);
    debug_assert_eq!(definition.font_id, menu_label_texts::FONT_ID);
    let placement = menu_label_stage_bounds(action);
    let width = placement.w.ceil().max(1.0) as u16;
    let height = placement.h.ceil().max(1.0) as u16;
    let contours = menu_label_flattened_contours(definition);
    let sample_count = MENU_LABEL_TEXTURE_SUPERSAMPLE * MENU_LABEL_TEXTURE_SUPERSAMPLE;
    let color = definition.color_rgb;
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];
    let text_placement = menu_label_placement(action);

    for y in 0..height {
        for x in 0..width {
            let mut coverage = 0;
            for sub_y in 0..MENU_LABEL_TEXTURE_SUPERSAMPLE {
                for sub_x in 0..MENU_LABEL_TEXTURE_SUPERSAMPLE {
                    let local = menu_label_texture_sample_local(
                        placement,
                        text_placement,
                        (width, height),
                        (x, y),
                        (sub_x, sub_y),
                    );
                    if menu_label_contains_local(&contours, local) {
                        coverage += 1;
                    }
                }
            }

            let offset = (usize::from(y) * usize::from(width) + usize::from(x)) * 4;
            pixels[offset] = color[0];
            pixels[offset + 1] = color[1];
            pixels[offset + 2] = color[2];
            pixels[offset + 3] = ((coverage * 255 + sample_count / 2) / sample_count) as u8;
        }
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    SwfTextTexture { texture, placement }
}

fn build_menu_value_textures() -> MenuValueTextures {
    MenuValueTextures {
        question: build_menu_value_texture(&menu_value_texts::QUESTION, MENU_QUESTION_TEXT),
        match_single: build_menu_value_texture(
            &menu_value_texts::MATCH_SINGLE,
            MENU_MATCH_SINGLE_TEXT,
        ),
        match_best_of_3: build_menu_value_texture(
            &menu_value_texts::MATCH_BEST_OF_3,
            MENU_MATCH_BEST_OF_TEXT,
        ),
        match_best_of_5: build_menu_value_texture(
            &menu_value_texts::MATCH_BEST_OF_5,
            MENU_MATCH_BEST_OF_TEXT,
        ),
        match_best_of_7: build_menu_value_texture(
            &menu_value_texts::MATCH_BEST_OF_7,
            MENU_MATCH_BEST_OF_TEXT,
        ),
        gravity_low: build_menu_value_texture(
            &menu_value_texts::GRAVITY_LOW,
            MENU_GRAVITY_LOW_TEXT,
        ),
        gravity_medium: build_menu_value_texture(
            &menu_value_texts::GRAVITY_MEDIUM,
            MENU_GRAVITY_MEDIUM_TEXT,
        ),
        gravity_high: build_menu_value_texture(
            &menu_value_texts::GRAVITY_HIGH,
            MENU_GRAVITY_HIGH_TEXT,
        ),
        gravity_very_high: build_menu_value_texture(
            &menu_value_texts::GRAVITY_VERY_HIGH,
            MENU_GRAVITY_VERY_HIGH_TEXT,
        ),
        gravity_black_hole: build_menu_value_texture(
            &menu_value_texts::GRAVITY_BLACK_HOLE,
            MENU_GRAVITY_BLACK_HOLE_TEXT,
        ),
        speed_disabled: build_menu_value_texture(
            &menu_value_texts::SPEED_DISABLED,
            MENU_SPEED_DISABLED_TEXT,
        ),
        speed_enabled: build_menu_value_texture(
            &menu_value_texts::SPEED_ENABLED,
            MENU_SPEED_ENABLED_TEXT,
        ),
    }
}

fn build_menu_value_texture(
    definition: &menu_value_texts::MenuValueDefinition,
    text_placement: SwfTextPlacement,
) -> SwfTextTexture {
    let placement = menu_value_stage_bounds(definition, text_placement);
    let width = placement.w.ceil().max(1.0) as u16;
    let height = placement.h.ceil().max(1.0) as u16;
    let contours = menu_value_flattened_contours(definition);
    let sample_count = MENU_LABEL_TEXTURE_SUPERSAMPLE * MENU_LABEL_TEXTURE_SUPERSAMPLE;
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];

    for y in 0..height {
        for x in 0..width {
            let mut coverage = 0;
            for sub_y in 0..MENU_LABEL_TEXTURE_SUPERSAMPLE {
                for sub_x in 0..MENU_LABEL_TEXTURE_SUPERSAMPLE {
                    let local = menu_label_texture_sample_local(
                        placement,
                        text_placement,
                        (width, height),
                        (x, y),
                        (sub_x, sub_y),
                    );
                    if menu_value_contains_local(&contours, local) {
                        coverage += 1;
                    }
                }
            }

            let offset = (usize::from(y) * usize::from(width) + usize::from(x)) * 4;
            pixels[offset] = definition.color_rgb[0];
            pixels[offset + 1] = definition.color_rgb[1];
            pixels[offset + 2] = definition.color_rgb[2];
            pixels[offset + 3] = ((coverage * 255 + sample_count / 2) / sample_count) as u8;
        }
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    SwfTextTexture { texture, placement }
}

fn menu_value_stage_bounds(
    definition: &menu_value_texts::MenuValueDefinition,
    placement: SwfTextPlacement,
) -> RectVisual {
    let bounds = menu_value_local_bounds(definition);
    RectVisual {
        x: placement
            .root_sx
            .mul_add(placement.local_tx + bounds.x_min, placement.root_tx),
        y: placement
            .root_sy
            .mul_add(placement.local_ty + bounds.y_min, placement.root_ty),
        w: placement.root_sx * (bounds.x_max - bounds.x_min),
        h: placement.root_sy * (bounds.y_max - bounds.y_min),
    }
}

fn menu_value_local_bounds(definition: &menu_value_texts::MenuValueDefinition) -> SwfBounds {
    SwfBounds {
        x_min: f32::from(definition.bounds_centipx[0]) / 100.0,
        x_max: f32::from(definition.bounds_centipx[1]) / 100.0,
        y_min: f32::from(definition.bounds_centipx[2]) / 100.0,
        y_max: f32::from(definition.bounds_centipx[3]) / 100.0,
    }
}

fn menu_value_flattened_contours(
    definition: &menu_value_texts::MenuValueDefinition,
) -> Vec<Vec<SwfPoint>> {
    definition
        .contours
        .iter()
        .copied()
        .map(menu_value_contour_points)
        .collect()
}

fn menu_value_contour_points(contour: menu_value_texts::MenuValueContour) -> Vec<SwfPoint> {
    let mut points =
        Vec::with_capacity(1 + contour.segments.len() * usize::from(MENU_LABEL_FLATTEN_STEPS));
    let mut current = menu_label_centipixel_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            menu_value_texts::MenuValueSegment::Line(to) => {
                current = menu_label_centipixel_point(to);
                points.push(current);
            },
            menu_value_texts::MenuValueSegment::Quad { control, to } => {
                let control = menu_label_centipixel_point(control);
                let to = menu_label_centipixel_point(to);
                for step in 1..=MENU_LABEL_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(MENU_LABEL_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn menu_value_contains_local(contours: &[Vec<SwfPoint>], local: SwfPoint) -> bool {
    contours
        .iter()
        .filter(|contour| point_in_polygon(local, contour))
        .count()
        % 2
        == 1
}

fn build_help_label_textures() -> HelpLabelTextures {
    HelpLabelTextures {
        title: build_help_label_texture(&help_label_texts::TITLE, HELP_TITLE_TEXT),
        body: build_help_label_texture(&help_label_texts::BODY, HELP_BODY_TEXT),
        back: build_help_label_texture(&help_label_texts::BACK, HELP_BACK_LABEL_TEXT),
        key_w: build_help_label_texture(&help_label_texts::KEY_W, HELP_KEY_W_TEXT),
        key_d: build_help_label_texture(&help_label_texts::KEY_D, HELP_KEY_D_TEXT),
        key_s: build_help_label_texture(&help_label_texts::KEY_S, HELP_KEY_S_TEXT),
        red_move_up: build_help_label_texture(
            &help_label_texts::RED_MOVE_UP,
            HELP_RED_MOVE_UP_TEXT,
        ),
        red_move_down: build_help_label_texture(
            &help_label_texts::RED_MOVE_DOWN,
            HELP_RED_MOVE_DOWN_TEXT,
        ),
        red_shoot: build_help_label_texture(&help_label_texts::RED_SHOOT, HELP_RED_SHOOT_TEXT),
        blue_shoot: build_help_label_texture(&help_label_texts::BLUE_SHOOT, HELP_BLUE_SHOOT_TEXT),
        blue_move_up: build_help_label_texture(
            &help_label_texts::BLUE_MOVE_UP,
            HELP_BLUE_MOVE_UP_TEXT,
        ),
        blue_move_down: build_help_label_texture(
            &help_label_texts::BLUE_MOVE_DOWN,
            HELP_BLUE_MOVE_DOWN_TEXT,
        ),
        player_red: build_help_label_texture(&help_label_texts::PLAYER_RED, HELP_PLAYER_RED_TEXT),
        player_blue: build_help_label_texture(
            &help_label_texts::PLAYER_BLUE,
            HELP_PLAYER_BLUE_TEXT,
        ),
    }
}

fn build_help_label_texture(
    definition: &help_label_texts::HelpLabelDefinition,
    text_placement: SwfTextPlacement,
) -> SwfTextTexture {
    let placement = help_label_stage_bounds(definition, text_placement);
    let width = placement.w.ceil().max(1.0) as u16;
    let height = placement.h.ceil().max(1.0) as u16;
    let sample_masks =
        help_label_sample_masks(definition, placement, text_placement, width, height);
    let sample_count =
        u32::from(HELP_LABEL_TEXTURE_SUPERSAMPLE) * u32::from(HELP_LABEL_TEXTURE_SUPERSAMPLE);
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];

    for (pixel_index, mask) in sample_masks.into_iter().enumerate() {
        let offset = pixel_index * 4;
        pixels[offset] = definition.color_rgb[0];
        pixels[offset + 1] = definition.color_rgb[1];
        pixels[offset + 2] = definition.color_rgb[2];
        pixels[offset + 3] = ((mask.count_ones() * 255 + sample_count / 2) / sample_count) as u8;
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    SwfTextTexture { texture, placement }
}

fn help_label_sample_masks(
    definition: &help_label_texts::HelpLabelDefinition,
    placement: RectVisual,
    text_placement: SwfTextPlacement,
    width: u16,
    height: u16,
) -> Vec<u32> {
    let contours = help_label_bounded_contours(definition);
    let mut sample_masks = vec![0_u32; usize::from(width) * usize::from(height)];

    for contour in &contours {
        let Some((x_min, x_max, y_min, y_max)) = help_label_contour_pixel_bounds(
            contour.bounds,
            placement,
            text_placement,
            width,
            height,
        ) else {
            continue;
        };

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let mut mask = 0_u32;
                for sub_y in 0..HELP_LABEL_TEXTURE_SUPERSAMPLE {
                    for sub_x in 0..HELP_LABEL_TEXTURE_SUPERSAMPLE {
                        let local = help_label_texture_sample_local(
                            placement,
                            text_placement,
                            (width, height),
                            (x, y),
                            (sub_x, sub_y),
                        );
                        if point_in_polygon(local, &contour.points) {
                            let bit = u32::from(sub_y * HELP_LABEL_TEXTURE_SUPERSAMPLE + sub_x);
                            mask ^= 1 << bit;
                        }
                    }
                }
                let offset = usize::from(y) * usize::from(width) + usize::from(x);
                sample_masks[offset] ^= mask;
            }
        }
    }
    sample_masks
}

struct BoundedHelpContour {
    points: Vec<SwfPoint>,
    bounds: SwfBounds,
}

fn help_label_bounded_contours(
    definition: &help_label_texts::HelpLabelDefinition,
) -> Vec<BoundedHelpContour> {
    help_label_flattened_contours(definition)
        .into_iter()
        .filter_map(|points| {
            let bounds = swf_points_bounds(&points)?;
            Some(BoundedHelpContour { points, bounds })
        })
        .collect()
}

fn swf_points_bounds(points: &[SwfPoint]) -> Option<SwfBounds> {
    let first = *points.first()?;
    let mut bounds = SwfBounds {
        x_min: first.x,
        x_max: first.x,
        y_min: first.y,
        y_max: first.y,
    };
    for &point in &points[1..] {
        bounds.x_min = bounds.x_min.min(point.x);
        bounds.x_max = bounds.x_max.max(point.x);
        bounds.y_min = bounds.y_min.min(point.y);
        bounds.y_max = bounds.y_max.max(point.y);
    }
    Some(bounds)
}

fn help_label_contour_pixel_bounds(
    bounds: SwfBounds,
    placement: RectVisual,
    text_placement: SwfTextPlacement,
    width: u16,
    height: u16,
) -> Option<(u16, u16, u16, u16)> {
    let x0 = help_label_local_x_to_texture_x(bounds.x_min, placement, text_placement, width);
    let x1 = help_label_local_x_to_texture_x(bounds.x_max, placement, text_placement, width);
    let y0 = help_label_local_y_to_texture_y(bounds.y_min, placement, text_placement, height);
    let y1 = help_label_local_y_to_texture_y(bounds.y_max, placement, text_placement, height);
    let x_min = x0.min(x1).floor() as i32 - 1;
    let x_max = x0.max(x1).ceil() as i32 + 1;
    let y_min = y0.min(y1).floor() as i32 - 1;
    let y_max = y0.max(y1).ceil() as i32 + 1;
    let width = i32::from(width);
    let height = i32::from(height);
    if x_max < 0 || y_max < 0 || x_min >= width || y_min >= height {
        return None;
    }
    Some((
        x_min.clamp(0, width - 1) as u16,
        x_max.clamp(0, width - 1) as u16,
        y_min.clamp(0, height - 1) as u16,
        y_max.clamp(0, height - 1) as u16,
    ))
}

fn help_label_local_x_to_texture_x(
    local_x: f32,
    placement: RectVisual,
    text_placement: SwfTextPlacement,
    width: u16,
) -> f32 {
    let stage_x = text_placement
        .root_sx
        .mul_add(text_placement.local_tx + local_x, text_placement.root_tx);
    (stage_x - placement.x) * f32::from(width) / placement.w
}

fn help_label_local_y_to_texture_y(
    local_y: f32,
    placement: RectVisual,
    text_placement: SwfTextPlacement,
    height: u16,
) -> f32 {
    let stage_y = text_placement
        .root_sy
        .mul_add(text_placement.local_ty + local_y, text_placement.root_ty);
    (stage_y - placement.y) * f32::from(height) / placement.h
}

fn help_label_texture_sample_local(
    placement: RectVisual,
    text_placement: SwfTextPlacement,
    texture_size: (u16, u16),
    pixel: (u16, u16),
    subpixel: (u16, u16),
) -> SwfPoint {
    let supersample = f32::from(HELP_LABEL_TEXTURE_SUPERSAMPLE);
    let (width, height) = texture_size;
    let (x, y) = pixel;
    let (sub_x, sub_y) = subpixel;
    let stage_x = placement.x
        + (f32::from(x) + (f32::from(sub_x) + 0.5) / supersample) * placement.w / f32::from(width);
    let stage_y = placement.y
        + (f32::from(y) + (f32::from(sub_y) + 0.5) / supersample) * placement.h / f32::from(height);
    SwfPoint::new(
        (stage_x - text_placement.root_tx) / text_placement.root_sx - text_placement.local_tx,
        (stage_y - text_placement.root_ty) / text_placement.root_sy - text_placement.local_ty,
    )
}

fn help_label_stage_bounds(
    definition: &help_label_texts::HelpLabelDefinition,
    placement: SwfTextPlacement,
) -> RectVisual {
    let bounds = help_label_local_bounds(definition);
    RectVisual {
        x: placement
            .root_sx
            .mul_add(placement.local_tx + bounds.x_min, placement.root_tx),
        y: placement
            .root_sy
            .mul_add(placement.local_ty + bounds.y_min, placement.root_ty),
        w: placement.root_sx * (bounds.x_max - bounds.x_min),
        h: placement.root_sy * (bounds.y_max - bounds.y_min),
    }
}

fn help_label_local_bounds(definition: &help_label_texts::HelpLabelDefinition) -> SwfBounds {
    SwfBounds {
        x_min: f32::from(definition.bounds_centipx[0]) / 100.0,
        x_max: f32::from(definition.bounds_centipx[1]) / 100.0,
        y_min: f32::from(definition.bounds_centipx[2]) / 100.0,
        y_max: f32::from(definition.bounds_centipx[3]) / 100.0,
    }
}

fn help_label_flattened_contours(
    definition: &help_label_texts::HelpLabelDefinition,
) -> Vec<Vec<SwfPoint>> {
    definition
        .contours
        .iter()
        .copied()
        .map(help_label_contour_points)
        .collect()
}

fn help_label_contour_points(contour: help_label_texts::HelpLabelContour) -> Vec<SwfPoint> {
    let mut points =
        Vec::with_capacity(1 + contour.segments.len() * usize::from(HELP_LABEL_FLATTEN_STEPS));
    let mut current = menu_label_centipixel_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            help_label_texts::HelpLabelSegment::Line(to) => {
                current = menu_label_centipixel_point(to);
                points.push(current);
            },
            help_label_texts::HelpLabelSegment::Quad { control, to } => {
                let control = menu_label_centipixel_point(control);
                let to = menu_label_centipixel_point(to);
                for step in 1..=HELP_LABEL_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(HELP_LABEL_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn build_chrome_textures() -> ChromeTextTextures {
    ChromeTextTextures {
        sponsor_link: HeaderLinkTextureSet {
            up: build_chrome_text_texture(&chrome_texts::SPONSOR_UP, HEADER_LINK_TEXT_PLACEMENT),
            over: build_chrome_text_texture(
                &chrome_texts::SPONSOR_OVER,
                HEADER_LINK_TEXT_PLACEMENT,
            ),
            down: build_chrome_text_texture(
                &chrome_texts::SPONSOR_DOWN,
                HEADER_LINK_TEXT_PLACEMENT,
            ),
        },
        back_link: HeaderLinkTextureSet {
            up: build_chrome_text_texture(&chrome_texts::BACK_UP, HEADER_LINK_TEXT_PLACEMENT),
            over: build_chrome_text_texture(&chrome_texts::BACK_OVER, HEADER_LINK_TEXT_PLACEMENT),
            down: build_chrome_text_texture(&chrome_texts::BACK_DOWN, HEADER_LINK_TEXT_PLACEMENT),
        },
        version: build_chrome_text_texture(&chrome_texts::VERSION, MENU_VERSION_TEXT),
    }
}

fn build_chrome_text_texture(
    definition: &chrome_texts::ChromeTextDefinition,
    text_placement: SwfTextPlacement,
) -> SwfTextTexture {
    let placement = chrome_text_stage_bounds(definition, text_placement);
    let width = placement.w.ceil().max(1.0) as u16;
    let height = placement.h.ceil().max(1.0) as u16;
    let contours = chrome_text_flattened_contours(definition);
    let sample_count = CHROME_TEXT_TEXTURE_SUPERSAMPLE * CHROME_TEXT_TEXTURE_SUPERSAMPLE;
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];

    for y in 0..height {
        for x in 0..width {
            let mut coverage = 0;
            for sub_y in 0..CHROME_TEXT_TEXTURE_SUPERSAMPLE {
                for sub_x in 0..CHROME_TEXT_TEXTURE_SUPERSAMPLE {
                    let local = chrome_text_texture_sample_local(
                        placement,
                        text_placement,
                        (width, height),
                        (x, y),
                        (sub_x, sub_y),
                    );
                    if chrome_text_contains_local(&contours, local) {
                        coverage += 1;
                    }
                }
            }

            let offset = (usize::from(y) * usize::from(width) + usize::from(x)) * 4;
            pixels[offset] = definition.color_rgb[0];
            pixels[offset + 1] = definition.color_rgb[1];
            pixels[offset + 2] = definition.color_rgb[2];
            pixels[offset + 3] = ((coverage * 255 + sample_count / 2) / sample_count) as u8;
        }
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    SwfTextTexture { texture, placement }
}

fn chrome_text_texture_sample_local(
    placement: RectVisual,
    text_placement: SwfTextPlacement,
    texture_size: (u16, u16),
    pixel: (u16, u16),
    subpixel: (u16, u16),
) -> SwfPoint {
    let supersample = f32::from(CHROME_TEXT_TEXTURE_SUPERSAMPLE);
    let (width, height) = texture_size;
    let (x, y) = pixel;
    let (sub_x, sub_y) = subpixel;
    let stage_x = placement.x
        + (f32::from(x) + (f32::from(sub_x) + 0.5) / supersample) * placement.w / f32::from(width);
    let stage_y = placement.y
        + (f32::from(y) + (f32::from(sub_y) + 0.5) / supersample) * placement.h / f32::from(height);
    SwfPoint::new(
        (stage_x - text_placement.root_tx) / text_placement.root_sx - text_placement.local_tx,
        (stage_y - text_placement.root_ty) / text_placement.root_sy - text_placement.local_ty,
    )
}

fn chrome_text_stage_bounds(
    definition: &chrome_texts::ChromeTextDefinition,
    placement: SwfTextPlacement,
) -> RectVisual {
    let bounds = chrome_text_local_bounds(definition);
    RectVisual {
        x: placement
            .root_sx
            .mul_add(placement.local_tx + bounds.x_min, placement.root_tx),
        y: placement
            .root_sy
            .mul_add(placement.local_ty + bounds.y_min, placement.root_ty),
        w: placement.root_sx * (bounds.x_max - bounds.x_min),
        h: placement.root_sy * (bounds.y_max - bounds.y_min),
    }
}

fn chrome_text_local_bounds(definition: &chrome_texts::ChromeTextDefinition) -> SwfBounds {
    SwfBounds {
        x_min: f32::from(definition.bounds_centipx[0]) / 100.0,
        x_max: f32::from(definition.bounds_centipx[1]) / 100.0,
        y_min: f32::from(definition.bounds_centipx[2]) / 100.0,
        y_max: f32::from(definition.bounds_centipx[3]) / 100.0,
    }
}

fn chrome_text_flattened_contours(
    definition: &chrome_texts::ChromeTextDefinition,
) -> Vec<Vec<SwfPoint>> {
    definition
        .contours
        .iter()
        .copied()
        .map(chrome_text_contour_points)
        .collect()
}

fn chrome_text_contour_points(contour: chrome_texts::ChromeTextContour) -> Vec<SwfPoint> {
    let mut points =
        Vec::with_capacity(1 + contour.segments.len() * usize::from(CHROME_TEXT_FLATTEN_STEPS));
    let mut current = menu_label_centipixel_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            chrome_texts::ChromeTextSegment::Line(to) => {
                current = menu_label_centipixel_point(to);
                points.push(current);
            },
            chrome_texts::ChromeTextSegment::Quad { control, to } => {
                let control = menu_label_centipixel_point(control);
                let to = menu_label_centipixel_point(to);
                for step in 1..=CHROME_TEXT_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(CHROME_TEXT_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn chrome_text_contains_local(contours: &[Vec<SwfPoint>], local: SwfPoint) -> bool {
    contours
        .iter()
        .filter(|contour| point_in_polygon(local, contour))
        .count()
        % 2
        == 1
}

fn build_announce_textures() -> AnnounceTextTextures {
    AnnounceTextTextures {
        blue_wins: build_announce_text_texture(&announce_texts::BLUE_WINS),
        red_wins: build_announce_text_texture(&announce_texts::RED_WINS),
        round: build_announce_text_texture(&announce_texts::ROUND),
        blue_match: build_announce_text_texture(&announce_texts::BLUE_MATCH),
        red_match: build_announce_text_texture(&announce_texts::RED_MATCH),
        round_number_1: build_round_number_text_texture(&round_number_texts::ONE),
        round_number_2: build_round_number_text_texture(&round_number_texts::TWO),
        round_number_3: build_round_number_text_texture(&round_number_texts::THREE),
    }
}

fn build_counter_digit_textures() -> CounterDigitTextures {
    CounterDigitTextures {
        digit_0: build_counter_digit_texture(&counter_digit_texts::DIGIT_0),
        digit_1: build_counter_digit_texture(&counter_digit_texts::DIGIT_1),
        digit_2: build_counter_digit_texture(&counter_digit_texts::DIGIT_2),
        digit_3: build_counter_digit_texture(&counter_digit_texts::DIGIT_3),
        digit_4: build_counter_digit_texture(&counter_digit_texts::DIGIT_4),
        digit_5: build_counter_digit_texture(&counter_digit_texts::DIGIT_5),
        digit_6: build_counter_digit_texture(&counter_digit_texts::DIGIT_6),
        digit_7: build_counter_digit_texture(&counter_digit_texts::DIGIT_7),
        digit_8: build_counter_digit_texture(&counter_digit_texts::DIGIT_8),
    }
}

fn build_counter_digit_texture(
    definition: &counter_digit_texts::CounterDigitDefinition,
) -> CounterDigitTexture {
    let local_bounds = counter_digit_local_bounds(definition);
    let width = (local_bounds.w * f32::from(COUNTER_DIGIT_TEXTURE_SCALE))
        .ceil()
        .max(1.0) as u16;
    let height = (local_bounds.h * f32::from(COUNTER_DIGIT_TEXTURE_SCALE))
        .ceil()
        .max(1.0) as u16;
    let contours = counter_digit_flattened_contours(definition);
    let sample_count = COUNTER_DIGIT_TEXTURE_SUPERSAMPLE * COUNTER_DIGIT_TEXTURE_SUPERSAMPLE;
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];

    for y in 0..height {
        for x in 0..width {
            let mut coverage = 0;
            for sub_y in 0..COUNTER_DIGIT_TEXTURE_SUPERSAMPLE {
                for sub_x in 0..COUNTER_DIGIT_TEXTURE_SUPERSAMPLE {
                    let local = counter_digit_texture_sample_local(
                        local_bounds,
                        (width, height),
                        (x, y),
                        (sub_x, sub_y),
                    );
                    if counter_digit_contains_local(&contours, local) {
                        coverage += 1;
                    }
                }
            }

            let offset = (usize::from(y) * usize::from(width) + usize::from(x)) * 4;
            pixels[offset] = definition.color_rgb[0];
            pixels[offset + 1] = definition.color_rgb[1];
            pixels[offset + 2] = definition.color_rgb[2];
            pixels[offset + 3] = ((coverage * 255 + sample_count / 2) / sample_count) as u8;
        }
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    CounterDigitTexture {
        texture,
        local_bounds,
        advance: f32::from(definition.advance_centipx) / 100.0,
    }
}

fn build_announce_text_texture(
    definition: &announce_texts::AnnounceTextDefinition,
) -> AnnounceTextTexture {
    let local_bounds = announce_text_local_bounds(definition);
    let width = (local_bounds.w * f32::from(ANNOUNCE_TEXT_TEXTURE_SCALE))
        .ceil()
        .max(1.0) as u16;
    let height = (local_bounds.h * f32::from(ANNOUNCE_TEXT_TEXTURE_SCALE))
        .ceil()
        .max(1.0) as u16;
    let contours = announce_text_flattened_contours(definition);
    let sample_count = ANNOUNCE_TEXT_TEXTURE_SUPERSAMPLE * ANNOUNCE_TEXT_TEXTURE_SUPERSAMPLE;
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];

    for y in 0..height {
        for x in 0..width {
            let mut coverage = 0;
            for sub_y in 0..ANNOUNCE_TEXT_TEXTURE_SUPERSAMPLE {
                for sub_x in 0..ANNOUNCE_TEXT_TEXTURE_SUPERSAMPLE {
                    let local = announce_text_texture_sample_local(
                        local_bounds,
                        (width, height),
                        (x, y),
                        (sub_x, sub_y),
                    );
                    if announce_text_contains_local(&contours, local) {
                        coverage += 1;
                    }
                }
            }

            let offset = (usize::from(y) * usize::from(width) + usize::from(x)) * 4;
            pixels[offset] = definition.color_rgb[0];
            pixels[offset + 1] = definition.color_rgb[1];
            pixels[offset + 2] = definition.color_rgb[2];
            pixels[offset + 3] = ((coverage * 255 + sample_count / 2) / sample_count) as u8;
        }
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    AnnounceTextTexture {
        texture,
        local_bounds,
    }
}

fn build_round_number_text_texture(
    definition: &round_number_texts::RoundNumberDefinition,
) -> AnnounceTextTexture {
    let local_bounds = round_number_text_local_bounds(definition);
    let width = (local_bounds.w * f32::from(ANNOUNCE_TEXT_TEXTURE_SCALE))
        .ceil()
        .max(1.0) as u16;
    let height = (local_bounds.h * f32::from(ANNOUNCE_TEXT_TEXTURE_SCALE))
        .ceil()
        .max(1.0) as u16;
    let contours = round_number_text_flattened_contours(definition);
    let sample_count = ANNOUNCE_TEXT_TEXTURE_SUPERSAMPLE * ANNOUNCE_TEXT_TEXTURE_SUPERSAMPLE;
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];

    for y in 0..height {
        for x in 0..width {
            let mut coverage = 0;
            for sub_y in 0..ANNOUNCE_TEXT_TEXTURE_SUPERSAMPLE {
                for sub_x in 0..ANNOUNCE_TEXT_TEXTURE_SUPERSAMPLE {
                    let local = announce_text_texture_sample_local(
                        local_bounds,
                        (width, height),
                        (x, y),
                        (sub_x, sub_y),
                    );
                    if round_number_text_contains_local(&contours, local) {
                        coverage += 1;
                    }
                }
            }

            let offset = (usize::from(y) * usize::from(width) + usize::from(x)) * 4;
            pixels[offset] = definition.color_rgb[0];
            pixels[offset + 1] = definition.color_rgb[1];
            pixels[offset + 2] = definition.color_rgb[2];
            pixels[offset + 3] = ((coverage * 255 + sample_count / 2) / sample_count) as u8;
        }
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    AnnounceTextTexture {
        texture,
        local_bounds,
    }
}

fn announce_text_texture_sample_local(
    local_bounds: RectVisual,
    texture_size: (u16, u16),
    pixel: (u16, u16),
    subpixel: (u16, u16),
) -> SwfPoint {
    let supersample = f32::from(ANNOUNCE_TEXT_TEXTURE_SUPERSAMPLE);
    let (width, height) = texture_size;
    let (x, y) = pixel;
    let (sub_x, sub_y) = subpixel;
    SwfPoint::new(
        local_bounds.x
            + (f32::from(x) + (f32::from(sub_x) + 0.5) / supersample) * local_bounds.w
                / f32::from(width),
        local_bounds.y
            + (f32::from(y) + (f32::from(sub_y) + 0.5) / supersample) * local_bounds.h
                / f32::from(height),
    )
}

fn counter_digit_texture_sample_local(
    local_bounds: RectVisual,
    texture_size: (u16, u16),
    pixel: (u16, u16),
    subpixel: (u16, u16),
) -> SwfPoint {
    let supersample = f32::from(COUNTER_DIGIT_TEXTURE_SUPERSAMPLE);
    let (width, height) = texture_size;
    let (x, y) = pixel;
    let (sub_x, sub_y) = subpixel;
    SwfPoint::new(
        local_bounds.x
            + (f32::from(x) + (f32::from(sub_x) + 0.5) / supersample) * local_bounds.w
                / f32::from(width),
        local_bounds.y
            + (f32::from(y) + (f32::from(sub_y) + 0.5) / supersample) * local_bounds.h
                / f32::from(height),
    )
}

fn announce_text_local_bounds(definition: &announce_texts::AnnounceTextDefinition) -> RectVisual {
    let x_min = f32::from(definition.bounds_centipx[0]) / 100.0;
    let x_max = f32::from(definition.bounds_centipx[1]) / 100.0;
    let y_min = f32::from(definition.bounds_centipx[2]) / 100.0;
    let y_max = f32::from(definition.bounds_centipx[3]) / 100.0;
    RectVisual {
        x: x_min,
        y: y_min,
        w: x_max - x_min,
        h: y_max - y_min,
    }
}

fn counter_digit_local_bounds(
    definition: &counter_digit_texts::CounterDigitDefinition,
) -> RectVisual {
    let x_min = f32::from(definition.bounds_centipx[0]) / 100.0;
    let x_max = f32::from(definition.bounds_centipx[1]) / 100.0;
    let y_min = f32::from(definition.bounds_centipx[2]) / 100.0;
    let y_max = f32::from(definition.bounds_centipx[3]) / 100.0;
    RectVisual {
        x: x_min,
        y: y_min,
        w: x_max - x_min,
        h: y_max - y_min,
    }
}

fn announce_text_flattened_contours(
    definition: &announce_texts::AnnounceTextDefinition,
) -> Vec<Vec<SwfPoint>> {
    definition
        .contours
        .iter()
        .copied()
        .map(announce_text_contour_points)
        .collect()
}

fn counter_digit_flattened_contours(
    definition: &counter_digit_texts::CounterDigitDefinition,
) -> Vec<Vec<SwfPoint>> {
    definition
        .contours
        .iter()
        .copied()
        .map(counter_digit_contour_points)
        .collect()
}

fn announce_text_contour_points(contour: announce_texts::AnnounceTextContour) -> Vec<SwfPoint> {
    let mut points =
        Vec::with_capacity(1 + contour.segments.len() * usize::from(ANNOUNCE_TEXT_FLATTEN_STEPS));
    let mut current = menu_label_centipixel_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            announce_texts::AnnounceTextSegment::Line(to) => {
                current = menu_label_centipixel_point(to);
                points.push(current);
            },
            announce_texts::AnnounceTextSegment::Quad { control, to } => {
                let control = menu_label_centipixel_point(control);
                let to = menu_label_centipixel_point(to);
                for step in 1..=ANNOUNCE_TEXT_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(ANNOUNCE_TEXT_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn counter_digit_contour_points(
    contour: counter_digit_texts::CounterDigitContour,
) -> Vec<SwfPoint> {
    let mut points =
        Vec::with_capacity(1 + contour.segments.len() * usize::from(COUNTER_DIGIT_FLATTEN_STEPS));
    let mut current = menu_label_centipixel_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            counter_digit_texts::CounterDigitSegment::Line(to) => {
                current = menu_label_centipixel_point(to);
                points.push(current);
            },
            counter_digit_texts::CounterDigitSegment::Quad { control, to } => {
                let control = menu_label_centipixel_point(control);
                let to = menu_label_centipixel_point(to);
                for step in 1..=COUNTER_DIGIT_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(COUNTER_DIGIT_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn announce_text_contains_local(contours: &[Vec<SwfPoint>], local: SwfPoint) -> bool {
    contours
        .iter()
        .filter(|contour| point_in_polygon(local, contour))
        .count()
        % 2
        == 1
}

fn counter_digit_contains_local(contours: &[Vec<SwfPoint>], local: SwfPoint) -> bool {
    contours
        .iter()
        .filter(|contour| point_in_polygon(local, contour))
        .count()
        % 2
        == 1
}

fn round_number_text_local_bounds(
    definition: &round_number_texts::RoundNumberDefinition,
) -> RectVisual {
    let x_min = f32::from(definition.bounds_centipx[0]) / 100.0;
    let x_max = f32::from(definition.bounds_centipx[1]) / 100.0;
    let y_min = f32::from(definition.bounds_centipx[2]) / 100.0;
    let y_max = f32::from(definition.bounds_centipx[3]) / 100.0;
    RectVisual {
        x: x_min,
        y: y_min,
        w: x_max - x_min,
        h: y_max - y_min,
    }
}

fn round_number_text_flattened_contours(
    definition: &round_number_texts::RoundNumberDefinition,
) -> Vec<Vec<SwfPoint>> {
    definition
        .contours
        .iter()
        .copied()
        .map(round_number_text_contour_points)
        .collect()
}

fn round_number_text_contour_points(
    contour: round_number_texts::RoundNumberContour,
) -> Vec<SwfPoint> {
    let mut points =
        Vec::with_capacity(1 + contour.segments.len() * usize::from(ANNOUNCE_TEXT_FLATTEN_STEPS));
    let mut current = menu_label_centipixel_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            round_number_texts::RoundNumberSegment::Line(to) => {
                current = menu_label_centipixel_point(to);
                points.push(current);
            },
            round_number_texts::RoundNumberSegment::Quad { control, to } => {
                let control = menu_label_centipixel_point(control);
                let to = menu_label_centipixel_point(to);
                for step in 1..=ANNOUNCE_TEXT_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(ANNOUNCE_TEXT_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn round_number_text_contains_local(contours: &[Vec<SwfPoint>], local: SwfPoint) -> bool {
    contours
        .iter()
        .filter(|contour| point_in_polygon(local, contour))
        .count()
        % 2
        == 1
}

fn menu_label_texture_sample_local(
    placement: RectVisual,
    text_placement: SwfTextPlacement,
    texture_size: (u16, u16),
    pixel: (u16, u16),
    subpixel: (u16, u16),
) -> SwfPoint {
    let supersample = f32::from(MENU_LABEL_TEXTURE_SUPERSAMPLE);
    let (width, height) = texture_size;
    let (x, y) = pixel;
    let (sub_x, sub_y) = subpixel;
    let stage_x = placement.x
        + (f32::from(x) + (f32::from(sub_x) + 0.5) / supersample) * placement.w / f32::from(width);
    let stage_y = placement.y
        + (f32::from(y) + (f32::from(sub_y) + 0.5) / supersample) * placement.h / f32::from(height);
    SwfPoint::new(
        (stage_x - text_placement.root_tx) / text_placement.root_sx - text_placement.local_tx,
        (stage_y - text_placement.root_ty) / text_placement.root_sy - text_placement.local_ty,
    )
}

fn menu_label_stage_bounds(action: MenuAction) -> RectVisual {
    let placement = menu_label_placement(action);
    let bounds = menu_label_local_bounds(menu_label_definition(action));
    RectVisual {
        x: placement
            .root_sx
            .mul_add(placement.local_tx + bounds.x_min, placement.root_tx),
        y: placement
            .root_sy
            .mul_add(placement.local_ty + bounds.y_min, placement.root_ty),
        w: placement.root_sx * (bounds.x_max - bounds.x_min),
        h: placement.root_sy * (bounds.y_max - bounds.y_min),
    }
}

fn menu_label_local_bounds(definition: &menu_label_texts::MenuLabelDefinition) -> SwfBounds {
    SwfBounds {
        x_min: f32::from(definition.bounds_centipx[0]) / 100.0,
        x_max: f32::from(definition.bounds_centipx[1]) / 100.0,
        y_min: f32::from(definition.bounds_centipx[2]) / 100.0,
        y_max: f32::from(definition.bounds_centipx[3]) / 100.0,
    }
}

fn menu_label_flattened_contours(
    definition: &menu_label_texts::MenuLabelDefinition,
) -> Vec<Vec<SwfPoint>> {
    definition
        .contours
        .iter()
        .copied()
        .map(menu_label_contour_points)
        .collect()
}

fn menu_label_contour_points(contour: menu_label_texts::MenuLabelContour) -> Vec<SwfPoint> {
    let mut points =
        Vec::with_capacity(1 + contour.segments.len() * usize::from(MENU_LABEL_FLATTEN_STEPS));
    let mut current = menu_label_centipixel_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            menu_label_texts::MenuLabelSegment::Line(to) => {
                current = menu_label_centipixel_point(to);
                points.push(current);
            },
            menu_label_texts::MenuLabelSegment::Quad { control, to } => {
                let control = menu_label_centipixel_point(control);
                let to = menu_label_centipixel_point(to);
                for step in 1..=MENU_LABEL_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(MENU_LABEL_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn menu_label_centipixel_point(point: [i16; 2]) -> SwfPoint {
    SwfPoint::new(f32::from(point[0]) / 100.0, f32::from(point[1]) / 100.0)
}

fn menu_label_contains_local(contours: &[Vec<SwfPoint>], local: SwfPoint) -> bool {
    contours
        .iter()
        .filter(|contour| point_in_polygon(local, contour))
        .count()
        % 2
        == 1
}

fn build_top_title_texture() -> SwfTextTexture {
    debug_assert_eq!(top_title_text81::TEXT, "Gravity");
    debug_assert_eq!(top_title_text81::FONT_ID, 54);
    debug_assert_eq!(top_title_text81::DEFINE_TEXT_ID, 81);
    let placement = top_title_stage_bounds();
    let width = placement.w.ceil().max(1.0) as u16;
    let height = placement.h.ceil().max(1.0) as u16;
    let contours = top_title_flattened_contours();
    let sample_count = TOP_TITLE_TEXTURE_SUPERSAMPLE * TOP_TITLE_TEXTURE_SUPERSAMPLE;
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];

    for y in 0..height {
        for x in 0..width {
            let mut coverage = 0;
            for sub_y in 0..TOP_TITLE_TEXTURE_SUPERSAMPLE {
                for sub_x in 0..TOP_TITLE_TEXTURE_SUPERSAMPLE {
                    let local = top_title_texture_sample_local(
                        placement, width, height, x, y, sub_x, sub_y,
                    );
                    if top_title_contains_local(&contours, local) {
                        coverage += 1;
                    }
                }
            }

            let offset = (usize::from(y) * usize::from(width) + usize::from(x)) * 4;
            pixels[offset] = 255;
            pixels[offset + 1] = 255;
            pixels[offset + 2] = 255;
            pixels[offset + 3] = ((coverage * 255 + sample_count / 2) / sample_count) as u8;
        }
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    SwfTextTexture { texture, placement }
}

fn top_title_texture_sample_local(
    placement: RectVisual,
    width: u16,
    height: u16,
    x: u16,
    y: u16,
    sub_x: u16,
    sub_y: u16,
) -> SwfPoint {
    let supersample = f32::from(TOP_TITLE_TEXTURE_SUPERSAMPLE);
    let stage_x = placement.x
        + (f32::from(x) + (f32::from(sub_x) + 0.5) / supersample) * placement.w / f32::from(width);
    let stage_y = placement.y
        + (f32::from(y) + (f32::from(sub_y) + 0.5) / supersample) * placement.h / f32::from(height);
    SwfPoint::new(
        (stage_x - TOP_TITLE_TEXT.root_tx) / TOP_TITLE_TEXT.root_sx - TOP_TITLE_TEXT.local_tx,
        (stage_y - TOP_TITLE_TEXT.root_ty) / TOP_TITLE_TEXT.root_sy - TOP_TITLE_TEXT.local_ty,
    )
}

fn top_title_stage_bounds() -> RectVisual {
    let bounds = top_title_local_bounds();
    RectVisual {
        x: TOP_TITLE_TEXT.root_sx.mul_add(
            TOP_TITLE_TEXT.local_tx + bounds.x_min,
            TOP_TITLE_TEXT.root_tx,
        ),
        y: TOP_TITLE_TEXT.root_sy.mul_add(
            TOP_TITLE_TEXT.local_ty + bounds.y_min,
            TOP_TITLE_TEXT.root_ty,
        ),
        w: TOP_TITLE_TEXT.root_sx * (bounds.x_max - bounds.x_min),
        h: TOP_TITLE_TEXT.root_sy * (bounds.y_max - bounds.y_min),
    }
}

fn top_title_local_bounds() -> SwfBounds {
    SwfBounds {
        x_min: f32::from(top_title_text81::BOUNDS_CENTIPX[0]) / 100.0,
        x_max: f32::from(top_title_text81::BOUNDS_CENTIPX[1]) / 100.0,
        y_min: f32::from(top_title_text81::BOUNDS_CENTIPX[2]) / 100.0,
        y_max: f32::from(top_title_text81::BOUNDS_CENTIPX[3]) / 100.0,
    }
}

fn top_title_flattened_contours() -> Vec<Vec<SwfPoint>> {
    top_title_text81::CONTOURS
        .iter()
        .copied()
        .map(top_title_contour_points)
        .collect()
}

fn top_title_contour_points(contour: top_title_text81::TopTitleContour) -> Vec<SwfPoint> {
    let mut points =
        Vec::with_capacity(1 + contour.segments.len() * usize::from(TOP_TITLE_FLATTEN_STEPS));
    let mut current = top_title_centipixel_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            top_title_text81::TopTitleSegment::Line(to) => {
                current = top_title_centipixel_point(to);
                points.push(current);
            },
            top_title_text81::TopTitleSegment::Quad { control, to } => {
                let control = top_title_centipixel_point(control);
                let to = top_title_centipixel_point(to);
                for step in 1..=TOP_TITLE_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(TOP_TITLE_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn top_title_centipixel_point(point: [i16; 2]) -> SwfPoint {
    SwfPoint::new(f32::from(point[0]) / 100.0, f32::from(point[1]) / 100.0)
}

fn top_title_contains_local(contours: &[Vec<SwfPoint>], local: SwfPoint) -> bool {
    contours
        .iter()
        .filter(|contour| point_in_polygon(local, contour))
        .count()
        % 2
        == 1
}

fn build_rounds_played_label_texture() -> SwfTextTexture {
    debug_assert_eq!(rounds_played_text77::TEXT, "rounds played:");
    debug_assert_eq!(rounds_played_text77::FONT_ID, 26);
    debug_assert_eq!(rounds_played_text77::DEFINE_TEXT_ID, 77);
    let placement = rounds_played_label_stage_bounds();
    let width = placement.w.ceil().max(1.0) as u16;
    let height = placement.h.ceil().max(1.0) as u16;
    let contours = rounds_played_label_flattened_contours();
    let sample_count =
        ROUNDS_PLAYED_LABEL_TEXTURE_SUPERSAMPLE * ROUNDS_PLAYED_LABEL_TEXTURE_SUPERSAMPLE;
    let color = [
        color_channel_to_u8(MUTED.r),
        color_channel_to_u8(MUTED.g),
        color_channel_to_u8(MUTED.b),
    ];
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];

    for y in 0..height {
        for x in 0..width {
            let mut coverage = 0;
            for sub_y in 0..ROUNDS_PLAYED_LABEL_TEXTURE_SUPERSAMPLE {
                for sub_x in 0..ROUNDS_PLAYED_LABEL_TEXTURE_SUPERSAMPLE {
                    let local = rounds_played_label_texture_sample_local(
                        placement, width, height, x, y, sub_x, sub_y,
                    );
                    if rounds_played_label_contains_local(&contours, local) {
                        coverage += 1;
                    }
                }
            }

            let offset = (usize::from(y) * usize::from(width) + usize::from(x)) * 4;
            pixels[offset] = color[0];
            pixels[offset + 1] = color[1];
            pixels[offset + 2] = color[2];
            pixels[offset + 3] = ((coverage * 255 + sample_count / 2) / sample_count) as u8;
        }
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    SwfTextTexture { texture, placement }
}

fn rounds_played_label_texture_sample_local(
    placement: RectVisual,
    width: u16,
    height: u16,
    x: u16,
    y: u16,
    sub_x: u16,
    sub_y: u16,
) -> SwfPoint {
    let supersample = f32::from(ROUNDS_PLAYED_LABEL_TEXTURE_SUPERSAMPLE);
    let stage_x = placement.x
        + (f32::from(x) + (f32::from(sub_x) + 0.5) / supersample) * placement.w / f32::from(width);
    let stage_y = placement.y
        + (f32::from(y) + (f32::from(sub_y) + 0.5) / supersample) * placement.h / f32::from(height);
    SwfPoint::new(
        (stage_x - ROUNDS_PLAYED_LABEL_TEXT.root_tx) / ROUNDS_PLAYED_LABEL_TEXT.root_sx
            - ROUNDS_PLAYED_LABEL_TEXT.local_tx,
        (stage_y - ROUNDS_PLAYED_LABEL_TEXT.root_ty) / ROUNDS_PLAYED_LABEL_TEXT.root_sy
            - ROUNDS_PLAYED_LABEL_TEXT.local_ty,
    )
}

fn rounds_played_label_stage_bounds() -> RectVisual {
    let bounds = rounds_played_label_local_bounds();
    RectVisual {
        x: ROUNDS_PLAYED_LABEL_TEXT.root_sx.mul_add(
            ROUNDS_PLAYED_LABEL_TEXT.local_tx + bounds.x_min,
            ROUNDS_PLAYED_LABEL_TEXT.root_tx,
        ),
        y: ROUNDS_PLAYED_LABEL_TEXT.root_sy.mul_add(
            ROUNDS_PLAYED_LABEL_TEXT.local_ty + bounds.y_min,
            ROUNDS_PLAYED_LABEL_TEXT.root_ty,
        ),
        w: ROUNDS_PLAYED_LABEL_TEXT.root_sx * (bounds.x_max - bounds.x_min),
        h: ROUNDS_PLAYED_LABEL_TEXT.root_sy * (bounds.y_max - bounds.y_min),
    }
}

fn rounds_played_label_local_bounds() -> SwfBounds {
    SwfBounds {
        x_min: f32::from(rounds_played_text77::BOUNDS_CENTIPX[0]) / 100.0,
        x_max: f32::from(rounds_played_text77::BOUNDS_CENTIPX[1]) / 100.0,
        y_min: f32::from(rounds_played_text77::BOUNDS_CENTIPX[2]) / 100.0,
        y_max: f32::from(rounds_played_text77::BOUNDS_CENTIPX[3]) / 100.0,
    }
}

fn rounds_played_label_flattened_contours() -> Vec<Vec<SwfPoint>> {
    rounds_played_text77::CONTOURS
        .iter()
        .copied()
        .map(rounds_played_label_contour_points)
        .collect()
}

fn rounds_played_label_contour_points(
    contour: rounds_played_text77::RoundsPlayedContour,
) -> Vec<SwfPoint> {
    let mut points = Vec::with_capacity(
        1 + contour.segments.len() * usize::from(ROUNDS_PLAYED_LABEL_FLATTEN_STEPS),
    );
    let mut current = rounds_played_label_centipixel_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            rounds_played_text77::RoundsPlayedSegment::Line(to) => {
                current = rounds_played_label_centipixel_point(to);
                points.push(current);
            },
            rounds_played_text77::RoundsPlayedSegment::Quad { control, to } => {
                let control = rounds_played_label_centipixel_point(control);
                let to = rounds_played_label_centipixel_point(to);
                for step in 1..=ROUNDS_PLAYED_LABEL_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(ROUNDS_PLAYED_LABEL_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn rounds_played_label_centipixel_point(point: [i16; 2]) -> SwfPoint {
    SwfPoint::new(f32::from(point[0]) / 100.0, f32::from(point[1]) / 100.0)
}

fn rounds_played_label_contains_local(contours: &[Vec<SwfPoint>], local: SwfPoint) -> bool {
    contours
        .iter()
        .filter(|contour| point_in_polygon(local, contour))
        .count()
        % 2
        == 1
}

fn build_sponsor_logo_assets(transform: LogoRootTransform) -> SponsorLogoAssets {
    SponsorLogoAssets {
        text_layers: build_sponsor_logo_textures(transform),
        foreground: build_sponsor_logo_texture(transform),
    }
}

fn build_sponsor_logo_textures(transform: LogoRootTransform) -> SponsorLogoTextTextures {
    SponsorLogoTextTextures {
        dark: build_sponsor_logo_text_texture(
            &sponsor_logo_texts::DARK,
            SPONSOR_LOGO_DEPTH_3_TEXT,
            transform,
        ),
        olive: build_sponsor_logo_text_texture(
            &sponsor_logo_texts::OLIVE,
            SPONSOR_LOGO_DEPTH_4_TEXT,
            transform,
        ),
    }
}

fn build_sponsor_logo_text_texture(
    definition: &sponsor_logo_texts::SponsorLogoTextDefinition,
    text_placement: SponsorLogoTextPlacement,
    transform: LogoRootTransform,
) -> SwfTextTexture {
    let placement = sponsor_logo_text_stage_bounds_at(definition, text_placement, transform);
    let width = placement.w.ceil().max(1.0) as u16;
    let height = placement.h.ceil().max(1.0) as u16;
    let contours = sponsor_logo_text_flattened_contours(definition);
    let sample_count =
        SPONSOR_LOGO_TEXT_TEXTURE_SUPERSAMPLE * SPONSOR_LOGO_TEXT_TEXTURE_SUPERSAMPLE;
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];

    for y in 0..height {
        for x in 0..width {
            let mut coverage = 0;
            for sub_y in 0..SPONSOR_LOGO_TEXT_TEXTURE_SUPERSAMPLE {
                for sub_x in 0..SPONSOR_LOGO_TEXT_TEXTURE_SUPERSAMPLE {
                    let local = sponsor_logo_text_texture_sample_local(
                        placement,
                        text_placement,
                        transform,
                        (width, height),
                        (x, y),
                        (sub_x, sub_y),
                    );
                    if sponsor_logo_text_contains_local(&contours, local) {
                        coverage += 1;
                    }
                }
            }

            let offset = (usize::from(y) * usize::from(width) + usize::from(x)) * 4;
            pixels[offset] = definition.color_rgb[0];
            pixels[offset + 1] = definition.color_rgb[1];
            pixels[offset + 2] = definition.color_rgb[2];
            pixels[offset + 3] = ((coverage * 255 + sample_count / 2) / sample_count) as u8;
        }
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    SwfTextTexture { texture, placement }
}

fn sponsor_logo_text_texture_sample_local(
    placement: RectVisual,
    text_placement: SponsorLogoTextPlacement,
    transform: LogoRootTransform,
    texture_size: (u16, u16),
    pixel: (u16, u16),
    subpixel: (u16, u16),
) -> SwfPoint {
    let supersample = f32::from(SPONSOR_LOGO_TEXT_TEXTURE_SUPERSAMPLE);
    let (width, height) = texture_size;
    let (x, y) = pixel;
    let (sub_x, sub_y) = subpixel;
    let stage_x = placement.x
        + (f32::from(x) + (f32::from(sub_x) + 0.5) / supersample) * placement.w / f32::from(width);
    let stage_y = placement.y
        + (f32::from(y) + (f32::from(sub_y) + 0.5) / supersample) * placement.h / f32::from(height);
    SwfPoint::new(
        ((stage_x - transform.tx) / transform.scale - text_placement.tx) / SPONSOR_LOGO_TEXT_SCALE,
        ((stage_y - transform.ty) / transform.scale - text_placement.ty) / SPONSOR_LOGO_TEXT_SCALE,
    )
}

#[cfg(test)]
fn sponsor_logo_text_stage_bounds(
    definition: &sponsor_logo_texts::SponsorLogoTextDefinition,
    text_placement: SponsorLogoTextPlacement,
) -> RectVisual {
    sponsor_logo_text_stage_bounds_at(definition, text_placement, SPONSOR_LOGO_ROOT_TRANSFORM)
}

fn sponsor_logo_text_stage_bounds_at(
    definition: &sponsor_logo_texts::SponsorLogoTextDefinition,
    text_placement: SponsorLogoTextPlacement,
    transform: LogoRootTransform,
) -> RectVisual {
    let bounds = sponsor_logo_text_local_bounds(definition);
    RectVisual {
        x: transform.scale.mul_add(
            SPONSOR_LOGO_TEXT_SCALE.mul_add(bounds.x_min, text_placement.tx),
            transform.tx,
        ),
        y: transform.scale.mul_add(
            SPONSOR_LOGO_TEXT_SCALE.mul_add(bounds.y_min, text_placement.ty),
            transform.ty,
        ),
        w: transform.scale * SPONSOR_LOGO_TEXT_SCALE * (bounds.x_max - bounds.x_min),
        h: transform.scale * SPONSOR_LOGO_TEXT_SCALE * (bounds.y_max - bounds.y_min),
    }
}

fn sponsor_logo_text_local_bounds(
    definition: &sponsor_logo_texts::SponsorLogoTextDefinition,
) -> SwfBounds {
    SwfBounds {
        x_min: f32::from(definition.bounds_centipx[0]) / 100.0,
        x_max: f32::from(definition.bounds_centipx[1]) / 100.0,
        y_min: f32::from(definition.bounds_centipx[2]) / 100.0,
        y_max: f32::from(definition.bounds_centipx[3]) / 100.0,
    }
}

fn sponsor_logo_text_flattened_contours(
    definition: &sponsor_logo_texts::SponsorLogoTextDefinition,
) -> Vec<Vec<SwfPoint>> {
    definition
        .contours
        .iter()
        .copied()
        .map(sponsor_logo_text_contour_points)
        .collect()
}

fn sponsor_logo_text_contour_points(
    contour: sponsor_logo_texts::SponsorLogoTextContour,
) -> Vec<SwfPoint> {
    let mut points = Vec::with_capacity(
        1 + contour.segments.len() * usize::from(SPONSOR_LOGO_TEXT_FLATTEN_STEPS),
    );
    let mut current = menu_label_centipixel_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            sponsor_logo_texts::SponsorLogoTextSegment::Line(to) => {
                current = menu_label_centipixel_point(to);
                points.push(current);
            },
            sponsor_logo_texts::SponsorLogoTextSegment::Quad { control, to } => {
                let control = menu_label_centipixel_point(control);
                let to = menu_label_centipixel_point(to);
                for step in 1..=SPONSOR_LOGO_TEXT_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(SPONSOR_LOGO_TEXT_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn sponsor_logo_text_contains_local(contours: &[Vec<SwfPoint>], local: SwfPoint) -> bool {
    contours
        .iter()
        .filter(|contour| point_in_polygon(local, contour))
        .count()
        % 2
        == 1
}

fn build_sponsor_logo_texture(transform: LogoRootTransform) -> SponsorLogoTexture {
    let placement = sponsor_logo_shape35_stage_bounds_at(transform);
    let width = placement.w.ceil().max(1.0) as u16;
    let height = placement.h.ceil().max(1.0) as u16;
    let contours = sponsor_logo_shape35_flattened_contours();
    let color = [
        color_channel_to_u8(SPONSOR_LOGO_FILL.r),
        color_channel_to_u8(SPONSOR_LOGO_FILL.g),
        color_channel_to_u8(SPONSOR_LOGO_FILL.b),
    ];
    let sample_count = SPONSOR_LOGO_TEXTURE_SUPERSAMPLE * SPONSOR_LOGO_TEXTURE_SUPERSAMPLE;
    let mut pixels = vec![0; usize::from(width) * usize::from(height) * 4];

    for y in 0..height {
        for x in 0..width {
            let mut coverage = 0;
            for sub_y in 0..SPONSOR_LOGO_TEXTURE_SUPERSAMPLE {
                for sub_x in 0..SPONSOR_LOGO_TEXTURE_SUPERSAMPLE {
                    let local = sponsor_logo_texture_sample_local(
                        placement,
                        transform,
                        (width, height),
                        (x, y),
                        (sub_x, sub_y),
                    );
                    if sponsor_logo_shape35_contains_local(&contours, local) {
                        coverage += 1;
                    }
                }
            }

            let offset = (usize::from(y) * usize::from(width) + usize::from(x)) * 4;
            pixels[offset] = color[0];
            pixels[offset + 1] = color[1];
            pixels[offset + 2] = color[2];
            pixels[offset + 3] = ((coverage * 255 + sample_count / 2) / sample_count) as u8;
        }
    }

    let texture = Texture2D::from_rgba8(width, height, &pixels);
    texture.set_filter(FilterMode::Linear);
    SponsorLogoTexture { texture, placement }
}

fn sponsor_logo_texture_sample_local(
    placement: RectVisual,
    transform: LogoRootTransform,
    texture_size: (u16, u16),
    pixel: (u16, u16),
    subpixel: (u16, u16),
) -> SwfPoint {
    let supersample = f32::from(SPONSOR_LOGO_TEXTURE_SUPERSAMPLE);
    let (width, height) = texture_size;
    let (x, y) = pixel;
    let (sub_x, sub_y) = subpixel;
    let stage_x = placement.x
        + (f32::from(x) + (f32::from(sub_x) + 0.5) / supersample) * placement.w / f32::from(width);
    let stage_y = placement.y
        + (f32::from(y) + (f32::from(sub_y) + 0.5) / supersample) * placement.h / f32::from(height);
    SwfPoint::new(
        (stage_x - transform.tx) / transform.scale,
        (stage_y - transform.ty) / transform.scale,
    )
}

#[cfg(test)]
fn sponsor_logo_shape35_stage_bounds() -> RectVisual {
    sponsor_logo_shape35_stage_bounds_at(SPONSOR_LOGO_ROOT_TRANSFORM)
}

fn sponsor_logo_shape35_stage_bounds_at(transform: LogoRootTransform) -> RectVisual {
    let bounds = sponsor_logo_shape35_local_bounds();
    RectVisual {
        x: transform.scale.mul_add(bounds.x_min, transform.tx),
        y: transform.scale.mul_add(bounds.y_min, transform.ty),
        w: transform.scale * (bounds.x_max - bounds.x_min),
        h: transform.scale * (bounds.y_max - bounds.y_min),
    }
}

fn sponsor_logo_shape35_local_bounds() -> SwfBounds {
    SwfBounds {
        x_min: f32::from(sponsor_logo_shape35::BOUNDS_TWIPS[0]) / 20.0,
        x_max: f32::from(sponsor_logo_shape35::BOUNDS_TWIPS[1]) / 20.0,
        y_min: f32::from(sponsor_logo_shape35::BOUNDS_TWIPS[2]) / 20.0,
        y_max: f32::from(sponsor_logo_shape35::BOUNDS_TWIPS[3]) / 20.0,
    }
}

fn sponsor_logo_shape35_flattened_contours() -> Vec<Vec<SwfPoint>> {
    sponsor_logo_shape35::CONTOURS
        .iter()
        .copied()
        .map(sponsor_logo_shape35_contour_points)
        .collect()
}

fn sponsor_logo_shape35_contour_points(
    contour: sponsor_logo_shape35::Shape35Contour,
) -> Vec<SwfPoint> {
    let mut points = Vec::with_capacity(
        1 + contour.segments.len() * usize::from(SPONSOR_LOGO_SHAPE35_FLATTEN_STEPS),
    );
    let mut current = shape35_twip_point(contour.start);
    points.push(current);
    for segment in contour.segments {
        match *segment {
            sponsor_logo_shape35::Shape35Segment::Line(to) => {
                current = shape35_twip_point(to);
                points.push(current);
            },
            sponsor_logo_shape35::Shape35Segment::Quad { control, to } => {
                let control = shape35_twip_point(control);
                let to = shape35_twip_point(to);
                for step in 1..=SPONSOR_LOGO_SHAPE35_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(SPONSOR_LOGO_SHAPE35_FLATTEN_STEPS);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn shape35_twip_point(point: [i16; 2]) -> SwfPoint {
    SwfPoint::new(f32::from(point[0]) / 20.0, f32::from(point[1]) / 20.0)
}

fn sponsor_logo_shape35_contains_local(contours: &[Vec<SwfPoint>], local: SwfPoint) -> bool {
    contours
        .iter()
        .filter(|contour| point_in_polygon(local, contour))
        .count()
        % 2
        == 1
}

fn color_channel_to_u8(channel: f32) -> u8 {
    (channel.clamp(0.0, 1.0) * 255.0).round() as u8
}

#[cfg(test)]
const fn header_link_label(link: HeaderLink) -> &'static str {
    match link {
        HeaderLink::Sponsor => "to neodelight",
        HeaderLink::BackToMenu => "back to menu",
    }
}

fn header_link_rect(link: HeaderLink) -> Rect {
    bounds_from_points(header_link_hit_polygon(link))
}

const fn header_link_hit_polygon(link: HeaderLink) -> &'static [SwfPoint; 4] {
    match link {
        // Button 99 hit shape 98, placed at (36.65, 7.7).
        HeaderLink::Sponsor => &SPONSOR_HEADER_HIT_POLYGON,
        // Button 124 hit shape 123, placed at (36.65, 7.7).
        HeaderLink::BackToMenu => &BACK_HEADER_HIT_POLYGON,
    }
}

fn bounds_from_points(points: &[SwfPoint; 4]) -> Rect {
    let mut x_min = points[0].x;
    let mut x_max = points[0].x;
    let mut y_min = points[0].y;
    let mut y_max = points[0].y;
    for point in &points[1..] {
        x_min = x_min.min(point.x);
        x_max = x_max.max(point.x);
        y_min = y_min.min(point.y);
        y_max = y_max.max(point.y);
    }
    Rect::new(x_min, y_min, x_max - x_min, y_max - y_min)
}

fn header_link_contains(link: HeaderLink, x: f32, y: f32) -> bool {
    if !header_link_rect(link).contains(vec2(x, y)) {
        return false;
    }
    point_in_polygon(SwfPoint::new(x, y), header_link_hit_polygon(link))
}

fn external_link_at(screen: Screen, x: f32, y: f32) -> Option<ExternalLinkAction> {
    if matches!(screen, Screen::Startup) {
        return None;
    }
    if screen == Screen::Menu && header_link_contains(HeaderLink::Sponsor, x, y) {
        return Some(ExternalLinkAction::NeodelightSelf);
    }
    if sponsor_logo_visible(screen) && sponsor_logo_button_contains(x, y) {
        return Some(ExternalLinkAction::NeodelightBlank);
    }
    if version_footer_visible(screen) && neokolor_link_button_contains(x, y) {
        return Some(ExternalLinkAction::NeokolorBlank);
    }
    None
}

const fn sponsor_logo_visible(screen: Screen) -> bool {
    matches!(screen, Screen::Menu | Screen::Help)
}

fn sponsor_logo_button_contains(x: f32, y: f32) -> bool {
    sponsor_logo_button_rect().contains(vec2(x, y))
}

fn neokolor_link_button_contains(x: f32, y: f32) -> bool {
    neokolor_link_button_rect().contains(vec2(x, y))
}

fn sponsor_logo_button_rect() -> Rect {
    placed_swf_hit_rect(
        SPONSOR_LOGO_ROOT_X,
        SPONSOR_LOGO_ROOT_Y,
        SPONSOR_LOGO_ROOT_SCALE,
        SPONSOR_LOGO_ROOT_SCALE,
        SPONSOR_LOGO_BUTTON_TX,
        SPONSOR_LOGO_BUTTON_TY,
        SPONSOR_LOGO_BUTTON_HIT_BOUNDS,
    )
}

fn neokolor_link_button_rect() -> Rect {
    placed_swf_hit_rect(
        NEOKOLOR_LINK_ROOT_X,
        NEOKOLOR_LINK_ROOT_Y,
        NEOKOLOR_LINK_BUTTON_SCALE_X,
        NEOKOLOR_LINK_BUTTON_SCALE_Y,
        NEOKOLOR_LINK_BUTTON_TX,
        NEOKOLOR_LINK_BUTTON_TY,
        NEOKOLOR_LINK_BUTTON_HIT_BOUNDS,
    )
}

fn placed_swf_hit_rect(
    root_x: f32,
    root_y: f32,
    scale_x: f32,
    scale_y: f32,
    child_x: f32,
    child_y: f32,
    bounds: SwfBounds,
) -> Rect {
    Rect::new(
        scale_x.mul_add(child_x + bounds.x_min, root_x),
        scale_y.mul_add(child_y + bounds.y_min, root_y),
        scale_x * (bounds.x_max - bounds.x_min),
        scale_y * (bounds.y_max - bounds.y_min),
    )
}

fn external_link_url(link: ExternalLinkAction) -> &'static str {
    match link {
        ExternalLinkAction::NeodelightBlank | ExternalLinkAction::NeodelightSelf => {
            "http://www.neodelight.com"
        },
        ExternalLinkAction::NeokolorBlank => "http://www.neokolor.com",
    }
}

#[cfg(test)]
const fn external_link_target(link: ExternalLinkAction) -> &'static str {
    match link {
        ExternalLinkAction::NeodelightBlank | ExternalLinkAction::NeokolorBlank => "_blank",
        ExternalLinkAction::NeodelightSelf => "_self",
    }
}

fn open_external_link(link: ExternalLinkAction) {
    let url = external_link_url(link);
    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    {
        if let Err(error) = open_url_with_system_handler(url) {
            eprintln!("failed to open SWF link {url}: {error}");
        }
    }
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    )))]
    {
        let _ = url;
    }
}

#[cfg(target_os = "windows")]
fn open_url_with_system_handler(url: &str) -> std::io::Result<()> {
    let status = std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "cmd /C start exited with {status}"
        )))
    }
}

#[cfg(target_os = "macos")]
fn open_url_with_system_handler(url: &str) -> std::io::Result<()> {
    let status = std::process::Command::new("open").arg(url).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!("open exited with {status}")))
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
fn open_url_with_system_handler(url: &str) -> std::io::Result<()> {
    let status = std::process::Command::new("xdg-open").arg(url).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "xdg-open exited with {status}"
        )))
    }
}

fn point_in_polygon(point: SwfPoint, polygon: &[SwfPoint]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut previous = polygon[polygon.len() - 1];
    for &current in polygon {
        if point_on_segment(point, previous, current) {
            return true;
        }
        let crosses_y = (current.y > point.y) != (previous.y > point.y);
        if crosses_y {
            let cross_x = (previous.x - current.x) * (point.y - current.y)
                / (previous.y - current.y)
                + current.x;
            if point.x < cross_x {
                inside = !inside;
            }
        }
        previous = current;
    }
    inside
}

fn point_on_segment(point: SwfPoint, a: SwfPoint, b: SwfPoint) -> bool {
    const HIT_EPSILON: f32 = 0.01;
    let length_squared = (b.y - a.y).mul_add(b.y - a.y, (b.x - a.x).powi(2));
    if length_squared <= HIT_EPSILON * HIT_EPSILON {
        return points_are_close(point, a);
    }
    let cross = (point.x - a.x).mul_add(-(b.y - a.y), (point.y - a.y) * (b.x - a.x));
    if cross.abs() > HIT_EPSILON {
        return false;
    }
    let dot = (point.y - a.y).mul_add(b.y - a.y, (point.x - a.x) * (b.x - a.x));
    if dot < -HIT_EPSILON {
        return false;
    }
    dot <= length_squared + HIT_EPSILON
}

fn points_are_close(a: SwfPoint, b: SwfPoint) -> bool {
    const HIT_EPSILON: f32 = 0.01;
    (a.x - b.x).abs() <= HIT_EPSILON && (a.y - b.y).abs() <= HIT_EPSILON
}

fn pop_duplicate_last_point(points: &mut Vec<SwfPoint>) {
    if points.len() > 1 && points_are_close(points[0], points[points.len() - 1]) {
        points.pop();
    }
}

fn header_link_state(
    link: HeaderLink,
    mouse_x: f32,
    mouse_y: f32,
    mouse_down: bool,
) -> HeaderLinkState {
    if header_link_contains(link, mouse_x, mouse_y) {
        if mouse_down {
            HeaderLinkState::Down
        } else {
            HeaderLinkState::Over
        }
    } else {
        HeaderLinkState::Up
    }
}

#[cfg(test)]
fn header_link_color(link: HeaderLink, mouse_x: f32, mouse_y: f32, mouse_down: bool) -> Color {
    match header_link_state(link, mouse_x, mouse_y, mouse_down) {
        HeaderLinkState::Up => HEADER_LINK_UP,
        HeaderLinkState::Over => HEADER_LINK_OVER,
        HeaderLinkState::Down => HEADER_LINK_DOWN,
    }
}

fn draw_gravity_mode_preview(mode: Polarisation) {
    for row in gravity_preview_mode_frame(mode).rows {
        draw_gravity_preview_arrow(row.left_arrow);
        draw_gravity_preview_arrow(row.right_arrow);
        draw_gravity_preview_ball(row.left_ball);
        draw_gravity_preview_ball(row.right_ball);
    }
}

fn gravity_preview_mode_frame(mode: Polarisation) -> &'static GravityPreviewModeFrame {
    match mode {
        Polarisation::Neutral => &GRAVITY_PREVIEW_NEUTRAL_FRAME,
        Polarisation::OppositeRepels => &GRAVITY_PREVIEW_OPPOSITE_REPELS_FRAME,
        Polarisation::SameRepels => &GRAVITY_PREVIEW_SAME_REPELS_FRAME,
        Polarisation::AllRepel => &GRAVITY_PREVIEW_ALL_REPEL_FRAME,
    }
}

fn draw_gravity_preview_ball(ball: GravityPreviewBallPlacement) {
    let visual = gravity_preview_ball_visual(ball);
    draw_radial_fade(
        visual.x,
        visual.y,
        RadialFadeVisual {
            radius: visual.radius,
            shape: BALL_GLOW_SHAPE,
            center: visual.color,
            transparent_stop_ratio: visual.transparent_stop_ratio,
        },
    );
}

fn gravity_preview_ball_visual(ball: GravityPreviewBallPlacement) -> GravityPreviewBallVisual {
    GravityPreviewBallVisual {
        x: gravity_preview_stage_x(ball.transform, 0.0),
        y: gravity_preview_stage_y(ball.transform, 0.0),
        radius: BALL_GLOW_RADIUS as f32 * ball.transform.sx.abs() * MENU_BUTTON_SCALE,
        color: ball.color,
        transparent_stop_ratio: BALL_GLOW_EDGE_STOP_RATIO,
    }
}

fn draw_gravity_preview_arrow(placement: GravityPreviewPlacement) {
    let visual = gravity_preview_arrow_visual(placement);
    draw_line(
        visual.start.x,
        visual.start.y,
        visual.end.x,
        visual.end.y,
        visual.line_width,
        visual.color,
    );
    draw_line(
        visual.end.x,
        visual.end.y,
        visual.head_a.x,
        visual.head_a.y,
        visual.line_width,
        visual.color,
    );
    draw_line(
        visual.end.x,
        visual.end.y,
        visual.head_b.x,
        visual.head_b.y,
        visual.line_width,
        visual.color,
    );
}

fn gravity_preview_arrow_visual(placement: GravityPreviewPlacement) -> GravityPreviewArrowVisual {
    GravityPreviewArrowVisual {
        start: gravity_preview_stage_point(placement, gravity_preview_arrow_shaft_start()),
        end: gravity_preview_stage_point(placement, gravity_preview_arrow_tip()),
        head_a: gravity_preview_stage_point(placement, gravity_preview_arrow_head_top()),
        head_b: gravity_preview_stage_point(placement, gravity_preview_arrow_head_bottom()),
        line_width: gravity_preview_arrow_shape41::LINE_WIDTH
            * ((placement.sx.abs() + placement.sy.abs()) * 0.5)
            * MENU_BUTTON_SCALE,
        color: GRAVITY_PREVIEW_ARROW,
    }
}

fn gravity_preview_arrow_shaft_start() -> SwfPoint {
    gravity_preview_arrow_shape41::SHAPE.shaft.start
}

fn gravity_preview_arrow_tip() -> SwfPoint {
    gravity_preview_arrow_shape41::SHAPE.shaft.points[0]
}

fn gravity_preview_arrow_head_top() -> SwfPoint {
    gravity_preview_arrow_shape41::SHAPE.head.start
}

fn gravity_preview_arrow_head_bottom() -> SwfPoint {
    gravity_preview_arrow_shape41::SHAPE.head.points[1]
}

fn gravity_preview_stage_point(placement: GravityPreviewPlacement, local: SwfPoint) -> Vec2 {
    vec2(
        gravity_preview_stage_x(placement, local.x),
        gravity_preview_stage_y(placement, local.y),
    )
}

fn gravity_preview_stage_x(placement: GravityPreviewPlacement, local_x: f32) -> f32 {
    local_x
        .mul_add(placement.sx, placement.tx)
        .mul_add(MENU_BUTTON_SCALE, GRAVITY_PREVIEW_ROOT_X)
}

fn gravity_preview_stage_y(placement: GravityPreviewPlacement, local_y: f32) -> f32 {
    local_y
        .mul_add(placement.sy, placement.ty)
        .mul_add(MENU_BUTTON_SCALE, GRAVITY_PREVIEW_ROOT_Y)
}

fn draw_playfield(goal_flash: GoalFlash) {
    draw_stage_frame_with_flash(PLAYFIELD_STAGE_FRAME, goal_flash);
}

fn draw_attached_balls_in_depth_range(
    world: &World,
    visuals: &GameplayVisualSnapshot,
    min_depth: u32,
    max_depth: u32,
) {
    if min_depth >= max_depth {
        return;
    }

    for depth in min_depth..max_depth {
        for ball in &world.balls {
            if ball.id == depth {
                let (x, y) = visuals.ball_position(ball.id).unwrap_or((ball.x, ball.y));
                draw_ball_at(*ball, x, y);
            }
        }
        for ball in &world.dying_balls {
            if ball.id == depth {
                let (x, y) = visuals
                    .dying_ball_position(ball.id)
                    .unwrap_or((ball.x, ball.y));
                draw_dying_ball_at(*ball, x, y);
            }
        }
    }
}

fn attached_ball_depth_end(world: &World) -> u32 {
    world
        .balls
        .iter()
        .map(|ball| ball.id)
        .chain(world.dying_balls.iter().map(|ball| ball.id))
        .max()
        .map_or(PLAYFIELD_FIRST_DYNAMIC_BALL_DEPTH, |depth| {
            depth.saturating_add(1)
        })
}

fn draw_score_meter(score: i32, side: Side, tick: u64) {
    let frame = score_meter_frame(score);
    if frame < SCORE_METER_MAX_FRAME {
        draw_score_meter_base_markers(side);
        for_each_score_marker(score, side, |visual| {
            draw_score_marker(visual.x, visual.y, side, visual.color, visual.scale);
        });
    } else {
        for_each_score_marker(score, side, |visual| {
            draw_score_marker_glyph(visual.x, visual.y, side, visual.color, visual.scale);
        });
        for_each_final_score_overlay(side, tick, draw_final_score_overlay);
    }
}

fn draw_score_meter_base_markers(side: Side) {
    for_each_score_meter_base_marker(side, |visual| {
        draw_marker_glyph(marker_glyph_visual(
            visual.x,
            visual.y,
            side,
            visual.color,
            visual.scale,
        ));
    });
}

fn draw_score_marker(x: f32, y: f32, side: Side, color: Color, scale: f32) {
    let visual = marker_glyph_visual(x, y, side, color, scale);
    draw_filled_polygon_fan(&visual.fill_points, visual.fill_center, visual.fill_color);
}

fn draw_score_marker_glyph(x: f32, y: f32, side: Side, color: Color, scale: f32) {
    draw_marker_glyph(marker_glyph_visual(x, y, side, color, scale));
}

fn draw_final_score_overlay(visual: FinalScoreOverlayVisual) {
    let points = marker_outline_points(visual.x, visual.y, 1.0);
    draw_closed_polyline(
        &points,
        side_marker_shapes::OUTLINE_LINE_WIDTH,
        visual.color,
    );
}

fn for_each_score_meter_base_marker(side: Side, mut emit: impl FnMut(ScoreMarkerVisual)) {
    for group in 0..SCORE_METER_LOCAL_YS.len() {
        for local_y in score_meter_group_local_ys(group) {
            emit(ScoreMarkerVisual {
                x: score_meter_x(side),
                y: SCORE_METER_BASE_Y + local_y,
                scale: 1.0,
                color: SIDE_MARKER_EMPTY_FILL,
            });
        }
    }
}

fn for_each_score_marker(score: i32, side: Side, mut emit: impl FnMut(ScoreMarkerVisual)) {
    let frame = score_meter_frame(score);
    if frame <= 1 {
        return;
    }
    if frame >= SCORE_METER_MAX_FRAME {
        for_final_score_marker(side, emit);
        return;
    }

    for (group, start_frame) in SCORE_METER_GROUP_START_FRAMES.iter().copied().enumerate() {
        if frame < start_frame {
            continue;
        }
        let phase = (frame - start_frame).min((SCORE_METER_RAMP_STEPS - 1) as u8) as usize;
        let color = score_meter_phase_color(side, phase);
        let scale = score_meter_phase_scale(side, phase);
        for local_y in score_meter_group_local_ys(group) {
            emit(ScoreMarkerVisual {
                x: score_meter_x(side),
                y: SCORE_METER_BASE_Y + local_y,
                scale,
                color,
            });
        }
    }
}

fn for_final_score_marker(side: Side, mut emit: impl FnMut(ScoreMarkerVisual)) {
    let x = score_meter_x(side);
    for placement in final_score_marker_placements(side) {
        emit(ScoreMarkerVisual {
            x: x + placement.local_x,
            y: SCORE_METER_BASE_Y + placement.local_y,
            scale: placement.scale,
            color: transformed_score_marker_color(
                score_marker_color(side),
                placement.mult,
                placement.add_rgb,
            ),
        });
    }
}

fn for_each_final_score_overlay(
    side: Side,
    tick: u64,
    mut emit: impl FnMut(FinalScoreOverlayVisual),
) {
    let color = final_score_overlay_color(side, final_score_overlay_frame(tick));
    for local in final_score_overlay_positions(side) {
        emit(FinalScoreOverlayVisual {
            x: score_meter_x(side) + local.x,
            y: SCORE_METER_BASE_Y + local.y,
            color,
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ScoreMeterFinalMarkerPlacement {
    local_x: f32,
    local_y: f32,
    scale: f32,
    mult: u8,
    add_rgb: [u8; 3],
}

impl ScoreMeterFinalMarkerPlacement {
    const fn new(local_x: f32, local_y: f32, scale: f32, mult: u8, add_rgb: [u8; 3]) -> Self {
        Self {
            local_x,
            local_y,
            scale,
            mult,
            add_rgb,
        }
    }
}

const SCORE_METER_BASE_Y: f32 = score_meter_constants::BASE_Y;
const SCORE_METER_RAMP_STEPS: usize = score_meter_constants::RAMP_STEPS;
const SCORE_METER_FINAL_MARKERS: usize = score_meter_constants::FINAL_MARKERS;
const SCORE_METER_FINAL_OVERLAY_FRAMES: usize = score_meter_constants::FINAL_OVERLAY_FRAMES;
const SCORE_METER_GROUP_START_FRAMES: [u8; 7] = score_meter_constants::GROUP_START_FRAMES;
const SCORE_METER_LOCAL_YS: [[f32; 2]; 7] = score_meter_constants::LOCAL_YS;
const RED_SCORE_PHASE_SCALES: [f32; SCORE_METER_RAMP_STEPS] =
    score_meter_constants::RED_PHASE_SCALES;
const BLUE_SCORE_PHASE_SCALES: [f32; SCORE_METER_RAMP_STEPS] =
    score_meter_constants::BLUE_PHASE_SCALES;
const RED_SCORE_PHASE_MULTS: [u8; SCORE_METER_RAMP_STEPS] = score_meter_constants::RED_PHASE_MULTS;
const BLUE_SCORE_PHASE_MULTS: [u8; SCORE_METER_RAMP_STEPS] =
    score_meter_constants::BLUE_PHASE_MULTS;
const RED_SCORE_PHASE_ADDS: [[u8; 3]; SCORE_METER_RAMP_STEPS] =
    score_meter_constants::RED_PHASE_ADDS;
const BLUE_SCORE_PHASE_ADDS: [[u8; 3]; SCORE_METER_RAMP_STEPS] =
    score_meter_constants::BLUE_PHASE_ADDS;
const RED_FINAL_SCORE_MARKERS: [ScoreMeterFinalMarkerPlacement; SCORE_METER_FINAL_MARKERS] =
    score_meter_constants::RED_FINAL_MARKERS;
const BLUE_FINAL_SCORE_MARKERS: [ScoreMeterFinalMarkerPlacement; SCORE_METER_FINAL_MARKERS] =
    score_meter_constants::BLUE_FINAL_MARKERS;
const RED_FINAL_SCORE_OVERLAY_LOCAL_POSITIONS: [SwfPoint; SCORE_METER_FINAL_OVERLAY_FRAMES] =
    score_meter_constants::RED_FINAL_OVERLAY_LOCAL_POSITIONS;
const BLUE_FINAL_SCORE_OVERLAY_LOCAL_POSITIONS: [SwfPoint; SCORE_METER_FINAL_OVERLAY_FRAMES] =
    score_meter_constants::BLUE_FINAL_OVERLAY_LOCAL_POSITIONS;
const RED_FINAL_SCORE_OVERLAY_RGB: [[u8; 3]; SCORE_METER_FINAL_OVERLAY_FRAMES] =
    score_meter_constants::RED_FINAL_OVERLAY_RGB;
const BLUE_FINAL_SCORE_OVERLAY_RGB: [[u8; 3]; SCORE_METER_FINAL_OVERLAY_FRAMES] =
    score_meter_constants::BLUE_FINAL_OVERLAY_RGB;

#[derive(Debug, Clone, Copy, PartialEq)]
struct FinalScoreOverlayVisual {
    x: f32,
    y: f32,
    color: Color,
}

fn score_meter_group_local_ys(group: usize) -> &'static [f32] {
    if group == 0 {
        &SCORE_METER_LOCAL_YS[group][..1]
    } else {
        &SCORE_METER_LOCAL_YS[group]
    }
}

const fn score_meter_x(side: Side) -> f32 {
    match side {
        Side::Blue => score_meter_constants::BLUE_X,
        Side::Red => score_meter_constants::RED_X,
    }
}

fn score_meter_phase_scale(side: Side, phase: usize) -> f32 {
    match side {
        Side::Blue => BLUE_SCORE_PHASE_SCALES[phase],
        Side::Red => RED_SCORE_PHASE_SCALES[phase],
    }
}

fn score_meter_phase_color(side: Side, phase: usize) -> Color {
    match side {
        Side::Blue => transformed_score_marker_color(
            score_marker_color(side),
            BLUE_SCORE_PHASE_MULTS[phase],
            BLUE_SCORE_PHASE_ADDS[phase],
        ),
        Side::Red => transformed_score_marker_color(
            score_marker_color(side),
            RED_SCORE_PHASE_MULTS[phase],
            RED_SCORE_PHASE_ADDS[phase],
        ),
    }
}

fn final_score_marker_placements(
    side: Side,
) -> &'static [ScoreMeterFinalMarkerPlacement; SCORE_METER_FINAL_MARKERS] {
    match side {
        Side::Blue => &BLUE_FINAL_SCORE_MARKERS,
        Side::Red => &RED_FINAL_SCORE_MARKERS,
    }
}

fn final_score_overlay_positions(
    side: Side,
) -> &'static [SwfPoint; SCORE_METER_FINAL_OVERLAY_FRAMES] {
    match side {
        Side::Blue => &BLUE_FINAL_SCORE_OVERLAY_LOCAL_POSITIONS,
        Side::Red => &RED_FINAL_SCORE_OVERLAY_LOCAL_POSITIONS,
    }
}

fn final_score_overlay_color(side: Side, frame: usize) -> Color {
    match side {
        Side::Blue => {
            swf_rgb_array(BLUE_FINAL_SCORE_OVERLAY_RGB[frame % SCORE_METER_FINAL_OVERLAY_FRAMES])
        },
        Side::Red => {
            swf_rgb_array(RED_FINAL_SCORE_OVERLAY_RGB[frame % SCORE_METER_FINAL_OVERLAY_FRAMES])
        },
    }
}

fn final_score_overlay_frame(tick: u64) -> usize {
    tick as usize % SCORE_METER_FINAL_OVERLAY_FRAMES
}

const fn transformed_score_marker_color(base: Color, mult: u8, add: [u8; 3]) -> Color {
    Color::new(
        base.r * mult as f32 / 256.0 + add[0] as f32 / 255.0,
        base.g * mult as f32 / 256.0 + add[1] as f32 / 255.0,
        base.b * mult as f32 / 256.0 + add[2] as f32 / 255.0,
        1.0,
    )
}

fn draw_match_pips(count: u8, side: Side) {
    for_each_match_pip(count, side, draw_match_pip);
}

const BLUE_MATCH_PIP_XS: [f32; 4] = [110.8, 140.75, 170.7, 200.65];
const RED_MATCH_PIP_XS: [f32; 4] = [336.35, 366.3, 396.25, 426.2];
const MATCH_PIP_Y: f32 = 18.7;

fn for_each_match_pip(count: u8, side: Side, mut emit: impl FnMut(MatchPipVisual)) {
    let count = usize::from(count.min(4));
    if count == 0 {
        return;
    }

    match side {
        Side::Blue => {
            for x in BLUE_MATCH_PIP_XS[(BLUE_MATCH_PIP_XS.len() - count)..]
                .iter()
                .copied()
            {
                emit(MatchPipVisual {
                    x,
                    y: MATCH_PIP_Y,
                    side,
                });
            }
        },
        Side::Red => {
            for x in RED_MATCH_PIP_XS[..count].iter().copied() {
                emit(MatchPipVisual {
                    x,
                    y: MATCH_PIP_Y,
                    side,
                });
            }
        },
    }
}

fn draw_match_pip(visual: MatchPipVisual) {
    draw_match_pip_scaled(visual.x, visual.y, visual.side, 1.0, 1.0);
}

const fn match_pip_palette(side: Side) -> MatchPipPalette {
    match side {
        Side::Blue => MatchPipPalette {
            outer: MATCH_PIP_OUTER,
            core: MATCH_PIP_BLUE_CORE,
            highlight: MATCH_PIP_BLUE_HIGHLIGHT,
            mid: MATCH_PIP_BLUE_MID,
            shine: MATCH_PIP_BLUE_SHINE,
            accent: MATCH_PIP_BLUE_ACCENT,
        },
        Side::Red => MatchPipPalette {
            outer: MATCH_PIP_OUTER,
            core: MATCH_PIP_RED_CORE,
            highlight: MATCH_PIP_RED_HIGHLIGHT,
            mid: MATCH_PIP_RED_MID,
            shine: MATCH_PIP_RED_SHINE,
            accent: MATCH_PIP_RED_ACCENT,
        },
    }
}

fn draw_match_pip_scaled(x: f32, y: f32, side: Side, scale: f32, alpha: f32) {
    if alpha <= 0.0 {
        return;
    }
    let radius = MATCH_PIP_RADIUS * scale.abs();
    if x + radius < 0.0
        || x - radius > STAGE_W as f32
        || y + radius < 0.0
        || y - radius > STAGE_H as f32
    {
        return;
    }

    let palette = match_pip_palette(side);
    match side {
        Side::Blue => {
            for contour in BLUE_MATCH_PIP_CONTOURS {
                draw_match_pip_contour(contour, x, y, scale, palette, alpha);
            }
            draw_match_pip_contour(BLUE_MATCH_PIP_ACCENT, x, y, scale, palette, alpha);
        },
        Side::Red => {
            for contour in RED_MATCH_PIP_CONTOURS {
                draw_match_pip_contour(contour, x, y, scale, palette, alpha);
            }
            draw_match_pip_contour(RED_MATCH_PIP_TOP_SHINE, x, y, scale, palette, alpha);
        },
    }
}

fn draw_match_pip_contour(
    contour: MatchPipContour,
    x: f32,
    y: f32,
    scale: f32,
    palette: MatchPipPalette,
    alpha: f32,
) {
    let points = match_pip_contour_points(contour, x, y, scale);
    draw_filled_polygon_triangulated(
        &points,
        with_alpha(match_pip_slot_color(contour.slot, palette), alpha),
    );
}

fn match_pip_contour_points(contour: MatchPipContour, x: f32, y: f32, scale: f32) -> Vec<SwfPoint> {
    let mut points =
        Vec::with_capacity(1 + contour.segments.len() * usize::from(MATCH_PIP_FLATTEN_STEPS));
    points.push(match_pip_stage_point(x, y, scale, contour.start));
    let mut current = contour.start;
    for segment in contour.segments {
        match *segment {
            SwfPathSegment::Line(to) => {
                points.push(match_pip_stage_point(x, y, scale, to));
                current = to;
            },
            SwfPathSegment::Quad { control, to } => {
                for step in 1..=MATCH_PIP_FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(MATCH_PIP_FLATTEN_STEPS);
                    points.push(match_pip_stage_point(
                        x,
                        y,
                        scale,
                        quadratic_point(current, control, to, t),
                    ));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn match_pip_stage_point(x: f32, y: f32, scale: f32, local: SwfPoint) -> SwfPoint {
    SwfPoint::new(local.x.mul_add(scale, x), local.y.mul_add(scale, y))
}

const fn match_pip_slot_color(slot: MatchPipSlot, palette: MatchPipPalette) -> Color {
    match slot {
        MatchPipSlot::Outer => palette.outer,
        MatchPipSlot::Core => palette.core,
        MatchPipSlot::Highlight => palette.highlight,
        MatchPipSlot::Mid => palette.mid,
        MatchPipSlot::Shine => palette.shine,
        MatchPipSlot::Accent => palette.accent,
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, color.a * alpha)
}

#[derive(Debug, Clone, Copy)]
struct AlphaStop {
    ratio: f32,
    alpha: f32,
}

#[derive(Debug, Clone, Copy)]
struct PaddleGlowVisual {
    scale_x: f32,
    scale_y: f32,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    body_y_min: f32,
    body_y_max: f32,
    color: Color,
}

#[derive(Debug, Clone, Copy)]
struct ScaledContourBand {
    origin: Vec2,
    visual: PaddleGlowVisual,
    center: SwfPoint,
    inner_ratio: f32,
    outer_ratio: f32,
    color: Color,
}

#[derive(Debug, Clone, PartialEq)]
struct PaddleReadyFlashVisual {
    points: Vec<SwfPoint>,
    center: SwfPoint,
    color: Color,
}

fn paddle_glow_visual(sx: f32, sy: f32, color: Color) -> PaddleGlowVisual {
    let shape = paddle_glow_shape90::SHAPE;
    PaddleGlowVisual {
        scale_x: sx,
        scale_y: sy,
        x_min: shape.bounds.x_min * sx,
        x_max: shape.bounds.x_max * sx,
        y_min: shape.bounds.y_min * sy,
        y_max: shape.bounds.y_max * sy,
        body_y_min: shape.body_bounds.y_min * sy,
        body_y_max: shape.body_bounds.y_max * sy,
        color,
    }
}

fn paddle_ready_flash_visual(scale: f32, alpha: f32) -> PaddleReadyFlashVisual {
    let bounds = paddle_ready_flash_shape127::SHAPE.bounds;
    PaddleReadyFlashVisual {
        points: paddle_ready_flash_points(scale),
        center: SwfPoint::new(
            (bounds.x_min + bounds.x_max) * 0.5 * scale,
            (bounds.y_min + bounds.y_max) * 0.5 * scale,
        ),
        color: with_alpha(
            swf_rgba_array(paddle_ready_flash_shape127::FILL_RGBA),
            alpha,
        ),
    }
}

fn paddle_ready_flash_points(scale: f32) -> Vec<SwfPoint> {
    let shape = paddle_ready_flash_shape127::SHAPE;
    let mut points = Vec::with_capacity(
        1 + shape.segments.len() * usize::from(paddle_ready_flash_shape127::FLATTEN_STEPS),
    );
    let mut current = shape.start;
    points.push(paddle_ready_flash_stage_point(scale, current));
    for segment in shape.segments {
        match *segment {
            SwfPathSegment::Line(to) => {
                points.push(paddle_ready_flash_stage_point(scale, to));
                current = to;
            },
            SwfPathSegment::Quad { control, to } => {
                for step in 1..=paddle_ready_flash_shape127::FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(paddle_ready_flash_shape127::FLATTEN_STEPS);
                    points.push(paddle_ready_flash_stage_point(
                        scale,
                        quadratic_point(current, control, to, t),
                    ));
                }
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn paddle_ready_flash_stage_point(scale: f32, local: SwfPoint) -> SwfPoint {
    SwfPoint::new(local.x * scale, local.y * scale)
}

fn draw_player_at(player: &gravityarcade::sim::Player, y: f64) {
    let x = match player.side {
        Side::Blue => gravityarcade::sim::RIGHT_PADDLE_X,
        Side::Red => gravityarcade::sim::LEFT_PADDLE_X,
    };
    let (sx, sy) = world_to_stage(x, y);
    let sx = sx as f32;
    let sy = sy as f32;
    let visual = paddle_charge_visual(player.side, player.energy_frame);
    let glow_x = visual.sx as f32;
    let glow_y = visual.sy as f32;
    let glow_center_y = sy + 0.25;
    let glow = paddle_glow_color(player.stun_ticks, visual.color);

    draw_paddle_charge_glow(sx, glow_center_y, paddle_glow_visual(glow_x, glow_y, glow));
    draw_paddle_body(paddle_body_visual(sx, sy));
    if let Some(flash) = visual.ready_flash.filter(|flash| flash.alpha > 0.0) {
        let ready = paddle_ready_flash_visual(flash.scale as f32, flash.alpha as f32);
        let points: Vec<SwfPoint> = ready
            .points
            .iter()
            .map(|point| SwfPoint::new(sx + point.x, glow_center_y + point.y))
            .collect();
        draw_filled_polygon_fan(
            &points,
            SwfPoint::new(sx + ready.center.x, glow_center_y + ready.center.y),
            ready.color,
        );
    }
}

fn draw_paddle_charge_glow(x: f32, y: f32, visual: PaddleGlowVisual) {
    if x + visual.x_min > STAGE_W as f32
        || x + visual.x_max < 0.0
        || y + visual.y_min > STAGE_H as f32
        || y + visual.y_max < 0.0
    {
        return;
    }

    draw_vertical_alpha_gradient(
        x + visual.x_min,
        y + visual.body_y_min,
        visual.x_max - visual.x_min,
        visual.body_y_max - visual.body_y_min,
        visual.color,
        &paddle_glow_shape90::LINEAR_ALPHA_STOPS,
    );
    draw_paddle_glow_cap_radial_fade(
        x,
        y,
        visual,
        SwfPoint::new(PADDLE_GLOW_CENTER_X, PADDLE_GLOW_TOP_CENTER_Y),
        &paddle_glow_top_cap_points(),
    );
    draw_paddle_glow_cap_radial_fade(
        x,
        y,
        visual,
        SwfPoint::new(PADDLE_GLOW_CENTER_X, PADDLE_GLOW_BOTTOM_CENTER_Y),
        &paddle_glow_bottom_cap_points(),
    );
}

fn draw_vertical_alpha_gradient(x: f32, y: f32, w: f32, h: f32, color: Color, stops: &[AlphaStop]) {
    for band in 0..PADDLE_GLOW_LINEAR_BANDS {
        let t0 = band as f32 / PADDLE_GLOW_LINEAR_BANDS as f32;
        let t1 = (band + 1) as f32 / PADDLE_GLOW_LINEAR_BANDS as f32;
        let alpha = alpha_from_stops(stops, (t0 + t1) * 0.5);
        if alpha <= 0.0 {
            continue;
        }
        draw_rectangle(
            x,
            h.mul_add(t0, y),
            w,
            h * (t1 - t0),
            Color::new(color.r, color.g, color.b, color.a * alpha),
        );
    }
}

fn alpha_from_stops(stops: &[AlphaStop], ratio: f32) -> f32 {
    let ratio = ratio.clamp(0.0, 1.0);
    for pair in stops.windows(2) {
        let left = pair[0];
        let right = pair[1];
        if ratio <= right.ratio {
            let span = right.ratio - left.ratio;
            if span <= f32::EPSILON {
                return right.alpha;
            }
            let t = (ratio - left.ratio) / span;
            return (right.alpha - left.alpha).mul_add(t, left.alpha);
        }
    }
    stops.last().map_or(0.0, |stop| stop.alpha)
}

fn paddle_glow_top_cap_points() -> Vec<SwfPoint> {
    paddle_glow_contour_points(paddle_glow_shape90::SHAPE.top_cap)
}

fn paddle_glow_bottom_cap_points() -> Vec<SwfPoint> {
    paddle_glow_contour_points(paddle_glow_shape90::SHAPE.bottom_cap)
}

fn paddle_glow_contour_points(contour: PaddleGlowContour) -> Vec<SwfPoint> {
    let mut points = Vec::with_capacity(
        1 + contour.segments.len() * usize::from(paddle_glow_shape90::FLATTEN_STEPS),
    );
    let mut current = contour.start;
    points.push(current);
    for segment in contour.segments {
        match *segment {
            SwfPathSegment::Line(to) => {
                points.push(to);
                current = to;
            },
            SwfPathSegment::Quad { control, to } => {
                push_untransformed_quadratic(
                    &mut points,
                    current,
                    control,
                    to,
                    paddle_glow_shape90::FLATTEN_STEPS,
                );
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn push_untransformed_quadratic(
    points: &mut Vec<SwfPoint>,
    start: SwfPoint,
    control: SwfPoint,
    end: SwfPoint,
    steps: u8,
) {
    for step in 1..=steps {
        let t = f32::from(step) / f32::from(steps);
        points.push(quadratic_point(start, control, end, t));
    }
}

fn draw_paddle_glow_cap_radial_fade(
    x: f32,
    y: f32,
    visual: PaddleGlowVisual,
    center: SwfPoint,
    cap_points: &[SwfPoint],
) {
    for band in 0..PADDLE_GLOW_RADIAL_BANDS {
        let inner_ratio = band as f32 / PADDLE_GLOW_RADIAL_BANDS as f32;
        let outer_ratio = (band + 1) as f32 / PADDLE_GLOW_RADIAL_BANDS as f32;
        let alpha = paddle_glow_radial_alpha_at((inner_ratio + outer_ratio) * 0.5);
        if alpha <= 0.0 {
            continue;
        }

        draw_scaled_contour_band(
            cap_points,
            ScaledContourBand {
                origin: vec2(x, y),
                visual,
                center,
                inner_ratio,
                outer_ratio,
                color: with_alpha(visual.color, alpha),
            },
        );
    }
}

fn draw_scaled_contour_band(points: &[SwfPoint], band: ScaledContourBand) {
    if points.len() < 3 {
        return;
    }

    for edge in points.windows(2) {
        draw_scaled_contour_band_segment(edge[0], edge[1], band);
    }
    if let (Some(first), Some(last)) = (points.first(), points.last()) {
        draw_scaled_contour_band_segment(*last, *first, band);
    }
}

fn draw_scaled_contour_band_segment(start: SwfPoint, end: SwfPoint, band: ScaledContourBand) {
    let inner0 = scaled_contour_stage_point(band, start, band.inner_ratio);
    let outer0 = scaled_contour_stage_point(band, start, band.outer_ratio);
    let inner1 = scaled_contour_stage_point(band, end, band.inner_ratio);
    let outer1 = scaled_contour_stage_point(band, end, band.outer_ratio);

    draw_triangle(inner0, outer0, outer1, band.color);
    draw_triangle(inner0, outer1, inner1, band.color);
}

fn scaled_contour_stage_point(band: ScaledContourBand, local: SwfPoint, ratio: f32) -> Vec2 {
    let scaled = SwfPoint::new(
        (local.x - band.center.x).mul_add(ratio, band.center.x),
        (local.y - band.center.y).mul_add(ratio, band.center.y),
    );
    vec2(
        scaled.x.mul_add(band.visual.scale_x, band.origin.x),
        scaled.y.mul_add(band.visual.scale_y, band.origin.y),
    )
}

fn paddle_glow_radial_alpha_at(ratio: f32) -> f32 {
    if ratio <= paddle_glow_shape90::RADIAL_PEAK_STOP_RATIO {
        1.0
    } else {
        (1.0 - ratio) / (1.0 - paddle_glow_shape90::RADIAL_PEAK_STOP_RATIO)
    }
    .clamp(0.0, 1.0)
}

fn paddle_glow_color(stun_ticks: u32, rgb: (u8, u8, u8)) -> Color {
    if let Some(stun_rgb) = paddle_stun_color(stun_ticks) {
        return color_from_rgb(stun_rgb);
    }

    color_from_rgb(rgb)
}

fn color_from_rgb(rgb: (u8, u8, u8)) -> Color {
    Color::new(
        f32::from(rgb.0) / 255.0,
        f32::from(rgb.1) / 255.0,
        f32::from(rgb.2) / 255.0,
        1.0,
    )
}

const fn ball_core_color(side: Side) -> Color {
    match side {
        Side::Blue => BALL_BLUE_CORE,
        Side::Red => BALL_RED_CORE,
    }
}

#[derive(Debug, Clone, Copy)]
struct RadialFadeVisual {
    radius: f32,
    shape: RadialGradientShape,
    center: Color,
    transparent_stop_ratio: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RadialBandVisual {
    inner_radius: f32,
    outer_radius: f32,
    color: Color,
}

#[derive(Debug, Clone, Copy)]
struct RadialShapeBand {
    origin: Vec2,
    scale: f32,
    inner_ratio: f32,
    outer_ratio: f32,
    color: Color,
}

#[derive(Debug, Clone, Copy)]
struct RadialRingVisual {
    radius: f32,
    shape: RadialGradientShape,
    inner_stop_ratio: f32,
    peak_stop_ratio: f32,
    outer_stop_ratio: f32,
    peak: Color,
}

fn active_ball_glow_visual(side: Side, scale: f32) -> RadialFadeVisual {
    RadialFadeVisual {
        radius: BALL_GLOW_RADIUS as f32 * scale,
        shape: BALL_GLOW_SHAPE,
        center: ball_core_color(side),
        transparent_stop_ratio: BALL_GLOW_EDGE_STOP_RATIO,
    }
}

fn burning_ball_fire_visual(scale: f32) -> RadialFadeVisual {
    RadialFadeVisual {
        radius: BALL_FIRE_RADIUS as f32 * scale,
        shape: BALL_FIRE_SHAPE,
        center: BALL_FIRE_CORE,
        transparent_stop_ratio: BALL_FIRE_EDGE_STOP_RATIO,
    }
}

fn dying_ball_ring_visual(scale: f32, alpha: f32) -> RadialRingVisual {
    RadialRingVisual {
        radius: BALL_CORE_RADIUS as f32 * scale,
        shape: BALL_DIE_RING_SHAPE,
        inner_stop_ratio: BALL_DIE_RING_INNER_STOP_RATIO,
        peak_stop_ratio: BALL_DIE_RING_PEAK_STOP_RATIO,
        outer_stop_ratio: BALL_DIE_RING_OUTER_STOP_RATIO,
        peak: with_alpha(BALL_DIE_RING_PEAK, alpha),
    }
}

fn draw_ball_at(ball: gravityarcade::sim::Ball, x: f64, y: f64) {
    let (sx, sy) = world_to_stage(x, y);
    let scale = ball.visual_scale() as f32;
    let sx = sx as f32;
    let sy = sy as f32;

    draw_radial_fade(sx, sy, active_ball_glow_visual(ball.color, scale));
    if ball.is_burning() {
        draw_radial_fade(sx, sy, burning_ball_fire_visual(scale));
    }
}

fn draw_radial_fade(x: f32, y: f32, visual: RadialFadeVisual) {
    if visual.radius <= f32::EPSILON {
        return;
    }

    let points = radial_shape_points(visual.shape);
    let scale = visual.radius / visual.shape.base_radius;
    for band in 0..RADIAL_FADE_STEPS {
        let Some(band) = radial_fade_band_visual(visual, band) else {
            continue;
        };
        draw_radial_shape_band(
            &points,
            RadialShapeBand {
                origin: vec2(x, y),
                scale,
                inner_ratio: band.inner_radius / visual.radius,
                outer_ratio: band.outer_radius / visual.radius,
                color: band.color,
            },
        );
    }
}

fn radial_fade_band_visual(visual: RadialFadeVisual, band: u32) -> Option<RadialBandVisual> {
    if band >= RADIAL_FADE_STEPS {
        return None;
    }

    let transparent_stop = visual.transparent_stop_ratio.clamp(0.0, 1.0);
    if transparent_stop <= f32::EPSILON {
        return None;
    }

    let band_start = transparent_stop * band as f32 / RADIAL_FADE_STEPS as f32;
    let band_end = transparent_stop * (band + 1) as f32 / RADIAL_FADE_STEPS as f32;
    let ratio = (band_start + band_end) * 0.5;
    let alpha = radial_fade_alpha_at(visual, ratio);
    if alpha <= 0.0 {
        return None;
    }

    Some(RadialBandVisual {
        inner_radius: visual.radius * band_start,
        outer_radius: visual.radius * band_end,
        color: Color::new(visual.center.r, visual.center.g, visual.center.b, alpha),
    })
}

fn radial_fade_alpha_at(visual: RadialFadeVisual, ratio: f32) -> f32 {
    let transparent_stop = visual.transparent_stop_ratio.clamp(0.0, 1.0);
    if transparent_stop <= f32::EPSILON {
        return 0.0;
    }
    visual.center.a * (1.0 - (ratio / transparent_stop).min(1.0)).clamp(0.0, 1.0)
}

fn draw_radial_ring(x: f32, y: f32, visual: RadialRingVisual) {
    if visual.radius <= f32::EPSILON {
        return;
    }

    let points = radial_shape_points(visual.shape);
    let scale = visual.radius / visual.shape.base_radius;
    let start = visual.inner_stop_ratio;
    let end = visual.outer_stop_ratio;
    for band in 0..RADIAL_RING_BANDS {
        let band_start = start + (end - start) * band as f32 / RADIAL_RING_BANDS as f32;
        let band_end = start + (end - start) * (band + 1) as f32 / RADIAL_RING_BANDS as f32;
        let t = (band_start + band_end) * 0.5;
        let alpha = if t <= visual.peak_stop_ratio {
            (t - visual.inner_stop_ratio) / (visual.peak_stop_ratio - visual.inner_stop_ratio)
        } else {
            (visual.outer_stop_ratio - t) / (visual.outer_stop_ratio - visual.peak_stop_ratio)
        }
        .clamp(0.0, 1.0);

        if alpha <= 0.0 {
            continue;
        }

        draw_radial_shape_band(
            &points,
            RadialShapeBand {
                origin: vec2(x, y),
                scale,
                inner_ratio: band_start,
                outer_ratio: band_end,
                color: Color::new(
                    visual.peak.r,
                    visual.peak.g,
                    visual.peak.b,
                    visual.peak.a * alpha,
                ),
            },
        );
    }
}

fn radial_shape_points(shape: RadialGradientShape) -> Vec<SwfPoint> {
    let mut points =
        Vec::with_capacity(1 + shape.segments.len() * usize::from(RADIAL_SHAPE_FLATTEN_STEPS));
    points.push(shape.start);
    let mut current = shape.start;
    for segment in shape.segments {
        match *segment {
            SwfPathSegment::Line(to) => {
                points.push(to);
                current = to;
            },
            SwfPathSegment::Quad { control, to } => {
                push_untransformed_quadratic(
                    &mut points,
                    current,
                    control,
                    to,
                    RADIAL_SHAPE_FLATTEN_STEPS,
                );
                current = to;
            },
        }
    }
    pop_duplicate_last_point(&mut points);
    points
}

fn draw_radial_shape_band(points: &[SwfPoint], band: RadialShapeBand) {
    if points.len() < 3 {
        return;
    }

    for edge in points.windows(2) {
        draw_radial_shape_band_segment(edge[0], edge[1], band);
    }
    if let (Some(first), Some(last)) = (points.first(), points.last()) {
        draw_radial_shape_band_segment(*last, *first, band);
    }
}

fn draw_radial_shape_band_segment(start: SwfPoint, end: SwfPoint, band: RadialShapeBand) {
    let inner0 = radial_shape_stage_point(band, start, band.inner_ratio);
    let outer0 = radial_shape_stage_point(band, start, band.outer_ratio);
    let inner1 = radial_shape_stage_point(band, end, band.inner_ratio);
    let outer1 = radial_shape_stage_point(band, end, band.outer_ratio);

    draw_triangle(inner0, outer0, outer1, band.color);
    draw_triangle(inner0, outer1, inner1, band.color);
}

fn radial_shape_stage_point(band: RadialShapeBand, local: SwfPoint, ratio: f32) -> Vec2 {
    vec2(
        (local.x * band.scale).mul_add(ratio, band.origin.x),
        (local.y * band.scale).mul_add(ratio, band.origin.y),
    )
}

fn draw_dying_ball_at(ball: DyingBall, x: f64, y: f64) {
    let visual = ball.visual();
    if visual.alpha <= 0.0 {
        return;
    }

    let (sx, sy) = world_to_stage(x, y);
    let sx = sx as f32;
    let sy = sy as f32;
    let alpha = visual.alpha as f32;
    draw_radial_ring(sx, sy, dying_ball_ring_visual(visual.scale as f32, alpha));
}

fn draw_round_intro(world: &World, announce_texts: &AnnounceTextTextures) {
    if world.round_intro_visual_ticks == 0 || world.winner.is_some() {
        return;
    }
    let round = u16::from(world.blue_matches + world.red_matches) + 1;
    for visual in round_intro_visual(round, world.round_intro_visual_ticks) {
        draw_announce_text_visual(visual, announce_texts);
    }
}

fn round_intro_visual(round: u16, ticks: u32) -> Vec<AnnounceTextVisual> {
    let frame = 195 + ROUND_INTRO_VISUAL_TICKS.saturating_sub(ticks);
    let Some(outer) = round_intro_outer_transform(frame) else {
        return Vec::new();
    };
    if outer.alpha_mult == 0 {
        return Vec::new();
    }

    let round_text = combine_transforms(
        outer,
        WinTextTransform::new(
            ROUND_TEXT_INNER_SCALE,
            ROUND_TEXT_INNER_TX,
            ROUND_TEXT_INNER_TY,
            outer.alpha_mult,
        ),
    );
    let number_text = combine_transforms(
        outer,
        WinTextTransform::new(
            ROUND_NUMBER_INNER_SCALE,
            ROUND_NUMBER_INNER_TX,
            ROUND_NUMBER_INNER_TY,
            outer.alpha_mult,
        ),
    );

    let mut visuals = vec![announce_text_visual(AnnounceTextKind::Round, round_text)];
    if let Some(number_visual) = round_number_announce_text_visual(round_label(round), number_text)
    {
        visuals.push(number_visual);
    }
    visuals
}

fn round_intro_outer_transform(frame: u32) -> Option<WinTextTransform> {
    match frame {
        212..=225 => Some(ROUND_INTRO_GROW_TRANSFORMS[(frame - 212) as usize]),
        226..=255 => Some(WinTextTransform::new(1.0, 0.0, 79.4, 256)),
        256..=267 => Some(ROUND_INTRO_FADE_TRANSFORMS[(frame - 256) as usize]),
        _ => None,
    }
}

const fn round_label(round: u16) -> &'static str {
    match round {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        _ => "?",
    }
}

fn draw_match_win_announce(world: &World, announce_texts: &AnnounceTextTextures) {
    let (side, ticks, total_ticks) = if world.final_win_announce_ticks > 0 {
        let Some(side) = world.final_win_announce_side else {
            return;
        };
        (
            side,
            world.final_win_announce_ticks,
            gravityarcade::sim::final_win_announce_ticks(side),
        )
    } else {
        if world.match_win_announce_ticks == 0 {
            return;
        }
        let Some(side) = world.match_win_announce_side else {
            return;
        };
        (
            side,
            world.match_win_announce_ticks,
            gravityarcade::sim::MATCH_WIN_ANNOUNCE_TICKS,
        )
    };

    if total_ticks == gravityarcade::sim::MATCH_WIN_ANNOUNCE_TICKS {
        if let Some(visual) = match_win_announce_text_visual(side, ticks) {
            draw_announce_text_visual(visual, announce_texts);
        }
        return;
    }

    for visual in final_win_announce_visual(side, ticks) {
        match visual {
            AnnounceVisual::Text(text) => draw_announce_text_visual(text, announce_texts),
            AnnounceVisual::Pip(pip) => {
                draw_match_pip_scaled(pip.x, pip.y, side, pip.scale, pip.alpha);
            },
        }
    }
}

fn draw_announce_text_visual(visual: AnnounceTextVisual, announce_texts: &AnnounceTextTextures) {
    if visual.alpha <= 0.0 {
        return;
    }

    let kind = visual.kind;
    let text = announce_text_texture(kind, announce_texts);
    let bounds = text.local_bounds;
    let x = visual
        .scale
        .mul_add(bounds.x - kind.bounds_center_x(), visual.x);
    let y = visual
        .scale
        .mul_add(bounds.y - kind.baseline_y(), visual.baseline_y);
    draw_texture_ex(
        &text.texture,
        x,
        y,
        Color::new(1.0, 1.0, 1.0, visual.alpha),
        DrawTextureParams {
            dest_size: Some(vec2(bounds.w * visual.scale, bounds.h * visual.scale)),
            ..Default::default()
        },
    );
}

fn announce_text_texture(
    kind: AnnounceTextKind,
    announce_texts: &AnnounceTextTextures,
) -> &AnnounceTextTexture {
    match kind {
        AnnounceTextKind::BlueWins => &announce_texts.blue_wins,
        AnnounceTextKind::RedWins => &announce_texts.red_wins,
        AnnounceTextKind::Round => &announce_texts.round,
        AnnounceTextKind::BlueMatch => &announce_texts.blue_match,
        AnnounceTextKind::RedMatch => &announce_texts.red_match,
        AnnounceTextKind::RoundNumber1 => &announce_texts.round_number_1,
        AnnounceTextKind::RoundNumber2 => &announce_texts.round_number_2,
        AnnounceTextKind::RoundNumber3 => &announce_texts.round_number_3,
    }
}

fn match_win_announce_text_visual(side: Side, ticks: u32) -> Option<AnnounceTextVisual> {
    let elapsed = gravityarcade::sim::MATCH_WIN_ANNOUNCE_TICKS.saturating_sub(ticks);
    let frame = match side {
        Side::Blue => 2 + elapsed,
        Side::Red => 98 + elapsed,
    };
    let transform = match side {
        Side::Blue => blue_win_text_transform(frame)?,
        Side::Red => red_win_text_transform(frame)?,
    };
    if transform.alpha_mult == 0 {
        return None;
    }

    Some(announce_text_visual(win_text_kind(side), transform))
}

fn final_win_announce_visual(side: Side, ticks: u32) -> Vec<AnnounceVisual> {
    let frame = match side {
        Side::Blue => gravityarcade::sim::BLUE_FINAL_WIN_ANNOUNCE_TICKS.saturating_sub(ticks) + 269,
        Side::Red => gravityarcade::sim::RED_FINAL_WIN_ANNOUNCE_TICKS.saturating_sub(ticks) + 432,
    };

    let transforms = match side {
        Side::Blue => final_blue_win_transforms(frame),
        Side::Red => final_red_win_transforms(frame),
    };
    transforms
        .into_iter()
        .flat_map(|transform| final_announce_visuals_for_transform(side, transform))
        .collect()
}

fn final_blue_win_transforms(frame: u32) -> Option<[WinTextTransform; 5]> {
    match frame {
        269..=284 => Some(final_grow_transforms(frame - 269)),
        285..=367 => Some(final_settled_transforms(256)),
        368..=405 => Some(final_settled_transforms(
            BLUE_FINAL_FADE_ALPHA[(frame - 368) as usize],
        )),
        _ => None,
    }
}

fn final_red_win_transforms(frame: u32) -> Option<[WinTextTransform; 5]> {
    match frame {
        432..=447 => Some(final_grow_transforms(frame - 432)),
        448..=530 => Some(final_settled_transforms(256)),
        531..=565 => Some(final_settled_transforms(
            RED_FINAL_FADE_ALPHA[(frame - 531) as usize],
        )),
        _ => None,
    }
}

fn final_grow_transforms(index: u32) -> [WinTextTransform; 5] {
    let outer = FINAL_GROW_TRANSFORMS[index as usize];
    let alpha = outer.alpha_mult;
    [
        combine_transforms(
            outer,
            WinTextTransform::new(
                WIN_TEXT_SETTLED_SCALE,
                FINAL_GROW_WIN_TEXT_TX,
                FINAL_GROW_WIN_TEXT_TY,
                alpha,
            ),
        ),
        combine_transforms(
            outer,
            WinTextTransform::new(
                FINAL_MATCH_TEXT_SCALE,
                FINAL_GROW_MATCH_TEXT_TX,
                FINAL_GROW_MATCH_TEXT_TY,
                alpha,
            ),
        ),
        combine_transforms(
            outer,
            WinTextTransform::new(
                FINAL_PIP_SETTLED_SCALE,
                FINAL_GROW_PIP_TXS[0],
                FINAL_GROW_PIP_TY,
                alpha,
            ),
        ),
        combine_transforms(
            outer,
            WinTextTransform::new(
                FINAL_PIP_SETTLED_SCALE,
                FINAL_GROW_PIP_TXS[1],
                FINAL_GROW_PIP_TY,
                alpha,
            ),
        ),
        combine_transforms(
            outer,
            WinTextTransform::new(
                FINAL_PIP_SETTLED_SCALE,
                FINAL_GROW_PIP_TXS[2],
                FINAL_GROW_PIP_TY,
                alpha,
            ),
        ),
    ]
}

fn final_settled_transforms(alpha_mult: u16) -> [WinTextTransform; 5] {
    [
        WinTextTransform::new(
            WIN_TEXT_SETTLED_SCALE,
            BLUE_WIN_TEXT_TX,
            FINAL_WIN_TEXT_TY,
            alpha_mult,
        ),
        WinTextTransform::new(
            FINAL_MATCH_TEXT_SCALE,
            FINAL_MATCH_TEXT_TX,
            FINAL_MATCH_TEXT_TY,
            alpha_mult,
        ),
        WinTextTransform::new(
            FINAL_PIP_SETTLED_SCALE,
            FINAL_PIP_TXS[0],
            FINAL_PIP_TY,
            alpha_mult,
        ),
        WinTextTransform::new(
            FINAL_PIP_SETTLED_SCALE,
            FINAL_PIP_TXS[1],
            FINAL_PIP_TY,
            alpha_mult,
        ),
        WinTextTransform::new(
            FINAL_PIP_SETTLED_SCALE,
            FINAL_PIP_TXS[2],
            FINAL_PIP_TY,
            alpha_mult,
        ),
    ]
}

fn combine_transforms(parent: WinTextTransform, child: WinTextTransform) -> WinTextTransform {
    WinTextTransform::new(
        parent.scale * child.scale,
        parent.scale.mul_add(child.tx, parent.tx),
        parent.scale.mul_add(child.ty, parent.ty),
        parent.alpha_mult,
    )
}

fn final_announce_visuals_for_transform(
    side: Side,
    transforms: [WinTextTransform; 5],
) -> Vec<AnnounceVisual> {
    if transforms[0].alpha_mult == 0 {
        return Vec::new();
    }

    vec![
        AnnounceVisual::Text(announce_text_visual(win_text_kind(side), transforms[0])),
        AnnounceVisual::Text(announce_text_visual(match_text_kind(side), transforms[1])),
        AnnounceVisual::Pip(announce_pip_visual(transforms[2])),
        AnnounceVisual::Pip(announce_pip_visual(transforms[3])),
        AnnounceVisual::Pip(announce_pip_visual(transforms[4])),
    ]
}

fn announce_text_visual(kind: AnnounceTextKind, transform: WinTextTransform) -> AnnounceTextVisual {
    announce_text_visual_with_baseline(
        kind,
        kind.text(),
        kind.bounds_center_x(),
        kind.baseline_y(),
        transform,
    )
}

fn round_number_announce_text_visual(
    text: &'static str,
    transform: WinTextTransform,
) -> Option<AnnounceTextVisual> {
    let kind = round_number_text_kind(text)?;
    Some(announce_text_visual_with_baseline(
        kind,
        text,
        kind.bounds_center_x(),
        kind.baseline_y(),
        transform,
    ))
}

fn announce_text_visual_with_baseline(
    kind: AnnounceTextKind,
    text: &'static str,
    bounds_center_x: f32,
    baseline_y: f32,
    transform: WinTextTransform,
) -> AnnounceTextVisual {
    AnnounceTextVisual {
        kind,
        text,
        x: transform
            .scale
            .mul_add(bounds_center_x, ANNOUNCE_ROOT_X + transform.tx),
        baseline_y: transform
            .scale
            .mul_add(baseline_y, ANNOUNCE_ROOT_Y + transform.ty),
        font_size: (12.0 * transform.scale).round().max(1.0) as u16,
        scale: transform.scale,
        alpha: f32::from(transform.alpha_mult) / 256.0,
    }
}

fn announce_pip_visual(transform: WinTextTransform) -> AnnouncePipVisual {
    AnnouncePipVisual {
        x: ANNOUNCE_ROOT_X + transform.tx,
        y: ANNOUNCE_ROOT_Y + transform.ty,
        scale: transform.scale,
        alpha: f32::from(transform.alpha_mult) / 256.0,
    }
}

const fn win_text_kind(side: Side) -> AnnounceTextKind {
    match side {
        Side::Blue => AnnounceTextKind::BlueWins,
        Side::Red => AnnounceTextKind::RedWins,
    }
}

const fn match_text_kind(side: Side) -> AnnounceTextKind {
    match side {
        Side::Blue => AnnounceTextKind::BlueMatch,
        Side::Red => AnnounceTextKind::RedMatch,
    }
}

const fn round_number_text_kind(text: &str) -> Option<AnnounceTextKind> {
    match text.as_bytes() {
        b"1" => Some(AnnounceTextKind::RoundNumber1),
        b"2" => Some(AnnounceTextKind::RoundNumber2),
        b"3" => Some(AnnounceTextKind::RoundNumber3),
        _ => None,
    }
}

fn blue_win_text_transform(frame: u32) -> Option<WinTextTransform> {
    match frame {
        19..=71 => Some(WinTextTransform::new(
            WIN_TEXT_SETTLED_SCALE,
            BLUE_WIN_TEXT_TX,
            BLUE_WIN_TEXT_TY,
            256,
        )),
        72..=96 => Some(WinTextTransform::new(
            WIN_TEXT_SETTLED_SCALE,
            BLUE_WIN_TEXT_TX,
            BLUE_WIN_TEXT_TY,
            WIN_TEXT_FADE_ALPHA[(frame - 72) as usize],
        )),
        _ => None,
    }
}

fn red_win_text_transform(frame: u32) -> Option<WinTextTransform> {
    match frame {
        99..=115 => Some(RED_WIN_GROW_TRANSFORMS[(frame - 99) as usize]),
        116..=168 => Some(WinTextTransform::new(
            WIN_TEXT_SETTLED_SCALE,
            BLUE_WIN_TEXT_TX,
            BLUE_WIN_TEXT_TY,
            256,
        )),
        169..=193 => Some(WinTextTransform::new(
            WIN_TEXT_SETTLED_SCALE,
            BLUE_WIN_TEXT_TX,
            BLUE_WIN_TEXT_TY,
            WIN_TEXT_FADE_ALPHA[(frame - 169) as usize],
        )),
        _ => None,
    }
}

fn draw_button_shape(visual: ButtonShapeVisual) {
    let points = button_outline_points(visual);
    draw_filled_polygon_fan(&points, visual.center, visual.fill);
    draw_closed_polyline(&points, visual.line_width, visual.outline);
}

fn button_shape_contains(visual: ButtonShapeVisual, x: f32, y: f32) -> bool {
    if !visual.rect.contains(vec2(x, y)) {
        return false;
    }
    let points = button_outline_points(visual);
    point_in_polygon(SwfPoint::new(x, y), &points)
}

fn draw_filled_polygon_fan(points: &[SwfPoint], center: SwfPoint, color: Color) {
    for edge in points.windows(2) {
        draw_triangle(
            vec2(center.x, center.y),
            vec2(edge[0].x, edge[0].y),
            vec2(edge[1].x, edge[1].y),
            color,
        );
    }
    if let (Some(first), Some(last)) = (points.first(), points.last()) {
        draw_triangle(
            vec2(center.x, center.y),
            vec2(last.x, last.y),
            vec2(first.x, first.y),
            color,
        );
    }
}

fn draw_filled_polygon_triangulated(points: &[SwfPoint], color: Color) {
    for triangle in triangulate_polygon_indices(points) {
        let a = points[triangle[0]];
        let b = points[triangle[1]];
        let c = points[triangle[2]];
        draw_triangle(vec2(a.x, a.y), vec2(b.x, b.y), vec2(c.x, c.y), color);
    }
}

fn triangulate_polygon_indices(points: &[SwfPoint]) -> Vec<[usize; 3]> {
    const AREA_EPSILON: f32 = 0.001;
    if points.len() < 3 || polygon_signed_area(points).abs() <= AREA_EPSILON {
        return Vec::new();
    }

    let orientation = polygon_signed_area(points).signum();
    let mut remaining = (0..points.len()).collect::<Vec<_>>();
    let mut triangles = Vec::with_capacity(points.len().saturating_sub(2));
    let mut guard = 0;

    while remaining.len() > 3 && guard < points.len() * points.len() {
        guard += 1;
        let len = remaining.len();
        let mut ear_position = None;

        for position in 0..len {
            let previous = remaining[(position + len - 1) % len];
            let current = remaining[position];
            let next = remaining[(position + 1) % len];

            if !polygon_corner_is_convex(
                points[previous],
                points[current],
                points[next],
                orientation,
            ) {
                continue;
            }

            let contains_other_point = remaining.iter().copied().any(|index| {
                index != previous
                    && index != current
                    && index != next
                    && point_in_triangle(
                        points[index],
                        points[previous],
                        points[current],
                        points[next],
                    )
            });
            if !contains_other_point {
                ear_position = Some(position);
                break;
            }
        }

        if let Some(position) = ear_position {
            let len = remaining.len();
            triangles.push([
                remaining[(position + len - 1) % len],
                remaining[position],
                remaining[(position + 1) % len],
            ]);
            remaining.remove(position);
        } else {
            return triangulate_polygon_fan_indices(points);
        }
    }

    if remaining.len() == 3 {
        triangles.push([remaining[0], remaining[1], remaining[2]]);
    }
    triangles
}

fn triangulate_polygon_fan_indices(points: &[SwfPoint]) -> Vec<[usize; 3]> {
    if points.len() < 3 {
        return Vec::new();
    }
    (1..points.len() - 1)
        .map(|index| [0, index, index + 1])
        .collect()
}

fn polygon_signed_area(points: &[SwfPoint]) -> f32 {
    let mut area = 0.0;
    for edge in points.windows(2) {
        area += edge[1].x.mul_add(-edge[0].y, edge[0].x * edge[1].y);
    }
    if let (Some(first), Some(last)) = (points.first(), points.last()) {
        area += first.x.mul_add(-last.y, last.x * first.y);
    }
    area * 0.5
}

fn polygon_corner_is_convex(
    previous: SwfPoint,
    current: SwfPoint,
    next: SwfPoint,
    orientation: f32,
) -> bool {
    const CONVEX_EPSILON: f32 = 0.0001;
    cross_points(previous, current, next) * orientation > CONVEX_EPSILON
}

fn point_in_triangle(point: SwfPoint, a: SwfPoint, b: SwfPoint, c: SwfPoint) -> bool {
    const TRIANGLE_EPSILON: f32 = 0.0001;
    let ab = cross_points(a, b, point);
    let bc = cross_points(b, c, point);
    let ca = cross_points(c, a, point);
    let has_negative = ab < -TRIANGLE_EPSILON || bc < -TRIANGLE_EPSILON || ca < -TRIANGLE_EPSILON;
    let has_positive = ab > TRIANGLE_EPSILON || bc > TRIANGLE_EPSILON || ca > TRIANGLE_EPSILON;
    !(has_negative && has_positive)
}

fn cross_points(a: SwfPoint, b: SwfPoint, c: SwfPoint) -> f32 {
    (b.y - a.y).mul_add(-(c.x - a.x), (b.x - a.x) * (c.y - a.y))
}

fn draw_closed_polyline(points: &[SwfPoint], line_width: f32, color: Color) {
    for edge in points.windows(2) {
        draw_line(
            edge[0].x, edge[0].y, edge[1].x, edge[1].y, line_width, color,
        );
    }
    if let (Some(first), Some(last)) = (points.first(), points.last()) {
        draw_line(last.x, last.y, first.x, first.y, line_width, color);
    }
}

fn draw_open_polyline(points: &[SwfPoint], line_width: f32, color: Color) {
    for edge in points.windows(2) {
        draw_line(
            edge[0].x, edge[0].y, edge[1].x, edge[1].y, line_width, color,
        );
    }
}

fn draw_panel_fill() {
    draw_filled_polygon_triangulated(&panel_left_shadow_points(), PANEL_SHADOW);
    draw_filled_polygon_triangulated(&panel_right_shadow_points(), PANEL_SHADOW);
    draw_filled_polygon_triangulated(&panel_center_points(), PANEL);
}

fn draw_playfield_mask() {
    draw_rectangle(
        0.0,
        0.0,
        STAGE_BOUNDS.x_max,
        PANEL_CENTER_BOUNDS.y_min,
        STAGE_RED,
    );
    draw_rectangle(
        0.0,
        PANEL_CENTER_BOUNDS.y_max,
        STAGE_BOUNDS.x_max,
        STAGE_BOUNDS.y_max - PANEL_CENTER_BOUNDS.y_max,
        STAGE_RED,
    );
    draw_rectangle(
        0.0,
        PANEL_CENTER_BOUNDS.y_min,
        7.05,
        PANEL_CENTER_BOUNDS.y_max - PANEL_CENTER_BOUNDS.y_min,
        STAGE_RED,
    );
    draw_rectangle(
        543.0,
        PANEL_CENTER_BOUNDS.y_min,
        STAGE_BOUNDS.x_max - 543.0,
        PANEL_CENTER_BOUNDS.y_max - PANEL_CENTER_BOUNDS.y_min,
        STAGE_RED,
    );
    draw_filled_polygon_triangulated(&playfield_mask_top_left_points(), STAGE_RED);
    draw_filled_polygon_triangulated(&playfield_mask_top_right_points(), STAGE_RED);
    draw_filled_polygon_triangulated(&playfield_mask_bottom_left_points(), STAGE_RED);
    draw_filled_polygon_triangulated(&playfield_mask_bottom_right_points(), STAGE_RED);
}

fn panel_left_shadow_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::PANEL_FILL_SHAPE.left_shadow)
}

fn panel_center_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::PANEL_FILL_SHAPE.center)
}

fn panel_right_shadow_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::PANEL_FILL_SHAPE.right_shadow)
}

fn playfield_mask_top_left_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::PLAYFIELD_MASK_SHAPE.top_left)
}

fn playfield_mask_top_right_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::PLAYFIELD_MASK_SHAPE.top_right)
}

fn playfield_mask_bottom_left_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::PLAYFIELD_MASK_SHAPE.bottom_left)
}

fn playfield_mask_bottom_right_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::PLAYFIELD_MASK_SHAPE.bottom_right)
}

fn panel_outline_primary_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::PANEL_OUTLINE_SHAPE.primary)
}

fn panel_outline_lower_right_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::PANEL_OUTLINE_SHAPE.lower_right)
}

fn retained_frame_mask_outline_primary_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::RETAINED_MASK_OUTLINE_SHAPE.primary)
}

fn retained_frame_mask_outline_lower_right_points() -> Vec<SwfPoint> {
    panel_chrome_contour_points(panel_chrome_shapes::RETAINED_MASK_OUTLINE_SHAPE.lower_right)
}

fn panel_chrome_contour_points(contour: PanelChromeContour) -> Vec<SwfPoint> {
    let flatten_steps = panel_chrome_shapes::FLATTEN_STEPS;
    let mut points = Vec::with_capacity(1 + contour.segments.len() * usize::from(flatten_steps));
    points.push(contour.start);
    let mut current = contour.start;
    for &segment in contour.segments {
        match segment {
            SwfPathSegment::Line(to) => {
                points.push(to);
                current = to;
            },
            SwfPathSegment::Quad { control, to } => {
                for step in 1..=flatten_steps {
                    let t = f32::from(step) / f32::from(flatten_steps);
                    points.push(quadratic_point(current, control, to, t));
                }
                current = to;
            },
        }
    }
    points
}

fn button_outline_points(visual: ButtonShapeVisual) -> Vec<SwfPoint> {
    let shape = button_shape22::SHAPE;
    let mut points =
        Vec::with_capacity(1 + shape.segments.len() * usize::from(button_shape22::FLATTEN_STEPS));
    let mut current = shape.start;
    points.push(button_stage_point(visual, current));
    for segment in shape.segments {
        match *segment {
            SwfPathSegment::Line(to) => {
                points.push(button_stage_point(visual, to));
                current = to;
            },
            SwfPathSegment::Quad { control, to } => {
                for step in 1..=button_shape22::FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(button_shape22::FLATTEN_STEPS);
                    points.push(button_stage_point(
                        visual,
                        quadratic_point(current, control, to, t),
                    ));
                }
                current = to;
            },
        }
    }
    if points.len() > 1 && points_are_close(points[0], points[points.len() - 1]) {
        points.pop();
    }
    points
}

fn quadratic_point(start: SwfPoint, control: SwfPoint, end: SwfPoint, t: f32) -> SwfPoint {
    let inv_t = 1.0 - t;
    SwfPoint::new(
        (t * t).mul_add(
            end.x,
            (2.0 * inv_t * t).mul_add(control.x, inv_t * inv_t * start.x),
        ),
        (t * t).mul_add(
            end.y,
            (2.0 * inv_t * t).mul_add(control.y, inv_t * inv_t * start.y),
        ),
    )
}

fn button_stage_point(visual: ButtonShapeVisual, local: SwfPoint) -> SwfPoint {
    SwfPoint::new(
        local.x.mul_add(visual.scale_x, visual.center.x),
        local.y.mul_add(visual.scale_y, visual.center.y),
    )
}

fn paddle_body_outline_points(visual: PaddleBodyVisual) -> Vec<SwfPoint> {
    let shape = paddle_body_shape89::SHAPE;
    let mut points = Vec::with_capacity(
        1 + shape.segments.len() * usize::from(paddle_body_shape89::FLATTEN_STEPS),
    );
    let mut current = shape.start;
    points.push(paddle_body_stage_point(visual, current));
    for segment in shape.segments {
        match *segment {
            SwfPathSegment::Line(to) => {
                points.push(paddle_body_stage_point(visual, to));
                current = to;
            },
            SwfPathSegment::Quad { control, to } => {
                for step in 1..=paddle_body_shape89::FLATTEN_STEPS {
                    let t = f32::from(step) / f32::from(paddle_body_shape89::FLATTEN_STEPS);
                    points.push(paddle_body_stage_point(
                        visual,
                        quadratic_point(current, control, to, t),
                    ));
                }
                current = to;
            },
        }
    }
    if points.len() > 1 && points_are_close(points[0], points[points.len() - 1]) {
        points.pop();
    }
    points
}

fn paddle_body_stage_point(visual: PaddleBodyVisual, local: SwfPoint) -> SwfPoint {
    SwfPoint::new(visual.center.x + local.x, visual.center.y + local.y)
}

fn draw_swf_text_left(text: &str, visual: SwfLineVisual, color: Color, font: Option<&Font>) {
    draw_text_ex(
        text,
        visual.x,
        visual.baseline_y,
        TextParams {
            font,
            font_size: visual.font_size,
            color,
            ..Default::default()
        },
    );
}

fn draw_swf_text_center(
    text: &str,
    visual: SwfCenterTextVisual,
    color: Color,
    font: Option<&Font>,
) {
    let dims = measure_text(text, font, visual.font_size, 1.0);
    draw_text_ex(
        text,
        dims.width.mul_add(-0.5, visual.center_x),
        visual.baseline_y,
        TextParams {
            font,
            font_size: visual.font_size,
            color,
            ..Default::default()
        },
    );
}

fn draw_interpolate_notice(enabled: bool) {
    let label = if enabled {
        "Interpolate: on"
    } else {
        "Interpolate: off"
    };
    let text_size = 18;
    let measured = measure_text(label, None, text_size, 1.0);
    let x = 10.0;
    let y = STAGE_H as f32 - 14.0;
    draw_rectangle(
        x - 5.0,
        y - f32::from(text_size) - 5.0,
        measured.width + 10.0,
        f32::from(text_size) + 10.0,
        Color::new(0.0, 0.0, 0.0, 0.72),
    );
    draw_text_ex(
        label,
        x,
        y,
        TextParams {
            font_size: text_size,
            color: TEXT,
            ..Default::default()
        },
    );
}

fn current_frame_time_step() -> f64 {
    capped_frame_time(get_frame_time())
}

fn capped_frame_time(raw_frame_time: f32) -> f64 {
    if raw_frame_time.is_finite() && raw_frame_time > 0.0 {
        f64::from(raw_frame_time).min(MAX_FRAME_TIME)
    } else {
        0.0
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Gravity Arcade".to_string(),
        window_width: STAGE_W as i32,
        window_height: STAGE_H as i32,
        high_dpi: false,
        sample_count: 4,
        window_resizable: false,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            apple_gfx_api: miniquad::conf::AppleGfxApi::OpenGl,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct DebugShot {
    path: String,
    ticks: u32,
}

#[cfg(debug_assertions)]
fn debug_shot_from_env() -> Option<DebugShot> {
    let spec = std::env::var("GRAVITYARCADE_SHOT").ok()?;
    let (path, ticks) = parse_debug_shot_spec(&spec);
    if path.is_empty() {
        eprintln!("gravityarcade: GRAVITYARCADE_SHOT path is empty; capture disabled");
        return None;
    }
    Some(DebugShot {
        path: path.to_owned(),
        ticks,
    })
}

#[cfg(debug_assertions)]
fn parse_debug_shot_spec(spec: &str) -> (&str, u32) {
    if let Some((path, raw_ticks)) = spec.rsplit_once(':')
        && !path.is_empty()
        && let Ok(ticks) = raw_ticks.parse::<u32>()
    {
        return (path, ticks);
    }
    (spec, 0)
}

#[cfg(debug_assertions)]
const DEBUG_WARP_EXPECTED: &str = "startup, xml_wait, menu, offline_menu, \
menu_polarisation_opposite, menu_polarisation_same, menu_polarisation_all, menu_matches_5, \
menu_matches_7, menu_matches_1, menu_gravity_low, menu_gravity_high, menu_gravity_very_high, \
menu_gravity_black_hole, menu_speed_fast, help, playing_idle, playing, playing_red, score_ramps, \
score_max, round_intro_1, round_intro, blue_win, red_win, blue_final, or red_final";
#[cfg(debug_assertions)]
const DEBUG_ROUND_INTRO_VISIBLE_TICKS: u32 = 43;
#[cfg(debug_assertions)]
const DEBUG_MATCH_WIN_VISIBLE_TICKS: u32 = 79;
#[cfg(debug_assertions)]
const DEBUG_BLUE_FINAL_WIN_VISIBLE_TICKS: u32 = 146;
#[cfg(debug_assertions)]
const DEBUG_RED_FINAL_WIN_VISIBLE_TICKS: u32 = 141;

#[cfg(debug_assertions)]
fn apply_debug_warp_from_env(game: &mut Game) {
    let Some(warp) = std::env::var("GRAVITYARCADE_WARP").ok() else {
        return;
    };
    apply_debug_warp(game, &warp);
}

#[cfg(debug_assertions)]
fn apply_debug_warp(game: &mut Game, warp: &str) {
    match warp {
        "startup" => {},
        "xml_wait" => {
            game.screen = Screen::Startup;
            game.startup_ticks = STARTUP_XML_WAIT_TICK;
        },
        "menu" => {
            game.screen = Screen::Menu;
            game.offline = false;
        },
        "offline_menu" => {
            game.screen = Screen::Menu;
            game.offline = true;
        },
        "menu_polarisation_opposite" => {
            apply_debug_menu_setting(game, |settings| {
                settings.polarisation = Polarisation::OppositeRepels;
            });
        },
        "menu_polarisation_same" => {
            apply_debug_menu_setting(game, |settings| {
                settings.polarisation = Polarisation::SameRepels;
            });
        },
        "menu_polarisation_all" => {
            apply_debug_menu_setting(game, |settings| {
                settings.polarisation = Polarisation::AllRepel;
            });
        },
        "menu_matches_5" => {
            apply_debug_menu_setting(game, |settings| {
                settings.matches = 5;
            });
        },
        "menu_matches_7" => {
            apply_debug_menu_setting(game, |settings| {
                settings.matches = 7;
            });
        },
        "menu_matches_1" => {
            apply_debug_menu_setting(game, |settings| {
                settings.matches = 1;
            });
        },
        "menu_gravity_low" => {
            apply_debug_menu_setting(game, |settings| {
                settings.gravity = GravityStrength::G1;
            });
        },
        "menu_gravity_high" => {
            apply_debug_menu_setting(game, |settings| {
                settings.gravity = GravityStrength::G3;
            });
        },
        "menu_gravity_very_high" => {
            apply_debug_menu_setting(game, |settings| {
                settings.gravity = GravityStrength::G4;
            });
        },
        "menu_gravity_black_hole" => {
            apply_debug_menu_setting(game, |settings| {
                settings.gravity = GravityStrength::G5;
            });
        },
        "menu_speed_fast" => {
            apply_debug_menu_setting(game, |settings| {
                settings.speed = SpeedMode::Fast;
            });
        },
        "help" => {
            game.screen = Screen::Help;
        },
        "playing_idle" => {
            apply_debug_playfield_ready(game);
        },
        "playing" => {
            apply_debug_playfield_ready(game);
            let _ = game.world.try_fire(Side::Blue);
        },
        "playing_red" => {
            apply_debug_playfield_ready(game);
            let _ = game.world.try_fire(Side::Red);
        },
        "score_ramps" => {
            apply_debug_playfield_ready(game);
            game.world.blue_score = 100;
            game.world.red_score = 360;
            game.world.tick = 6;
        },
        "score_max" => {
            apply_debug_playfield_ready(game);
            game.world.blue_score = gravityarcade::sim::SCORE_METER_MAX;
            game.world.red_score = gravityarcade::sim::SCORE_METER_MAX;
            game.world.tick = 6;
        },
        "round_intro_1" => {
            apply_debug_round_intro_1(game);
        },
        "round_intro" => {
            apply_debug_round_intro(game);
        },
        "blue_win" => {
            apply_debug_match_win(game, Side::Blue);
        },
        "red_win" => {
            apply_debug_match_win(game, Side::Red);
        },
        "blue_final" => {
            apply_debug_final_win(game, Side::Blue);
        },
        "red_final" => {
            apply_debug_final_win(game, Side::Red);
        },
        _ => {
            eprintln!(
                "gravityarcade: unknown GRAVITYARCADE_WARP '{warp}', expected {DEBUG_WARP_EXPECTED}"
            );
        },
    }
    game.sync_gameplay_visuals();
}

#[cfg(debug_assertions)]
fn apply_debug_menu_setting(game: &mut Game, configure: impl FnOnce(&mut Settings)) {
    game.screen = Screen::Menu;
    game.offline = false;
    game.settings = Settings::default();
    configure(&mut game.settings);
}

#[cfg(debug_assertions)]
fn apply_debug_playfield_ready(game: &mut Game) {
    game.screen = Screen::Playing;
    game.world = World::new(game.settings);
    game.world.round_intro_ticks = 0;
    game.world.round_intro_visual_ticks = 0;
    game.world.blue.gun_ready = true;
    game.world.red.gun_ready = true;
}

#[cfg(debug_assertions)]
fn apply_debug_round_intro(game: &mut Game) {
    apply_debug_playfield_ready(game);
    game.world.blue_matches = 1;
    game.world.round_intro_visual_ticks = DEBUG_ROUND_INTRO_VISIBLE_TICKS;
}

#[cfg(debug_assertions)]
fn apply_debug_round_intro_1(game: &mut Game) {
    game.screen = Screen::Playing;
    game.world = World::new(game.settings);
    game.world.round_intro_visual_ticks = DEBUG_ROUND_INTRO_VISIBLE_TICKS;
}

#[cfg(debug_assertions)]
fn apply_debug_match_win(game: &mut Game, side: Side) {
    apply_debug_playfield_ready(game);
    match side {
        Side::Blue => game.world.blue_matches = 1,
        Side::Red => game.world.red_matches = 1,
    }
    game.world.match_win_announce_ticks = DEBUG_MATCH_WIN_VISIBLE_TICKS;
    game.world.match_win_announce_side = Some(side);
}

#[cfg(debug_assertions)]
fn apply_debug_final_win(game: &mut Game, side: Side) {
    apply_debug_playfield_ready(game);
    let wins_needed = game.world.settings.wins_needed();
    match side {
        Side::Blue => game.world.blue_matches = wins_needed,
        Side::Red => game.world.red_matches = wins_needed,
    }
    game.world.winner = Some(side);
    game.world.final_win_announce_ticks = match side {
        Side::Blue => DEBUG_BLUE_FINAL_WIN_VISIBLE_TICKS,
        Side::Red => DEBUG_RED_FINAL_WIN_VISIBLE_TICKS,
    };
    game.world.final_win_announce_side = Some(side);
}

#[cfg(debug_assertions)]
async fn capture_debug_shot(game: &mut Game, shot: &DebugShot) {
    for _ in 0..shot.ticks {
        game.debug_fixed_tick();
    }
    let canvas = render_target(STAGE_W as u32, STAGE_H as u32);
    canvas.texture.set_filter(FilterMode::Nearest);
    let mut camera =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, STAGE_W as f32, STAGE_H as f32));
    camera.render_target = Some(canvas.clone());
    set_camera(&camera);
    game.draw_with_interpolation_alpha(Some(1.0));
    set_default_camera();
    next_frame().await;
    canvas.texture.get_texture_data().export_png(&shot.path);
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::panic,
        reason = "Tests assert recovered SWF invariants and should fail loudly"
    )]

    use super::*;

    const SWF_TAG_END: u16 = 0;
    const SWF_TAG_SHOW_FRAME: u16 = 1;
    const SWF_TAG_PLACE_OBJECT: u16 = 4;
    const SWF_TAG_REMOVE_OBJECT: u16 = 5;
    const SWF_TAG_DO_ACTION: u16 = 12;
    const SWF_TAG_DEFINE_SOUND: u16 = 14;
    const SWF_TAG_START_SOUND: u16 = 15;
    const SWF_TAG_DEFINE_BUTTON_SOUND: u16 = 17;
    const SWF_TAG_PLACE_OBJECT2: u16 = 26;
    const SWF_TAG_REMOVE_OBJECT2: u16 = 28;
    const SWF_TAG_DEFINE_SPRITE: u16 = 39;
    const SWF_TAG_SOUND_STREAM_HEAD2: u16 = 45;
    const SWF_TAG_PLACE_OBJECT3: u16 = 70;

    const SWF_SOUND_REFLECT_ID: u16 = 1;
    const SWF_SOUND_MERGE_ID: u16 = 6;
    const SWF_SOUND_SHOT_ID: u16 = 12;
    const SWF_SOUND_BUTTON_ROLLOVER_ID: u16 = 23;
    const SWF_SOUND_BUTTON_PRESS_ID: u16 = 24;
    const SWF_SOUND_SCORE_LINE_ID: u16 = 85;
    const SWF_SOUND_PADDLE_STUN_ID: u16 = 131;
    const SWF_SOUND_ROUND_START_ID: u16 = 156;
    const ROUNDS_PLAYED_HELP_CLIP_ID: u16 = 119;

    #[test]
    fn game_assets_start_empty_so_startup_does_not_bake_textures() {
        let assets = GameAssets::new(None, None, None);

        assert!(assets.top_title.get().is_none());
        assert!(assets.menu_labels.get().is_none());
        assert!(assets.menu_values.get().is_none());
        assert!(assets.help_labels.get().is_none());
        assert!(assets.chrome_texts.get().is_none());
        assert!(assets.announce_texts.get().is_none());
        assert!(assets.counter_digits.get().is_none());
        assert!(assets.rounds_played_label.get().is_none());
        assert!(assets.sponsor_logo.get().is_none());
    }

    #[test]
    fn gameplay_visual_snapshots_interpolate_current_entities_only() {
        let previous = GameplayVisualSnapshot {
            blue_y: 0.0,
            red_y: 20.0,
            balls: vec![EntityVisualSnapshot {
                id: 10,
                x: 0.0,
                y: 10.0,
            }],
            dying_balls: vec![
                EntityVisualSnapshot {
                    id: 11,
                    x: 100.0,
                    y: 50.0,
                },
                EntityVisualSnapshot {
                    id: 12,
                    x: 300.0,
                    y: 300.0,
                },
            ],
        };
        let current = GameplayVisualSnapshot {
            blue_y: 10.0,
            red_y: 0.0,
            balls: vec![
                EntityVisualSnapshot {
                    id: 10,
                    x: 30.0,
                    y: 40.0,
                },
                EntityVisualSnapshot {
                    id: 13,
                    x: 70.0,
                    y: 90.0,
                },
            ],
            dying_balls: vec![EntityVisualSnapshot {
                id: 11,
                x: 80.0,
                y: 70.0,
            }],
        };

        let visuals = previous.interpolate(&current, 0.5);

        assert_close_f64(visuals.blue_y, 5.0);
        assert_close_f64(visuals.red_y, 10.0);
        assert_eq!(
            visuals.balls,
            vec![
                EntityVisualSnapshot {
                    id: 10,
                    x: 15.0,
                    y: 25.0,
                },
                EntityVisualSnapshot {
                    id: 13,
                    x: 70.0,
                    y: 90.0,
                },
            ]
        );
        assert_eq!(
            visuals.dying_balls,
            vec![EntityVisualSnapshot {
                id: 11,
                x: 90.0,
                y: 60.0,
            }]
        );
    }

    #[test]
    fn interpolate_toggle_defaults_on_but_never_mutates_world_state() {
        let mut game = Game::new(AudioBank::default(), GameAssets::new(None, None, None));
        game.screen = Screen::Playing;
        game.accumulator = 0.5 / TICK_HZ;
        game.previous_visuals = GameplayVisualSnapshot {
            blue_y: 0.0,
            red_y: 0.0,
            balls: vec![EntityVisualSnapshot {
                id: 10,
                x: 0.0,
                y: 0.0,
            }],
            dying_balls: Vec::new(),
        };
        game.current_visuals = GameplayVisualSnapshot {
            blue_y: 20.0,
            red_y: -20.0,
            balls: vec![EntityVisualSnapshot {
                id: 10,
                x: 100.0,
                y: 50.0,
            }],
            dying_balls: Vec::new(),
        };
        game.world.blue.y = 20.0;
        game.world.red.y = -20.0;

        assert!(game.interpolate);
        let visuals = game.gameplay_visuals(None);
        assert_close_f64(visuals.blue_y, 10.0);
        assert_close_f64(visuals.red_y, -10.0);
        assert_eq!(visuals.ball_position(10), Some((50.0, 25.0)));
        assert_close_f64(game.world.blue.y, 20.0);
        assert_close_f64(game.world.red.y, -20.0);

        game.interpolate = false;
        let visuals = game.gameplay_visuals(None);
        assert_close_f64(visuals.blue_y, 20.0);
        assert_close_f64(visuals.red_y, -20.0);
        assert_eq!(visuals.ball_position(10), Some((100.0, 50.0)));
    }

    #[cfg(debug_assertions)]
    #[test]
    fn debug_score_meter_warps_cover_partial_and_max_meter_states() {
        let mut game = Game::new(AudioBank::default(), GameAssets::new(None, None, None));

        apply_debug_warp(&mut game, "score_ramps");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(game.world.round_intro_ticks, 0);
        assert_eq!(game.world.round_intro_visual_ticks, 0);
        assert_eq!(game.world.blue_score, 100);
        assert_eq!(game.world.red_score, 360);
        assert_eq!(score_meter_frame(game.world.blue_score), 11);
        assert_eq!(score_meter_frame(game.world.red_score), 37);

        apply_debug_warp(&mut game, "score_max");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(game.world.blue_score, gravityarcade::sim::SCORE_METER_MAX);
        assert_eq!(game.world.red_score, gravityarcade::sim::SCORE_METER_MAX);
        assert_eq!(
            score_meter_frame(game.world.blue_score),
            SCORE_METER_MAX_FRAME
        );
        assert_eq!(
            score_meter_frame(game.world.red_score),
            SCORE_METER_MAX_FRAME
        );
        assert_eq!(final_score_overlay_frame(game.world.tick), 6);
    }

    #[cfg(debug_assertions)]
    #[test]
    fn debug_menu_setting_warps_cover_swf_button_cycle_states() {
        let mut game = Game::new(AudioBank::default(), GameAssets::new(None, None, None));

        apply_debug_warp(&mut game, "menu_polarisation_opposite");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.polarisation, Polarisation::OppositeRepels);

        apply_debug_warp(&mut game, "menu_polarisation_same");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.polarisation, Polarisation::SameRepels);

        apply_debug_warp(&mut game, "menu_polarisation_all");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.polarisation, Polarisation::AllRepel);

        apply_debug_warp(&mut game, "menu_matches_5");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.matches, 5);

        apply_debug_warp(&mut game, "menu_matches_7");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.matches, 7);

        apply_debug_warp(&mut game, "menu_matches_1");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.matches, 1);

        apply_debug_warp(&mut game, "menu_gravity_low");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.gravity, GravityStrength::G1);

        apply_debug_warp(&mut game, "menu_gravity_high");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.gravity, GravityStrength::G3);

        apply_debug_warp(&mut game, "menu_gravity_very_high");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.gravity, GravityStrength::G4);

        apply_debug_warp(&mut game, "menu_gravity_black_hole");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.gravity, GravityStrength::G5);

        apply_debug_warp(&mut game, "menu_speed_fast");
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);
        assert_eq!(game.settings.speed, SpeedMode::Fast);
    }

    #[cfg(debug_assertions)]
    #[test]
    fn debug_playfield_warps_cover_idle_and_active_ball_states() {
        let mut game = Game::new(AudioBank::default(), GameAssets::new(None, None, None));

        apply_debug_warp(&mut game, "playing_idle");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(game.world.round_intro_ticks, 0);
        assert_eq!(game.world.round_intro_visual_ticks, 0);
        assert!(game.world.blue.gun_ready);
        assert!(game.world.red.gun_ready);
        assert!(game.world.balls.is_empty());

        apply_debug_warp(&mut game, "playing");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(game.world.round_intro_ticks, 0);
        assert_eq!(game.world.round_intro_visual_ticks, 0);
        assert_eq!(game.world.balls.len(), 1);
        assert_eq!(game.world.balls[0].color, Side::Blue);

        apply_debug_warp(&mut game, "playing_red");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(game.world.round_intro_ticks, 0);
        assert_eq!(game.world.round_intro_visual_ticks, 0);
        assert_eq!(game.world.balls.len(), 1);
        assert_eq!(game.world.balls[0].color, Side::Red);
    }

    #[cfg(debug_assertions)]
    #[test]
    fn debug_announcement_warps_cover_swf_sprite_162_timelines() {
        let mut game = Game::new(AudioBank::default(), GameAssets::new(None, None, None));

        apply_debug_warp(&mut game, "round_intro_1");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(
            game.world.round_intro_visual_ticks,
            DEBUG_ROUND_INTRO_VISIBLE_TICKS
        );
        let first_round_visual = round_intro_visual(1, game.world.round_intro_visual_ticks);
        assert_eq!(first_round_visual.len(), 2);
        assert_eq!(first_round_visual[0].kind, AnnounceTextKind::Round);
        assert_eq!(first_round_visual[1].kind, AnnounceTextKind::RoundNumber1);

        apply_debug_warp(&mut game, "round_intro");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(
            game.world.round_intro_visual_ticks,
            DEBUG_ROUND_INTRO_VISIBLE_TICKS
        );
        let round = u16::from(game.world.blue_matches + game.world.red_matches) + 1;
        let round_visual = round_intro_visual(round, game.world.round_intro_visual_ticks);
        assert_eq!(round, 2);
        assert_eq!(round_visual.len(), 2);
        assert_eq!(round_visual[0].kind, AnnounceTextKind::Round);
        assert_eq!(round_visual[1].kind, AnnounceTextKind::RoundNumber2);

        apply_debug_warp(&mut game, "blue_win");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(game.world.blue_matches, 1);
        assert_eq!(
            game.world.match_win_announce_ticks,
            DEBUG_MATCH_WIN_VISIBLE_TICKS
        );
        assert_eq!(game.world.match_win_announce_side, Some(Side::Blue));
        assert!(
            match_win_announce_text_visual(Side::Blue, game.world.match_win_announce_ticks)
                .is_some()
        );

        apply_debug_warp(&mut game, "red_win");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(game.world.red_matches, 1);
        assert_eq!(
            game.world.match_win_announce_ticks,
            DEBUG_MATCH_WIN_VISIBLE_TICKS
        );
        assert_eq!(game.world.match_win_announce_side, Some(Side::Red));
        assert!(
            match_win_announce_text_visual(Side::Red, game.world.match_win_announce_ticks)
                .is_some()
        );

        apply_debug_warp(&mut game, "blue_final");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(game.world.winner, Some(Side::Blue));
        assert_eq!(game.world.blue_matches, game.world.settings.wins_needed());
        assert_eq!(
            game.world.final_win_announce_ticks,
            DEBUG_BLUE_FINAL_WIN_VISIBLE_TICKS
        );
        assert_eq!(game.world.final_win_announce_side, Some(Side::Blue));
        assert_eq!(
            final_win_announce_visual(Side::Blue, game.world.final_win_announce_ticks).len(),
            5
        );

        apply_debug_warp(&mut game, "red_final");
        assert_eq!(game.screen, Screen::Playing);
        assert_eq!(game.world.winner, Some(Side::Red));
        assert_eq!(game.world.red_matches, game.world.settings.wins_needed());
        assert_eq!(
            game.world.final_win_announce_ticks,
            DEBUG_RED_FINAL_WIN_VISIBLE_TICKS
        );
        assert_eq!(game.world.final_win_announce_side, Some(Side::Red));
        assert_eq!(
            final_win_announce_visual(Side::Red, game.world.final_win_announce_ticks).len(),
            5
        );
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SwfStartSound {
        sprite: Option<u16>,
        frame: u16,
        sound_id: u16,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SwfButtonSounds {
        button_id: u16,
        sound_ids: [u16; 4],
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct SwfSoundInfo {
        in_point: Option<u32>,
        out_point: Option<u32>,
        loop_count: Option<u16>,
        envelope: Vec<(u32, u16, u16)>,
    }

    type SwfButtonSoundTransitionInfo = Option<(u16, SwfSoundInfo)>;
    type SwfButtonSoundInfoRecord = (u16, [SwfButtonSoundTransitionInfo; 4]);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SwfRootPlacement {
        frame: u16,
        depth: u16,
        character_id: Option<u16>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SwfRootRemoval {
        frame: u16,
        depth: u16,
    }

    struct SwfTag<'a> {
        code: u16,
        body: &'a [u8],
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.01,
            "expected {actual} to be within 0.01 of {expected}"
        );
    }

    fn assert_close_f64(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 0.000_001,
            "expected {actual} to be within 0.000001 of {expected}"
        );
    }

    fn assert_color(actual: Color, expected: Color) {
        assert_close(actual.r, expected.r);
        assert_close(actual.g, expected.g);
        assert_close(actual.b, expected.b);
        assert_close(actual.a, expected.a);
    }

    fn assert_rect_visual(actual: RectVisual, x: f32, y: f32, w: f32, h: f32) {
        assert_close(actual.x, x);
        assert_close(actual.y, y);
        assert_close(actual.w, w);
        assert_close(actual.h, h);
    }

    fn assert_rect(actual: Rect, x: f32, y: f32, w: f32, h: f32) {
        assert_close(actual.x, x);
        assert_close(actual.y, y);
        assert_close(actual.w, w);
        assert_close(actual.h, h);
    }

    fn assert_point(actual: Vec2, x: f32, y: f32) {
        assert_close(actual.x, x);
        assert_close(actual.y, y);
    }

    fn assert_swf_point(actual: SwfPoint, x: f32, y: f32) {
        assert_close(actual.x, x);
        assert_close(actual.y, y);
    }

    fn read_le_u16(bytes: &[u8], offset: usize) -> u16 {
        u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
    }

    fn read_le_u32(bytes: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ])
    }

    fn swf_first_tag_offset(bytes: &[u8]) -> usize {
        assert_eq!(&bytes[0..3], b"FWS");
        let rect_bits = usize::from(bytes[8] >> 3);
        let rect_bytes = (5 + rect_bits * 4).div_ceil(8);
        8 + rect_bytes + 4
    }

    fn collect_swf_tags(bytes: &[u8], mut offset: usize) -> Vec<SwfTag<'_>> {
        let mut tags = Vec::new();
        while offset + 2 <= bytes.len() {
            let header = read_le_u16(bytes, offset);
            offset += 2;
            let code = header >> 6;
            let short_len = usize::from(header & 0x3f);
            let len = if short_len == 0x3f {
                assert!(offset + 4 <= bytes.len());
                let long_len = read_le_u32(bytes, offset) as usize;
                offset += 4;
                long_len
            } else {
                short_len
            };
            assert!(offset + len <= bytes.len());
            let body = &bytes[offset..offset + len];
            offset += len;
            tags.push(SwfTag { code, body });
            if code == SWF_TAG_END {
                break;
            }
        }
        tags
    }

    fn collect_swf_start_sounds(
        bytes: &[u8],
        offset: usize,
        sprite: Option<u16>,
        sounds: &mut Vec<SwfStartSound>,
    ) {
        let mut frame = 1;
        for tag in collect_swf_tags(bytes, offset) {
            match tag.code {
                SWF_TAG_START_SOUND => sounds.push(SwfStartSound {
                    sprite,
                    frame,
                    sound_id: read_le_u16(tag.body, 0),
                }),
                SWF_TAG_DEFINE_SPRITE => {
                    let sprite_id = read_le_u16(tag.body, 0);
                    collect_swf_start_sounds(tag.body, 4, Some(sprite_id), sounds);
                },
                SWF_TAG_SHOW_FRAME => frame += 1,
                SWF_TAG_END => break,
                _ => {},
            }
        }
    }

    fn collect_swf_start_sound_infos(
        bytes: &[u8],
        offset: usize,
        sprite: Option<u16>,
        sounds: &mut Vec<(SwfStartSound, SwfSoundInfo)>,
    ) {
        let mut frame = 1;
        for tag in collect_swf_tags(bytes, offset) {
            match tag.code {
                SWF_TAG_START_SOUND => {
                    let (sound_info, next_offset) = read_swf_sound_info(tag.body, 2);
                    assert_eq!(next_offset, tag.body.len());
                    sounds.push((
                        SwfStartSound {
                            sprite,
                            frame,
                            sound_id: read_le_u16(tag.body, 0),
                        },
                        sound_info,
                    ));
                },
                SWF_TAG_DEFINE_SPRITE => {
                    let sprite_id = read_le_u16(tag.body, 0);
                    collect_swf_start_sound_infos(tag.body, 4, Some(sprite_id), sounds);
                },
                SWF_TAG_SHOW_FRAME => frame += 1,
                SWF_TAG_END => break,
                _ => {},
            }
        }
    }

    fn collect_swf_button_sounds(bytes: &[u8], offset: usize, sounds: &mut Vec<SwfButtonSounds>) {
        for tag in collect_swf_tags(bytes, offset) {
            match tag.code {
                SWF_TAG_DEFINE_BUTTON_SOUND => sounds.push(read_swf_button_sounds(tag.body)),
                SWF_TAG_DEFINE_SPRITE => {
                    collect_swf_button_sounds(tag.body, 4, sounds);
                },
                SWF_TAG_END => break,
                _ => {},
            }
        }
    }

    fn collect_swf_button_sound_infos(
        bytes: &[u8],
        offset: usize,
        sounds: &mut Vec<SwfButtonSoundInfoRecord>,
    ) {
        for tag in collect_swf_tags(bytes, offset) {
            match tag.code {
                SWF_TAG_DEFINE_BUTTON_SOUND => {
                    let mut body_offset = 2;
                    let mut transitions = [None, None, None, None];
                    for transition in &mut transitions {
                        let sound_id = read_le_u16(tag.body, body_offset);
                        body_offset += 2;
                        if sound_id != 0 {
                            let (sound_info, next_offset) =
                                read_swf_sound_info(tag.body, body_offset);
                            body_offset = next_offset;
                            *transition = Some((sound_id, sound_info));
                        }
                    }
                    assert_eq!(body_offset, tag.body.len());
                    sounds.push((read_le_u16(tag.body, 0), transitions));
                },
                SWF_TAG_DEFINE_SPRITE => {
                    collect_swf_button_sound_infos(tag.body, 4, sounds);
                },
                SWF_TAG_END => break,
                _ => {},
            }
        }
    }

    fn collect_root_display_list_events(
        bytes: &[u8],
        offset: usize,
    ) -> (Vec<SwfRootPlacement>, Vec<SwfRootRemoval>) {
        let mut frame = 1;
        let mut placements = Vec::new();
        let mut removals = Vec::new();
        for tag in collect_swf_tags(bytes, offset) {
            match tag.code {
                SWF_TAG_PLACE_OBJECT2 => {
                    let flags = tag.body[0];
                    let depth = read_le_u16(tag.body, 1);
                    let character_id = if flags & 0x02 != 0 {
                        Some(read_le_u16(tag.body, 3))
                    } else {
                        None
                    };
                    placements.push(SwfRootPlacement {
                        frame,
                        depth,
                        character_id,
                    });
                },
                SWF_TAG_REMOVE_OBJECT2 => {
                    removals.push(SwfRootRemoval {
                        frame,
                        depth: read_le_u16(tag.body, 0),
                    });
                },
                SWF_TAG_SHOW_FRAME => frame += 1,
                SWF_TAG_END => break,
                _ => {},
            }
        }
        (placements, removals)
    }

    fn read_swf_button_sounds(body: &[u8]) -> SwfButtonSounds {
        let mut offset = 2;
        let mut sound_ids = [0; 4];
        for sound_id in &mut sound_ids {
            *sound_id = read_le_u16(body, offset);
            offset += 2;
            if *sound_id != 0 {
                offset = skip_swf_sound_info(body, offset);
            }
        }
        SwfButtonSounds {
            button_id: read_le_u16(body, 0),
            sound_ids,
        }
    }

    fn skip_swf_sound_info(body: &[u8], offset: usize) -> usize {
        read_swf_sound_info(body, offset).1
    }

    fn read_swf_sound_info(body: &[u8], mut offset: usize) -> (SwfSoundInfo, usize) {
        let flags = body[offset];
        offset += 1;
        let mut sound_info = SwfSoundInfo {
            in_point: None,
            out_point: None,
            loop_count: None,
            envelope: Vec::new(),
        };
        if flags & 0x01 != 0 {
            sound_info.in_point = Some(read_le_u32(body, offset));
            offset += 4;
        }
        if flags & 0x02 != 0 {
            sound_info.out_point = Some(read_le_u32(body, offset));
            offset += 4;
        }
        if flags & 0x04 != 0 {
            sound_info.loop_count = Some(read_le_u16(body, offset));
            offset += 2;
        }
        if flags & 0x08 != 0 {
            let envelope_points = usize::from(body[offset]);
            offset += 1;
            for _ in 0..envelope_points {
                sound_info.envelope.push((
                    read_le_u32(body, offset),
                    read_le_u16(body, offset + 4),
                    read_le_u16(body, offset + 6),
                ));
                offset += 8;
            }
        }
        (sound_info, offset)
    }

    fn assert_text_visual(actual: SwfTextVisual, x: f32, baseline_y: f32, font_size: u16) {
        assert_close(actual.x, x);
        assert_close(actual.baseline_y, baseline_y);
        assert_eq!(actual.font_size, font_size);
    }

    fn assert_line_visual(actual: SwfLineVisual, x: f32, baseline_y: f32, font_size: u16) {
        assert_close(actual.x, x);
        assert_close(actual.baseline_y, baseline_y);
        assert_eq!(actual.font_size, font_size);
    }

    fn assert_right_text_visual(
        actual: SwfRightTextVisual,
        right_x: f32,
        baseline_y: f32,
        font_size: u16,
    ) {
        assert_close(actual.right_x, right_x);
        assert_close(actual.baseline_y, baseline_y);
        assert_eq!(actual.font_size, font_size);
    }

    #[allow(clippy::too_many_arguments)]
    fn assert_chrome_text_definition(
        actual: &chrome_texts::ChromeTextDefinition,
        text: &str,
        define_text_id: u16,
        font_ids: &[u16],
        color_rgb: [u8; 3],
        bounds_centipx: [i16; 4],
        contour_count: usize,
        edge_count: usize,
        flattened_point_count: usize,
    ) {
        assert_eq!(actual.text, text);
        assert_eq!(actual.define_text_id, define_text_id);
        assert_eq!(actual.font_ids, font_ids);
        assert_eq!(actual.color_rgb, color_rgb);
        assert_eq!(actual.bounds_centipx, bounds_centipx);
        assert_eq!(actual.contours.len(), contour_count);
        assert_eq!(
            actual
                .contours
                .iter()
                .map(|contour| contour.segments.len())
                .sum::<usize>(),
            edge_count
        );
        assert_eq!(
            chrome_text_flattened_contours(actual)
                .iter()
                .map(Vec::len)
                .sum::<usize>(),
            flattened_point_count
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn assert_announce_text_definition(
        actual: &announce_texts::AnnounceTextDefinition,
        text: &str,
        define_text_id: u16,
        font_ids: &[u16],
        color_rgb: [u8; 3],
        bounds_centipx: [i16; 4],
        contour_count: usize,
        edge_count: usize,
        flattened_point_count: usize,
    ) {
        assert_eq!(actual.text, text);
        assert_eq!(actual.define_text_id, define_text_id);
        assert_eq!(actual.font_ids, font_ids);
        assert_eq!(actual.color_rgb, color_rgb);
        assert_eq!(actual.bounds_centipx, bounds_centipx);
        assert_eq!(actual.contours.len(), contour_count);
        assert_eq!(
            actual
                .contours
                .iter()
                .map(|contour| contour.segments.len())
                .sum::<usize>(),
            edge_count
        );
        assert_eq!(
            announce_text_flattened_contours(actual)
                .iter()
                .map(Vec::len)
                .sum::<usize>(),
            flattened_point_count
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn assert_round_number_definition(
        actual: &round_number_texts::RoundNumberDefinition,
        text: &str,
        define_edit_text_id: u16,
        font_id: u16,
        color_rgb: [u8; 3],
        bounds_centipx: [i16; 4],
        contour_count: usize,
        edge_count: usize,
        flattened_point_count: usize,
    ) {
        assert_eq!(actual.text, text);
        assert_eq!(actual.define_edit_text_id, define_edit_text_id);
        assert_eq!(actual.font_id, font_id);
        assert_eq!(actual.color_rgb, color_rgb);
        assert_eq!(actual.bounds_centipx, bounds_centipx);
        assert_eq!(actual.contours.len(), contour_count);
        assert_eq!(
            actual
                .contours
                .iter()
                .map(|contour| contour.segments.len())
                .sum::<usize>(),
            edge_count
        );
        assert_eq!(
            round_number_text_flattened_contours(actual)
                .iter()
                .map(Vec::len)
                .sum::<usize>(),
            flattened_point_count
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn assert_counter_digit_definition(
        actual: &counter_digit_texts::CounterDigitDefinition,
        text: &str,
        font_id: u16,
        color_rgb: [u8; 3],
        advance_centipx: i16,
        bounds_centipx: [i16; 4],
        contour_count: usize,
        edge_count: usize,
        flattened_point_count: usize,
    ) {
        assert_eq!(actual.text, text);
        assert_eq!(actual.font_id, font_id);
        assert_eq!(actual.color_rgb, color_rgb);
        assert_eq!(actual.advance_centipx, advance_centipx);
        assert_eq!(actual.bounds_centipx, bounds_centipx);
        assert_eq!(actual.contours.len(), contour_count);
        assert_eq!(
            actual
                .contours
                .iter()
                .map(|contour| contour.segments.len())
                .sum::<usize>(),
            edge_count
        );
        assert_eq!(
            counter_digit_flattened_contours(actual)
                .iter()
                .map(Vec::len)
                .sum::<usize>(),
            flattened_point_count
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn assert_sponsor_logo_text_definition(
        actual: &sponsor_logo_texts::SponsorLogoTextDefinition,
        text: &str,
        define_text_id: u16,
        font_ids: &[u16],
        color_rgb: [u8; 3],
        bounds_centipx: [i16; 4],
        contour_count: usize,
        edge_count: usize,
        flattened_point_count: usize,
    ) {
        assert_eq!(actual.text, text);
        assert_eq!(actual.define_text_id, define_text_id);
        assert_eq!(actual.font_ids, font_ids);
        assert_eq!(actual.color_rgb, color_rgb);
        assert_eq!(actual.bounds_centipx, bounds_centipx);
        assert_eq!(actual.contours.len(), contour_count);
        assert_eq!(
            actual
                .contours
                .iter()
                .map(|contour| contour.segments.len())
                .sum::<usize>(),
            edge_count
        );
        assert_eq!(
            sponsor_logo_text_flattened_contours(actual)
                .iter()
                .map(Vec::len)
                .sum::<usize>(),
            flattened_point_count
        );
    }

    fn score_marker_visuals(score: i32, side: Side) -> Vec<ScoreMarkerVisual> {
        let mut visuals = Vec::new();
        for_each_score_marker(score, side, |visual| visuals.push(visual));
        visuals
    }

    fn score_meter_base_marker_visuals(side: Side) -> Vec<ScoreMarkerVisual> {
        let mut visuals = Vec::new();
        for_each_score_meter_base_marker(side, |visual| visuals.push(visual));
        visuals
    }

    fn final_score_overlay_visuals(side: Side, tick: u64) -> Vec<FinalScoreOverlayVisual> {
        let mut visuals = Vec::new();
        for_each_final_score_overlay(side, tick, |visual| visuals.push(visual));
        visuals
    }

    fn match_pip_visuals(count: u8, side: Side) -> Vec<MatchPipVisual> {
        let mut visuals = Vec::new();
        for_each_match_pip(count, side, |visual| visuals.push(visual));
        visuals
    }

    #[test]
    fn window_config_preserves_fixed_swf_stage() {
        let conf = window_conf();
        assert_eq!(conf.window_width, 550);
        assert_eq!(conf.window_height, 400);
        assert!(!conf.high_dpi);
        assert!(!conf.window_resizable);
        assert_eq!(conf.platform.swap_interval, Some(1));
        assert_eq!(
            conf.platform.apple_gfx_api,
            miniquad::conf::AppleGfxApi::OpenGl
        );
    }

    #[test]
    fn frame_time_step_is_capped_for_fixed_tick_accumulator() {
        assert_close_f64(MAX_FRAME_TIME, 0.25);
        assert_close_f64(capped_frame_time(1.0 / 60.0), f64::from(1.0_f32 / 60.0));
        assert_close_f64(capped_frame_time(1.5), MAX_FRAME_TIME);
        assert_close_f64(capped_frame_time(0.0), 0.0);
        assert_close_f64(capped_frame_time(-1.0), 0.0);
        assert_close_f64(capped_frame_time(f32::NAN), 0.0);
        assert_close_f64(capped_frame_time(f32::INFINITY), 0.0);
    }

    #[test]
    fn player_key_bindings_match_swf_sprite_132() {
        assert_eq!(BLUE_UP_KEY, KeyCode::Up);
        assert_eq!(BLUE_DOWN_KEY, KeyCode::Down);
        assert_eq!(BLUE_FIRE_KEY, KeyCode::Left);
        assert_eq!(RED_UP_KEY, KeyCode::W);
        assert_eq!(RED_DOWN_KEY, KeyCode::S);
        assert_eq!(RED_FIRE_KEY, KeyCode::D);
    }

    #[test]
    fn menu_values_use_decoded_swf_text_records() {
        assert_eq!(menu_match_type_value(1), "single match");
        assert_eq!(menu_match_type_value(3), "best of 3");
        assert_eq!(menu_match_type_value(5), "best of 5");
        assert_eq!(menu_match_type_value(7), "best of 7");
        assert_eq!(menu_gravity_strength_value(GravityStrength::G1), "low");
        assert_eq!(menu_gravity_strength_value(GravityStrength::G2), "medium");
        assert_eq!(menu_gravity_strength_value(GravityStrength::G3), "high");
        assert_eq!(
            menu_gravity_strength_value(GravityStrength::G4),
            "very high"
        );
        assert_eq!(
            menu_gravity_strength_value(GravityStrength::G5),
            "black hole"
        );
        assert_eq!(menu_speed_value(SpeedMode::Normal), "disabled");
        assert_eq!(menu_speed_value(SpeedMode::Fast), "enabled");
        assert_eq!(top_gravity_factor_value(GravityStrength::G1), "10");
        assert_eq!(top_gravity_factor_value(GravityStrength::G2), "17");
        assert_eq!(top_gravity_factor_value(GravityStrength::G3), "50");
        assert_eq!(top_gravity_factor_value(GravityStrength::G4), "120");
        assert_eq!(top_gravity_factor_value(GravityStrength::G5), "210");
    }

    #[test]
    fn menu_values_use_recovered_swf_glyph_contours() {
        assert_eq!(menu_match_type_value_definition(1).define_text_id, 43);
        assert_eq!(menu_match_type_value_definition(3).define_text_id, 44);
        assert_eq!(menu_match_type_value_definition(5).define_text_id, 45);
        assert_eq!(menu_match_type_value_definition(7).define_text_id, 46);
        assert_eq!(
            menu_gravity_strength_value_definition(GravityStrength::G1).define_text_id,
            56
        );
        assert_eq!(
            menu_gravity_strength_value_definition(GravityStrength::G5).define_text_id,
            62
        );
        assert_eq!(
            menu_speed_value_definition(SpeedMode::Normal).define_text_id,
            68
        );
        assert_eq!(
            menu_speed_value_definition(SpeedMode::Fast).define_text_id,
            69
        );

        let values = [
            (
                &menu_value_texts::QUESTION,
                MENU_QUESTION_TEXT,
                "?",
                55,
                &[54][..],
                [255, 255, 255],
                [0.6, 10.5, 3.05, 11.9],
                2,
                26,
                95,
                [260.59, 80.88, 21.35, 19.08],
            ),
            (
                &menu_value_texts::MATCH_SINGLE,
                menu_match_type_value_placement(1),
                "single match",
                43,
                &[26][..],
                [255, 255, 255],
                [19.05, 116.35, 3.2, 18.3],
                15,
                214,
                628,
                [235.80, 200.60, 65.35, 10.14],
            ),
            (
                &menu_value_texts::MATCH_BEST_OF_3,
                menu_match_type_value_placement(3),
                "best of 3",
                44,
                &[26][..],
                [255, 255, 255],
                [33.5, 102.45, 3.2, 15.15],
                10,
                135,
                411,
                [245.51, 200.60, 46.31, 8.03],
            ),
            (
                &menu_value_texts::MATCH_BEST_OF_5,
                menu_match_type_value_placement(5),
                "best of 5",
                45,
                &[26][..],
                [255, 255, 255],
                [33.5, 102.45, 3.2, 15.15],
                10,
                128,
                377,
                [245.51, 200.60, 46.31, 8.03],
            ),
            (
                &menu_value_texts::MATCH_BEST_OF_7,
                menu_match_type_value_placement(7),
                "best of 7",
                46,
                &[26][..],
                [255, 255, 255],
                [33.5, 102.45, 3.2, 15.15],
                10,
                124,
                355,
                [245.51, 200.60, 46.31, 8.03],
            ),
            (
                &menu_value_texts::GRAVITY_LOW,
                menu_gravity_strength_value_placement(GravityStrength::G1),
                "low",
                56,
                &[26][..],
                [255, 255, 255],
                [52.5, 83.55, 3.2, 15.15],
                4,
                34,
                85,
                [258.27, 257.85, 20.85, 8.03],
            ),
            (
                &menu_value_texts::GRAVITY_MEDIUM,
                menu_gravity_strength_value_placement(GravityStrength::G2),
                "medium",
                57,
                &[26][..],
                [255, 255, 255],
                [36.25, 99.75, 3.2, 15.15],
                9,
                114,
                321,
                [247.35, 257.85, 42.65, 8.03],
            ),
            (
                &menu_value_texts::GRAVITY_HIGH,
                menu_gravity_strength_value_placement(GravityStrength::G3),
                "high",
                58,
                &[26][..],
                [255, 255, 255],
                [49.95, 86.0, 3.2, 18.3],
                6,
                86,
                248,
                [256.56, 257.85, 24.21, 10.14],
            ),
            (
                &menu_value_texts::GRAVITY_VERY_HIGH,
                menu_gravity_strength_value_placement(GravityStrength::G4),
                "very high",
                59,
                &[26][..],
                [255, 255, 255],
                [31.25, 103.75, 3.2, 18.3],
                11,
                136,
                373,
                [244.00, 257.85, 48.69, 10.14],
            ),
            (
                &menu_value_texts::GRAVITY_BLACK_HOLE,
                menu_gravity_strength_value_placement(GravityStrength::G5),
                "black hole",
                62,
                &[54, 26][..],
                [255, 255, 255],
                [27.35, 108.5, 3.05, 15.15],
                13,
                129,
                381,
                [241.38, 257.75, 54.50, 8.13],
            ),
            (
                &menu_value_texts::SPEED_DISABLED,
                menu_speed_value_placement(SpeedMode::Normal),
                "disabled",
                68,
                &[26][..],
                [255, 255, 255],
                [34.75, 100.75, 3.2, 15.15],
                14,
                144,
                453,
                [247.64, 314.05, 44.35, 8.03],
            ),
            (
                &menu_value_texts::SPEED_ENABLED,
                menu_speed_value_placement(SpeedMode::Fast),
                "enabled",
                69,
                &[26][..],
                [255, 51, 51],
                [35.9, 99.55, 3.2, 15.15],
                12,
                123,
                381,
                [248.41, 314.05, 42.77, 8.03],
            ),
        ];

        for (
            definition,
            placement,
            expected_text,
            expected_define_text_id,
            expected_font_ids,
            expected_color_rgb,
            expected_bounds,
            expected_contours,
            expected_edges,
            expected_flattened_points,
            expected_stage_bounds,
        ) in values
        {
            assert_eq!(definition.text, expected_text);
            assert_eq!(definition.define_text_id, expected_define_text_id);
            assert_eq!(definition.font_ids, expected_font_ids);
            assert_eq!(definition.color_rgb, expected_color_rgb);

            let bounds = menu_value_local_bounds(definition);
            assert_close(bounds.x_min, expected_bounds[0]);
            assert_close(bounds.x_max, expected_bounds[1]);
            assert_close(bounds.y_min, expected_bounds[2]);
            assert_close(bounds.y_max, expected_bounds[3]);
            assert_rect_visual(
                menu_value_stage_bounds(definition, placement),
                expected_stage_bounds[0],
                expected_stage_bounds[1],
                expected_stage_bounds[2],
                expected_stage_bounds[3],
            );

            let edge_count: usize = definition
                .contours
                .iter()
                .map(|contour| contour.segments.len())
                .sum();
            assert_eq!(definition.contours.len(), expected_contours);
            assert_eq!(edge_count, expected_edges);
            let contours = menu_value_flattened_contours(definition);
            assert_eq!(
                contours.iter().map(Vec::len).sum::<usize>(),
                expected_flattened_points
            );
        }
    }

    #[test]
    fn menu_text_uses_swf_define_text_placements() {
        assert_eq!(MENU_START_BUTTON_DEPTH, 4);
        assert_eq!(MENU_START_LABEL_DEPTH, 6);
        assert_eq!(MENU_GRAVITY_PREVIEW_DEPTH, 7);
        assert_eq!(MENU_MATCH_VALUE_DEPTH, 20);
        assert_eq!(MENU_POLARISATION_BUTTON_DEPTH, 22);
        assert_eq!(MENU_POLARISATION_LABEL_DEPTH, 24);
        assert_eq!(MENU_MATCH_BUTTON_DEPTH, 25);
        assert_eq!(MENU_MATCH_LABEL_DEPTH, 27);
        assert_eq!(MENU_HELP_BUTTON_DEPTH, 28);
        assert_eq!(MENU_HELP_LABEL_DEPTH, 30);
        assert_eq!(MENU_QUESTION_DEPTH, 31);
        assert_eq!(MENU_GRAVITY_VALUE_DEPTH, 32);
        assert_eq!(MENU_GRAVITY_BUTTON_DEPTH, 35);
        assert_eq!(MENU_GRAVITY_LABEL_DEPTH, 37);
        assert_eq!(MENU_SPEED_BUTTON_DEPTH, 38);
        assert_eq!(MENU_SPEED_LABEL_DEPTH, 40);
        assert_eq!(MENU_SPEED_VALUE_DEPTH, 41);
        assert_eq!(MENU_TOP_GRAVITY_FACTOR_DEPTH, 43);
        assert_eq!(MENU_VERSION_DEPTH, 44);
        assert_eq!(MENU_FRAME_MASK_DEPTH, 50);
        assert_eq!(MENU_TOP_TITLE_DEPTH, 51);

        let (help, help_visual) = menu_button_label(MenuAction::Help);
        assert_eq!(help, "how to play");
        assert_text_visual(help_visual, 268.71, 66.57, 11);

        let (polarisation, polarisation_visual) = menu_button_label(MenuAction::Polarisation);
        assert_eq!(polarisation, "gravity mode");
        assert_text_visual(polarisation_visual, 268.55, 123.87, 11);

        let (matches, matches_visual) = menu_button_label(MenuAction::Matches);
        assert_eq!(matches, "match type");
        assert_text_visual(matches_visual, 268.72, 181.17, 11);

        let (gravity, gravity_visual) = menu_button_label(MenuAction::Gravity);
        assert_eq!(gravity, "gravity strenght");
        assert_text_visual(gravity_visual, 268.58, 238.42, 11);

        let (speed, speed_visual) = menu_button_label(MenuAction::Speed);
        assert_eq!(speed, "SpeedGravity");
        assert_text_visual(speed_visual, 268.86, 295.72, 11);

        let (start, start_visual) = menu_button_label(MenuAction::Start);
        assert_eq!(start, "start game");
        assert_text_visual(start_visual, 268.53, 351.72, 11);

        assert_text_visual(MENU_QUESTION_TEXT.visual(), 271.27, 99.74, 26);
        assert_text_visual(menu_match_type_text(1), 268.48, 208.53, 11);
        assert_text_visual(menu_match_type_text(3), 268.66, 208.53, 11);
        assert_text_visual(
            menu_gravity_strength_text(GravityStrength::G1),
            268.70,
            265.78,
            11,
        );
        assert_text_visual(
            menu_gravity_strength_text(GravityStrength::G5),
            268.63,
            265.78,
            11,
        );
        assert_text_visual(menu_speed_text(SpeedMode::Normal), 269.82, 321.98, 11);
        assert_text_visual(menu_speed_text(SpeedMode::Fast), 269.80, 321.98, 11);
        assert_text_visual(HELP_BACK_LABEL_TEXT.visual(), 271.22, 353.70, 13);
    }

    #[test]
    fn menu_button_labels_use_recovered_swf_glyph_contours() {
        let labels = [
            (
                MenuAction::Help,
                &menu_label_texts::HELP,
                "how to play",
                53,
                [31.2, 120.85, 3.2, 18.3],
                13,
                139,
                379,
                [238.60, 58.65, 60.21, 10.14],
            ),
            (
                MenuAction::Polarisation,
                &menu_label_texts::POLARISATION,
                "gravity mode",
                49,
                [25.8, 125.8, 3.2, 18.3],
                17,
                202,
                577,
                [234.98, 115.95, 67.16, 10.14],
            ),
            (
                MenuAction::Matches,
                &menu_label_texts::MATCHES,
                "match type",
                51,
                [32.35, 119.7, 3.2, 18.3],
                12,
                168,
                453,
                [239.38, 173.25, 58.67, 10.14],
            ),
            (
                MenuAction::Gravity,
                &menu_label_texts::GRAVITY,
                "gravity strenght",
                65,
                [15.75, 135.9, 3.2, 18.3],
                20,
                285,
                774,
                [228.23, 230.50, 80.70, 10.14],
            ),
            (
                MenuAction::Speed,
                &menu_label_texts::SPEED,
                "SpeedGravity",
                67,
                [25.1, 126.5, 3.2, 18.3],
                18,
                209,
                605,
                [234.81, 287.80, 68.10, 10.14],
            ),
            (
                MenuAction::Start,
                &menu_label_texts::START,
                "start game",
                40,
                [33.95, 117.55, 4.3, 18.3],
                13,
                200,
                584,
                [240.45, 344.54, 56.15, 9.40],
            ),
        ];

        for (
            action,
            definition,
            expected_text,
            expected_define_text_id,
            expected_bounds,
            expected_contours,
            expected_edges,
            expected_flattened_points,
            expected_stage_bounds,
        ) in labels
        {
            let resolved = menu_label_definition(action);
            assert_eq!(resolved.text, expected_text);
            assert_eq!(resolved.define_text_id, expected_define_text_id);
            assert_eq!(resolved.font_id, menu_label_texts::FONT_ID);
            assert_eq!(resolved.font_id, definition.font_id);
            assert_eq!(resolved.color_rgb, [255, 255, 255]);

            let bounds = menu_label_local_bounds(definition);
            assert_close(bounds.x_min, expected_bounds[0]);
            assert_close(bounds.x_max, expected_bounds[1]);
            assert_close(bounds.y_min, expected_bounds[2]);
            assert_close(bounds.y_max, expected_bounds[3]);
            assert_rect_visual(
                menu_label_stage_bounds(action),
                expected_stage_bounds[0],
                expected_stage_bounds[1],
                expected_stage_bounds[2],
                expected_stage_bounds[3],
            );

            let edge_count: usize = definition
                .contours
                .iter()
                .map(|contour| contour.segments.len())
                .sum();
            assert_eq!(definition.contours.len(), expected_contours);
            assert_eq!(edge_count, expected_edges);
            let contours = menu_label_flattened_contours(definition);
            assert_eq!(
                contours.iter().map(Vec::len).sum::<usize>(),
                expected_flattened_points
            );
        }

        let start_contours = menu_label_flattened_contours(&menu_label_texts::START);
        assert!(menu_label_contains_local(
            &start_contours,
            SwfPoint::new(43.0, 8.0)
        ));
        assert!(!menu_label_contains_local(
            &start_contours,
            SwfPoint::new(75.75, 11.3)
        ));
    }

    #[test]
    fn gravity_preview_rows_match_swf_sprite_42_frames() {
        assert_color(
            GRAVITY_PREVIEW_BLUE,
            Color::new(153.0 / 255.0, 1.0, 1.0, 1.0),
        );
        assert_color(
            GRAVITY_PREVIEW_RED_LEFT,
            Color::new(1.0, 190.0 / 255.0, 115.0 / 255.0, 1.0),
        );
        assert_color(
            GRAVITY_PREVIEW_RED_RIGHT,
            Color::new(1.0, 191.0 / 255.0, 119.0 / 255.0, 1.0),
        );
        assert_eq!(
            placement_constants::GRAVITY_PREVIEW_RED_LEFT_ADD_RGB,
            [255, 190, 115]
        );
        assert_eq!(
            placement_constants::GRAVITY_PREVIEW_RED_RIGHT_ADD_RGB,
            [255, 191, 119]
        );
        assert_color(
            GRAVITY_PREVIEW_ARROW,
            Color::new(1.0, 251.0 / 255.0, 204.0 / 255.0, 1.0),
        );
        assert_close(GRAVITY_PREVIEW_ROOT_X, 254.75);
        assert_close(GRAVITY_PREVIEW_ROOT_Y, 139.85);
        assert_close(GRAVITY_PREVIEW_SPRITE_SCALE, 0.422_256_47);
        assert_eq!(gravity_preview_arrow_shape41::DEFINE_SHAPE_ID, 41);
        assert_eq!(
            gravity_preview_arrow_shape41::BOUNDS_TWIPS,
            [-355, 139, -153, 153]
        );
        assert_eq!(gravity_preview_arrow_shape41::LINE_RGB, [255, 251, 204]);
        assert_close(gravity_preview_arrow_shape41::SHAPE.bounds.x_min, -17.75);
        assert_close(gravity_preview_arrow_shape41::SHAPE.bounds.x_max, 6.95);
        assert_close(gravity_preview_arrow_shape41::SHAPE.bounds.y_min, -7.65);
        assert_close(gravity_preview_arrow_shape41::SHAPE.bounds.y_max, 7.65);
        assert_close(gravity_preview_arrow_shape41::LINE_WIDTH, 2.0);
        assert_eq!(gravity_preview_arrow_shape41::SHAPE.head.points.len(), 2);
        assert_eq!(gravity_preview_arrow_shape41::SHAPE.shaft.points.len(), 1);
        assert_swf_point(gravity_preview_arrow_shaft_start(), -16.75, 0.15);
        assert_swf_point(gravity_preview_arrow_tip(), 5.95, 0.15);
        assert_swf_point(gravity_preview_arrow_head_top(), -0.05, -6.65);
        assert_swf_point(gravity_preview_arrow_head_bottom(), 0.20, 6.65);

        let neutral = gravity_preview_mode_frame(Polarisation::Neutral);
        let opposite = gravity_preview_mode_frame(Polarisation::OppositeRepels);
        let same = gravity_preview_mode_frame(Polarisation::SameRepels);
        let all = gravity_preview_mode_frame(Polarisation::AllRepel);

        assert_eq!(neutral.swf_frame, 2);
        assert_eq!(opposite.swf_frame, 3);
        assert_eq!(same.swf_frame, 4);
        assert_eq!(all.swf_frame, 5);
        assert_eq!(Polarisation::Neutral.swf_value(), 0);
        assert_eq!(Polarisation::OppositeRepels.swf_value(), -1);
        assert_eq!(Polarisation::SameRepels.swf_value(), 1);
        assert_eq!(Polarisation::AllRepel.swf_value(), 2);

        let rows = neutral.rows;
        assert_color(rows[0].left_ball.color, GRAVITY_PREVIEW_BLUE);
        assert_color(rows[0].right_ball.color, GRAVITY_PREVIEW_RED_RIGHT);
        assert!(!rows[0].same_color);

        assert_color(rows[1].left_ball.color, GRAVITY_PREVIEW_RED_LEFT);
        assert_color(rows[1].right_ball.color, GRAVITY_PREVIEW_RED_RIGHT);
        assert!(rows[1].same_color);

        assert_color(rows[2].left_ball.color, GRAVITY_PREVIEW_BLUE);
        assert_color(rows[2].right_ball.color, GRAVITY_PREVIEW_BLUE);
        assert!(rows[2].same_color);

        let repels = |mode: Polarisation| rows.map(|row| mode.reverses_force(row.same_color));
        assert_eq!(repels(Polarisation::Neutral), [false, false, false]);
        assert_eq!(repels(Polarisation::OppositeRepels), [true, false, false]);
        assert_eq!(repels(Polarisation::SameRepels), [false, true, true]);
        assert_eq!(repels(Polarisation::AllRepel), [true, true, true]);

        let neutral_left = gravity_preview_ball_visual(neutral.rows[0].left_ball);
        assert_close(neutral_left.x, 256.56);
        assert_close(neutral_left.y, 139.58);
        assert_close(neutral_left.radius, 3.97);
        assert_close(
            neutral_left.transparent_stop_ratio,
            BALL_GLOW_EDGE_STOP_RATIO,
        );
        assert_color(neutral_left.color, GRAVITY_PREVIEW_BLUE);

        let neutral_right = gravity_preview_ball_visual(neutral.rows[0].right_ball);
        assert_close(neutral_right.x, 275.47);
        assert_close(neutral_right.y, 139.58);
        assert_color(neutral_right.color, GRAVITY_PREVIEW_RED_RIGHT);

        let opposite_left = gravity_preview_ball_visual(opposite.rows[0].left_ball);
        let opposite_right = gravity_preview_ball_visual(opposite.rows[0].right_ball);
        assert_close(opposite_left.x, 260.19);
        assert_close(opposite_right.x, 271.88);
        assert_color(opposite_left.color, GRAVITY_PREVIEW_BLUE);
        assert_color(opposite_right.color, GRAVITY_PREVIEW_RED_RIGHT);

        let same_mid_right = gravity_preview_ball_visual(same.rows[1].right_ball);
        assert_close(same_mid_right.x, 271.88);
        assert_close(same_mid_right.y, 148.08);
        assert_color(same_mid_right.color, GRAVITY_PREVIEW_RED_LEFT);

        let all_left = gravity_preview_ball_visual(all.rows[0].left_ball);
        let all_right = gravity_preview_ball_visual(all.rows[0].right_ball);
        assert_close(all_left.x, 260.02);
        assert_close(all_right.x, 271.91);

        let neutral_left_arrow = gravity_preview_arrow_visual(neutral.rows[0].left_arrow);
        assert_point(neutral_left_arrow.start, 256.72, 139.59);
        assert_point(neutral_left_arrow.end, 263.15, 139.59);
        assert_point(neutral_left_arrow.head_a, 261.45, 137.66);
        assert_point(neutral_left_arrow.head_b, 261.52, 141.43);
        assert_close(neutral_left_arrow.line_width, 0.57);
        assert_color(neutral_left_arrow.color, GRAVITY_PREVIEW_ARROW);

        let opposite_left_arrow = gravity_preview_arrow_visual(opposite.rows[0].left_arrow);
        let opposite_right_arrow = gravity_preview_arrow_visual(opposite.rows[0].right_arrow);
        assert_point(opposite_left_arrow.start, 260.07, 139.59);
        assert_point(opposite_left_arrow.end, 253.63, 139.59);
        assert_point(opposite_left_arrow.head_a, 255.34, 137.66);
        assert_point(opposite_left_arrow.head_b, 255.26, 141.43);
        assert_point(opposite_right_arrow.start, 272.00, 139.59);
        assert_point(opposite_right_arrow.end, 278.43, 139.59);
        assert_point(opposite_right_arrow.head_a, 276.73, 137.66);
        assert_point(opposite_right_arrow.head_b, 276.80, 141.43);
    }

    #[test]
    fn active_ball_and_announce_text_palettes_match_swf_colors() {
        assert_color(
            ball_core_color(Side::Blue),
            Color::new(153.0 / 255.0, 1.0, 1.0, 1.0),
        );
        assert_color(
            ball_core_color(Side::Red),
            Color::new(1.0, 191.0 / 255.0, 119.0 / 255.0, 1.0),
        );
        assert_color(
            BALL_FIRE_CORE,
            Color::new(1.0, 250.0 / 255.0, 17.0 / 255.0, 1.0),
        );
        assert_eq!(placement_constants::BALL_RED_ADD_RGB, [255, 191, 119]);
        assert_eq!(placement_constants::BALL_FIRE_ADD_RGB, [255, 250, 17]);
        assert_eq!(announce_texts::BLUE_WINS.color_rgb, [136, 247, 255]);
        assert_eq!(announce_texts::BLUE_MATCH.color_rgb, [136, 247, 255]);
        assert_eq!(announce_texts::RED_WINS.color_rgb, [255, 178, 51]);
        assert_eq!(announce_texts::RED_MATCH.color_rgb, [255, 178, 51]);
        assert_eq!(announce_texts::ROUND.color_rgb, [255, 255, 255]);
    }

    #[test]
    fn help_text_uses_swf_define_text_placements() {
        assert_text_visual(HELP_TITLE_TEXT.visual(), 272.42, 73.54, 17);
        assert_eq!(
            HELP_BODY_LINES[0],
            "1. Try to shoot as many balls as possible into your opponents goal-line."
        );
        assert_eq!(
            HELP_BODY_LINES[7],
            "and the balls take up speed, when they are reflected by a paddel."
        );
        assert_line_visual(help_body_line_visual(0), 81.75, 105.45, 12);
        assert_line_visual(help_body_line_visual(7), 81.75, 224.45, 12);

        assert_text_visual(HELP_KEY_W_TEXT.visual(), 159.26, 258.32, 14);
        assert_text_visual(HELP_KEY_S_TEXT.visual(), 159.25, 286.32, 14);
        assert_text_visual(HELP_KEY_D_TEXT.visual(), 187.07, 286.32, 14);
        assert_text_visual(HELP_RED_MOVE_UP_TEXT.visual(), 114.53, 259.75, 12);
        assert_text_visual(HELP_RED_SHOOT_TEXT.visual(), 219.90, 288.75, 12);
        assert_text_visual(HELP_BLUE_SHOOT_TEXT.visual(), 328.85, 288.75, 12);
        assert_text_visual(HELP_BLUE_MOVE_DOWN_TEXT.visual(), 442.08, 288.75, 12);
        assert_text_visual(HELP_PLAYER_RED_TEXT.visual(), 173.62, 319.25, 12);
        assert_text_visual(HELP_PLAYER_BLUE_TEXT.visual(), 374.57, 319.25, 12);
        assert_color(
            HELP_RED_TEXT,
            Color::new(1.0, 129.0 / 255.0, 34.0 / 255.0, 1.0),
        );
        assert_color(
            HELP_BLUE_TEXT,
            Color::new(102.0 / 255.0, 197.0 / 255.0, 1.0, 1.0),
        );
    }

    #[test]
    fn help_controls_follow_swf_frame_57_display_list() {
        assert_eq!(HELP_CONTROL_ROOT_FRAME, 57);
        assert_eq!(HELP_VERSION_DEPTH, 44);
        assert_eq!(HELP_ROUNDS_PLAYED_DEPTH, 47);
        assert_eq!(HELP_FRAME_MASK_DEPTH, 50);
        assert_eq!(HELP_TOP_TITLE_DEPTH, 51);
        assert_eq!(HELP_RETAINED_EMPTY_FACTOR_SPRITE_DEPTH, 52);
        assert_eq!(HELP_RETAINED_EMPTY_FACTOR_SPRITE_ID, 83);
        assert_eq!(HELP_RETAINED_EMPTY_FACTOR_CHILD_SPRITE_ID, 82);
        assert_eq!(HELP_LEFT_GOAL_DEPTH, 58);
        assert_eq!(HELP_RIGHT_GOAL_DEPTH, 60);
        assert_eq!(HELP_RIGHT_PADDLE_DEPTH, 62);
        assert_eq!(HELP_LEFT_PADDLE_DEPTH, 63);
        assert_eq!(HELP_RIGHT_PADDLE_GLOW_DEPTH, 64);
        assert_eq!(HELP_LEFT_PADDLE_GLOW_DEPTH, 65);
        assert_eq!(HELP_PANEL_OUTLINE_DEPTH, 66);
        assert_eq!(HELP_FIRST_SIDE_MARKER_DEPTH, 67);
        assert_eq!(HELP_LAST_SIDE_MARKER_DEPTH, 126);
        assert_eq!(HELP_BACK_LINK_DEPTH, 127);
        assert_eq!(HELP_SPONSOR_LOGO_DEPTH, 129);
        assert_eq!(
            HELP_CONTENT_DEPTHS_AND_CHARS,
            [
                (2, 101),
                (3, 102),
                (4, 103),
                (6, 104),
                (7, 105),
                (8, 107),
                (9, 105),
                (10, 108),
                (11, 105),
                (12, 109),
                (13, 105),
                (14, 110),
                (15, 105),
                (16, 110),
                (17, 105),
                (18, 110),
                (19, 111),
                (20, 112),
                (21, 113),
                (22, 114),
                (23, 115),
                (24, 116),
                (25, 117),
                (26, 118),
            ]
        );
        assert_eq!(
            HELP_CONTENT_DISPLAY_LIST,
            [
                HelpContentVisual::Title,
                HelpContentVisual::Body,
                HelpContentVisual::BackButton,
                HelpContentVisual::BackLabel,
                HelpContentVisual::Control(HelpControlVisual::RedUpKeycap),
                HelpContentVisual::Control(HelpControlVisual::RedUpKey),
                HelpContentVisual::Control(HelpControlVisual::RedShootKeycap),
                HelpContentVisual::Control(HelpControlVisual::RedShootKey),
                HelpContentVisual::Control(HelpControlVisual::RedDownKeycap),
                HelpContentVisual::Control(HelpControlVisual::RedDownKey),
                HelpContentVisual::Control(HelpControlVisual::BlueUpKeycap),
                HelpContentVisual::Control(HelpControlVisual::BlueUpArrow),
                HelpContentVisual::Control(HelpControlVisual::BlueShootKeycap),
                HelpContentVisual::Control(HelpControlVisual::BlueShootArrow),
                HelpContentVisual::Control(HelpControlVisual::BlueDownKeycap),
                HelpContentVisual::Control(HelpControlVisual::BlueDownArrow),
                HelpContentVisual::Control(HelpControlVisual::RedMoveUpLabel),
                HelpContentVisual::Control(HelpControlVisual::RedMoveDownLabel),
                HelpContentVisual::Control(HelpControlVisual::RedShootLabel),
                HelpContentVisual::Control(HelpControlVisual::BlueShootLabel),
                HelpContentVisual::Control(HelpControlVisual::BlueMoveUpLabel),
                HelpContentVisual::Control(HelpControlVisual::BlueMoveDownLabel),
                HelpContentVisual::Control(HelpControlVisual::RedPlayerLabel),
                HelpContentVisual::Control(HelpControlVisual::BluePlayerLabel),
            ]
        );
        assert_eq!(
            HELP_CONTROL_DEPTHS_AND_CHARS,
            [
                (7, 105),
                (8, 107),
                (9, 105),
                (10, 108),
                (11, 105),
                (12, 109),
                (13, 105),
                (14, 110),
                (15, 105),
                (16, 110),
                (17, 105),
                (18, 110),
                (19, 111),
                (20, 112),
                (21, 113),
                (22, 114),
                (23, 115),
                (24, 116),
                (25, 117),
                (26, 118),
            ]
        );
        assert_eq!(
            HELP_CONTROL_DISPLAY_LIST,
            [
                HelpControlVisual::RedUpKeycap,
                HelpControlVisual::RedUpKey,
                HelpControlVisual::RedShootKeycap,
                HelpControlVisual::RedShootKey,
                HelpControlVisual::RedDownKeycap,
                HelpControlVisual::RedDownKey,
                HelpControlVisual::BlueUpKeycap,
                HelpControlVisual::BlueUpArrow,
                HelpControlVisual::BlueShootKeycap,
                HelpControlVisual::BlueShootArrow,
                HelpControlVisual::BlueDownKeycap,
                HelpControlVisual::BlueDownArrow,
                HelpControlVisual::RedMoveUpLabel,
                HelpControlVisual::RedMoveDownLabel,
                HelpControlVisual::RedShootLabel,
                HelpControlVisual::BlueShootLabel,
                HelpControlVisual::BlueMoveUpLabel,
                HelpControlVisual::BlueMoveDownLabel,
                HelpControlVisual::RedPlayerLabel,
                HelpControlVisual::BluePlayerLabel,
            ]
        );
    }

    #[test]
    fn help_labels_use_recovered_swf_glyph_contours() {
        assert!(help_label_texts::BODY.text.starts_with(HELP_BODY_LINES[0]));
        assert!(
            help_label_texts::BODY.text.contains(
                "    ...or reject eatch other - that depends on the gravity mode you play."
            )
        );
        assert!(help_label_texts::BODY.text.contains(
            "6. In SpeedGravity-Mode faster balls score more     and the balls take up speed"
        ));

        let labels = [
            (
                &help_label_texts::TITLE,
                HELP_TITLE_TEXT,
                "How to play ?",
                101,
                &[26][..],
                [255, 255, 255],
                [36.85, 113.3, 2.95, 14.25],
                15,
                160,
                439,
                [217.40, 60.80, 110.05, 16.27],
            ),
            (
                &help_label_texts::BODY,
                HELP_BODY_TEXT,
                help_label_texts::BODY.text,
                102,
                &[26][..],
                [255, 255, 255],
                [-114.15, 277.8, 2.95, 133.25],
                559,
                6486,
                19131,
                [81.90, 96.60, 391.95, 130.30],
            ),
            (
                &help_label_texts::BACK,
                HELP_BACK_LABEL_TEXT,
                "back",
                104,
                &[26][..],
                [255, 255, 255],
                [56.4, 95.6, 3.2, 15.15],
                6,
                74,
                209,
                [255.35, 344.06, 31.73, 9.76],
            ),
            (
                &help_label_texts::KEY_W,
                HELP_KEY_W_TEXT,
                "w",
                107,
                &[106][..],
                [0, 36, 85],
                [65.55, 89.75, 5.05, 16.3],
                1,
                14,
                14,
                [149.85, 249.58, 18.81, 8.74],
            ),
            (
                &help_label_texts::KEY_D,
                HELP_KEY_D_TEXT,
                "d",
                108,
                &[106][..],
                [0, 36, 85],
                [67.6, 88.25, 3.4, 16.5],
                2,
                22,
                73,
                [179.05, 276.29, 16.05, 10.18],
            ),
            (
                &help_label_texts::KEY_S,
                HELP_KEY_S_TEXT,
                "s",
                109,
                &[106][..],
                [0, 36, 85],
                [68.05, 87.75, 5.05, 16.5],
                1,
                33,
                117,
                [151.60, 277.58, 15.31, 8.90],
            ),
            (
                &help_label_texts::RED_MOVE_UP,
                HELP_RED_MOVE_UP_TEXT,
                "move up",
                111,
                &[26][..],
                [255, 129, 34],
                [49.95, 100.1, 5.25, 14.25],
                9,
                99,
                288,
                [89.45, 253.20, 50.15, 9.00],
            ),
            (
                &help_label_texts::RED_MOVE_DOWN,
                HELP_RED_MOVE_DOWN_TEXT,
                "move down",
                112,
                &[26][..],
                [255, 129, 34],
                [42.25, 107.8, 2.95, 11.9],
                12,
                123,
                348,
                [74.35, 279.90, 65.55, 8.95],
            ),
            (
                &help_label_texts::RED_SHOOT,
                HELP_RED_SHOOT_TEXT,
                "shoot",
                113,
                &[26][..],
                [255, 129, 34],
                [57.85, 91.75, 2.95, 11.9],
                7,
                84,
                264,
                [202.95, 279.90, 33.90, 8.95],
            ),
            (
                &help_label_texts::BLUE_SHOOT,
                HELP_BLUE_SHOOT_TEXT,
                "shoot",
                114,
                &[26][..],
                [102, 197, 255],
                [57.85, 91.75, 2.95, 11.9],
                7,
                84,
                264,
                [311.90, 279.90, 33.90, 8.95],
            ),
            (
                &help_label_texts::BLUE_MOVE_UP,
                HELP_BLUE_MOVE_UP_TEXT,
                "move up",
                115,
                &[26][..],
                [102, 197, 255],
                [49.95, 100.1, 5.25, 14.25],
                9,
                99,
                288,
                [409.60, 253.20, 50.15, 9.00],
            ),
            (
                &help_label_texts::BLUE_MOVE_DOWN,
                HELP_BLUE_MOVE_DOWN_TEXT,
                "move down",
                116,
                &[26][..],
                [102, 197, 255],
                [42.25, 107.8, 2.95, 11.9],
                12,
                123,
                348,
                [409.30, 279.90, 65.55, 8.95],
            ),
            (
                &help_label_texts::PLAYER_RED,
                HELP_PLAYER_RED_TEXT,
                "Player Red",
                117,
                &[26][..],
                [255, 129, 34],
                [44.4, 105.75, 2.95, 14.25],
                15,
                142,
                397,
                [142.95, 310.40, 61.35, 11.30],
            ),
            (
                &help_label_texts::PLAYER_BLUE,
                HELP_PLAYER_BLUE_TEXT,
                "Player Blue",
                118,
                &[26][..],
                [102, 197, 255],
                [42.55, 107.6, 2.95, 14.25],
                16,
                150,
                420,
                [342.05, 310.40, 65.05, 11.30],
            ),
        ];

        for (
            definition,
            placement,
            expected_text,
            expected_define_text_id,
            expected_font_ids,
            expected_color_rgb,
            expected_bounds,
            expected_contours,
            expected_edges,
            expected_flattened_points,
            expected_stage_bounds,
        ) in labels
        {
            assert_eq!(definition.text, expected_text);
            assert_eq!(definition.define_text_id, expected_define_text_id);
            assert_eq!(definition.font_ids, expected_font_ids);
            assert_eq!(definition.color_rgb, expected_color_rgb);

            let bounds = help_label_local_bounds(definition);
            assert_close(bounds.x_min, expected_bounds[0]);
            assert_close(bounds.x_max, expected_bounds[1]);
            assert_close(bounds.y_min, expected_bounds[2]);
            assert_close(bounds.y_max, expected_bounds[3]);
            assert_rect_visual(
                help_label_stage_bounds(definition, placement),
                expected_stage_bounds[0],
                expected_stage_bounds[1],
                expected_stage_bounds[2],
                expected_stage_bounds[3],
            );

            let edge_count: usize = definition
                .contours
                .iter()
                .map(|contour| contour.segments.len())
                .sum();
            assert_eq!(definition.contours.len(), expected_contours);
            assert_eq!(edge_count, expected_edges);
            let contours = help_label_flattened_contours(definition);
            assert_eq!(
                contours.iter().map(Vec::len).sum::<usize>(),
                expected_flattened_points
            );
        }
    }

    #[test]
    fn optimized_help_label_masks_match_naive_even_odd_fill() {
        for (definition, placement) in [
            (&help_label_texts::TITLE, HELP_TITLE_TEXT),
            (&help_label_texts::BACK, HELP_BACK_LABEL_TEXT),
            (&help_label_texts::KEY_W, HELP_KEY_W_TEXT),
            (&help_label_texts::RED_MOVE_DOWN, HELP_RED_MOVE_DOWN_TEXT),
        ] {
            assert_help_label_masks_match_naive(definition, placement, HelpMaskCheck::Exhaustive);
        }

        assert_help_label_masks_match_naive(
            &help_label_texts::BODY,
            HELP_BODY_TEXT,
            HelpMaskCheck::Grid {
                x_step: 11,
                y_step: 7,
            },
        );
    }

    #[derive(Debug, Clone, Copy)]
    enum HelpMaskCheck {
        Exhaustive,
        Grid { x_step: usize, y_step: usize },
    }

    fn assert_help_label_masks_match_naive(
        definition: &help_label_texts::HelpLabelDefinition,
        placement: SwfTextPlacement,
        check: HelpMaskCheck,
    ) {
        let bounds = help_label_stage_bounds(definition, placement);
        let width = bounds.w.ceil().max(1.0) as u16;
        let height = bounds.h.ceil().max(1.0) as u16;
        let optimized = help_label_sample_masks(definition, bounds, placement, width, height);
        let contours = help_label_flattened_contours(definition);
        let x_step = match check {
            HelpMaskCheck::Exhaustive => 1,
            HelpMaskCheck::Grid { x_step, .. } => x_step,
        };
        let y_step = match check {
            HelpMaskCheck::Exhaustive => 1,
            HelpMaskCheck::Grid { y_step, .. } => y_step,
        };

        for y in (0..usize::from(height)).step_by(y_step) {
            for x in (0..usize::from(width)).step_by(x_step) {
                let optimized_mask = optimized[y * usize::from(width) + x];
                let naive_mask =
                    naive_help_label_sample_mask(&contours, bounds, placement, width, height, x, y);
                assert_eq!(
                    optimized_mask, naive_mask,
                    "mask mismatch for DefineText {} at pixel ({x}, {y})",
                    definition.define_text_id
                );
            }
        }
    }

    fn naive_help_label_sample_mask(
        contours: &[Vec<SwfPoint>],
        bounds: RectVisual,
        placement: SwfTextPlacement,
        width: u16,
        height: u16,
        x: usize,
        y: usize,
    ) -> u32 {
        let mut mask = 0_u32;
        for sub_y in 0..HELP_LABEL_TEXTURE_SUPERSAMPLE {
            for sub_x in 0..HELP_LABEL_TEXTURE_SUPERSAMPLE {
                let local = help_label_texture_sample_local(
                    bounds,
                    placement,
                    (width, height),
                    (x as u16, y as u16),
                    (sub_x, sub_y),
                );
                let filled = contours
                    .iter()
                    .filter(|contour| point_in_polygon(local, contour))
                    .count()
                    % 2
                    == 1;
                if filled {
                    let bit = u32::from(sub_y * HELP_LABEL_TEXTURE_SUPERSAMPLE + sub_x);
                    mask |= 1 << bit;
                }
            }
        }
        mask
    }

    #[test]
    fn help_keycap_and_arrow_shapes_match_swf_frame_57() {
        assert_eq!(help_control_shapes::KEYCAP_DEFINE_SHAPE_ID, 105);
        assert_eq!(
            help_control_shapes::KEYCAP_BOUNDS_TWIPS,
            [-312, 313, -312, 313],
        );
        assert_close(help_control_shapes::KEYCAP_SHAPE.bounds.x_min, -15.6);
        assert_close(help_control_shapes::KEYCAP_SHAPE.bounds.x_max, 15.65);
        assert_close(help_control_shapes::KEYCAP_SHAPE.bounds.y_min, -15.6);
        assert_close(help_control_shapes::KEYCAP_SHAPE.bounds.y_max, 15.65);
        assert_eq!(help_control_shapes::KEYCAP_FILL_RGB, [207, 192, 150]);
        assert_eq!(help_control_shapes::KEYCAP_SHADOW_RGB, [163, 150, 118]);
        assert_eq!(help_control_shapes::KEYCAP_LINE_RGB, [255, 238, 187]);
        assert_close(HELP_KEYCAP_SCALE, 0.777_298);
        assert_close(help_control_shapes::KEYCAP_LINE_WIDTH, 2.0);

        let keycap = keycap_visual(154.1, 256.15);
        assert_rect_visual(keycap.bounds, 141.97, 244.02, 24.29, 24.29);
        assert_swf_point(keycap.center, 154.10, 256.15);
        assert_close(keycap.scale, 0.777_298);
        assert_close(keycap.line_width, 1.55);
        assert_color(
            keycap.fill,
            Color::new(207.0 / 255.0, 192.0 / 255.0, 150.0 / 255.0, 1.0),
        );
        assert_color(
            keycap.shadow,
            Color::new(163.0 / 255.0, 150.0 / 255.0, 118.0 / 255.0, 1.0),
        );
        assert_color(
            keycap.outline,
            Color::new(1.0, 238.0 / 255.0, 187.0 / 255.0, 1.0),
        );

        let top_edge = keycap_top_edge_points(keycap);
        assert_eq!(top_edge.len(), 4);
        assert_swf_point(top_edge[0], 162.46, 244.88);
        assert_swf_point(top_edge[3], 148.11, 244.80);

        let inner_edge = keycap_inner_edge_points(keycap);
        assert_eq!(inner_edge.len(), 36);
        assert_swf_point(inner_edge[0], 148.11, 244.80);
        assert_swf_point(inner_edge[17], 148.19, 264.23);
        assert_swf_point(inner_edge[18], 161.80, 264.23);
        assert_swf_point(inner_edge[35], 162.46, 244.88);

        let outer_edge = keycap_outer_edge_points(keycap);
        assert_eq!(outer_edge.len(), 37);
        assert_swf_point(outer_edge[0], 162.46, 244.88);
        assert_swf_point(outer_edge[17], 161.60, 267.54);
        assert_swf_point(outer_edge[18], 146.64, 267.54);
        assert_swf_point(outer_edge[36], 148.11, 244.80);

        assert_eq!(keycap_inner_fill_points(keycap).len(), 38);
        assert_eq!(keycap_outer_fill_points(keycap).len(), 71);

        assert_eq!(help_control_shapes::ARROW_DEFINE_SHAPE_ID, 110);
        assert_eq!(
            help_control_shapes::ARROW_BOUNDS_TWIPS,
            [-205, 205, -212, 213],
        );
        assert_close(help_control_shapes::ARROW_SHAPE.bounds.x_min, -10.25);
        assert_close(help_control_shapes::ARROW_SHAPE.bounds.x_max, 10.25);
        assert_close(help_control_shapes::ARROW_SHAPE.bounds.y_min, -10.6);
        assert_close(help_control_shapes::ARROW_SHAPE.bounds.y_max, 10.65);
        assert_eq!(help_control_shapes::ARROW_FILL_RGB, [0, 36, 85]);
        assert_close(HELP_ARROW_SCALE, 0.587_722_8);
        assert_close(HELP_ARROW_OFFSET_X, 0.7);
        assert_close(HELP_ARROW_OFFSET_Y, -1.75);

        let up = arrow_glyph_visual(390.1, 256.15, ArrowDir::Up);
        assert_eq!(up.points.len(), 23);
        assert_swf_point(up.center, 390.80, 254.40);
        assert_swf_point(up.points[0], 396.82, 253.90);
        assert_swf_point(up.points[1], 393.30, 253.90);
        assert_swf_point(up.points[2], 393.30, 258.90);
        assert_swf_point(up.points[21], 384.78, 253.90);
        assert_swf_point(up.points[22], 390.65, 248.17);
        assert_color(up.color, swf_rgb_array(help_control_shapes::ARROW_FILL_RGB));

        let left = arrow_glyph_visual(362.3, 284.1, ArrowDir::Left);
        assert_eq!(left.points.len(), 23);
        assert_swf_point(left.center, 363.00, 282.35);
        assert_swf_point(left.points[0], 362.50, 276.33);
        assert_swf_point(left.points[1], 362.50, 279.85);
        assert_swf_point(left.points[2], 367.50, 279.85);
        assert_swf_point(left.points[21], 362.50, 288.37);
        assert_swf_point(left.points[22], 356.77, 282.50);

        let down = arrow_glyph_visual(390.1, 284.1, ArrowDir::Down);
        assert_eq!(down.points.len(), 23);
        assert_swf_point(down.center, 390.80, 282.35);
        assert_swf_point(down.points[0], 396.82, 282.85);
        assert_swf_point(down.points[1], 393.30, 282.85);
        assert_swf_point(down.points[2], 393.30, 277.85);
        assert_swf_point(down.points[21], 384.78, 282.85);
        assert_swf_point(down.points[22], 390.65, 288.58);
    }

    #[test]
    fn top_title_and_footer_use_swf_root_frame_placements() {
        assert_eq!(top_title_text81::TEXT, "Gravity");
        assert_eq!(top_title_text81::FONT_ID, 54);
        assert_eq!(top_title_text81::DEFINE_TEXT_ID, 81);
        assert_text_visual(TOP_TITLE_TEXT.visual(), 271.79, 24.70, 20);
        let title_bounds = top_title_local_bounds();
        assert_close(title_bounds.x_min, 0.5);
        assert_close(title_bounds.x_max, 46.05);
        assert_close(title_bounds.y_min, 3.05);
        assert_close(title_bounds.y_max, 14.25);
        let title_stage_bounds = top_title_stage_bounds();
        assert_rect_visual(title_stage_bounds, 233.10, 9.83, 77.38, 19.03);
        let contour_edge_count: usize = top_title_text81::CONTOURS
            .iter()
            .map(|contour| contour.segments.len())
            .sum();
        assert_eq!(top_title_text81::CONTOURS.len(), 9);
        assert_eq!(contour_edge_count, 105);
        let title_contours = top_title_flattened_contours();
        assert_eq!(title_contours.iter().map(Vec::len).sum::<usize>(), 267);
        assert!(top_title_contains_local(
            &title_contours,
            SwfPoint::new(9.0, 8.0)
        ));
        assert!(!top_title_contains_local(
            &title_contours,
            SwfPoint::new(13.0, 8.0)
        ));
        assert_line_visual(TOP_GRAVITY_FACTOR_TEXT, 345.0, 18.05, 12);
        assert_eq!(TOP_GRAVITY_FACTOR_DEFINE_EDIT_TEXT_ID, 72);
        assert_eq!(TOP_GRAVITY_FACTOR_FONT_ID, 71);
        assert_eq!(TOP_GRAVITY_FACTOR_FONT_NAME, "Times New Roman");
        assert_eq!(
            TOP_GRAVITY_FACTOR_DEVICE_FALLBACK_FONT_NAME,
            "Liberation Serif"
        );
        assert_eq!(TOP_GRAVITY_FACTOR_EMBEDDED_GLYPHS, 0);
        assert!(
            TIMES_NEW_ROMAN_SYSTEM_FONT_PATHS
                .iter()
                .any(|path| path.contains("times") || path.contains("Times New Roman"))
        );
        #[cfg(all(unix, not(target_os = "macos")))]
        assert!(
            TIMES_NEW_ROMAN_SYSTEM_FONT_PATHS
                .iter()
                .any(|path| path.contains("LiberationSerif") || path.contains("Caladea"))
        );
        assert_text_visual(ROUNDS_PLAYED_LABEL_TEXT.visual(), 250.45, 373.15, 9);
        assert_eq!(rounds_played_text77::TEXT, "rounds played:");
        assert_eq!(rounds_played_text77::FONT_ID, 26);
        assert_eq!(rounds_played_text77::DEFINE_TEXT_ID, 77);
        let rounds_label_bounds = rounds_played_label_local_bounds();
        assert_close(rounds_label_bounds.x_min, 33.25);
        assert_close(rounds_label_bounds.x_max, 117.05);
        assert_close(rounds_label_bounds.y_min, 2.95);
        assert_close(rounds_label_bounds.y_max, 14.25);
        let rounds_label_stage_bounds = rounds_played_label_stage_bounds();
        assert_rect_visual(rounds_label_stage_bounds, 219.20, 366.55, 62.50, 8.43);
        let rounds_label_edge_count: usize = rounds_played_text77::CONTOURS
            .iter()
            .map(|contour| contour.segments.len())
            .sum();
        assert_eq!(rounds_played_text77::CONTOURS.len(), 20);
        assert_eq!(rounds_label_edge_count, 211);
        let rounds_label_contours = rounds_played_label_flattened_contours();
        assert_eq!(
            rounds_label_contours.iter().map(Vec::len).sum::<usize>(),
            646
        );
        assert!(rounds_played_label_contains_local(
            &rounds_label_contours,
            SwfPoint::new(62.0, 8.0)
        ));
        assert!(!rounds_played_label_contains_local(
            &rounds_label_contours,
            SwfPoint::new(72.0, 8.0)
        ));
        assert_right_text_visual(ROUNDS_PLAYED_VALUE_TEXT, 315.46, 373.15, 9);
        assert_text_visual(MENU_VERSION_TEXT.visual(), 276.98, 386.90, 9);
        assert_chrome_text_definition(
            &chrome_texts::VERSION,
            "version 1.33 - (c) by :neokolor 2001-2 and Sir Isaac Newton 1686",
            73,
            &[26],
            [140, 140, 176],
            [-10105, 25035, 295, 1430],
            75,
            783,
            2379,
        );
        assert_rect_visual(
            chrome_text_stage_bounds(&chrome_texts::VERSION, MENU_VERSION_TEXT),
            145.93,
            380.30,
            262.09,
            8.47,
        );
    }

    #[test]
    fn rounds_played_value_uses_recovered_font26_counter_digits_when_embedded() {
        assert_eq!(ROUNDS_PLAYED_DEFINE_EDIT_TEXT_ID, 78);
        assert_eq!(ROUNDS_PLAYED_CLIP_FRAME, 56);
        assert_eq!(ROUNDS_PLAYED_CLIP_DEPTH, 47);
        assert_eq!(ROUNDS_PLAYED_CLIP_ID, 79);
        assert_eq!(ROUNDS_PLAYED_HELP_CLIP_ID, 119);
        assert_eq!(SWF_VISIBLE_PROPERTY_INDEX, 7);
        assert!(rounds_played_clip_visible(Screen::Menu, false));
        assert!(!rounds_played_clip_visible(Screen::Menu, true));
        assert!(rounds_played_clip_visible(Screen::Help, false));
        assert!(rounds_played_clip_visible(Screen::Help, true));
        assert!(!rounds_played_clip_visible(Screen::Startup, false));
        assert!(!rounds_played_clip_visible(Screen::Playing, false));
        assert!(version_footer_visible(Screen::Menu));
        assert!(version_footer_visible(Screen::Help));
        assert!(!version_footer_visible(Screen::Startup));
        assert!(!version_footer_visible(Screen::Playing));
        assert!(sponsor_logo_visible(Screen::Menu));
        assert!(sponsor_logo_visible(Screen::Help));
        assert!(!sponsor_logo_visible(Screen::Startup));
        assert!(!sponsor_logo_visible(Screen::Playing));
        assert_eq!(COUNTER_FONT_ID, 26);
        assert_eq!(COUNTER_FONT_NAME, "Trebuchet MS");
        assert_eq!(COUNTER_EMBEDDED_DIGITS, "012345678");
        assert_eq!(COUNTER_MISSING_DEVICE_DIGIT, '9');
        assert_eq!(COUNTER_DEVICE_FONT_NAME, "Trebuchet MS");
        assert_eq!(COUNTER_DEVICE_FALLBACK_FONT_NAME, "Liberation Sans");
        assert!(
            TREBUCHET_SYSTEM_FONT_PATHS
                .iter()
                .any(|path| path.contains("trebuc") || path.contains("Trebuchet MS"))
        );
        assert!(
            ARIAL_SYSTEM_FONT_PATHS
                .iter()
                .any(|path| path.contains("Arial") || path.contains("arial"))
        );
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let arial_local = ARIAL_SYSTEM_FONT_PATHS
                .iter()
                .position(|path| path.contains("/usr/local") && path.contains("Arial"));
            let arial_substitute = ARIAL_SYSTEM_FONT_PATHS
                .iter()
                .position(|path| path.contains("LiberationSans"));
            assert!(
                matches!((arial_local, arial_substitute), (Some(local), Some(substitute)) if local < substitute)
            );

            let times_local = TIMES_NEW_ROMAN_SYSTEM_FONT_PATHS
                .iter()
                .position(|path| path.contains("/usr/local") && path.contains("Times_New_Roman"));
            let times_substitute = TIMES_NEW_ROMAN_SYSTEM_FONT_PATHS
                .iter()
                .position(|path| path.contains("LiberationSerif") || path.contains("Caladea"));
            assert!(
                matches!((times_local, times_substitute), (Some(local), Some(substitute)) if local < substitute)
            );

            let trebuchet_local = TREBUCHET_SYSTEM_FONT_PATHS
                .iter()
                .position(|path| path.contains("/usr/local") && path.contains("Trebuchet_MS"));
            let trebuchet_substitute = TREBUCHET_SYSTEM_FONT_PATHS
                .iter()
                .position(|path| path.contains("Carlito") || path.contains("LiberationSans"));
            assert!(
                matches!((trebuchet_local, trebuchet_substitute), (Some(local), Some(substitute)) if local < substitute)
            );

            assert!(
                ARIAL_SYSTEM_FONT_PATHS
                    .iter()
                    .any(|path| path.contains("LiberationSans"))
            );
            assert!(
                TREBUCHET_SYSTEM_FONT_PATHS
                    .iter()
                    .any(|path| path.contains("Carlito") || path.contains("LiberationSans"))
            );
        }

        assert_counter_digit_definition(
            &counter_digit_texts::DIGIT_0,
            "0",
            26,
            [140, 140, 176],
            630,
            [38, 594, 307, 1192],
            2,
            15,
            60,
        );
        assert_counter_digit_definition(
            &counter_digit_texts::DIGIT_1,
            "1",
            26,
            [140, 140, 176],
            630,
            [117, 409, 319, 1180],
            1,
            8,
            14,
        );
        assert_counter_digit_definition(
            &counter_digit_texts::DIGIT_2,
            "2",
            26,
            [140, 140, 176],
            630,
            [35, 568, 307, 1180],
            1,
            16,
            43,
        );
        assert_counter_digit_definition(
            &counter_digit_texts::DIGIT_3,
            "3",
            26,
            [140, 140, 176],
            630,
            [64, 552, 307, 1192],
            1,
            23,
            77,
        );
        assert_counter_digit_definition(
            &counter_digit_texts::DIGIT_4,
            "4",
            26,
            [140, 140, 176],
            630,
            [15, 604, 319, 1180],
            2,
            14,
            14,
        );
        assert_counter_digit_definition(
            &counter_digit_texts::DIGIT_5,
            "5",
            26,
            [140, 140, 176],
            630,
            [76, 564, 319, 1192],
            1,
            16,
            43,
        );
        assert_counter_digit_definition(
            &counter_digit_texts::DIGIT_6,
            "6",
            26,
            [140, 140, 176],
            630,
            [49, 580, 307, 1195],
            2,
            17,
            65,
        );
        assert_counter_digit_definition(
            &counter_digit_texts::DIGIT_7,
            "7",
            26,
            [140, 140, 176],
            630,
            [49, 606, 319, 1180],
            1,
            12,
            21,
        );
        assert_counter_digit_definition(
            &counter_digit_texts::DIGIT_8,
            "8",
            26,
            [140, 140, 176],
            630,
            [53, 580, 307, 1192],
            3,
            26,
            104,
        );

        assert_close(
            counter_digit_embedded_text_width("0")
                .expect("0 should have recovered counter digit contours"),
            6.3,
        );
        assert_close(
            counter_digit_embedded_text_width("18")
                .expect("18 should have recovered counter digit contours"),
            12.6,
        );
        assert_close(
            counter_digit_embedded_text_width("808")
                .expect("808 should have recovered counter digit contours"),
            18.9,
        );
        assert_close(
            counter_digit_text_stage_width("18", None),
            12.6 * COUNTER_DIGIT_STAGE_SCALE,
        );
        assert!(counter_digit_definition(COUNTER_MISSING_DEVICE_DIGIT).is_none());
        assert!(counter_digit_embedded_text_width("19").is_none());
        assert_eq!(
            counter_digit_sources("19"),
            [CounterDigitSource::Embedded, CounterDigitSource::Device]
        );
        assert_eq!(
            counter_digit_sources("90"),
            [CounterDigitSource::Device, CounterDigitSource::Embedded]
        );
    }

    #[test]
    fn help_frame_replaces_menu_rounds_played_clip_with_visible_counter_clone() {
        let swf = include_bytes!("../gravity_arcade.swf");
        let (placements, removals) =
            collect_root_display_list_events(swf, swf_first_tag_offset(swf));

        assert!(placements.contains(&SwfRootPlacement {
            frame: ROUNDS_PLAYED_CLIP_FRAME,
            depth: ROUNDS_PLAYED_CLIP_DEPTH,
            character_id: Some(ROUNDS_PLAYED_CLIP_ID),
        }));
        assert!(removals.contains(&SwfRootRemoval {
            frame: 57,
            depth: ROUNDS_PLAYED_CLIP_DEPTH,
        }));
        assert!(placements.contains(&SwfRootPlacement {
            frame: 57,
            depth: ROUNDS_PLAYED_CLIP_DEPTH,
            character_id: Some(ROUNDS_PLAYED_HELP_CLIP_ID),
        }));
        assert!(removals.contains(&SwfRootRemoval {
            frame: 58,
            depth: ROUNDS_PLAYED_CLIP_DEPTH,
        }));
    }

    #[test]
    fn button_shapes_use_swf_shape_22_bounds_and_scaled_lines() {
        assert_eq!(button_shape22::DEFINE_SHAPE_ID, 22);
        assert_eq!(button_shape22::FILL_RGB, [0, 102, 153]);
        assert_eq!(button_shape22::LINE_RGB, [255, 255, 255]);
        assert_eq!(button_shape22::FLATTEN_STEPS, 4);
        assert_close(button_shape22::SHAPE.bounds.x_min, -66.45);
        assert_close(button_shape22::SHAPE.bounds.x_max, 66.5);
        assert_close(button_shape22::SHAPE.bounds.y_min, -13.0);
        assert_close(button_shape22::SHAPE.bounds.y_max, 13.0);
        assert_swf_point(button_shape22::SHAPE.start, 60.5, -12.0);
        assert_close(button_shape22::LINE_WIDTH, 2.0);

        let menu_button = menu_buttons()[0].visual;
        assert_rect(menu_button.rect, 221.37, 54.62, 89.29, 17.46);
        assert_swf_point(menu_button.center, 266.0, 63.35);
        assert_close(menu_button.scale_x, MENU_BUTTON_SCALE);
        assert_close(menu_button.scale_y, MENU_BUTTON_SCALE);
        assert_close(menu_button.line_width, 1.34);
        let menu_points = button_outline_points(menu_button);
        assert_swf_point(
            menu_points[0],
            button_stage_point(menu_button, button_shape22::SHAPE.start).x,
            button_stage_point(menu_button, button_shape22::SHAPE.start).y,
        );
        assert_eq!(menu_points.len(), 36);
        assert_swf_point(
            button_stage_point(menu_button, SwfPoint::new(-65.45, -7.0)),
            222.04,
            58.65,
        );
        assert!(button_shape_contains(menu_button, 266.0, 63.35));
        assert!(menu_button.rect.contains(vec2(221.67, 54.92)));
        assert!(!button_shape_contains(menu_button, 221.67, 54.92));
        assert_color(
            menu_button.fill,
            Color::new(0.0, 102.0 / 255.0, 153.0 / 255.0, 1.0),
        );
        assert_color(menu_button.outline, SWF_WHITE);

        let back = help_back_visual();
        assert_rect(back.rect, 214.21, 338.58, 107.62, 21.23);
        assert_swf_point(back.center, 268.0, 349.2);
        assert_close(back.scale_x, HELP_BACK_BUTTON_SCALE_X);
        assert_close(back.scale_y, HELP_BACK_BUTTON_SCALE_Y);
        assert_close(back.line_width, 1.63);
        assert!(button_shape_contains(back, 268.0, 349.2));
        assert!(back.rect.contains(vec2(214.51, 338.88)));
        assert!(!button_shape_contains(back, 214.51, 338.88));
        assert_color(back.fill, menu_button.fill);
        assert_color(back.outline, menu_button.outline);
    }

    #[test]
    fn button_hover_surfaces_match_swf_define_button_sound_records() {
        let buttons = menu_buttons();
        assert_eq!(
            hovered_button_with_startup_timing(
                Screen::Startup,
                STARTUP_XML_WAIT_TICK + STARTUP_ABORT_BUTTON_FIRST_FRAME - 2,
                &buttons,
                STARTUP_ABORT_BUTTON_X,
                STARTUP_ABORT_BUTTON_Y
            ),
            None
        );
        assert_eq!(
            hovered_button_with_startup_timing(
                Screen::Startup,
                STARTUP_XML_WAIT_TICK + STARTUP_ABORT_BUTTON_FIRST_FRAME - 1,
                &buttons,
                STARTUP_ABORT_BUTTON_X,
                STARTUP_ABORT_BUTTON_Y
            ),
            Some(ButtonSurface::StartupAbort)
        );
        assert_eq!(
            hovered_button(Screen::Menu, &buttons, 266.0, 63.35),
            Some(ButtonSurface::Menu(MenuAction::Help))
        );
        assert_eq!(
            hovered_button(Screen::Menu, &buttons, 266.0, 120.7),
            Some(ButtonSurface::Menu(MenuAction::Polarisation))
        );
        assert_eq!(
            hovered_button(Screen::Menu, &buttons, 266.0, 177.95),
            Some(ButtonSurface::Menu(MenuAction::Matches))
        );
        assert_eq!(
            hovered_button(Screen::Menu, &buttons, 266.0, 235.25),
            Some(ButtonSurface::Menu(MenuAction::Gravity))
        );
        assert_eq!(
            hovered_button(Screen::Menu, &buttons, 266.3, 292.5),
            Some(ButtonSurface::Menu(MenuAction::Speed))
        );
        assert_eq!(
            hovered_button(Screen::Menu, &buttons, 266.0, 348.5),
            Some(ButtonSurface::Menu(MenuAction::Start))
        );
        assert_eq!(hovered_button(Screen::Menu, &buttons, 10.0, 10.0), None);
        assert_eq!(
            hovered_button(Screen::Help, &buttons, 268.0, 349.2),
            Some(ButtonSurface::HelpBack)
        );
        assert_eq!(hovered_button(Screen::Help, &buttons, 10.0, 10.0), None);
        assert_eq!(hovered_button(Screen::Playing, &buttons, 10.0, 10.0), None);
    }

    #[test]
    fn define_button_sound_records_use_original_rollover_and_press_sounds() {
        let swf = include_bytes!("../gravity_arcade.swf");
        let mut button_sounds = Vec::new();
        collect_swf_button_sounds(swf, swf_first_tag_offset(swf), &mut button_sounds);

        assert_eq!(button_sounds.len(), 8);
        for button_id in [25, 39, 48, 50, 52, 64, 66, 103] {
            let record = button_sounds
                .iter()
                .find(|record| record.button_id == button_id)
                .copied()
                .unwrap_or_else(|| panic!("button {button_id} should have a DefineButtonSound"));
            assert_eq!(
                record.sound_ids,
                [
                    0,
                    SWF_SOUND_BUTTON_ROLLOVER_ID,
                    SWF_SOUND_BUTTON_PRESS_ID,
                    0
                ]
            );
        }
    }

    #[test]
    fn cursor_pointer_covers_all_swf_clickable_surfaces() {
        let buttons = menu_buttons();

        assert_eq!(
            interactive_surface_at(Screen::Menu, 0, &buttons, 266.0, 63.35),
            Some(InteractiveSurface::Button)
        );
        assert_eq!(
            cursor_icon_for_hover(Screen::Menu, 0, &buttons, 266.0, 63.35),
            CursorIcon::Pointer
        );
        assert_eq!(
            interactive_surface_at(Screen::Help, 0, &buttons, 268.0, 349.2),
            Some(InteractiveSurface::Button)
        );
        assert_eq!(
            cursor_icon_for_hover(Screen::Help, 0, &buttons, 268.0, 349.2),
            CursorIcon::Pointer
        );

        assert_eq!(hovered_button(Screen::Help, &buttons, 9.0, 12.0), None);
        assert_eq!(
            interactive_surface_at(Screen::Help, 0, &buttons, 9.0, 12.0),
            Some(InteractiveSurface::HeaderBack)
        );
        assert_eq!(
            cursor_icon_for_hover(Screen::Playing, 0, &buttons, 9.0, 12.0),
            CursorIcon::Pointer
        );

        assert_eq!(hovered_button(Screen::Menu, &buttons, 508.65, 17.4), None);
        assert_eq!(
            interactive_surface_at(Screen::Menu, 0, &buttons, 508.65, 17.4),
            Some(InteractiveSurface::ExternalLink)
        );
        assert_eq!(
            interactive_surface_at(Screen::Help, 0, &buttons, 508.65, 17.4),
            Some(InteractiveSurface::ExternalLink)
        );
        assert_eq!(
            interactive_surface_at(Screen::Playing, 0, &buttons, 508.65, 17.4),
            None
        );
        assert_eq!(
            interactive_surface_at(Screen::Menu, 0, &buttons, 275.0, 383.15),
            Some(InteractiveSurface::ExternalLink)
        );
        assert_eq!(
            interactive_surface_at(Screen::Help, 0, &buttons, 275.0, 383.15),
            Some(InteractiveSurface::ExternalLink)
        );
        assert_eq!(
            interactive_surface_at(Screen::Menu, 0, &buttons, 10.0, 10.0),
            Some(InteractiveSurface::ExternalLink)
        );
        assert_eq!(
            cursor_icon_for_hover(Screen::Menu, 0, &buttons, 500.0, 350.0),
            CursorIcon::Default
        );
        assert_eq!(
            interactive_surface_at(Screen::Startup, 0, &buttons, 508.65, 17.4),
            None
        );

        assert_eq!(
            interactive_surface_at(
                Screen::Startup,
                STARTUP_XML_WAIT_TICK + STARTUP_ABORT_BUTTON_FIRST_FRAME - 2,
                &buttons,
                STARTUP_ABORT_BUTTON_X,
                STARTUP_ABORT_BUTTON_Y
            ),
            None
        );
        assert_eq!(
            interactive_surface_at(
                Screen::Startup,
                STARTUP_XML_WAIT_TICK + STARTUP_ABORT_BUTTON_FIRST_FRAME - 1,
                &buttons,
                STARTUP_ABORT_BUTTON_X,
                STARTUP_ABORT_BUTTON_Y
            ),
            Some(InteractiveSurface::Button)
        );
    }

    #[test]
    fn startup_preloader_and_xml_abort_follow_swf_root_frames() {
        let buttons = menu_buttons();
        let first_abort_tick = STARTUP_XML_WAIT_TICK + STARTUP_ABORT_BUTTON_FIRST_FRAME - 1;
        let full_abort_tick = STARTUP_XML_WAIT_TICK + STARTUP_ABORT_BUTTON_FULL_ALPHA_FRAME - 1;

        assert_eq!(STARTUP_ARIAL_FONT_ID, 17);
        assert_eq!(STARTUP_TITLE_DEFINE_EDIT_TEXT_ID, 18);
        assert_eq!(STARTUP_PERCENT_DEFINE_EDIT_TEXT_ID, 19);
        assert_eq!(STARTUP_GRAVITY_DEFINE_EDIT_TEXT_ID, 20);
        assert_eq!(STARTUP_RETRIEVING_DEFINE_EDIT_TEXT_ID, 21);
        assert_eq!(STARTUP_ABORT_BUTTON_ID, 25);
        assert_eq!(STARTUP_ABORT_ROLLOVER_SOUND_ID, 23);
        assert_eq!(STARTUP_ABORT_PRESS_SOUND_ID, 24);
        assert_eq!(STARTUP_ABORT_LABEL_DEFINE_TEXT_ID, 27);
        assert_eq!(STARTUP_ABORT_CLIP_ID, 29);
        assert_eq!(SWF_GOTO_FRAME_ROOT_FRAME_OFFSET, 1);
        assert_eq!(STARTUP_ABORT_GOTO_FRAME, 55);
        assert_eq!(STARTUP_MENU_ROOT_FRAME, 56);
        assert_eq!(
            swf_goto_frame_root_frame(STARTUP_ABORT_GOTO_FRAME),
            STARTUP_MENU_ROOT_FRAME
        );
        assert_eq!(STARTUP_DEVICE_FALLBACK_FONT_NAME, "Liberation Sans");

        assert_eq!(startup_phase(0), StartupPhase::Loading);
        assert_eq!(startup_phase(1), StartupPhase::Loading);
        assert_eq!(startup_phase(STARTUP_XML_WAIT_TICK), StartupPhase::XmlWait);
        assert_eq!(
            startup_loading_title_text(SWF_FILE_LEN_BYTES),
            "loading 82 kbyte..."
        );
        assert_eq!(
            startup_loading_percent_text(SWF_FILE_LEN_BYTES, SWF_FILE_LEN_BYTES),
            "100 %"
        );
        assert_eq!(STARTUP_GRAVITY_TEXT_VALUE, "\"Gravity\"");
        assert_eq!(STARTUP_RETRIEVING_TEXT_VALUE, "retrieving online data");
        assert_eq!(STARTUP_ABORT_LABEL, "abort");

        assert_swf_point(
            SwfPoint::new(
                STARTUP_LOADING_TITLE_TEXT.center_x,
                STARTUP_LOADING_TITLE_TEXT.baseline_y,
            ),
            275.03,
            191.95,
        );
        assert_swf_point(
            SwfPoint::new(
                STARTUP_RETRIEVING_TEXT.center_x,
                STARTUP_RETRIEVING_TEXT.baseline_y,
            ),
            275.0,
            190.5,
        );

        assert_eq!(startup_abort_button_alpha(first_abort_tick - 1), None);
        assert_close(
            startup_abort_button_alpha(first_abort_tick)
                .expect("abort button should exist on sprite 29 frame 16"),
            0.0,
        );
        assert_close(
            startup_abort_button_alpha(first_abort_tick + 1)
                .expect("abort button should fade in on sprite 29 frame 17"),
            3.0 / 256.0,
        );
        assert_close(
            startup_abort_button_alpha(full_abort_tick)
                .expect("abort button should be fully opaque on sprite 29 frame 90"),
            1.0,
        );

        let abort_button = startup_abort_button_visual();
        assert_swf_point(
            abort_button.center,
            STARTUP_ABORT_BUTTON_X,
            STARTUP_ABORT_BUTTON_Y,
        );
        assert_rect(abort_button.rect, 229.82, 224.32, 89.29, 17.46);
        assert!(startup_abort_button_contains(
            first_abort_tick,
            STARTUP_ABORT_BUTTON_X,
            STARTUP_ABORT_BUTTON_Y
        ));
        assert_eq!(
            swf_release_action(
                action_surface_at(
                    Screen::Startup,
                    first_abort_tick,
                    &buttons,
                    STARTUP_ABORT_BUTTON_X,
                    STARTUP_ABORT_BUTTON_Y
                ),
                action_surface_at(
                    Screen::Startup,
                    first_abort_tick,
                    &buttons,
                    STARTUP_ABORT_BUTTON_X,
                    STARTUP_ABORT_BUTTON_Y
                )
            ),
            Some(ActionSurface::Button(ButtonSurface::StartupAbort))
        );
        assert_eq!(
            action_surface_at(
                Screen::Startup,
                first_abort_tick - 1,
                &buttons,
                STARTUP_ABORT_BUTTON_X,
                STARTUP_ABORT_BUTTON_Y
            ),
            None
        );
        assert_eq!(
            surface_transition(ActionSurface::Button(ButtonSurface::StartupAbort)),
            SurfaceTransition::MenuImmediate { set_offline: true }
        );
    }

    #[test]
    fn swf_button_actions_follow_release_and_start_drag_out_flags() {
        let buttons = menu_buttons();
        let at = |screen, x, y| action_surface_at(screen, 0, &buttons, x, y);
        let help = ActionSurface::Button(ButtonSurface::Menu(MenuAction::Help));
        let gravity = ActionSurface::Button(ButtonSurface::Menu(MenuAction::Gravity));
        let start = ActionSurface::Button(ButtonSurface::Menu(MenuAction::Start));
        let help_back = ActionSurface::Button(ButtonSurface::HelpBack);
        let header_back = ActionSurface::HeaderBack;
        let sponsor = ActionSurface::ExternalLink(ExternalLinkAction::NeodelightBlank);
        let footer = ActionSurface::ExternalLink(ExternalLinkAction::NeokolorBlank);

        assert_eq!(at(Screen::Menu, 266.0, 63.35), Some(help));
        assert_eq!(
            swf_release_action(
                at(Screen::Menu, 266.0, 63.35),
                at(Screen::Menu, 266.0, 63.35),
            ),
            Some(help)
        );
        assert_eq!(
            swf_release_action(
                at(Screen::Menu, 266.0, 63.35),
                at(Screen::Menu, 500.0, 350.0),
            ),
            None
        );
        assert_eq!(
            swf_drag_out_action(
                at(Screen::Menu, 266.0, 63.35),
                at(Screen::Menu, 500.0, 350.0),
            ),
            None
        );
        assert_eq!(
            swf_release_or_drag_out_action(
                at(Screen::Menu, 266.0, 63.35),
                at(Screen::Menu, 500.0, 350.0),
            ),
            None
        );
        assert_eq!(
            swf_release_action(
                at(Screen::Menu, 266.0, 63.35),
                at(Screen::Menu, 266.0, 235.25),
            ),
            None
        );
        assert_eq!(
            swf_drag_out_action(
                at(Screen::Menu, 266.0, 63.35),
                at(Screen::Menu, 266.0, 235.25),
            ),
            None
        );
        assert_eq!(swf_drag_out_action(Some(help), Some(help)), None);
        assert_eq!(at(Screen::Menu, 266.0, 235.25), Some(gravity));
        assert_eq!(
            swf_drag_out_action(
                at(Screen::Menu, 266.0, 235.25),
                at(Screen::Menu, 500.0, 350.0),
            ),
            None
        );
        assert_eq!(
            swf_release_or_drag_out_action(
                at(Screen::Menu, 266.0, 235.25),
                at(Screen::Menu, 500.0, 350.0),
            ),
            None
        );
        assert_eq!(at(Screen::Menu, 266.0, 348.5), Some(start));
        assert_eq!(
            swf_release_action(
                at(Screen::Menu, 266.0, 348.5),
                at(Screen::Menu, 266.0, 348.5),
            ),
            Some(start)
        );
        assert_eq!(
            swf_release_action(
                at(Screen::Menu, 266.0, 348.5),
                at(Screen::Menu, 500.0, 350.0),
            ),
            None
        );
        assert_eq!(
            swf_drag_out_action(
                at(Screen::Menu, 266.0, 348.5),
                at(Screen::Menu, 500.0, 350.0),
            ),
            Some(start)
        );
        assert_eq!(
            swf_release_or_drag_out_action(
                at(Screen::Menu, 266.0, 348.5),
                at(Screen::Menu, 500.0, 350.0),
            ),
            Some(start)
        );
        assert_eq!(
            swf_drag_out_action(
                at(Screen::Menu, 266.0, 348.5),
                at(Screen::Menu, 266.0, 63.35),
            ),
            Some(start)
        );
        assert_eq!(swf_drag_out_action(Some(start), Some(start)), None);
        assert_eq!(
            swf_release_action(None, at(Screen::Menu, 266.0, 235.25)),
            None
        );

        assert_eq!(
            swf_release_action(
                at(Screen::Help, 268.0, 349.2),
                at(Screen::Help, 268.0, 349.2),
            ),
            Some(help_back)
        );
        assert_eq!(HELP_BACK_GOTO_FRAME, 55);
        assert_eq!(
            swf_goto_frame_root_frame(HELP_BACK_GOTO_FRAME),
            STARTUP_MENU_ROOT_FRAME
        );
        assert_eq!(
            surface_transition(help_back),
            SurfaceTransition::MenuImmediate { set_offline: false }
        );
        assert_eq!(
            swf_release_action(
                at(Screen::Playing, 9.0, 12.0),
                at(Screen::Playing, 9.0, 12.0),
            ),
            Some(header_back)
        );
        assert_eq!(HEADER_BACK_GOTO_FRAME, 55);
        assert_eq!(
            swf_goto_frame_root_frame(HEADER_BACK_GOTO_FRAME),
            STARTUP_MENU_ROOT_FRAME
        );
        assert_eq!(
            surface_transition(header_back),
            SurfaceTransition::MenuImmediate { set_offline: false }
        );
        assert_eq!(
            surface_transition(ActionSurface::Button(ButtonSurface::StartupAbort)),
            SurfaceTransition::MenuImmediate { set_offline: true }
        );
        assert_eq!(
            swf_release_action(
                at(Screen::Menu, 508.65, 17.4),
                at(Screen::Menu, 508.65, 17.4),
            ),
            Some(sponsor)
        );
        assert_eq!(
            swf_release_action(
                at(Screen::Menu, 275.0, 383.15),
                at(Screen::Menu, 275.0, 383.15),
            ),
            Some(footer)
        );
    }

    #[test]
    fn game_mouse_flows_apply_swf_button_actions_to_state() {
        let mut game = Game::new(AudioBank::default(), GameAssets::new(None, None, None));

        game.screen = Screen::Menu;
        game.mouse_pressed(266.0, 63.35);
        game.mouse_released(266.0, 63.35);
        assert_eq!(game.screen, Screen::Help);

        game.mouse_pressed(268.0, 349.2);
        game.mouse_released(268.0, 349.2);
        assert_eq!(game.screen, Screen::Menu);
        assert!(!game.offline);

        game.screen = Screen::Help;
        game.mouse_pressed(9.0, 12.0);
        game.mouse_released(9.0, 12.0);
        assert_eq!(game.screen, Screen::Menu);

        game.screen = Screen::Playing;
        game.mouse_pressed(9.0, 12.0);
        game.mouse_released(9.0, 12.0);
        assert_eq!(game.screen, Screen::Menu);

        game.screen = Screen::Startup;
        game.startup_ticks = STARTUP_XML_WAIT_TICK + STARTUP_ABORT_BUTTON_FULL_ALPHA_FRAME - 1;
        game.mouse_pressed(STARTUP_ABORT_BUTTON_X, STARTUP_ABORT_BUTTON_Y);
        game.mouse_released(STARTUP_ABORT_BUTTON_X, STARTUP_ABORT_BUTTON_Y);
        assert_eq!(game.screen, Screen::Menu);
        assert!(game.offline);

        game.screen = Screen::Menu;
        game.offline = false;
        game.mouse_pressed(266.0, 348.5);
        game.mouse_dragged(500.0, 350.0);
        assert_eq!(game.screen, Screen::Playing);

        game.screen = Screen::Menu;
        game.settings.gravity = GravityStrength::G2;
        game.mouse_pressed(266.0, 235.25);
        game.mouse_dragged(500.0, 350.0);
        game.mouse_released(500.0, 350.0);
        assert_eq!(game.screen, Screen::Menu);
        assert_eq!(game.settings.gravity, GravityStrength::G2);
    }

    #[test]
    fn header_links_use_swf_button_hit_shapes_and_states() {
        assert_eq!(placement_constants::SPONSOR_HEADER_HIT_DEFINE_SHAPE_ID, 98);
        assert_eq!(placement_constants::BACK_HEADER_HIT_DEFINE_SHAPE_ID, 123);

        let sponsor = header_link_rect(HeaderLink::Sponsor);
        assert_close(sponsor.x, 6.0);
        assert_close(sponsor.y, -3.0);
        assert_close(sponsor.w, 112.25);
        assert_close(sponsor.h, 20.95);
        let sponsor_polygon = header_link_hit_polygon(HeaderLink::Sponsor);
        assert_swf_point(sponsor_polygon[0], 115.0, 17.95);
        assert_swf_point(sponsor_polygon[1], 7.75, 16.5);
        assert_swf_point(sponsor_polygon[2], 6.0, -2.9);
        assert_swf_point(sponsor_polygon[3], 118.25, -3.0);
        assert!(header_link_contains(HeaderLink::Sponsor, 9.0, 12.0));
        assert!(sponsor.contains(vec2(6.1, 17.8)));
        assert!(!header_link_contains(HeaderLink::Sponsor, 6.1, 17.8));

        let back = header_link_rect(HeaderLink::BackToMenu);
        assert_close(back.x, 6.0);
        assert_close(back.y, -2.0);
        assert_close(back.w, 80.8);
        assert_close(back.h, 17.5);
        let back_polygon = header_link_hit_polygon(HeaderLink::BackToMenu);
        assert_swf_point(back_polygon[0], 85.8, 14.5);
        assert_swf_point(back_polygon[1], 6.0, 15.5);
        assert_swf_point(back_polygon[2], 6.0, -2.0);
        assert_swf_point(back_polygon[3], 86.8, -2.0);
        assert!(header_link_contains(HeaderLink::BackToMenu, 9.0, 12.0));
        assert!(back.contains(vec2(86.6, 15.3)));
        assert!(!header_link_contains(HeaderLink::BackToMenu, 86.6, 15.3));

        assert_eq!(header_link_label(HeaderLink::Sponsor), "to neodelight");
        assert_eq!(header_link_label(HeaderLink::BackToMenu), "back to menu");
        assert_line_visual(HEADER_LINK_TEXT, 9.0, 12.0, 12);
        assert_chrome_text_definition(
            &chrome_texts::SPONSOR_UP,
            "to neodelight",
            95,
            &[54],
            [0, 0, 0],
            [30, 8005, 285, 1425],
            19,
            204,
            594,
        );
        assert_chrome_text_definition(
            &chrome_texts::SPONSOR_OVER,
            "to neodelight",
            96,
            &[54],
            [204, 204, 204],
            [30, 8005, 285, 1425],
            19,
            204,
            594,
        );
        assert_chrome_text_definition(
            &chrome_texts::SPONSOR_DOWN,
            "to neodelight",
            97,
            &[54],
            [255, 255, 255],
            [30, 8005, 285, 1425],
            19,
            204,
            594,
        );
        assert_rect_visual(
            chrome_text_stage_bounds(&chrome_texts::SPONSOR_UP, HEADER_LINK_TEXT_PLACEMENT),
            9.30,
            3.05,
            79.75,
            11.40,
        );
        assert_chrome_text_definition(
            &chrome_texts::BACK_UP,
            "back to menu",
            120,
            &[54],
            [0, 0, 0],
            [65, 8150, 285, 1190],
            14,
            168,
            462,
        );
        assert_chrome_text_definition(
            &chrome_texts::BACK_OVER,
            "back to menu",
            121,
            &[54],
            [204, 204, 204],
            [65, 8150, 285, 1190],
            14,
            168,
            462,
        );
        assert_chrome_text_definition(
            &chrome_texts::BACK_DOWN,
            "back to menu",
            122,
            &[54],
            [255, 255, 255],
            [65, 8150, 285, 1190],
            14,
            168,
            462,
        );
        assert_rect_visual(
            chrome_text_stage_bounds(&chrome_texts::BACK_UP, HEADER_LINK_TEXT_PLACEMENT),
            9.65,
            3.05,
            80.85,
            9.05,
        );
        assert_eq!(SPONSOR_LOGO_LABEL, "neodelight");
        assert_eq!(SPONSOR_LOGO_MENU_FRAME, 56);
        assert_eq!(SPONSOR_LOGO_HELP_FRAME, 57);
        assert_eq!(SPONSOR_LOGO_REMOVED_FRAME, 58);
        assert_eq!(SPONSOR_LOGO_DEPTH, 129);
        let sponsor_layers = sponsor_logo_text_layers();
        assert_line_visual(sponsor_layers[0].visual, 477.90, 17.97, 10);
        assert_color(sponsor_layers[0].color, SPONSOR_LOGO_DARK);
        assert_line_visual(sponsor_layers[1].visual, 477.64, 17.71, 10);
        assert_color(sponsor_layers[1].color, SPONSOR_LOGO_OLIVE);
        assert_sponsor_logo_text_definition(
            &sponsor_logo_texts::DARK,
            "neodelight",
            33,
            &[32],
            [51, 102, 0],
            [155, 8045, 80, 3255],
            19,
            285,
            837,
        );
        assert_sponsor_logo_text_definition(
            &sponsor_logo_texts::OLIVE,
            "neodelight",
            34,
            &[32],
            [153, 153, 0],
            [155, 8045, 80, 3255],
            19,
            285,
            837,
        );
        assert_rect_visual(
            sponsor_logo_text_stage_bounds(&sponsor_logo_texts::DARK, SPONSOR_LOGO_DEPTH_3_TEXT),
            479.18,
            4.78,
            65.05,
            26.18,
        );
        assert_rect_visual(
            sponsor_logo_text_stage_bounds(&sponsor_logo_texts::OLIVE, SPONSOR_LOGO_DEPTH_4_TEXT),
            478.92,
            4.52,
            65.05,
            26.18,
        );

        let logo_bounds = sponsor_logo_shape35_local_bounds();
        assert_close(logo_bounds.x_min, -114.3);
        assert_close(logo_bounds.x_max, 117.1);
        assert_close(logo_bounds.y_min, -50.35);
        assert_close(logo_bounds.y_max, 50.65);
        assert_eq!(sponsor_logo_shape35::FILL_RGB, [102, 204, 0]);
        assert_color(
            SPONSOR_LOGO_FILL,
            Color::new(102.0 / 255.0, 204.0 / 255.0, 0.0, 1.0),
        );
        let logo_stage_bounds = sponsor_logo_shape35_stage_bounds();
        assert_close(logo_stage_bounds.x, 478.69);
        assert_close(logo_stage_bounds.y, 4.20);
        assert_close(logo_stage_bounds.w, 60.66);
        assert_close(logo_stage_bounds.h, 26.48);
        let contour_edge_count: usize = sponsor_logo_shape35::CONTOURS
            .iter()
            .map(|contour| contour.segments.len())
            .sum();
        assert_eq!(sponsor_logo_shape35::CONTOURS.len(), 35);
        assert_eq!(contour_edge_count, 678);
        let logo_contours = sponsor_logo_shape35_flattened_contours();
        assert_eq!(logo_contours.iter().map(Vec::len).sum::<usize>(), 1401);
        assert!(sponsor_logo_shape35_contains_local(
            &logo_contours,
            SwfPoint::new(29.3, -24.1)
        ));
        assert!(!sponsor_logo_shape35_contains_local(
            &logo_contours,
            SwfPoint::new(9.7, -12.7)
        ));
        assert_eq!(
            header_link_state(HeaderLink::BackToMenu, 9.0, 12.0, false),
            HeaderLinkState::Over
        );
        assert_eq!(
            header_link_state(HeaderLink::BackToMenu, 9.0, 12.0, true),
            HeaderLinkState::Down
        );
        assert_eq!(
            header_link_state(HeaderLink::BackToMenu, 200.0, 12.0, false),
            HeaderLinkState::Up
        );
        assert_color(
            header_link_color(HeaderLink::BackToMenu, 9.0, 12.0, false),
            HEADER_LINK_OVER,
        );
        assert_color(
            header_link_color(HeaderLink::BackToMenu, 9.0, 12.0, true),
            HEADER_LINK_DOWN,
        );
        assert_color(
            header_link_color(HeaderLink::BackToMenu, 200.0, 12.0, false),
            HEADER_LINK_UP,
        );
    }

    #[test]
    fn external_links_use_swf_get_url_buttons_and_hit_bounds() {
        assert_eq!(
            external_link_url(ExternalLinkAction::NeodelightBlank),
            "http://www.neodelight.com"
        );
        assert_eq!(
            external_link_target(ExternalLinkAction::NeodelightBlank),
            "_blank"
        );
        assert_eq!(
            external_link_url(ExternalLinkAction::NeodelightSelf),
            "http://www.neodelight.com"
        );
        assert_eq!(
            external_link_target(ExternalLinkAction::NeodelightSelf),
            "_self"
        );
        assert_eq!(
            external_link_url(ExternalLinkAction::NeokolorBlank),
            "http://www.neokolor.com"
        );
        assert_eq!(
            external_link_target(ExternalLinkAction::NeokolorBlank),
            "_blank"
        );
        assert_eq!(NEOKOLOR_LINK_ROOT_FRAME, 56);
        assert_eq!(NEOKOLOR_LINK_HELP_FRAME, 57);
        assert_eq!(NEOKOLOR_LINK_REMOVED_FRAME, 58);
        assert_eq!(NEOKOLOR_LINK_DEPTH, 44);
        assert_eq!(NEOKOLOR_LINK_BUTTON_ID, 75);
        assert_eq!(
            placement_constants::SPONSOR_LOGO_BUTTON_HIT_DEFINE_SHAPE_ID,
            30
        );
        assert_eq!(
            placement_constants::NEOKOLOR_LINK_BUTTON_HIT_DEFINE_SHAPE_ID,
            74
        );

        assert_rect(sponsor_logo_button_rect(), 475.88, -0.23, 67.23, 33.45);
        assert!(sponsor_logo_button_contains(508.65, 17.4));
        assert!(!sponsor_logo_button_contains(544.0, 17.4));
        assert_eq!(
            external_link_at(Screen::Menu, 508.65, 17.4),
            Some(ExternalLinkAction::NeodelightBlank)
        );
        assert_eq!(
            external_link_at(Screen::Help, 508.65, 17.4),
            Some(ExternalLinkAction::NeodelightBlank)
        );
        assert_eq!(external_link_at(Screen::Playing, 508.65, 17.4), None);

        assert_rect(neokolor_link_button_rect(), 143.50, 375.65, 263.0, 15.0);
        assert!(neokolor_link_button_contains(275.0, 383.15));
        assert!(!neokolor_link_button_contains(275.0, 392.0));
        assert_eq!(
            external_link_at(Screen::Menu, 275.0, 383.15),
            Some(ExternalLinkAction::NeokolorBlank)
        );
        assert_eq!(
            external_link_at(Screen::Help, 275.0, 383.15),
            Some(ExternalLinkAction::NeokolorBlank)
        );
        assert_eq!(external_link_at(Screen::Playing, 275.0, 383.15), None);

        assert_eq!(
            external_link_at(Screen::Menu, 9.0, 12.0),
            Some(ExternalLinkAction::NeodelightSelf)
        );
        assert_eq!(external_link_at(Screen::Help, 9.0, 12.0), None);
    }

    #[test]
    fn side_marker_colors_and_bounds_match_swf_shapes() {
        assert_eq!(side_marker_shapes::RED_FILL_DEFINE_SHAPE_ID, 92);
        assert_eq!(side_marker_shapes::OUTLINE_DEFINE_SHAPE_ID, 93);
        assert_eq!(side_marker_shapes::BLUE_FILL_DEFINE_SHAPE_ID, 94);
        assert_eq!(
            side_marker_shapes::RED_FILL_BOUNDS_TWIPS,
            [-112, 113, -112, 113]
        );
        assert_eq!(
            side_marker_shapes::OUTLINE_BOUNDS_TWIPS,
            [-180, 180, -180, 180]
        );
        assert_eq!(
            side_marker_shapes::BLUE_FILL_BOUNDS_TWIPS,
            [-113, 112, -112, 113]
        );
        let blue_bounds = marker_fill_bounds(Side::Blue);
        assert_close(blue_bounds.x_min, -5.65);
        assert_close(blue_bounds.x_max, 5.6);
        assert_close(blue_bounds.y_min, -5.6);
        assert_close(blue_bounds.y_max, 5.65);
        let red_bounds = marker_fill_bounds(Side::Red);
        assert_close(red_bounds.x_min, -5.6);
        assert_close(red_bounds.x_max, 5.65);
        assert_close(red_bounds.y_min, -5.6);
        assert_close(red_bounds.y_max, 5.65);
        assert_close(side_marker_shapes::OUTLINE_SHAPE.bounds.x_min, -9.0);
        assert_close(side_marker_shapes::OUTLINE_SHAPE.bounds.x_max, 9.0);
        assert_close(side_marker_shapes::OUTLINE_SHAPE.bounds.y_min, -9.0);
        assert_close(side_marker_shapes::OUTLINE_SHAPE.bounds.y_max, 9.0);
        assert_close(side_marker_shapes::OUTLINE_LINE_WIDTH, 1.0);
        assert_color(SIDE_MARKER_EMPTY_FILL, Color::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(side_marker_shapes::OUTLINE_RGB, [0, 0, 0]);
        assert_eq!(side_marker_shapes::BLUE_FILL_RGB, [127, 212, 212]);
        assert_eq!(side_marker_shapes::RED_FILL_RGB, [255, 171, 34]);
        assert_color(
            SCORE_MARKER_BLUE,
            Color::new(127.0 / 255.0, 212.0 / 255.0, 212.0 / 255.0, 1.0),
        );
        assert_color(
            SCORE_MARKER_RED,
            Color::new(1.0, 171.0 / 255.0, 34.0 / 255.0, 1.0),
        );

        let blue = marker_glyph_visual(19.0, 214.45, Side::Blue, SCORE_MARKER_BLUE, 1.0);
        assert_swf_point(blue.fill_center, 18.98, 214.48);
        assert_color(blue.fill_color, SCORE_MARKER_BLUE);
        assert_eq!(blue.fill_points.len(), 32);
        assert_swf_point(blue.fill_points[0], 18.95, 208.85);
        assert_swf_point(blue.fill_points[4], 22.95, 210.50);
        assert_swf_point(blue.fill_points[8], 24.60, 214.50);
        assert_swf_point(blue.fill_points[16], 18.95, 220.10);
        assert_swf_point(blue.fill_points[24], 13.35, 214.50);
        assert_eq!(blue.outline_points.len(), 32);
        assert_swf_point(blue.outline_points[0], 25.00, 208.45);
        assert_swf_point(blue.outline_points[4], 27.50, 214.45);
        assert_swf_point(blue.outline_points[8], 25.00, 220.45);
        assert_swf_point(blue.outline_points[12], 19.00, 222.95);
        assert_swf_point(blue.outline_points[16], 13.00, 220.45);
        assert_swf_point(blue.outline_points[20], 10.50, 214.45);
        assert_close(blue.outline_line_width, 1.0);
        assert_color(blue.outline, Color::new(0.0, 0.0, 0.0, 1.0));

        let red = marker_glyph_visual(530.95, 214.45, Side::Red, SCORE_MARKER_RED, 1.0);
        assert_swf_point(red.fill_center, 530.98, 214.48);
        assert_color(red.fill_color, SCORE_MARKER_RED);
        assert_eq!(red.fill_points.len(), 32);
        assert_swf_point(red.fill_points[0], 534.95, 210.50);
        assert_swf_point(red.fill_points[4], 536.60, 214.50);
        assert_swf_point(red.fill_points[8], 534.95, 218.45);
        assert_swf_point(red.fill_points[12], 530.95, 220.10);
        assert_swf_point(red.fill_points[16], 527.00, 218.45);
    }

    #[test]
    fn static_side_markers_are_menu_chrome_not_playfield_contents() {
        assert_eq!(
            static_side_marker_ys(MENU_STAGE_FRAME),
            SIDE_MARKER_YS.as_slice()
        );
        assert_close(STATIC_LEFT_SIDE_MARKER_X, 17.5);
        assert_close(STATIC_RIGHT_SIDE_MARKER_X, 532.45);

        let mut menu_markers = Vec::new();
        for_each_static_side_marker(MENU_STAGE_FRAME, |visual| menu_markers.push(visual));
        assert_eq!(menu_markers.len(), 30);
        assert_eq!(
            menu_markers[0],
            StaticSideMarkerVisual {
                x: 532.45,
                y: 214.45,
                side: Side::Red,
            }
        );
        assert_eq!(
            menu_markers[2],
            StaticSideMarkerVisual {
                x: 532.45,
                y: 50.05,
                side: Side::Blue,
            }
        );
        assert_eq!(
            menu_markers[14],
            StaticSideMarkerVisual {
                x: 532.45,
                y: 50.05,
                side: Side::Red,
            }
        );
        assert_eq!(
            menu_markers[15],
            StaticSideMarkerVisual {
                x: 17.5,
                y: 214.45,
                side: Side::Blue,
            }
        );
        assert_eq!(
            menu_markers[28],
            StaticSideMarkerVisual {
                x: 17.5,
                y: 378.85,
                side: Side::Blue,
            }
        );
        assert_eq!(
            menu_markers[29],
            StaticSideMarkerVisual {
                x: 17.5,
                y: 50.05,
                side: Side::Blue,
            }
        );

        assert!(static_side_marker_ys(PLAYFIELD_STAGE_FRAME).is_empty());
        let mut playfield_markers = Vec::new();
        for_each_static_side_marker(PLAYFIELD_STAGE_FRAME, |visual| {
            playfield_markers.push(visual);
        });
        assert!(playfield_markers.is_empty());
    }

    #[test]
    fn static_paddle_glows_match_swf_root_frame_56_shape_90() {
        assert_eq!(STATIC_PADDLE_GLOW_ROOT_FRAME, 56);
        assert_eq!(STATIC_PADDLE_GLOW_CHARACTER_ID, 90);
        assert_eq!(STATIC_RIGHT_PADDLE_GLOW_DEPTH, 64);
        assert_eq!(STATIC_LEFT_PADDLE_GLOW_DEPTH, 65);
        assert_close(STATIC_PADDLE_GLOW_SCALE_X, 0.531_463_6);
        assert_close(STATIC_PADDLE_GLOW_SCALE_Y, 0.752_914_4);

        let glows = static_goal_paddle_glows(MENU_STAGE_FRAME);
        assert_eq!(glows.len(), 2);

        let right = glows[0];
        assert_close(right.x, 514.7);
        assert_close(right.y, 214.45);
        assert_close(right.scale_x, STATIC_PADDLE_GLOW_SCALE_X);
        assert_close(right.scale_y, STATIC_PADDLE_GLOW_SCALE_Y);
        assert_color(right.color, STATIC_RIGHT_PADDLE_GLOW_COLOR);
        let right_shape = paddle_glow_visual(right.scale_x, right.scale_y, right.color);
        assert_close(right.x + right_shape.x_min, 511.30);
        assert_close(right.x + right_shape.x_max, 518.26);
        assert_close(right.y + right_shape.y_min, 188.74);
        assert_close(right.y + right_shape.y_max, 239.83);

        let left = glows[1];
        assert_close(left.x, 35.0);
        assert_close(left.y, 214.4);
        assert_close(left.scale_x, STATIC_PADDLE_GLOW_SCALE_X);
        assert_close(left.scale_y, STATIC_PADDLE_GLOW_SCALE_Y);
        assert_color(left.color, STATIC_LEFT_PADDLE_GLOW_COLOR);
        assert_color(
            STATIC_RIGHT_PADDLE_GLOW_COLOR,
            Color::new(221.0 / 255.0, 236.0 / 255.0, 1.0, 1.0),
        );
        assert_color(
            STATIC_LEFT_PADDLE_GLOW_COLOR,
            Color::new(1.0, 239.0 / 255.0, 187.0 / 255.0, 1.0),
        );
        assert_eq!(
            placement_constants::STATIC_RIGHT_PADDLE_GLOW_ADD_RGB,
            [221, 236, 255]
        );
        assert_eq!(
            placement_constants::STATIC_LEFT_PADDLE_GLOW_ADD_RGB,
            [255, 239, 187]
        );

        assert!(static_goal_paddle_glows(PLAYFIELD_STAGE_FRAME).is_empty());
    }

    #[test]
    fn paddle_body_uses_swf_shape_89_profile() {
        assert_eq!(paddle_body_shape89::DEFINE_SHAPE_ID, 89);
        assert_eq!(paddle_body_shape89::FILL_RGB, [255, 255, 153]);
        assert_eq!(paddle_body_shape89::LINE_RGB, [255, 255, 255]);
        assert_eq!(paddle_body_shape89::FLATTEN_STEPS, 4);
        assert_close(paddle_body_shape89::SHAPE.bounds.x_min, -1.55);
        assert_close(paddle_body_shape89::SHAPE.bounds.x_max, 1.9);
        assert_close(paddle_body_shape89::SHAPE.bounds.y_min, -25.5);
        assert_close(paddle_body_shape89::SHAPE.bounds.y_max, 25.5);
        assert_swf_point(paddle_body_shape89::SHAPE.start, -0.75, -23.55);
        assert_close(paddle_body_shape89::LINE_WIDTH, 1.0);

        let local = paddle_body_visual(0.0, 0.0);
        assert_rect_visual(paddle_body_bounds(local), -1.55, -25.5, 3.45, 51.0);
        assert_swf_point(local.center, 0.0, 0.0);
        assert_close(local.line_width, 1.0);
        assert_color(local.fill, Color::new(1.0, 1.0, 153.0 / 255.0, 1.0));
        assert_color(local.outline, SWF_WHITE);
        let local_points = paddle_body_outline_points(local);
        assert_eq!(local_points.len(), 36);
        assert_swf_point(local_points[0], -0.75, -23.55);
        assert_swf_point(
            paddle_body_stage_point(local, SwfPoint::new(-1.05, 20.0)),
            -1.05,
            20.0,
        );
        assert_swf_point(
            paddle_body_stage_point(local, SwfPoint::new(1.4, -20.0)),
            1.4,
            -20.0,
        );

        let left_root = paddle_body_visual(35.0, 214.15);
        assert_rect_visual(paddle_body_bounds(left_root), 33.45, 188.65, 3.45, 51.0);
        assert_swf_point(left_root.center, 35.0, 214.15);
        let left_points = paddle_body_outline_points(left_root);
        assert_swf_point(left_points[0], 34.25, 190.60);
        let right_root = paddle_body_visual(514.7, 214.2);
        assert_rect_visual(paddle_body_bounds(right_root), 513.15, 188.70, 3.45, 51.0);
        assert_swf_point(right_root.center, 514.7, 214.2);
    }

    #[test]
    fn panel_mask_and_retained_outline_follow_root_frame_depths() {
        assert_eq!(panel_chrome_shapes::PANEL_FILL_DEFINE_SHAPE_ID, 37);
        assert_eq!(panel_chrome_shapes::RETAINED_MASK_DEFINE_SHAPE_ID, 80);
        assert_eq!(panel_chrome_shapes::PANEL_OUTLINE_DEFINE_SHAPE_ID, 91);
        assert_eq!(panel_chrome_shapes::PLAYFIELD_MASK_DEFINE_SHAPE_ID, 141);
        assert_eq!(
            panel_chrome_shapes::PANEL_FILL_BOUNDS_TWIPS,
            [141, 10860, 720, 7859]
        );
        assert_eq!(
            panel_chrome_shapes::RETAINED_MASK_BOUNDS_TWIPS,
            [0, 10999, 0, 7999]
        );
        assert_eq!(
            panel_chrome_shapes::PANEL_OUTLINE_BOUNDS_TWIPS,
            [121, 10880, 700, 7879]
        );
        assert_eq!(
            panel_chrome_shapes::PLAYFIELD_MASK_BOUNDS_TWIPS,
            [0, 10999, 0, 7999]
        );
        assert_eq!(panel_chrome_shapes::PANEL_FILL_RGB, [0, 0, 102]);
        assert_eq!(panel_chrome_shapes::PANEL_SHADOW_RGB, [0, 0, 82]);
        assert_eq!(panel_chrome_shapes::MASK_FILL_RGB, [99, 0, 0]);
        assert_eq!(panel_chrome_shapes::OUTLINE_RGB, [255, 255, 255]);
        assert_close(STAGE_BOUNDS.x_min, 0.0);
        assert_close(STAGE_BOUNDS.x_max, 549.95);
        assert_close(STAGE_BOUNDS.y_min, 0.0);
        assert_close(STAGE_BOUNDS.y_max, 399.95);
        assert_color(STAGE_RED, Color::new(99.0 / 255.0, 0.0, 0.0, 1.0));
        assert_rect_visual(rect_from_swf_bounds(STAGE_BOUNDS), 0.0, 0.0, 549.95, 399.95);
        assert_close(panel_chrome_shapes::PANEL_FILL_SHAPE.bounds.x_min, 7.05);
        assert_close(panel_chrome_shapes::PANEL_FILL_SHAPE.bounds.x_max, 543.0);
        assert_close(panel_chrome_shapes::PANEL_FILL_SHAPE.bounds.y_min, 36.0);
        assert_close(panel_chrome_shapes::PANEL_FILL_SHAPE.bounds.y_max, 392.95);
        assert_rect_visual(
            rect_from_swf_bounds(PANEL_CENTER_BOUNDS),
            35.05,
            36.0,
            479.95,
            356.95,
        );
        assert_color(PANEL, Color::new(0.0, 0.0, 102.0 / 255.0, 1.0));
        assert_color(PANEL_SHADOW, Color::new(0.0, 0.0, 82.0 / 255.0, 1.0));
        let center = panel_center_points();
        assert_eq!(center.len(), 5);
        assert_swf_point(center[0], 515.0, 36.0);
        assert_swf_point(center[1], 515.0, 392.95);
        assert_swf_point(center[4], 515.0, 36.0);
        let left_shadow = panel_left_shadow_points();
        assert_eq!(left_shadow.len(), 21);
        assert_swf_point(left_shadow[0], 35.05, 392.95);
        assert_swf_point(left_shadow[1], 12.05, 392.95);
        assert_swf_point(left_shadow[9], 7.05, 387.95);
        assert_swf_point(left_shadow[10], 7.05, 41.0);
        assert_swf_point(left_shadow[19], 35.05, 36.0);
        assert_swf_point(left_shadow[20], 35.05, 392.95);
        let right_shadow = panel_right_shadow_points();
        assert_eq!(right_shadow.len(), 25);
        assert_swf_point(right_shadow[0], 538.0, 392.95);
        assert_swf_point(right_shadow[1], 515.0, 392.95);
        assert_swf_point(right_shadow[2], 515.0, 36.0);
        assert_swf_point(right_shadow[8], 541.55, 37.5);
        assert_swf_point(right_shadow[16], 542.75, 389.5);
        assert_swf_point(right_shadow[24], 538.0, 392.95);
        assert_close(panel_chrome_shapes::PANEL_OUTLINE_SHAPE.bounds.x_min, 6.05);
        assert_close(panel_chrome_shapes::PANEL_OUTLINE_SHAPE.bounds.x_max, 544.0);
        assert_close(panel_chrome_shapes::PANEL_OUTLINE_SHAPE.bounds.y_min, 35.0);
        assert_close(
            panel_chrome_shapes::PANEL_OUTLINE_SHAPE.bounds.y_max,
            393.95,
        );
        assert_close(panel_chrome_shapes::OUTLINE_LINE_WIDTH, 2.0);
        assert_color(SWF_WHITE, Color::new(1.0, 1.0, 1.0, 1.0));
        let primary = panel_outline_primary_points();
        assert_eq!(primary.len(), 41);
        assert_swf_point(primary[0], 541.5, 37.45);
        assert_swf_point(primary[4], 538.0, 36.0);
        assert_swf_point(primary[13], 7.05, 41.0);
        assert_swf_point(primary[22], 12.05, 392.95);
        assert_swf_point(primary[23], 538.0, 392.95);
        assert_swf_point(primary[40], 541.5, 37.45);
        let lower_right = panel_outline_lower_right_points();
        assert_eq!(lower_right.len(), 9);
        assert_swf_point(lower_right[0], 539.55, 392.70);
        assert_swf_point(lower_right[8], 542.75, 389.50);
        let retained_primary = retained_frame_mask_outline_primary_points();
        assert_eq!(retained_primary.len(), 41);
        assert_swf_point(retained_primary[0], 542.75, 389.50);
        assert_swf_point(retained_primary[4], 543.0, 41.0);
        assert_swf_point(retained_primary[9], 541.5, 37.45);
        assert_swf_point(retained_primary[22], 7.05, 41.0);
        assert_swf_point(retained_primary[31], 12.05, 392.95);
        assert_swf_point(retained_primary[36], 541.5, 391.45);
        assert_swf_point(retained_primary[40], 542.75, 389.50);
        let retained_lower_right = retained_frame_mask_outline_lower_right_points();
        assert_eq!(retained_lower_right.len(), 9);
        assert_swf_point(retained_lower_right[0], 542.75, 389.50);
        assert_swf_point(retained_lower_right[8], 539.55, 392.70);

        assert!(panel_outline_enabled(MENU_STAGE_FRAME));
        assert!(!panel_outline_enabled(PLAYFIELD_STAGE_FRAME));
        assert_eq!(PLAYFIELD_ROOT_MASK_DEPTH, 46);
        assert_eq!(PLAYFIELD_ROOT_MASK_CHARACTER_ID, 141);
        assert_eq!(PLAYFIELD_RETAINED_FRAME_MASK_DEPTH, 50);
        assert_eq!(PLAYFIELD_RETAINED_FRAME_MASK_CHARACTER_ID, 80);
        assert_close(panel_chrome_shapes::OUTLINE_LINE_WIDTH, 2.0);
        assert_eq!(PLAYFIELD_TOP_TITLE_DEPTH, 51);
        assert_eq!(PLAYFIELD_RETAINED_EMPTY_FACTOR_SPRITE_DEPTH, 52);
        assert_eq!(PLAYFIELD_RETAINED_EMPTY_FACTOR_SPRITE_ID, 83);
        assert_close(panel_chrome_shapes::PLAYFIELD_MASK_SHAPE.bounds.x_min, 0.0);
        assert_close(
            panel_chrome_shapes::PLAYFIELD_MASK_SHAPE.bounds.x_max,
            549.95,
        );
        assert_close(panel_chrome_shapes::PLAYFIELD_MASK_SHAPE.bounds.y_min, 0.0);
        assert_close(
            panel_chrome_shapes::PLAYFIELD_MASK_SHAPE.bounds.y_max,
            399.95,
        );
        let mask_top_left = playfield_mask_top_left_points();
        assert_eq!(mask_top_left.len(), 11);
        assert_swf_point(mask_top_left[0], 7.05, 41.0);
        assert_swf_point(mask_top_left[1], 7.05, 36.0);
        assert_swf_point(mask_top_left[2], 12.05, 36.0);
        assert_swf_point(mask_top_left[10], 7.05, 41.0);
        let mask_top_right = playfield_mask_top_right_points();
        assert_eq!(mask_top_right.len(), 12);
        assert_swf_point(mask_top_right[0], 538.0, 36.0);
        assert_swf_point(mask_top_right[1], 543.0, 36.0);
        assert_swf_point(mask_top_right[2], 543.0, 41.0);
        assert_swf_point(mask_top_right[11], 538.0, 36.0);
        let mask_bottom_left = playfield_mask_bottom_left_points();
        assert_eq!(mask_bottom_left.len(), 11);
        assert_swf_point(mask_bottom_left[0], 7.05, 387.95);
        assert_swf_point(mask_bottom_left[1], 7.05, 392.95);
        assert_swf_point(mask_bottom_left[2], 12.05, 392.95);
        assert_swf_point(mask_bottom_left[10], 7.05, 387.95);
        let mask_bottom_right = playfield_mask_bottom_right_points();
        assert_eq!(mask_bottom_right.len(), 17);
        assert_swf_point(mask_bottom_right[0], 538.0, 392.95);
        assert_swf_point(mask_bottom_right[1], 543.0, 392.95);
        assert_swf_point(mask_bottom_right[2], 543.0, 387.95);
        assert_swf_point(mask_bottom_right[9], 541.55, 391.50);
        assert_swf_point(mask_bottom_right[13], 539.55, 392.70);
        assert_swf_point(mask_bottom_right[16], 538.0, 392.95);
    }

    #[test]
    fn paddle_charge_glow_profile_matches_swf_shape_90() {
        assert_eq!(paddle_glow_shape90::DEFINE_SHAPE_ID, 90);
        assert_eq!(paddle_glow_shape90::BOUNDS_TWIPS, [-128, 134, -683, 674]);
        assert_eq!(
            paddle_glow_shape90::BODY_GRADIENT_RGBA,
            [
                [0, 102, 255, 255, 0],
                [87, 173, 255, 255, 255],
                [166, 161, 255, 255, 238],
                [255, 153, 255, 255, 0],
            ],
        );
        assert_eq!(
            paddle_glow_shape90::TOP_CAP_GRADIENT_RGBA,
            [[74, 153, 255, 255, 255], [255, 153, 255, 255, 0]],
        );
        assert_eq!(
            paddle_glow_shape90::BOTTOM_CAP_GRADIENT_RGBA,
            paddle_glow_shape90::TOP_CAP_GRADIENT_RGBA,
        );

        let color = Color::new(0.25, 0.5, 0.75, 1.0);
        let glow = paddle_glow_visual(1.0, 1.0, color);

        assert_close(glow.x_min, -6.4);
        assert_close(glow.x_max, 6.7);
        assert_close(glow.y_min, -34.15);
        assert_close(glow.y_max, 33.7);
        assert_close(glow.body_y_min, -22.25);
        assert_close(glow.body_y_max, 21.65);
        assert_color(glow.color, color);

        assert_close(
            alpha_from_stops(&paddle_glow_shape90::LINEAR_ALPHA_STOPS, 0.0),
            0.0,
        );
        assert_close(
            alpha_from_stops(&paddle_glow_shape90::LINEAR_ALPHA_STOPS, 87.0 / 255.0),
            1.0,
        );
        assert_close(
            alpha_from_stops(&paddle_glow_shape90::LINEAR_ALPHA_STOPS, 166.0 / 255.0),
            238.0 / 255.0,
        );
        assert_close(
            alpha_from_stops(&paddle_glow_shape90::LINEAR_ALPHA_STOPS, 1.0),
            0.0,
        );
        assert_close(paddle_glow_shape90::RADIAL_PEAK_STOP_RATIO, 74.0 / 255.0);
        assert_close(paddle_glow_radial_alpha_at(0.0), 1.0);
        assert_close(
            paddle_glow_radial_alpha_at(paddle_glow_shape90::RADIAL_PEAK_STOP_RATIO),
            1.0,
        );
        assert_close(paddle_glow_radial_alpha_at(1.0), 0.0);
    }

    #[test]
    fn paddle_charge_glow_caps_follow_swf_shape_90_contours() {
        assert_close(PADDLE_GLOW_CENTER_X, 0.15);
        assert_close(PADDLE_GLOW_TOP_CENTER_Y, -22.15);
        assert_close(PADDLE_GLOW_BOTTOM_CENTER_Y, 21.7);
        assert_eq!(paddle_glow_shape90::FLATTEN_STEPS, 4);
        assert_eq!(PADDLE_GLOW_RADIAL_BANDS, 18);
        assert_swf_point(paddle_glow_shape90::SHAPE.body.start, 6.65, -22.25);
        assert_eq!(paddle_glow_shape90::SHAPE.body.segments.len(), 6);
        assert_swf_point(paddle_glow_shape90::SHAPE.top_cap.start, -6.4, -22.25);
        assert_swf_point(paddle_glow_shape90::SHAPE.bottom_cap.start, 6.65, 21.65);

        let top = paddle_glow_top_cap_points();
        assert_eq!(top.len(), 17);
        assert_swf_point(top[0], -6.4, -22.25);
        assert_swf_point(top[4], -4.5, -30.7);
        assert_swf_point(top[8], 0.15, -34.15);
        assert_swf_point(top[12], 4.75, -30.7);
        assert_swf_point(top[16], 6.65, -22.25);

        let bottom = paddle_glow_bottom_cap_points();
        assert_eq!(bottom.len(), 19);
        assert_swf_point(bottom[0], 6.65, 21.65);
        assert_swf_point(bottom[1], 6.65, 21.7);
        assert_swf_point(bottom[5], 4.75, 30.2);
        assert_swf_point(bottom[9], 0.15, 33.7);
        assert_swf_point(bottom[13], -4.5, 30.2);
        assert_swf_point(bottom[17], -6.4, 21.7);
        assert_swf_point(bottom[18], -6.4, 21.65);

        let visual = paddle_glow_visual(2.0, 0.5, SWF_WHITE);
        let center = SwfPoint::new(PADDLE_GLOW_CENTER_X, PADDLE_GLOW_TOP_CENTER_Y);
        let stage = scaled_contour_stage_point(
            ScaledContourBand {
                origin: vec2(10.0, 20.0),
                visual,
                center,
                inner_ratio: 0.0,
                outer_ratio: 1.0,
                color: SWF_WHITE,
            },
            top[8],
            1.0,
        );
        assert_point(stage, 10.3, 2.925);
    }

    #[test]
    fn paddle_ready_flash_profile_matches_swf_shape_127() {
        assert_eq!(paddle_ready_flash_shape127::DEFINE_SHAPE_ID, 127);
        assert_eq!(
            paddle_ready_flash_shape127::BOUNDS_TWIPS,
            [-77, 87, -82, 82]
        );
        assert_eq!(paddle_ready_flash_shape127::FILL_RGBA, [102, 102, 102, 171]);
        assert_eq!(paddle_ready_flash_shape127::FLATTEN_STEPS, 4);
        assert_close(paddle_ready_flash_shape127::SHAPE.bounds.x_min, -3.85);
        assert_close(paddle_ready_flash_shape127::SHAPE.bounds.x_max, 4.35);
        assert_close(paddle_ready_flash_shape127::SHAPE.bounds.y_min, -4.1);
        assert_close(paddle_ready_flash_shape127::SHAPE.bounds.y_max, 4.1);
        assert_swf_point(paddle_ready_flash_shape127::SHAPE.start, 3.15, -2.90);
        assert_eq!(paddle_ready_flash_shape127::SHAPE.segments.len(), 8);

        let final_charge = paddle_charge_visual(Side::Blue, 200)
            .ready_flash
            .expect("frame 200 should place the ready flash");
        assert_close(final_charge.alpha as f32, 1.0);
        let ready = paddle_ready_flash_visual(1.0, final_charge.alpha as f32);

        assert_swf_point(ready.center, 0.25, 0.0);
        assert_eq!(ready.points.len(), 32);
        assert_swf_point(ready.points[0], 3.15, -2.90);
        assert_swf_point(ready.points[4], 4.35, 0.0);
        assert_swf_point(ready.points[8], 3.15, 2.90);
        assert_swf_point(ready.points[12], 0.25, 4.10);
        assert_swf_point(ready.points[16], -2.65, 2.90);
        assert_swf_point(ready.points[20], -3.85, 0.0);
        assert_swf_point(ready.points[24], -2.65, -2.90);
        assert_swf_point(ready.points[28], 0.25, -4.10);
        assert_color(
            ready.color,
            Color::new(102.0 / 255.0, 102.0 / 255.0, 102.0 / 255.0, 171.0 / 255.0),
        );

        let first_charge = paddle_charge_visual(Side::Blue, 197)
            .ready_flash
            .expect("frame 197 should place the ready flash");
        assert_close(first_charge.alpha as f32, 0.0);
        let first_ready_frame =
            paddle_ready_flash_visual(first_charge.scale as f32, first_charge.alpha as f32);
        assert_swf_point(first_ready_frame.center, 0.121_952_06, 0.0);
        assert_swf_point(first_ready_frame.points[4], 2.121_966, 0.0);
        assert_close(first_ready_frame.color.a, 0.0);

        let mid_charge = paddle_charge_visual(Side::Blue, 199)
            .ready_flash
            .expect("frame 199 should place the ready flash");
        assert_close(mid_charge.alpha as f32, 171.0 / 256.0);
        let mid_ready = paddle_ready_flash_visual(mid_charge.scale as f32, mid_charge.alpha as f32);
        assert_close(mid_ready.color.a, (171.0 / 255.0) * (171.0 / 256.0));
    }

    #[test]
    fn active_ball_radial_profiles_follow_swf_sprite_16_child_shapes() {
        assert_eq!(ball_shapes::BALL_GLOW_DEFINE_SHAPE_ID, 2);
        assert_eq!(ball_shapes::BALL_GLOW_BOUNDS_TWIPS, [-280, 280, -280, 280]);
        assert_eq!(
            ball_shapes::BALL_GLOW_GRADIENT_RGBA,
            [[0, 153, 255, 255, 255], [230, 153, 255, 255, 0]]
        );
        assert_eq!(ball_shapes::BALL_FIRE_DEFINE_SHAPE_ID, 9);
        assert_eq!(ball_shapes::BALL_FIRE_BOUNDS_TWIPS, [-110, 110, -110, 110]);
        assert_eq!(
            ball_shapes::BALL_FIRE_GRADIENT_RGBA,
            [[0, 255, 68, 68, 255], [250, 238, 0, 0, 0]]
        );

        let blue = active_ball_glow_visual(Side::Blue, 1.0);
        assert_close(blue.radius, 14.0);
        assert_close(blue.shape.base_radius, 14.0);
        assert_swf_point(blue.shape.start, 9.85, -9.9);
        assert_close(blue.transparent_stop_ratio, 230.0 / 255.0);
        assert_color(blue.center, Color::new(153.0 / 255.0, 1.0, 1.0, 1.0));

        let red = active_ball_glow_visual(Side::Red, 1.0);
        assert_close(red.radius, 14.0);
        assert_close(red.transparent_stop_ratio, 230.0 / 255.0);
        assert_color(
            red.center,
            Color::new(1.0, 191.0 / 255.0, 119.0 / 255.0, 1.0),
        );

        let fire = burning_ball_fire_visual(1.0);
        assert_close(fire.radius, 5.5);
        assert_close(fire.shape.base_radius, 5.5);
        assert_swf_point(fire.shape.start, 3.9, -3.9);
        assert_close(fire.transparent_stop_ratio, 250.0 / 255.0);
        assert_color(
            fire.center,
            Color::new(1.0, 250.0 / 255.0, 17.0 / 255.0, 1.0),
        );

        assert_close(radial_fade_alpha_at(blue, 0.0), 1.0);
        assert_close(radial_fade_alpha_at(blue, 230.0 / 255.0), 0.0);
        assert!(radial_fade_band_visual(blue, RADIAL_FADE_STEPS).is_none());

        let Some(first_blue_band) = radial_fade_band_visual(blue, 0) else {
            panic!("shape 2 should emit a first radial band");
        };
        assert_close(first_blue_band.inner_radius, 0.0);
        assert_close(
            first_blue_band.outer_radius,
            14.0 * (230.0 / 255.0) / RADIAL_FADE_STEPS as f32,
        );
        assert_color(
            first_blue_band.color,
            Color::new(153.0 / 255.0, 1.0, 1.0, 1.0 - 0.5 / 18.0),
        );

        let Some(last_blue_band) = radial_fade_band_visual(blue, RADIAL_FADE_STEPS - 1) else {
            panic!("shape 2 should emit a last radial band");
        };
        assert_close(last_blue_band.outer_radius, 14.0 * 230.0 / 255.0);
        assert_close(last_blue_band.color.a, 1.0 / 36.0);

        let Some(last_fire_band) = radial_fade_band_visual(fire, RADIAL_FADE_STEPS - 1) else {
            panic!("shape 9 should emit a last radial band");
        };
        assert_close(last_fire_band.outer_radius, 5.5 * 250.0 / 255.0);
    }

    #[test]
    fn ball_radial_gradients_use_recovered_shape_contours() {
        assert_eq!(RADIAL_SHAPE_FLATTEN_STEPS, 4);

        let glow_points = radial_shape_points(BALL_GLOW_SHAPE);
        assert_eq!(glow_points.len(), 32);
        assert_swf_point(glow_points[0], 9.85, -9.9);
        assert_swf_point(glow_points[4], 14.0, 0.0);
        assert_swf_point(glow_points[8], 9.85, 9.85);
        assert_swf_point(glow_points[12], 0.0, 14.0);
        assert_swf_point(glow_points[20], -14.0, 0.0);
        assert_swf_point(glow_points[28], 0.0, -14.0);

        let fire_points = radial_shape_points(BALL_FIRE_SHAPE);
        assert_eq!(fire_points.len(), 32);
        assert_swf_point(fire_points[0], 3.9, -3.9);
        assert_swf_point(fire_points[4], 5.5, 0.0);
        assert_swf_point(fire_points[12], 0.0, 5.5);
        assert_swf_point(fire_points[20], -5.5, 0.0);
        assert_swf_point(fire_points[28], 0.0, -5.5);

        let band = RadialShapeBand {
            origin: vec2(10.0, 20.0),
            scale: 2.0,
            inner_ratio: 0.0,
            outer_ratio: 1.0,
            color: SWF_WHITE,
        };
        assert_point(
            radial_shape_stage_point(band, glow_points[4], 1.0),
            38.0,
            20.0,
        );
        assert_point(
            radial_shape_stage_point(band, glow_points[4], 0.5),
            24.0,
            20.0,
        );
    }

    #[test]
    fn scored_ball_die_ring_uses_swf_shape_4_radial_stops() {
        assert_eq!(ball_shapes::BALL_DIE_RING_DEFINE_SHAPE_ID, 4);
        assert_eq!(
            ball_shapes::BALL_DIE_RING_BOUNDS_TWIPS,
            [-230, 230, -230, 230]
        );
        assert_eq!(
            ball_shapes::BALL_DIE_RING_GRADIENT_RGBA,
            [
                [154, 222, 245, 255, 0],
                [212, 221, 254, 255, 255],
                [255, 222, 253, 255, 0],
            ]
        );

        let ring = dying_ball_ring_visual(1.0, 0.5);

        assert_close(ring.radius, 11.5);
        assert_close(ring.shape.base_radius, 11.5);
        assert_swf_point(ring.shape.start, 8.1, -8.15);
        assert_close(ring.inner_stop_ratio, 154.0 / 255.0);
        assert_close(ring.peak_stop_ratio, 212.0 / 255.0);
        assert_close(ring.outer_stop_ratio, 1.0);
        assert_color(
            ring.peak,
            Color::new(221.0 / 255.0, 254.0 / 255.0, 1.0, 0.5),
        );

        let ring_points = radial_shape_points(ring.shape);
        assert_eq!(ring_points.len(), 32);
        assert_swf_point(ring_points[4], 11.5, 0.0);
        assert_swf_point(ring_points[12], 0.0, 11.5);
        assert_swf_point(ring_points[20], -11.5, 0.0);
        assert_swf_point(ring_points[28], 0.0, -11.5);
    }

    #[test]
    fn goal_flash_events_follow_swf_redline_and_blueline_targets() {
        let mut flash = GoalFlash::default();

        flash.apply_events(&[RoundEvent::Score {
            side: Side::Blue,
            burning: false,
        }]);
        assert_eq!(flash.left, 6);
        assert_eq!(flash.right, 0);

        flash.tick();
        assert_eq!(flash.left, 5);
        assert_eq!(flash.right, 0);

        flash.apply_events(&[RoundEvent::Score {
            side: Side::Red,
            burning: false,
        }]);
        assert_eq!(flash.left, 5);
        assert_eq!(flash.right, 6);

        for _ in 0..6 {
            flash.tick();
        }
        assert_eq!(flash.left, 0);
        assert_eq!(flash.right, 0);
    }

    #[test]
    fn goal_line_flash_visual_matches_swf_sprite_125_and_126_frames() {
        assert_close(MENU_LEFT_GOAL_X, 35.05);
        assert_close(MENU_RIGHT_GOAL_X, 515.0);
        assert_eq!(PLAYFIELD_ROOT_FRAME, 58);
        assert_eq!(PLAYFIELD_SPRITE_ID, 140);
        assert_eq!(PLAYFIELD_SPRITE_DEPTH, 3);
        assert_eq!(PLAYFIELD_LEFT_GOAL_DEPTH, 1);
        assert_eq!(PLAYFIELD_RIGHT_GOAL_DEPTH, 3);
        assert_eq!(PLAYFIELD_LEFT_GOAL_CHARACTER_ID, 125);
        assert_eq!(PLAYFIELD_RIGHT_GOAL_CHARACTER_ID, 126);
        assert_eq!(PLAYFIELD_BLUE_PLAYER_DEPTH, 5);
        assert_eq!(PLAYFIELD_RED_PLAYER_DEPTH, 8);
        assert_eq!(PLAYFIELD_FIRST_DYNAMIC_BALL_DEPTH, 10);
        assert_eq!(PLAYFIELD_PAIR_CONTROLLER_DEPTH, 11);
        assert_eq!(PLAYFIELD_PAIR_CONTROLLER_CHARACTER_ID, 38);
        assert_eq!(PLAYFIELD_BLUE_SCORE_METER_DEPTH, 13);
        assert_eq!(PLAYFIELD_RED_SCORE_METER_DEPTH, 42);
        assert_eq!(PLAYFIELD_ANNOUNCE_DEPTH, 58);
        assert_eq!(PLAYFIELD_BLUE_MATCH_PIPS_DEPTH, 66);
        assert_eq!(PLAYFIELD_RED_MATCH_PIPS_DEPTH, 71);
        assert_close(PLAYFIELD_ROOT_X, 273.45);
        assert_close(PLAYFIELD_LEFT_GOAL_LOCAL_X, -238.5);
        assert_close(PLAYFIELD_RIGHT_GOAL_LOCAL_X, 241.45);
        assert_close(stage_left_goal_x(MENU_STAGE_FRAME), 35.05);
        assert_close(stage_right_goal_x(MENU_STAGE_FRAME), 515.0);
        assert_close(stage_left_goal_x(PLAYFIELD_STAGE_FRAME), 34.95);
        assert_close(stage_right_goal_x(PLAYFIELD_STAGE_FRAME), 514.9);
        assert_eq!(goal_line_shapes::RED_DEFINE_SHAPE_ID, 84);
        assert_eq!(goal_line_shapes::BLUE_DEFINE_SHAPE_ID, 87);
        assert_eq!(goal_line_shapes::BOUNDS_TWIPS, [-20, 20, -3589, 3590]);
        assert_eq!(goal_line_shapes::RED_FILL_RGB, [255, 166, 34]);
        assert_eq!(goal_line_shapes::BLUE_FILL_RGB, [0, 179, 221]);
        assert_close(goal_line_shapes::SHAPE.bounds.x_min, -1.0);
        assert_close(goal_line_shapes::SHAPE.bounds.x_max, 1.0);
        assert_close(goal_line_shapes::SHAPE.bounds.y_min, -179.45);
        assert_close(goal_line_shapes::SHAPE.bounds.y_max, 179.5);
        assert_swf_point(goal_line_shapes::SHAPE.start, -0.7, -179.15);
        assert_eq!(goal_line_shapes::SHAPE.segments.len(), 10);
        assert_close(GOAL_LINE_CENTER_Y, 214.45);
        let goal_red = goal_line_color(Side::Red);
        let goal_blue = goal_line_color(Side::Blue);
        assert_color(goal_red, Color::new(1.0, 166.0 / 255.0, 34.0 / 255.0, 1.0));
        assert_color(
            goal_blue,
            Color::new(0.0, 179.0 / 255.0, 221.0 / 255.0, 1.0),
        );

        let left_start = goal_line_visual(35.05, Side::Red, 6, &LEFT_GOAL_FLASH);
        assert_swf_point(left_start.origin, 35.30, 214.50);
        assert_close(left_start.scale_x, 1.75);
        assert_close(left_start.scale_y, 0.999_862_7);
        assert_rect_visual(goal_line_bounds(left_start), 33.55, 35.07, 3.50, 358.90);
        assert_swf_point(goal_line_fill_center(left_start), 35.30, 214.52);
        let left_start_points = goal_line_points(left_start);
        assert_eq!(left_start_points.len(), 10);
        assert_swf_point(left_start_points[0], 34.08, 35.37);
        assert_swf_point(left_start_points[1], 35.30, 35.07);
        assert_swf_point(left_start_points[3], 37.05, 36.07);
        assert_swf_point(left_start_points[6], 35.30, 393.98);
        assert_swf_point(left_start_points[8], 33.55, 392.98);
        assert_color(left_start.color, SWF_WHITE);

        let left_end = goal_line_visual(35.05, Side::Red, 1, &LEFT_GOAL_FLASH);
        assert_swf_point(left_end.origin, 35.05, 214.45);
        assert_close(left_end.scale_x, 1.0);
        assert_close(left_end.scale_y, 1.0);
        assert_rect_visual(goal_line_bounds(left_end), 34.05, 35.00, 2.0, 358.95);
        assert_swf_point(goal_line_fill_center(left_end), 35.05, 214.48);
        let left_end_points = goal_line_points(left_end);
        assert_swf_point(left_end_points[0], 34.35, 35.30);
        assert_swf_point(left_end_points[1], 35.05, 35.00);
        assert_swf_point(left_end_points[3], 36.05, 36.00);
        assert_swf_point(left_end_points[6], 35.05, 393.95);
        assert_swf_point(left_end_points[8], 34.05, 392.95);
        assert_color(left_end.color, goal_red);

        let right_second = goal_line_visual(515.0, Side::Blue, 5, &RIGHT_GOAL_FLASH);
        assert_swf_point(right_second.origin, 515.00, 214.45);
        assert_close(right_second.scale_x, 1.599_975_6);
        assert_close(right_second.scale_y, 1.0);
        assert_rect_visual(goal_line_bounds(right_second), 513.40, 35.00, 3.20, 358.95);
        assert_color(
            right_second.color,
            Color::new(
                204.0 / 255.0,
                goal_blue.g.mul_add(51.0 / 256.0, 204.0 / 255.0),
                goal_blue.b.mul_add(51.0 / 256.0, 204.0 / 255.0),
                1.0,
            ),
        );

        let idle = goal_line_visual(515.0, Side::Blue, 0, &RIGHT_GOAL_FLASH);
        assert_swf_point(idle.origin, 515.0, 214.45);
        assert_close(idle.scale_x, 1.0);
        assert_close(idle.scale_y, 1.0);
        assert_rect_visual(goal_line_bounds(idle), 514.00, 35.00, 2.0, 358.95);
        assert_color(idle.color, goal_blue);

        let playfield_left = goal_line_visual(
            stage_left_goal_x(PLAYFIELD_STAGE_FRAME),
            Side::Red,
            0,
            &LEFT_GOAL_FLASH,
        );
        assert_swf_point(playfield_left.origin, 34.95, 214.45);
        assert_rect_visual(goal_line_bounds(playfield_left), 33.95, 35.00, 2.0, 358.95);

        let playfield_right = goal_line_visual(
            stage_right_goal_x(PLAYFIELD_STAGE_FRAME),
            Side::Blue,
            0,
            &RIGHT_GOAL_FLASH,
        );
        assert_swf_point(playfield_right.origin, 514.90, 214.45);
        assert_rect_visual(
            goal_line_bounds(playfield_right),
            513.90,
            35.00,
            2.0,
            358.95,
        );
    }

    #[test]
    fn sprite_38_pair_controller_is_action_only_not_rendered_art() {
        let swf = include_bytes!("../gravity_arcade.swf");
        let root_tags = collect_swf_tags(swf, swf_first_tag_offset(swf));
        let sprite_38 = root_tags.iter().find(|tag| {
            tag.code == SWF_TAG_DEFINE_SPRITE
                && tag.body.len() >= 4
                && read_le_u16(tag.body, 0) == PLAYFIELD_PAIR_CONTROLLER_CHARACTER_ID
        });
        let Some(sprite_38) = sprite_38 else {
            panic!("DefineSprite 38 should exist in gravity_arcade.swf");
        };

        assert_eq!(read_le_u16(sprite_38.body, 2), 3);
        let mut show_frames = 0;
        let mut actions = 0;
        let mut sound_stream_heads = 0;
        for tag in collect_swf_tags(sprite_38.body, 4) {
            match tag.code {
                SWF_TAG_SHOW_FRAME => show_frames += 1,
                SWF_TAG_DO_ACTION => actions += 1,
                SWF_TAG_SOUND_STREAM_HEAD2 => sound_stream_heads += 1,
                SWF_TAG_PLACE_OBJECT
                | SWF_TAG_REMOVE_OBJECT
                | SWF_TAG_PLACE_OBJECT2
                | SWF_TAG_REMOVE_OBJECT2
                | SWF_TAG_PLACE_OBJECT3 => {
                    panic!("sprite 38 should not mutate the display list");
                },
                _ => {},
            }
        }

        assert_eq!(show_frames, 3);
        assert_eq!(actions, 3);
        assert_eq!(sound_stream_heads, 1);
    }

    #[test]
    fn attached_ball_depth_end_keeps_dying_ball_visible_after_id_reset() {
        let mut world = World::new(Settings::default());
        world.next_ball_id = PLAYFIELD_FIRST_DYNAMIC_BALL_DEPTH;
        world
            .dying_balls
            .push(DyingBall::from_scored_ball(gravityarcade::sim::Ball {
                id: 37,
                x: 0.0,
                y: 0.0,
                prev_y: 0.0,
                vx: 0.0,
                vy: 0.0,
                energy: 100.0,
                color: Side::Blue,
                age: 0,
            }));

        assert_eq!(attached_ball_depth_end(&world), 38);
    }

    #[test]
    fn score_meter_visuals_follow_swf_sprite_136_and_139_frames() {
        assert_eq!(score_meter_constants::PLAYFIELD_SPRITE_ID, 140);
        assert_eq!(score_meter_constants::RED_SCORE_METER_SPRITE_ID, 136);
        assert_eq!(score_meter_constants::BLUE_SCORE_METER_SPRITE_ID, 139);

        let blue_empty = score_meter_base_marker_visuals(Side::Blue);
        assert_eq!(blue_empty.len(), 13);
        assert_close(blue_empty[0].x, 19.0);
        assert_close(blue_empty[0].y, 214.45);
        assert_close(blue_empty[0].scale, 1.0);
        assert_color(blue_empty[0].color, SIDE_MARKER_EMPTY_FILL);
        assert_close(blue_empty[1].y, 187.05);
        assert_close(blue_empty[2].y, 241.85);
        assert_close(blue_empty[12].y, 378.85);

        let red_empty = score_meter_base_marker_visuals(Side::Red);
        assert_eq!(red_empty.len(), 13);
        assert_close(red_empty[0].x, 530.95);
        assert_close(red_empty[0].y, 214.45);

        let blue_start = score_marker_visuals(10, Side::Blue);
        assert_eq!(blue_start.len(), 1);
        assert_close(blue_start[0].x, 19.0);
        assert_close(blue_start[0].y, 214.45);
        assert_close(blue_start[0].scale, 0.241_073_61);
        assert_color(
            blue_start[0].color,
            Color::new(0.0, 46.0 / 255.0, 47.0 / 255.0, 1.0),
        );

        let red_start = score_marker_visuals(10, Side::Red);
        assert_eq!(red_start.len(), 1);
        assert_close(red_start[0].x, 530.95);
        assert_close(red_start[0].y, 214.45);
        assert_close(red_start[0].scale, 0.240_005_5);
        assert_color(
            red_start[0].color,
            Color::new(45.0 / 255.0, 32.0 / 255.0, 0.0, 1.0),
        );

        let next_pair = score_marker_visuals(100, Side::Blue);
        assert_eq!(next_pair.len(), 3);
        assert_close(next_pair[0].scale, 1.0);
        assert_close(next_pair[1].x, 19.0);
        assert_close(next_pair[1].y, 187.05);
        assert_close(next_pair[1].scale, 0.241_073_61);
        assert_close(next_pair[2].y, 241.85);
    }

    #[test]
    fn max_score_meter_uses_swf_frame_71_final_sprite() {
        assert_eq!(score_meter_constants::RED_FINAL_SCORE_METER_SPRITE_ID, 135);
        assert_eq!(score_meter_constants::BLUE_FINAL_SCORE_METER_SPRITE_ID, 138);
        assert_eq!(score_meter_constants::RED_FINAL_OVERLAY_SPRITE_ID, 134);
        assert_eq!(score_meter_constants::BLUE_FINAL_OVERLAY_SPRITE_ID, 137);
        assert_eq!(score_meter_constants::FINAL_OVERLAY_OUTLINE_SHAPE_ID, 133);

        let blue_final = score_marker_visuals(700, Side::Blue);
        assert_eq!(blue_final.len(), 13);
        assert_close(blue_final[0].x, 19.0);
        assert_close(blue_final[0].y, 214.45);
        assert_close(blue_final[0].scale, 1.0);
        assert_color(blue_final[0].color, SIDE_MARKER_EMPTY_FILL);
        assert_close(blue_final[1].x, 19.0);
        assert_close(blue_final[1].y, 187.05);
        assert_close(blue_final[1].scale, 0.241_073_61);
        assert_color(
            blue_final[1].color,
            Color::new(0.0, 99.0 / 255.0, 102.0 / 255.0, 1.0),
        );

        let red_final = score_marker_visuals(700, Side::Red);
        assert_eq!(red_final.len(), 13);
        assert_close(red_final[0].x, 530.95);
        assert_close(red_final[0].y, 214.45);
        assert_close(red_final[0].scale, 1.0);
        assert_color(red_final[0].color, SIDE_MARKER_EMPTY_FILL);
        assert_close(red_final[1].x, 530.95);
        assert_close(red_final[1].y, 187.05);
        assert_close(red_final[1].scale, 0.240_005_5);
        assert_color(
            red_final[1].color,
            Color::new(102.0 / 255.0, 73.0 / 255.0, 0.0, 1.0),
        );

        let blue_overlay = final_score_overlay_visuals(Side::Blue, 0);
        assert_eq!(blue_overlay.len(), 13);
        assert_close(blue_overlay[0].x, 19.0);
        assert_close(blue_overlay[0].y, 50.05);
        assert_color(
            blue_overlay[0].color,
            Color::new(34.0 / 255.0, 224.0 / 255.0, 1.0, 1.0),
        );
        assert_close(blue_overlay[6].x, 18.95);
        assert_close(blue_overlay[6].y, 214.45);

        let red_overlay = final_score_overlay_visuals(Side::Red, 0);
        assert_eq!(red_overlay.len(), 13);
        assert_close(red_overlay[0].x, 531.0);
        assert_close(red_overlay[0].y, 50.05);
        assert_color(
            red_overlay[0].color,
            Color::new(1.0, 210.0 / 255.0, 136.0 / 255.0, 1.0),
        );
        assert_close(red_overlay[6].x, 530.95);
        assert_close(red_overlay[6].y, 214.45);

        let blue_mid_cycle = final_score_overlay_visuals(Side::Blue, 6);
        assert_color(
            blue_mid_cycle[0].color,
            Color::new(0.0, 124.0 / 255.0, 153.0 / 255.0, 1.0),
        );
        let red_looped = final_score_overlay_visuals(Side::Red, 13);
        assert_color(red_looped[0].color, red_overlay[0].color);
    }

    #[test]
    fn match_pip_palette_matches_swf_shapes() {
        assert_eq!(match_pip_shapes::BLUE_DEFINE_SHAPE_ID, 147);
        assert_eq!(match_pip_shapes::RED_DEFINE_SHAPE_ID, 151);
        assert_eq!(match_pip_shapes::BLUE_BOUNDS_TWIPS, [-223, 223, -223, 223]);
        assert_eq!(match_pip_shapes::RED_BOUNDS_TWIPS, [-223, 223, -223, 223]);
        assert_eq!(
            match_pip_shapes::BLUE_FILL_RGB,
            [
                [140, 140, 176],
                [0, 184, 209],
                [115, 232, 255],
                [0, 197, 237],
                [204, 249, 255],
                [74, 212, 255],
            ]
        );
        assert_eq!(
            match_pip_shapes::RED_FILL_RGB,
            [
                [140, 140, 176],
                [255, 222, 115],
                [209, 149, 0],
                [237, 170, 0],
                [255, 204, 74],
                [255, 241, 204],
            ]
        );
        assert_close(MATCH_PIP_RADIUS, 11.15);
        assert_color(
            MATCH_PIP_OUTER,
            Color::new(140.0 / 255.0, 140.0 / 255.0, 176.0 / 255.0, 1.0),
        );

        let blue = match_pip_palette(Side::Blue);
        assert_color(blue.outer, MATCH_PIP_OUTER);
        assert_color(
            blue.core,
            Color::new(0.0, 184.0 / 255.0, 209.0 / 255.0, 1.0),
        );
        assert_color(
            blue.highlight,
            Color::new(115.0 / 255.0, 232.0 / 255.0, 1.0, 1.0),
        );
        assert_color(blue.mid, Color::new(0.0, 197.0 / 255.0, 237.0 / 255.0, 1.0));
        assert_color(
            blue.shine,
            Color::new(204.0 / 255.0, 249.0 / 255.0, 1.0, 1.0),
        );
        assert_color(
            blue.accent,
            Color::new(74.0 / 255.0, 212.0 / 255.0, 1.0, 1.0),
        );

        let red = match_pip_palette(Side::Red);
        assert_color(red.outer, MATCH_PIP_OUTER);
        assert_color(red.core, Color::new(209.0 / 255.0, 149.0 / 255.0, 0.0, 1.0));
        assert_color(
            red.highlight,
            Color::new(1.0, 222.0 / 255.0, 115.0 / 255.0, 1.0),
        );
        assert_color(red.mid, Color::new(237.0 / 255.0, 170.0 / 255.0, 0.0, 1.0));
        assert_color(
            red.accent,
            Color::new(1.0, 204.0 / 255.0, 74.0 / 255.0, 1.0),
        );
        assert_color(
            red.shine,
            Color::new(1.0, 241.0 / 255.0, 204.0 / 255.0, 1.0),
        );
    }

    #[test]
    fn match_pip_contours_follow_swf_define_shapes_147_and_151() {
        assert_eq!(MATCH_PIP_FLATTEN_STEPS, 4);
        assert_eq!(BLUE_MATCH_PIP_CONTOURS.len() + 1, 13);
        assert_eq!(RED_MATCH_PIP_CONTOURS.len() + 1, 13);

        let blue_left = BLUE_MATCH_PIP_CONTOURS[1];
        assert_eq!(blue_left.slot, MatchPipSlot::Core);
        assert_swf_point(blue_left.start, -8.1, -7.7);
        assert_eq!(blue_left.segments.len(), 8);
        match blue_left.segments[0] {
            SwfPathSegment::Quad { control, to } => {
                assert_swf_point(control, -9.8, -4.9);
                assert_swf_point(to, -9.6, -0.9);
            },
            SwfPathSegment::Line(_) => panic!("expected first blue core segment to be quadratic"),
        }

        let blue_outer_right = BLUE_MATCH_PIP_CONTOURS[3];
        assert_eq!(blue_outer_right.slot, MatchPipSlot::Highlight);
        assert_swf_point(blue_outer_right.start, -9.6, -0.9);
        assert_eq!(blue_outer_right.segments.len(), 12);

        let blue_accent = BLUE_MATCH_PIP_ACCENT;
        assert_eq!(blue_accent.slot, MatchPipSlot::Accent);
        assert_swf_point(blue_accent.start, 3.55, -7.0);
        assert_eq!(blue_accent.segments.len(), 3);
        assert_color(
            match_pip_slot_color(blue_accent.slot, match_pip_palette(Side::Blue)),
            Color::new(74.0 / 255.0, 212.0 / 255.0, 1.0, 1.0),
        );

        let red_left = RED_MATCH_PIP_CONTOURS[4];
        assert_eq!(red_left.slot, MatchPipSlot::Core);
        assert_swf_point(red_left.start, -8.1, -7.7);
        assert_color(
            match_pip_slot_color(red_left.slot, match_pip_palette(Side::Red)),
            Color::new(209.0 / 255.0, 149.0 / 255.0, 0.0, 1.0),
        );

        let red_accent = RED_MATCH_PIP_CONTOURS[9];
        assert_eq!(red_accent.slot, MatchPipSlot::Accent);
        assert_swf_point(red_accent.start, 3.55, -7.0);
        assert_color(
            match_pip_slot_color(red_accent.slot, match_pip_palette(Side::Red)),
            Color::new(1.0, 204.0 / 255.0, 74.0 / 255.0, 1.0),
        );
    }

    #[test]
    fn match_pip_contours_cover_recovered_shape_bounds() {
        let blue_bounds = match_pip_local_bounds(
            BLUE_MATCH_PIP_CONTOURS
                .iter()
                .copied()
                .chain([BLUE_MATCH_PIP_ACCENT]),
        );
        assert_swf_point(blue_bounds.0, -11.15, -11.15);
        assert_swf_point(blue_bounds.1, 11.15, 11.15);

        let red_bounds = match_pip_local_bounds(
            RED_MATCH_PIP_CONTOURS
                .iter()
                .copied()
                .chain([RED_MATCH_PIP_TOP_SHINE]),
        );
        assert_swf_point(red_bounds.0, -11.15, -11.15);
        assert_swf_point(red_bounds.1, 11.15, 11.15);
    }

    #[test]
    fn match_pip_contours_triangulate_without_circle_fallbacks() {
        let gray_seam = match_pip_contour_points(BLUE_MATCH_PIP_CONTOURS[0], 0.0, 0.0, 1.0);
        assert!(triangulate_polygon_indices(&gray_seam).is_empty());

        let outer_right = match_pip_contour_points(BLUE_MATCH_PIP_CONTOURS[3], 0.0, 0.0, 1.0);
        assert_eq!(outer_right.len(), 33);
        assert!(polygon_signed_area(&outer_right).abs() > 200.0);

        let triangles = triangulate_polygon_indices(&outer_right);
        assert_eq!(triangles.len(), outer_right.len() - 2);
        assert!(
            triangles
                .iter()
                .flatten()
                .all(|&index| index < outer_right.len())
        );
    }

    fn match_pip_local_bounds(
        contours: impl Iterator<Item = MatchPipContour>,
    ) -> (SwfPoint, SwfPoint) {
        let mut min = SwfPoint::new(f32::INFINITY, f32::INFINITY);
        let mut max = SwfPoint::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
        for contour in contours {
            for point in match_pip_contour_points(contour, 0.0, 0.0, 1.0) {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                max.x = max.x.max(point.x);
                max.y = max.y.max(point.y);
            }
        }
        (min, max)
    }

    #[test]
    fn match_pip_hud_positions_follow_swf_sprites_163_and_164() {
        assert_close(MATCH_PIP_Y, 18.7);
        assert_eq!(match_pip_visuals(0, Side::Blue), Vec::new());

        let blue_one = match_pip_visuals(1, Side::Blue);
        assert_eq!(blue_one.len(), 1);
        assert_close(blue_one[0].x, 200.65);
        assert_close(blue_one[0].y, MATCH_PIP_Y);
        assert_eq!(blue_one[0].side, Side::Blue);

        let blue_full = match_pip_visuals(4, Side::Blue);
        assert_eq!(blue_full.len(), 4);
        assert_close(blue_full[0].x, 110.8);
        assert_close(blue_full[1].x, 140.75);
        assert_close(blue_full[2].x, 170.7);
        assert_close(blue_full[3].x, 200.65);

        let red_one = match_pip_visuals(1, Side::Red);
        assert_eq!(red_one.len(), 1);
        assert_close(red_one[0].x, 336.35);
        assert_close(red_one[0].y, MATCH_PIP_Y);
        assert_eq!(red_one[0].side, Side::Red);

        let red_full = match_pip_visuals(9, Side::Red);
        assert_eq!(red_full.len(), 4);
        assert_close(red_full[0].x, 336.35);
        assert_close(red_full[1].x, 366.3);
        assert_close(red_full[2].x, 396.25);
        assert_close(red_full[3].x, 426.2);
    }

    #[test]
    fn announce_text_records_use_recovered_swf_glyph_contours() {
        assert_announce_text_definition(
            &announce_texts::BLUE_WINS,
            "Blue Wins",
            146,
            &[26],
            [136, 247, 255],
            [575, 6215, 295, 1190],
            12,
            127,
            346,
        );
        assert_announce_text_definition(
            &announce_texts::RED_WINS,
            "Red Wins",
            150,
            &[26],
            [255, 178, 51],
            [740, 6055, 295, 1190],
            11,
            119,
            323,
        );
        assert_announce_text_definition(
            &announce_texts::ROUND,
            "Round",
            152,
            &[26],
            [255, 255, 255],
            [5655, 9360, 295, 1190],
            8,
            80,
            218,
        );
        assert_announce_text_definition(
            &announce_texts::BLUE_MATCH,
            "Match",
            157,
            &[26],
            [136, 247, 255],
            [1495, 5215, 295, 1190],
            6,
            87,
            216,
        );
        assert_announce_text_definition(
            &announce_texts::RED_MATCH,
            "Match",
            160,
            &[26],
            [255, 178, 51],
            [1495, 5215, 295, 1190],
            6,
            87,
            216,
        );

        assert_rect_visual(
            announce_text_local_bounds(&announce_texts::BLUE_WINS),
            5.75,
            2.95,
            56.40,
            8.95,
        );
        assert_rect_visual(
            announce_text_local_bounds(&announce_texts::ROUND),
            56.55,
            2.95,
            37.05,
            8.95,
        );
    }

    #[test]
    fn round_number_edit_text_uses_recovered_embedded_digit_contours() {
        assert_round_number_definition(
            &round_number_texts::ONE,
            "1",
            154,
            153,
            [255, 255, 255],
            [-200, 5015, -200, 1820],
            1,
            21,
            39,
        );
        assert_round_number_definition(
            &round_number_texts::TWO,
            "2",
            154,
            153,
            [255, 255, 255],
            [-200, 5015, -200, 1820],
            1,
            34,
            94,
        );
        assert_round_number_definition(
            &round_number_texts::THREE,
            "3",
            154,
            153,
            [255, 255, 255],
            [-200, 5015, -200, 1820],
            1,
            58,
            166,
        );

        assert_rect_visual(
            round_number_text_local_bounds(&round_number_texts::ONE),
            -2.0,
            -2.0,
            52.15,
            20.20,
        );
        assert_eq!(
            round_number_text_kind("1"),
            Some(AnnounceTextKind::RoundNumber1)
        );
        assert_eq!(
            round_number_text_kind("2"),
            Some(AnnounceTextKind::RoundNumber2)
        );
        assert_eq!(
            round_number_text_kind("3"),
            Some(AnnounceTextKind::RoundNumber3)
        );
        assert_eq!(round_number_text_kind("4"), None);
        assert!(
            round_number_announce_text_visual(
                "4",
                WinTextTransform::new(ROUND_NUMBER_INNER_SCALE, 0.0, 0.0, 256),
            )
            .is_none()
        );
    }

    #[test]
    fn round_intro_visual_uses_swf_sprite_162_newgame_frames() {
        assert!(round_intro_visual(1, ROUND_INTRO_VISUAL_TICKS).is_empty());
        assert!(round_intro_visual(1, 56).is_empty());

        let growing = round_intro_visual(2, 55);
        assert_eq!(growing.len(), 2);
        assert_eq!(growing[0].kind, AnnounceTextKind::Round);
        assert_eq!(growing[0].text, "Round");
        assert_close(growing[0].x, 288.67);
        assert_close(growing[0].baseline_y, 184.04);
        assert_eq!(growing[0].font_size, 22);
        assert_close(growing[0].alpha, 20.0 / 256.0);
        assert_eq!(growing[1].kind, AnnounceTextKind::RoundNumber2);
        assert_eq!(growing[1].text, "2");
        assert_close(growing[1].x, 283.06);
        assert_close(growing[1].baseline_y, 221.30);
        assert_eq!(growing[1].font_size, 43);

        let settled = round_intro_visual(7, 43);
        assert_eq!(settled.len(), 1);
        assert_close(settled[0].x, 294.54);
        assert_close(settled[0].baseline_y, 178.02);
        assert_eq!(settled[0].font_size, 45);
        assert_close(settled[0].alpha, 1.0);

        let fading = round_intro_visual(3, 12);
        assert_eq!(fading.len(), 2);
        assert_eq!(fading[1].kind, AnnounceTextKind::RoundNumber3);
        assert_eq!(fading[1].text, "3");
        assert_close(fading[0].x, 295.58);
        assert_close(fading[0].baseline_y, 177.57);
        assert_eq!(fading[0].font_size, 49);
        assert_close(fading[0].alpha, 235.0 / 256.0);
        assert_close(fading[1].baseline_y, 260.64);
        assert_eq!(fading[1].font_size, 97);

        assert!(round_intro_visual(1, 1).is_empty());
    }

    #[test]
    fn games_played_delta_matches_swf_cgi_pusher_counter() {
        assert_eq!(
            games_played_delta(&[
                RoundEvent::Score {
                    side: Side::Blue,
                    burning: false,
                },
                RoundEvent::MatchWin { side: Side::Blue },
                RoundEvent::RoundStart,
                RoundEvent::MatchWin { side: Side::Red },
            ]),
            2
        );
    }

    #[test]
    fn match_win_audio_cues_follow_swf_sprite_162_start_sound_tags() {
        assert_eq!(cues_for_events(&[RoundEvent::RoundStart]), Vec::new());
        assert_eq!(
            cues_for_events(&[RoundEvent::RoundIntroSound]),
            vec![Cue::RoundStart]
        );
        assert_eq!(
            cues_for_events(&[RoundEvent::MatchWin { side: Side::Blue }]),
            Vec::new()
        );
        assert_eq!(
            cues_for_events(&[RoundEvent::MatchWin { side: Side::Red }]),
            Vec::new()
        );
        assert_eq!(
            cues_for_events(&[RoundEvent::RoundLostSound { side: Side::Blue }]),
            vec![Cue::RoundLost]
        );
        assert_eq!(
            cues_for_events(&[RoundEvent::RoundLostSound { side: Side::Red }]),
            vec![Cue::RoundLost]
        );
        assert_eq!(
            cues_for_events(&[RoundEvent::FinalMatchWinSound { side: Side::Blue }]),
            vec![Cue::BlueMatchWin]
        );
        assert_eq!(
            cues_for_events(&[RoundEvent::FinalMatchWinSound { side: Side::Red }]),
            vec![Cue::RedMatchWin]
        );
    }

    #[test]
    fn gameplay_audio_cues_follow_swf_sound_sources() {
        assert_eq!(
            cues_for_events(&[RoundEvent::Shot { side: Side::Blue }]),
            vec![Cue::Shot]
        );
        assert_eq!(
            cues_for_events(&[RoundEvent::PaddleHit { side: Side::Red }]),
            vec![Cue::Reflect]
        );
        assert_eq!(
            cues_for_events(&[RoundEvent::WallBounce]),
            vec![Cue::Reflect]
        );
        assert_eq!(cues_for_events(&[RoundEvent::Merge]), vec![Cue::Merge]);
        assert_eq!(
            cues_for_events(&[RoundEvent::Score {
                side: Side::Blue,
                burning: false,
            }]),
            vec![Cue::ScoreLine]
        );
        assert_eq!(
            cues_for_events(&[RoundEvent::Score {
                side: Side::Red,
                burning: true,
            }]),
            vec![Cue::ScoreLine, Cue::PaddleStun]
        );

        let swf = include_bytes!("../gravity_arcade.swf");
        let root_tags = collect_swf_tags(swf, swf_first_tag_offset(swf));
        let define_sound_ids = root_tags
            .iter()
            .filter(|tag| tag.code == SWF_TAG_DEFINE_SOUND)
            .map(|tag| read_le_u16(tag.body, 0))
            .collect::<Vec<_>>();
        for sound_id in [
            SWF_SOUND_REFLECT_ID,
            SWF_SOUND_MERGE_ID,
            SWF_SOUND_SHOT_ID,
            SWF_SOUND_SCORE_LINE_ID,
            SWF_SOUND_PADDLE_STUN_ID,
        ] {
            assert!(
                define_sound_ids.contains(&sound_id),
                "DefineSound {sound_id} should exist"
            );
        }

        let mut start_sounds = Vec::new();
        collect_swf_start_sounds(swf, swf_first_tag_offset(swf), None, &mut start_sounds);
        for expected in [
            SwfStartSound {
                sprite: Some(8),
                frame: 2,
                sound_id: SWF_SOUND_MERGE_ID,
            },
            SwfStartSound {
                sprite: Some(16),
                frame: 1,
                sound_id: SWF_SOUND_SHOT_ID,
            },
            SwfStartSound {
                sprite: Some(86),
                frame: 2,
                sound_id: SWF_SOUND_SCORE_LINE_ID,
            },
            SwfStartSound {
                sprite: Some(88),
                frame: 2,
                sound_id: SWF_SOUND_SCORE_LINE_ID,
            },
            SwfStartSound {
                sprite: Some(125),
                frame: 2,
                sound_id: SWF_SOUND_SCORE_LINE_ID,
            },
            SwfStartSound {
                sprite: Some(126),
                frame: 2,
                sound_id: SWF_SOUND_SCORE_LINE_ID,
            },
            SwfStartSound {
                sprite: Some(132),
                frame: 4,
                sound_id: SWF_SOUND_PADDLE_STUN_ID,
            },
        ] {
            assert!(
                start_sounds.contains(&expected),
                "missing SWF StartSound placement: {expected:?}"
            );
        }
    }

    #[test]
    fn audio_assets_keep_native_wav_conversions_and_choose_runtime_format() {
        macro_rules! assert_swf_wav_asset {
            ($sound_id:expr, $name:literal, $byte_len:expr) => {{
                let bytes = include_bytes!(concat!("../assets/sounds/", $name, ".wav"));
                assert_eq!(
                    bytes.len(),
                    $byte_len,
                    "DefineSound {} ({}) should match the native WAV conversion size",
                    $sound_id,
                    $name
                );
                assert!(
                    bytes.starts_with(b"RIFF"),
                    "DefineSound {} ({}) should start with a WAV RIFF header",
                    $sound_id,
                    $name
                );
            }};
        }

        assert_swf_wav_asset!(1, "reflect", 21_966);
        assert_swf_wav_asset!(6, "merge", 6_700);
        assert_swf_wav_asset!(12, "shot", 11_564);
        assert_swf_wav_asset!(23, "sound_23", 9_260);
        assert_swf_wav_asset!(24, "sound_24", 30_030);
        assert_swf_wav_asset!(85, "score_line", 13_902);
        assert_swf_wav_asset!(131, "paddle_stun", 27_944);
        assert_swf_wav_asset!(149, "round_lost", 39_212);
        assert_swf_wav_asset!(156, "round_start", 28_618);
        assert_swf_wav_asset!(159, "blue_match_win", 50_766);
        assert_swf_wav_asset!(161, "red_match_win", 48_462);

        let swf = include_bytes!("../gravity_arcade.swf");
        let mut start_sound_infos = Vec::new();
        collect_swf_start_sound_infos(swf, swf_first_tag_offset(swf), None, &mut start_sound_infos);
        assert!(start_sound_infos.contains(&(
            SwfStartSound {
                sprite: Some(8),
                frame: 2,
                sound_id: SWF_SOUND_MERGE_ID,
            },
            SwfSoundInfo {
                in_point: None,
                out_point: Some(13_312),
                loop_count: None,
                envelope: vec![
                    (200, 23_283, 20_696),
                    (7_100, 18_109, 12_647),
                    (12_400, 0, 0)
                ],
            },
        )));
        assert!(start_sound_infos.contains(&(
            SwfStartSound {
                sprite: Some(16),
                frame: 1,
                sound_id: SWF_SOUND_SHOT_ID,
            },
            SwfSoundInfo {
                in_point: None,
                out_point: None,
                loop_count: None,
                envelope: vec![(0, 9_773, 9_485)],
            },
        )));
        assert!(start_sound_infos.contains(&(
            SwfStartSound {
                sprite: Some(132),
                frame: 4,
                sound_id: SWF_SOUND_PADDLE_STUN_ID,
            },
            SwfSoundInfo {
                in_point: None,
                out_point: Some(55_800),
                loop_count: None,
                envelope: Vec::new(),
            },
        )));
        assert!(start_sound_infos.contains(&(
            SwfStartSound {
                sprite: Some(162),
                frame: 212,
                sound_id: SWF_SOUND_ROUND_START_ID,
            },
            SwfSoundInfo {
                in_point: None,
                out_point: Some(57_148),
                loop_count: None,
                envelope: vec![
                    (0, 0, 0),
                    (7_500, 32_768, 32_768),
                    (41_100, 32_768, 32_768),
                    (57_247, 0, 0),
                ],
            },
        )));

        let mut button_sound_infos = Vec::new();
        collect_swf_button_sound_infos(swf, swf_first_tag_offset(swf), &mut button_sound_infos);
        let rollover = button_sound_infos
            .iter()
            .find(|(button_id, _)| *button_id == 25)
            .and_then(|(_, transitions)| transitions[1].as_ref());
        assert_eq!(
            rollover,
            Some(&(
                SWF_SOUND_BUTTON_ROLLOVER_ID,
                SwfSoundInfo {
                    in_point: None,
                    out_point: None,
                    loop_count: None,
                    envelope: vec![(0, 3_162, 3_737)],
                },
            ))
        );

        let selected_reflect = swf_sound_bytes!("reflect");
        assert_eq!(AUDIO_RUNTIME_FORMAT, "wav");
        assert_eq!(
            selected_reflect.len(),
            include_bytes!("../assets/sounds/reflect.wav").len()
        );
        assert!(selected_reflect.starts_with(b"RIFF"));
    }

    #[test]
    fn blue_win_announce_uses_swf_sprite_162_frames() {
        assert_eq!(match_win_announce_text_visual(Side::Blue, 96), None);

        let settled = match_win_announce_text_visual(Side::Blue, 79)
            .expect("blue announce should show settled text at sprite 162 frame 19");
        assert_eq!(settled.kind, AnnounceTextKind::BlueWins);
        assert_eq!(settled.text, "Blue Wins");
        assert_close(settled.x, 294.87);
        assert_close(settled.baseline_y, 162.16);
        assert_eq!(settled.font_size, 67);
        assert_close(settled.alpha, 1.0);

        let fading = match_win_announce_text_visual(Side::Blue, 26)
            .expect("blue announce should fade text at sprite 162 frame 72");
        assert_close(fading.x, settled.x);
        assert_close(fading.baseline_y, settled.baseline_y);
        assert_eq!(fading.font_size, settled.font_size);
        assert_close(fading.alpha, 246.0 / 256.0);

        assert_eq!(match_win_announce_text_visual(Side::Blue, 0), None);
    }

    #[test]
    fn red_win_announce_uses_swf_sprite_162_frames() {
        assert_eq!(match_win_announce_text_visual(Side::Red, 96), None);
        assert_eq!(match_win_announce_text_visual(Side::Red, 95), None);

        let growing = match_win_announce_text_visual(Side::Red, 94)
            .expect("red announce should grow text at sprite 162 frame 100");
        assert_eq!(growing.kind, AnnounceTextKind::RedWins);
        assert_eq!(growing.text, "Red Wins");
        assert_close(growing.x, 289.03);
        assert_close(growing.baseline_y, 132.28);
        assert_eq!(growing.font_size, 24);
        assert_close(growing.alpha, 5.0 / 256.0);

        let settled = match_win_announce_text_visual(Side::Red, 79)
            .expect("red announce should show settled text at sprite 162 frame 115");
        assert_close(settled.x, 295.02);
        assert_close(settled.baseline_y, 162.16);
        assert_eq!(settled.font_size, 67);
        assert_close(settled.alpha, 1.0);

        let fading = match_win_announce_text_visual(Side::Red, 25)
            .expect("red announce should fade text at sprite 162 frame 169");
        assert_close(fading.x, settled.x);
        assert_close(fading.baseline_y, settled.baseline_y);
        assert_eq!(fading.font_size, settled.font_size);
        assert_close(fading.alpha, 246.0 / 256.0);

        assert_eq!(match_win_announce_text_visual(Side::Red, 0), None);
    }

    #[test]
    fn blue_final_announce_uses_swf_sprite_162_final_layout() {
        assert!(final_win_announce_visual(Side::Blue, 162).is_empty());

        let growing = final_win_announce_visual(Side::Blue, 161);
        assert_eq!(growing.len(), 5);
        let AnnounceVisual::Text(growing_title) = growing[0] else {
            panic!("first blue final visual should be title text");
        };
        assert_eq!(growing_title.kind, AnnounceTextKind::BlueWins);
        assert_eq!(growing_title.text, "Blue Wins");
        assert_close(growing_title.x, 288.99);
        assert_close(growing_title.baseline_y, 121.01);
        assert_eq!(growing_title.font_size, 24);
        assert_close(growing_title.alpha, 5.0 / 256.0);
        let AnnounceVisual::Text(growing_match) = growing[1] else {
            panic!("second blue final visual should be Match text");
        };
        assert_eq!(growing_match.kind, AnnounceTextKind::BlueMatch);
        assert_eq!(growing_match.text, "Match");
        assert_close(growing_match.x, 290.98);
        assert_close(growing_match.baseline_y, 203.01);
        assert_eq!(growing_match.font_size, 36);
        let AnnounceVisual::Pip(growing_center_pip) = growing[3] else {
            panic!("fourth blue final visual should be center match pip");
        };
        assert_close(growing_center_pip.x, 282.95);
        assert_close(growing_center_pip.y, 148.19);
        assert_close(growing_center_pip.scale, 1.49);

        let settled = final_win_announce_visual(Side::Blue, 146);
        assert_eq!(settled.len(), 5);
        let AnnounceVisual::Text(settled_title) = settled[0] else {
            panic!("first settled blue final visual should be title text");
        };
        assert_close(settled_title.x, 294.87);
        assert_close(settled_title.baseline_y, 130.16);
        assert_eq!(settled_title.font_size, 67);
        assert_close(settled_title.alpha, 1.0);
        let AnnounceVisual::Text(settled_match) = settled[1] else {
            panic!("second settled blue final visual should be Match text");
        };
        assert_eq!(settled_match.text, "Match");
        assert_close(settled_match.x, 300.51);
        assert_close(settled_match.baseline_y, 362.70);
        assert_eq!(settled_match.font_size, 103);
        let AnnounceVisual::Pip(left_pip) = settled[2] else {
            panic!("third settled blue final visual should be left match pip");
        };
        assert_close(left_pip.x, 157.30);
        assert_close(left_pip.y, 207.25);
        assert_close(left_pip.scale, 4.23);

        let fading = final_win_announce_visual(Side::Blue, 63);
        let AnnounceVisual::Text(fading_title) = fading[0] else {
            panic!("first fading blue final visual should be title text");
        };
        assert_close(fading_title.alpha, 249.0 / 256.0);

        assert!(final_win_announce_visual(Side::Blue, 24).is_empty());
    }

    #[test]
    fn red_final_announce_uses_swf_sprite_162_final_layout() {
        assert!(final_win_announce_visual(Side::Red, 157).is_empty());

        let growing = final_win_announce_visual(Side::Red, 156);
        assert_eq!(growing.len(), 5);
        let AnnounceVisual::Text(growing_title) = growing[0] else {
            panic!("first red final visual should be title text");
        };
        assert_eq!(growing_title.kind, AnnounceTextKind::RedWins);
        assert_eq!(growing_title.text, "Red Wins");
        assert_close(growing_title.x, 289.03);
        assert_close(growing_title.baseline_y, 121.01);
        assert_eq!(growing_title.font_size, 24);
        assert_close(growing_title.alpha, 5.0 / 256.0);

        let settled = final_win_announce_visual(Side::Red, 141);
        assert_eq!(settled.len(), 5);
        let AnnounceVisual::Text(settled_title) = settled[0] else {
            panic!("first settled red final visual should be title text");
        };
        assert_eq!(settled_title.kind, AnnounceTextKind::RedWins);
        assert_eq!(settled_title.text, "Red Wins");
        assert_close(settled_title.x, 295.02);
        assert_close(settled_title.baseline_y, 130.16);
        assert_eq!(settled_title.font_size, 67);
        assert_close(settled_title.alpha, 1.0);
        let AnnounceVisual::Pip(right_pip) = settled[4] else {
            panic!("fifth settled red final visual should be right match pip");
        };
        assert_close(right_pip.x, 398.20);
        assert_close(right_pip.y, 207.25);
        assert_close(right_pip.scale, 4.23);

        let fading = final_win_announce_visual(Side::Red, 58);
        let AnnounceVisual::Text(fading_title) = fading[0] else {
            panic!("first fading red final visual should be title text");
        };
        assert_close(fading_title.alpha, 249.0 / 256.0);

        assert!(final_win_announce_visual(Side::Red, 23).is_empty());
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    #[cfg(debug_assertions)]
    let debug_shot = debug_shot_from_env();
    let device_arial_font = load_device_arial_font();
    let device_serif_font = load_device_serif_font();
    let device_trebuchet_font = load_device_trebuchet_font();
    let assets = GameAssets::new(device_arial_font, device_serif_font, device_trebuchet_font);
    let mut game = Game::new(AudioBank::default(), assets);
    #[cfg(debug_assertions)]
    {
        apply_debug_warp_from_env(&mut game);
        if let Some(shot) = &debug_shot {
            capture_debug_shot(&mut game, shot).await;
            return;
        }
    }
    if game.screen == Screen::Startup {
        game.draw();
        next_frame().await;
    }
    game.audio = AudioBank::load().await;
    if game.screen == Screen::Startup {
        for _ in 0..2 {
            game.draw();
            next_frame().await;
        }
    }
    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
