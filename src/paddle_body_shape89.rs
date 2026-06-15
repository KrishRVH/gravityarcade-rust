// Generated from gravity_arcade.swf DefineShape 89 via reference/extract_shapes.py --contours.
// Coordinates are stored as SWF shape-local pixels.

use super::{PaddleBodyShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};

pub const FLATTEN_STEPS: u8 = 4;
#[cfg(test)]
pub const DEFINE_SHAPE_ID: u16 = 89;
pub const FILL_RGB: [u8; 3] = [255, 255, 153];
pub const LINE_RGB: [u8; 3] = [255, 255, 255];
pub const LINE_WIDTH: f32 = 1.0;

const SEGMENTS: [SwfPathSegment; 12] = [
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.4, -25.0),
        to: SwfPoint::new(0.05, -25.0),
    },
    SwfPathSegment::Line(SwfPoint::new(0.25, -25.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(0.7, -25.0),
        to: SwfPoint::new(1.05, -23.55),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(1.4, -22.05),
        to: SwfPoint::new(1.4, -20.0),
    },
    SwfPathSegment::Line(SwfPoint::new(1.4, 20.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(1.4, 22.05),
        to: SwfPoint::new(1.05, 23.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(0.7, 25.0),
        to: SwfPoint::new(0.25, 25.0),
    },
    SwfPathSegment::Line(SwfPoint::new(0.05, 25.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-0.4, 25.0),
        to: SwfPoint::new(-0.75, 23.5),
    },
    SwfPathSegment::Quad {
        control: SwfPoint::new(-1.1, 22.05),
        to: SwfPoint::new(-1.05, 20.0),
    },
    SwfPathSegment::Line(SwfPoint::new(-1.05, -20.0)),
    SwfPathSegment::Quad {
        control: SwfPoint::new(-1.1, -22.05),
        to: SwfPoint::new(-0.75, -23.55),
    },
];

pub const SHAPE: PaddleBodyShapeDefinition = PaddleBodyShapeDefinition {
    bounds: SwfBounds {
        x_min: -1.55,
        x_max: 1.9,
        y_min: -25.5,
        y_max: 25.5,
    },
    start: SwfPoint::new(-0.75, -23.55),
    segments: &SEGMENTS,
};
