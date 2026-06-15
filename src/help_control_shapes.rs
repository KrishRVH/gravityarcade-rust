// Generated from gravity_arcade.swf DefineShape 105 and 110 via reference/extract_shapes.py --contours.
// Coordinates are stored as SWF shape-local pixels.

use super::{
    HelpArrowShapeDefinition, HelpControlContour, HelpKeycapShapeDefinition, SwfBounds,
    SwfPathSegment, SwfPoint,
};

pub const FLATTEN_STEPS: u8 = 4;
#[cfg(test)]
pub const KEYCAP_DEFINE_SHAPE_ID: u16 = 105;
#[cfg(test)]
pub const KEYCAP_BOUNDS_TWIPS: [i16; 4] = [-312, 313, -312, 313];
pub const KEYCAP_FILL_RGB: [u8; 3] = [207, 192, 150];
pub const KEYCAP_SHADOW_RGB: [u8; 3] = [163, 150, 118];
pub const KEYCAP_LINE_RGB: [u8; 3] = [255, 238, 187];
pub const KEYCAP_LINE_WIDTH: f32 = 2.0;
#[cfg(test)]
pub const ARROW_DEFINE_SHAPE_ID: u16 = 110;
#[cfg(test)]
pub const ARROW_BOUNDS_TWIPS: [i16; 4] = [-205, 205, -212, 213];
pub const ARROW_FILL_RGB: [u8; 3] = [0, 36, 85];

const KEYCAP_FILL_SEGMENTS: [SwfPathSegment; 14] = [
    SwfPathSegment::Line(SwfPoint::new(10.75, -14.5)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.45, -14.3),
        to: SwfPoint::new(12.0, -13.75),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(12.9, -12.85),
        to: SwfPoint::new(12.9, -11.6),
    },
    SwfPathSegment::Line(SwfPoint::new(12.9, 7.4)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(12.9, 8.65),
        to: SwfPoint::new(12.0, 9.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.15, 10.4),
        to: SwfPoint::new(9.9, 10.4),
    },
    SwfPathSegment::Line(SwfPoint::new(-7.6, 10.4)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-8.85, 10.4),
        to: SwfPoint::new(-9.75, 9.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-10.6, 8.65),
        to: SwfPoint::new(-10.6, 7.4),
    },
    SwfPathSegment::Line(SwfPoint::new(-10.6, -11.6)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-10.6, -12.85),
        to: SwfPoint::new(-9.75, -13.75),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-8.9, -14.55),
        to: SwfPoint::new(-7.7, -14.6),
    },
    SwfPathSegment::Line(SwfPoint::new(9.65, -14.6)),
    SwfPathSegment::Line(SwfPoint::new(9.9, -14.6)),
];

const KEYCAP_SHADOW_SEGMENTS: [SwfPathSegment; 23] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-8.9, -14.55),
        to: SwfPoint::new(-9.75, -13.75),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-10.6, -12.85),
        to: SwfPoint::new(-10.6, -11.6),
    },
    SwfPathSegment::Line(SwfPoint::new(-10.6, 7.4)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-10.6, 8.65),
        to: SwfPoint::new(-9.75, 9.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-8.85, 10.4),
        to: SwfPoint::new(-7.6, 10.4),
    },
    SwfPathSegment::Line(SwfPoint::new(9.9, 10.4)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.15, 10.4),
        to: SwfPoint::new(12.0, 9.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(12.9, 8.65),
        to: SwfPoint::new(12.9, 7.4),
    },
    SwfPathSegment::Line(SwfPoint::new(12.9, -11.6)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(12.9, -12.85),
        to: SwfPoint::new(12.0, -13.75),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.45, -14.3),
        to: SwfPoint::new(10.75, -14.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(12.1, -14.2),
        to: SwfPoint::new(13.15, -13.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(14.65, -11.65),
        to: SwfPoint::new(14.65, -9.6),
    },
    SwfPathSegment::Line(SwfPoint::new(14.65, 9.65)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(14.65, 11.7),
        to: SwfPoint::new(13.15, 13.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.7, 14.65),
        to: SwfPoint::new(9.65, 14.65),
    },
    SwfPathSegment::Line(SwfPoint::new(-9.6, 14.65)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-11.65, 14.65),
        to: SwfPoint::new(-13.15, 13.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-14.6, 11.7),
        to: SwfPoint::new(-14.6, 9.65),
    },
    SwfPathSegment::Line(SwfPoint::new(-14.6, -9.6)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-14.6, -11.65),
        to: SwfPoint::new(-13.15, -13.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-11.65, -14.6),
        to: SwfPoint::new(-9.6, -14.6),
    },
    SwfPathSegment::Line(SwfPoint::new(-7.7, -14.6)),
];

const KEYCAP_TOP_STROKE_SEGMENTS: [SwfPathSegment; 3] = [
    SwfPathSegment::Line(SwfPoint::new(9.9, -14.6)),
    SwfPathSegment::Line(SwfPoint::new(9.65, -14.6)),
    SwfPathSegment::Line(SwfPoint::new(-7.7, -14.6)),
];

