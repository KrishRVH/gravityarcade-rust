// Generated from gravity_arcade.swf DefineShape 22 via reference/extract_shapes.py --contours.
// Coordinates are stored as SWF shape-local pixels.

use super::{ButtonShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};

pub const FLATTEN_STEPS: u8 = 4;
#[cfg(test)]
pub const DEFINE_SHAPE_ID: u16 = 22;
pub const FILL_RGB: [u8; 3] = [0, 102, 153];
pub const LINE_RGB: [u8; 3] = [255, 255, 255];
pub const LINE_WIDTH: f32 = 2.0;

const SEGMENTS: [SwfPathSegment; 12] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(62.55, -12.0),
        to: SwfPoint::new(64.0, -10.55),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(65.5, -9.05),
        to: SwfPoint::new(65.5, -7.0),
    },
    SwfPathSegment::Line(SwfPoint::new(65.5, 7.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(65.5, 9.05),
        to: SwfPoint::new(64.0, 10.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(62.55, 12.0),
        to: SwfPoint::new(60.5, 12.0),
    },
    SwfPathSegment::Line(SwfPoint::new(-60.45, 12.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-62.5, 12.0),
        to: SwfPoint::new(-64.0, 10.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-65.45, 9.05),
        to: SwfPoint::new(-65.45, 7.0),
    },
    SwfPathSegment::Line(SwfPoint::new(-65.45, -7.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-65.45, -9.05),
        to: SwfPoint::new(-64.0, -10.55),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-62.5, -12.0),
        to: SwfPoint::new(-60.45, -12.0),
    },
    SwfPathSegment::Line(SwfPoint::new(60.5, -12.0)),
];

pub const SHAPE: ButtonShapeDefinition = ButtonShapeDefinition {
    bounds: SwfBounds {
        x_min: -66.45,
        x_max: 66.5,
        y_min: -13.0,
        y_max: 13.0,
    },
    start: SwfPoint::new(60.5, -12.0),
    segments: &SEGMENTS,
};
