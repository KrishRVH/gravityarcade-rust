// Generated from gravity_arcade.swf placements and hit shapes.
// Placement/color-transform source: reference/swf_deep.json.

use super::{SwfBounds, SwfPoint};

pub const BALL_RED_ADD_RGB: [u8; 3] = [255, 191, 119];
pub const BALL_FIRE_ADD_RGB: [u8; 3] = [255, 250, 17];
pub const GRAVITY_PREVIEW_RED_RIGHT_ADD_RGB: [u8; 3] = [255, 191, 119];
pub const GRAVITY_PREVIEW_RED_LEFT_ADD_RGB: [u8; 3] = [255, 190, 115];
pub const STATIC_RIGHT_PADDLE_GLOW_ADD_RGB: [u8; 3] = [221, 236, 255];
pub const STATIC_LEFT_PADDLE_GLOW_ADD_RGB: [u8; 3] = [255, 239, 187];

pub const STATIC_PADDLE_GLOW_SCALE_X: f32 = 0.531_463_6;
pub const STATIC_PADDLE_GLOW_SCALE_Y: f32 = 0.752_914_4;

pub const SPONSOR_LOGO_ROOT_X: f32 = 508.65;
pub const SPONSOR_LOGO_ROOT_Y: f32 = 17.4;
pub const SPONSOR_LOGO_ROOT_SCALE: f32 = 0.262_146;
pub const SPONSOR_LOGO_BUTTON_TX: f32 = 3.2;
pub const SPONSOR_LOGO_BUTTON_TY: f32 = -3.45;

pub const NEOKOLOR_LINK_ROOT_X: f32 = 275.0;
pub const NEOKOLOR_LINK_ROOT_Y: f32 = 383.7;
pub const NEOKOLOR_LINK_BUTTON_TX: f32 = 0.0;
pub const NEOKOLOR_LINK_BUTTON_TY: f32 = -0.55;
pub const NEOKOLOR_LINK_BUTTON_SCALE_X: f32 = 1.234_741_2;
pub const NEOKOLOR_LINK_BUTTON_SCALE_Y: f32 = 1.0;

#[cfg(test)]
pub const SPONSOR_LOGO_BUTTON_HIT_DEFINE_SHAPE_ID: u16 = 30;
#[cfg(test)]
pub const NEOKOLOR_LINK_BUTTON_HIT_DEFINE_SHAPE_ID: u16 = 74;
#[cfg(test)]
pub const SPONSOR_HEADER_HIT_DEFINE_SHAPE_ID: u16 = 98;
#[cfg(test)]
pub const BACK_HEADER_HIT_DEFINE_SHAPE_ID: u16 = 123;

pub const SPONSOR_LOGO_BUTTON_HIT_BOUNDS: SwfBounds = SwfBounds {
    x_min: -128.2,
    x_max: 128.25,
    y_min: -63.8,
    y_max: 63.8,
};

pub const NEOKOLOR_LINK_BUTTON_HIT_BOUNDS: SwfBounds = SwfBounds {
    x_min: -106.5,
    x_max: 106.5,
    y_min: -7.5,
    y_max: 7.5,
};

pub const SPONSOR_HEADER_HIT: [SwfPoint; 4] = [
    SwfPoint::new(115.0, 17.95),
    SwfPoint::new(7.75, 16.5),
    SwfPoint::new(6.0, -2.9),
    SwfPoint::new(118.25, -3.0),
];

pub const BACK_HEADER_HIT: [SwfPoint; 4] = [
    SwfPoint::new(85.8, 14.5),
    SwfPoint::new(6.0, 15.5),
    SwfPoint::new(6.0, -2.0),
    SwfPoint::new(86.8, -2.0),
];