const KEYCAP_INNER_STROKE_SEGMENTS: [SwfPathSegment; 11] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-8.9, -14.55),
        to: SwfPoint::new(-9.75, -13.75),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-10.6, -12.85),
        to: SwfPoint::new(-10.6, -11.6),
    },
    SwfPathSegment::Line(SwfPoint::new(-10.6, 7.4)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-10.6, 8.65),
        to: SwfPoint::new(-9.75, 9.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-8.85, 10.4),
        to: SwfPoint::new(-7.6, 10.4),
    },
    SwfPathSegment::Line(SwfPoint::new(9.9, 10.4)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.15, 10.4),
        to: SwfPoint::new(12.0, 9.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(12.9, 8.65),
        to: SwfPoint::new(12.9, 7.4),
    },
    SwfPathSegment::Line(SwfPoint::new(12.9, -11.6)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(12.9, -12.85),
        to: SwfPoint::new(12.0, -13.75),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.45, -14.3),
        to: SwfPoint::new(10.75, -14.5),
    },
];

const KEYCAP_OUTER_STROKE_SEGMENTS: [SwfPathSegment; 12] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(12.1, -14.2),
        to: SwfPoint::new(13.15, -13.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(14.65, -11.65),
        to: SwfPoint::new(14.65, -9.6),
    },
    SwfPathSegment::Line(SwfPoint::new(14.65, 9.65)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(14.65, 11.7),
        to: SwfPoint::new(13.15, 13.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(11.7, 14.65),
        to: SwfPoint::new(9.65, 14.65),
    },
    SwfPathSegment::Line(SwfPoint::new(-9.6, 14.65)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-11.65, 14.65),
        to: SwfPoint::new(-13.15, 13.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-14.6, 11.7),
        to: SwfPoint::new(-14.6, 9.65),
    },
    SwfPathSegment::Line(SwfPoint::new(-14.6, -9.6)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-14.6, -11.65),
        to: SwfPoint::new(-13.15, -13.15),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-11.65, -14.6),
        to: SwfPoint::new(-9.6, -14.6),
    },
    SwfPathSegment::Line(SwfPoint::new(-7.7, -14.6)),
];

const ARROW_FILL_SEGMENTS: [SwfPathSegment; 11] = [
    SwfPathSegment::Line(SwfPoint::new(4.25, -0.85)),
    SwfPathSegment::Line(SwfPoint::new(4.25, 7.65)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(4.25, 8.9),
        to: SwfPoint::new(3.35, 9.75),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(2.5, 10.65),
        to: SwfPoint::new(1.25, 10.65),
    },
    SwfPathSegment::Line(SwfPoint::new(-1.25, 10.65)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-2.5, 10.65),
        to: SwfPoint::new(-3.4, 9.75),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-4.25, 8.9),
        to: SwfPoint::new(-4.25, 7.65),
    },
    SwfPathSegment::Line(SwfPoint::new(-4.25, -0.85)),
    SwfPathSegment::Line(SwfPoint::new(-10.25, -0.85)),
    SwfPathSegment::Line(SwfPoint::new(-0.25, -10.6)),
    SwfPathSegment::Line(SwfPoint::new(10.25, -0.85)),
];

pub const KEYCAP_SHAPE: HelpKeycapShapeDefinition = HelpKeycapShapeDefinition {
    bounds: SwfBounds {
        x_min: -15.6,
        x_max: 15.65,
        y_min: -15.6,
        y_max: 15.65,
    },
    fill: HelpControlContour {
        start: SwfPoint::new(9.9, -14.6),
        segments: &KEYCAP_FILL_SEGMENTS,
    },
    shadow: HelpControlContour {
        start: SwfPoint::new(-7.7, -14.6),
        segments: &KEYCAP_SHADOW_SEGMENTS,
    },
    top_stroke: HelpControlContour {
        start: SwfPoint::new(10.75, -14.5),
        segments: &KEYCAP_TOP_STROKE_SEGMENTS,
    },
    inner_stroke: HelpControlContour {
        start: SwfPoint::new(-7.7, -14.6),
        segments: &KEYCAP_INNER_STROKE_SEGMENTS,
    },
    outer_stroke: HelpControlContour {
        start: SwfPoint::new(10.75, -14.5),
        segments: &KEYCAP_OUTER_STROKE_SEGMENTS,
    },
};

pub const ARROW_SHAPE: HelpArrowShapeDefinition = HelpArrowShapeDefinition {
    bounds: SwfBounds {
        x_min: -10.25,
        x_max: 10.25,
        y_min: -10.6,
        y_max: 10.65,
    },
    fill: HelpControlContour {
        start: SwfPoint::new(10.25, -0.85),
        segments: &ARROW_FILL_SEGMENTS,
    },
};
